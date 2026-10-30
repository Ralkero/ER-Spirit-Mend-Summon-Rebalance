// Only scalar snapshots leave this game-task collector. No actor pointers persist.
static LAST_HUD_SAMPLE: Mutex<Option<Instant>> = Mutex::new(None);
fn refresh_hud() {
    if !config().hud || !crate::hud::INSTALLED.load(Ordering::Acquire) {
        return;
    }
    if !enabled() {
        crate::hud::clear();
        return;
    }
    let now = Instant::now();
    let Ok(mut last) = LAST_HUD_SAMPLE.try_lock() else {
        return;
    };
    if last.is_some_and(|t| now.duration_since(t) < Duration::from_millis(100)) {
        return;
    }
    *last = Some(now);
    drop(last);
    let _timer = timing::HUD_SAMPLE.start(config().debug);
    if std::panic::catch_unwind(collect_hud).is_err() {
        crate::hud::clear();
    }
}
fn collect_hud() {
    let Ok(world) = (unsafe { WorldChrMan::instance() }) else {
        crate::hud::clear();
        return;
    };
    let Some(player) = world.main_player.as_ref() else {
        crate::hud::clear();
        return;
    };
    let pc = &player.chr_ins;
    if pc.chr_flags1c5.death_flag() {
        crate::hud::clear();
        return;
    }
    let manager = unsafe {
        std::ptr::addr_of!(world.summon_buddy_manager)
            .cast::<*const SummonBuddyManager>()
            .read()
    };
    if !pointer_shape(manager) {
        crate::hud::clear();
        return;
    }
    let m = unsafe { &*manager };
    if m.groups.len() > 32 || m.disappear_requested {
        crate::hud::clear();
        return;
    }
    let mapping: Vec<_> = m
        .trigger_speffect_to_buddy_map
        .iter()
        .map(|(a, b)| (*a, *b))
        .collect();
    let mut members: Vec<crate::hud::Member> = Vec::new();
    let mut seen = HashSet::new();
    let mut pages = PageChecks::new();
    for owner in m
        .groups
        .iter()
        .filter(|o| o.first == pc.event_entity_id as i32)
    {
        if owner.second.len() > crate::capacity::MAX_GROUP_MEMBERS {
            crate::hud::clear();
            return;
        }
        for g in owner.second.iter() {
            if g.is_remote {
                continue;
            }
            let Some(&(effect, _)) = mapping.iter().find(|(_, b)| *b == g.buddy_param_id) else {
                continue;
            };
            let Some((base, _)) = policy::unique_family(&mapping, effect) else {
                continue;
            };
            let ptr = g.chr_ins.as_ptr();
            if g.disappear_requested && retiring_registry_member(world, ptr) {
                continue;
            }
            if g.warp_requested || g.disappear_requested {
                continue;
            }
            let Some(actor) = registered_spirit(world, ptr) else {
                continue;
            };
            if !world
                .chr_ins_by_handle(&actor.field_ins_handle)
                .is_some_and(|r| std::ptr::eq(r, ptr))
            {
                continue;
            }
            let Some(data) = (unsafe { health_ptr(actor, Inspection::Query, &mut pages) }) else {
                continue;
            };
            let (hp, max) = unsafe { ((*data).hp, (*data).max_hp) };
            if hp <= 0 || actor.chr_flags1c5.death_flag() {
                continue;
            }
            if max <= 0 || hp > max {
                continue;
            }
            let key = crate::hud::MemberKey {
                base,
                handle: actor.field_ins_handle,
            };
            if !key.handle.is_empty() && seen.insert(key) {
                members.push(crate::hud::Member {
                    key,
                    hp: hp as u32,
                    max: max as u32,
                });
            }
        }
    }
    let repo = unsafe { SoloParamRepository::instance() }.ok();
    crate::hud::publish(
        (world as *const _ as usize, pc.event_entity_id),
        members,
        |base| {
            // Resolve a real Goods row whose trigger belongs to this family; do not
            // assume effect ID equals item ID for naming.
            if let Some(r) = repo.filter(|r| param_ready::<EquipParamGoods>(r)) {
                if let Some((id, _)) = r
                    .rows::<EquipParamGoods>()
                    .find(|(_, row)| row.ref_id_default() == base)
                {
                    if let Some(name) = crate::hud::goods_name(id as i32) {
                        return name;
                    }
                }
            }
            format!("Spirit Ash {base}")
        },
    );
}
