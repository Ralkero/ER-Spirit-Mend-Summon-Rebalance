// Multi-type policy and narrowly authorized native spawn handoff.
// Included into runtime so it reuses the existing ownership/page validation.
#[derive(Clone, Copy)]
struct PendingSpawn {
    world: usize,
    manager: usize,
    owner: i32,
    goods: i32,
    effect: i32,
}
static PENDING_SPAWN: Mutex<Option<PendingSpawn>> = Mutex::new(None);

fn pending_spawn_cost(manager: *mut SummonBuddyManager, goods: i32) -> bool {
    let Ok(world) = (unsafe { WorldChrMan::instance() }) else {
        return false;
    };
    let expected = unsafe {
        std::ptr::addr_of!(world.summon_buddy_manager)
            .cast::<usize>()
            .read()
    };
    if !pointer_shape(manager) || expected != manager as usize {
        return false;
    }
    let Some(player) = world.main_player.as_ref() else {
        return false;
    };
    if player.chr_ins.chr_flags1c5.death_flag() {
        return false;
    }
    let m = unsafe { &*manager };
    let goods = if goods >= 0 {
        goods
    } else {
        let Some(item) = player
            .chr_ins
            .tae_queued_use_item
            .as_valid()
            .filter(|i| i.category() == ItemCategory::Goods)
        else {
            return false;
        };
        item.param_id() as i32
    };
    let Ok(pending) = PENDING_SPAWN.try_lock() else {
        return false;
    };
    pending.is_some_and(|token| {
        token.world == world as *const _ as usize
            && token.manager == manager as usize
            && token.owner == player.chr_ins.event_entity_id as i32
            && token.goods == goods
            && token.goods == m.requested_summon_goods_id
            && token.effect == m.request_summon_speffect_id
            && !m.disappear_requested
    })
}
thread_local! {
    // Synchronous native call only; never survives the call or crosses threads.
    static ADD_COMMIT_MANAGER: Cell<usize> = const { Cell::new(0) };
}
struct AddCommitScope;
impl Drop for AddCommitScope {
    fn drop(&mut self) {
        ADD_COMMIT_MANAGER.with(|v| v.set(0));
    }
}

fn refresh_pending_spawn() {
    let Ok(mut pending) = PENDING_SPAWN.try_lock() else {
        return;
    };
    let Some(token) = *pending else {
        return;
    };
    let valid = unsafe { WorldChrMan::instance() }
        .ok()
        .is_some_and(|world| {
            if world as *const _ as usize != token.world {
                return false;
            }
            let Some(player) = world.main_player.as_ref() else {
                return false;
            };
            let manager = unsafe {
                std::ptr::addr_of!(world.summon_buddy_manager)
                    .cast::<*const SummonBuddyManager>()
                    .read()
            };
            if manager as usize != token.manager || !pointer_shape(manager) {
                return false;
            }
            let m = unsafe { &*manager };
            player.chr_ins.event_entity_id as i32 == token.owner
                && !player.chr_ins.chr_flags1c5.death_flag()
                && m.request_summon_speffect_id == token.effect
                && m.requested_summon_goods_id == token.goods
        });
    if !valid {
        *pending = None;
    }
}

unsafe fn snapshot(
    manager: *mut SummonBuddyManager,
    goods: i32,
    native: i32,
    inspection: Inspection,
) -> Option<Snapshot> {
    if config().multiple {
        unsafe { snapshot_multi(manager, goods, native, inspection) }
    } else {
        unsafe { snapshot_single(manager, goods, native, inspection) }
    }
}

unsafe fn snapshot_multi(
    manager: *mut SummonBuddyManager,
    goods: i32,
    native: i32,
    inspection: Inspection,
) -> Option<Snapshot> {
    let _timer = timing::SNAPSHOT.start(config().debug);
    if !pointer_shape(manager) {
        return None;
    }
    let world = unsafe { WorldChrMan::instance() }.ok()?;
    let expected = unsafe {
        std::ptr::addr_of!(world.summon_buddy_manager)
            .cast::<usize>()
            .read()
    };
    if expected != manager as usize {
        return None;
    }
    if world.summon_buddy_chr_set.capacity != crate::capacity::ENTRIES {
        return None;
    }
    let pc = &world.main_player.as_ref()?.chr_ins;
    let m = unsafe { &*manager };
    if m.groups.len() > 32 {
        return None;
    }
    let local_exists = m
        .groups
        .iter()
        .any(|entry| entry.first == pc.event_entity_id as i32 && !entry.second.is_empty());
    if native != 2 && !m.player_has_alive_summon && !local_exists {
        let _idle = timing::SNAPSHOT_IDLE.start(config().debug);
        return Some(Snapshot {
            action: Action::Vanilla,
            goods,
            targets: Vec::new(),
            retiring_slots: 0,
        });
    }
    let _full = timing::SNAPSHOT_FULL.start(config().debug);
    let mut pages = PageChecks::new();
    if !inspect_pointer(manager, inspection, &mut pages) {
        return None;
    }
    let pdata = unsafe { health_ptr(pc, inspection, &mut pages) }?;
    if unsafe { (*pdata).hp } <= 0 || pc.chr_flags1c5.death_flag() {
        return None;
    }
    let goods = if goods >= 0 {
        goods
    } else {
        pc.tae_queued_use_item
            .as_valid()
            .filter(|i| i.category() == ItemCategory::Goods)?
            .param_id() as i32
    };
    let repo = unsafe { SoloParamRepository::instance() }.ok()?;
    if !param_ready::<EquipParamGoods>(repo) {
        return None;
    }
    let item = repo.get::<EquipParamGoods>(goods as u32)?;
    if item.ref_category() != 2 {
        return None;
    }
    let mapping: Vec<_> = m
        .trigger_speffect_to_buddy_map
        .iter()
        .map(|(a, b)| (*a, *b))
        .collect();
    let (_, family) = policy::unique_family(&mapping, item.ref_id_default())?;
    let mut targets = Vec::new();
    let mut seen = HashSet::new();
    let mut retired = 0;
    let mut unresolved_death = false;
    for entry in m.groups.iter() {
        if entry.first != pc.event_entity_id as i32 {
            continue;
        }
        if entry.second.len() > crate::capacity::MAX_GROUP_MEMBERS {
            return None;
        }
        for group in entry.second.iter() {
            // Unrelated groups are never heal targets. Do not read their actors
            // during high-frequency UI queries or let their HP affect the heal.
            if !family.contains(&group.buddy_param_id) {
                continue;
            }
            if group.is_remote || group.warp_requested {
                return None;
            }
            let chr = group.chr_ins.as_ptr();
            if group.disappear_requested && retiring_registry_member(world, chr) {
                retired += 1;
                continue;
            }
            let actor = registered_spirit(world, chr)?;
            if !world
                .chr_ins_by_handle(&actor.field_ins_handle)
                .is_some_and(|r| std::ptr::eq(r, chr))
            {
                return None;
            }
            let data = unsafe { health_ptr(actor, inspection, &mut pages) }?;
            let health = unsafe {
                Health {
                    hp: (*data).hp,
                    max: (*data).max_hp,
                }
            };
            if health.hp <= 0 || actor.chr_flags1c5.death_flag() {
                // HP depletion alone does not prove permanent death (skeletons).
                // A fresh group requires native retirement evidence or removal.
                unresolved_death = true;
                continue;
            }
            if group.disappear_requested || health.max <= 0 || health.hp > health.max {
                return None;
            }
            if seen.insert(chr as usize) {
                targets.push((data, health));
                let _validated = timing::TARGET_VALIDATED.start(config().debug);
            }
        }
    }
    // Native goods-specific state -1 also means "different from last Ash".
    // Query the native active-group state to retain area/cooldown/lifecycle
    // gates, then independently enforce the selected Ash's family ownership.
    // No manager flags or active-item fields are modified for this query.
    let active_state = unsafe { native_state(manager, -1) };
    let eligible = !m.disappear_requested
        && m.request_summon_speffect_id == -1
        && m.item_use_cooldown_timer.is_finite()
        && m.item_use_cooldown_timer <= 0.0
        && (area_allowed(m) || (!m.player_has_alive_summon && matches!(active_state, 0 | 1)));
    let wounded = targets
        .iter()
        .any(|(_, h)| policy::healed(*h, config().percent).is_some_and(|new| new > h.hp));
    let mut action = policy::multi_action(
        active_state,
        !targets.is_empty(),
        eligible,
        wounded,
        config().heal,
    );
    if action == Action::AddGroup {
        if unresolved_death {
            return None;
        }
        // Allocation remains native. Reject before committing if the whole new
        // group (including mounts) cannot fit. Never evict existing groups.
        if !param_ready::<BuddyParam>(repo) {
            return None;
        }
        let required = family.iter().try_fold(0usize, |count, id| {
            let buddy = repo.get::<BuddyParam>(*id as u32)?;
            count.checked_add(if buddy.npc_param_id_ridden() > 0 {
                2
            } else {
                1
            })
        })?;
        let set = &world.summon_buddy_chr_set;
        if !pointer_shape(set.entries.as_ptr()) {
            return None;
        }
        let free = (crate::capacity::FIRST_SPIRIT_SLOT..set.capacity as usize)
            .filter(|index| unsafe {
                std::ptr::addr_of!((*set.entries.as_ptr().add(*index)).chr_ins)
                    .read()
                    .is_none()
            })
            .count();
        if required == 0 || free < required {
            action = Action::Reject;
        }
    }
    let _resolved = timing::SNAPSHOT_RESOLVED.start(config().debug);
    Some(Snapshot {
        action,
        goods,
        targets,
        retiring_slots: retired,
    })
}

unsafe fn add_group(
    manager: *mut SummonBuddyManager,
    effect: i32,
    goods: i32,
    original: unsafe extern "system" fn(*mut SummonBuddyManager, i32) -> bool,
) -> bool {
    let Ok(world) = (unsafe { WorldChrMan::instance() }) else {
        return false;
    };
    let Some(player) = world.main_player.as_ref() else {
        return false;
    };
    let Ok(repo) = (unsafe { SoloParamRepository::instance() }) else {
        return false;
    };
    if !param_ready::<EquipParamGoods>(repo)
        || !repo
            .get::<EquipParamGoods>(goods as u32)
            .is_some_and(|r| r.ref_id_default() == effect)
    {
        return false;
    }
    let m = unsafe { &*manager };
    if m.requested_summon_goods_id != goods
        || m.request_summon_speffect_id != -1
        || m.disappear_requested
    {
        return false;
    }
    let token = PendingSpawn {
        world: world as *const _ as usize,
        manager: manager as usize,
        owner: player.chr_ins.event_entity_id as i32,
        goods,
        effect,
    };
    // The snapshot already validated the selected family and eligibility in
    // this callback. No async input queue or forced request is created here.
    refresh_pending_spawn();
    if ADD_COMMIT_MANAGER.with(|v| v.get()) != 0 {
        return false;
    }
    let Ok(mut pending) = PENDING_SPAWN.lock() else {
        return false;
    };
    if pending.is_some() {
        return false;
    }
    *pending = Some(token);
    drop(pending);
    ADD_COMMIT_MANAGER.with(|v| v.set(manager as usize));
    let scope = AddCommitScope;
    sync_anywhere_origin(manager);
    let result = unsafe { original(manager, effect) };
    drop(scope);
    if !result {
        if let Ok(mut p) = PENDING_SPAWN.lock() {
            *p = None;
        }
    }
    debug(format!(
        "Add-group native request goods={goods} effect={effect} result={result}; native cost path; existing groups retained only for this request"
    ));
    result
}

/// Only the native state call INSIDE DoSummon is redirected, and only while a
/// validated AddGroup call is synchronously on this thread. Never maps -1 to 0.
pub unsafe extern "system" fn inner_summon_state(
    manager: *mut SummonBuddyManager,
    goods: i32,
) -> i32 {
    let native = unsafe { native_state(manager, goods) };
    if enabled()
        && ADD_COMMIT_MANAGER.with(|v| v.get() == manager as usize)
        && matches!(native, 0..=2)
    {
        0
    } else {
        native
    }
}

/// This call site is inside the pending-spawn branch, not global cleanup.
/// Preserve the owner's entries only for the exact request accepted above.
/// No group disappear flags or timers are cleared: native area/death exits stay.
pub unsafe extern "system" fn clear_for_spawn(manager: *mut SummonBuddyManager, owner: i32) {
    let preserve = guarded("additional summon clear guard", false, || {
        if !enabled() || !pointer_shape(manager) {
            return false;
        }
        let Ok(world) = (unsafe { WorldChrMan::instance() }) else {
            return false;
        };
        let expected = unsafe {
            std::ptr::addr_of!(world.summon_buddy_manager)
                .cast::<usize>()
                .read()
        };
        if expected != manager as usize {
            return false;
        }
        let Some(player) = world.main_player.as_ref() else {
            return false;
        };
        let pc = &player.chr_ins;
        if pc.event_entity_id as i32 != owner || pc.chr_flags1c5.death_flag() {
            return false;
        }
        let m = unsafe { &*manager };
        if m.disappear_requested || !area_allowed(m) || m.groups.len() > 32 {
            return false;
        }
        let Ok(pending) = PENDING_SPAWN.try_lock() else {
            return false;
        };
        let Some(token) = *pending else {
            return false;
        };
        let valid = token.world == world as *const _ as usize
            && token.manager == manager as usize
            && token.owner == owner
            && token.goods == m.requested_summon_goods_id
            && token.effect == m.request_summon_speffect_id;
        if valid {
            debug(format!(
                "Preserved owner groups for additional summon goods={} effect={} owner={owner}",
                token.goods, token.effect
            ));
        }
        valid
    });
    if !preserve {
        let f: unsafe extern "system" fn(*mut SummonBuddyManager, i32) =
            unsafe { std::mem::transmute(hooks::CLEAR.load(Ordering::Acquire)) };
        unsafe {
            f(manager, owner);
        }
    }
}
