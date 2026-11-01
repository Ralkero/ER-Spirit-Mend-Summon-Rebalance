// Included into runtime. Opt-in accounting; no new native hook or actor writes.
struct DuplicateState {
    context: Option<(usize, usize, u64)>,
    last: Option<Instant>,
    report: Option<Instant>,
    book: crate::groups::Groups,
}
static DUPLICATE_GROUPS: Mutex<DuplicateState> = Mutex::new(DuplicateState {
    context: None,
    last: None,
    report: None,
    book: crate::groups::Groups::new(),
});

/// Registry metadata first; actor handles only through the existing live gate.
/// Inactive/temporarily-dead slots retain membership without dereferencing actors.
fn duplicate_observations(
    world: &WorldChrMan,
    m: &SummonBuddyManager,
    owner: i32,
) -> Option<Vec<crate::groups::Observation>> {
    let set = &world.summon_buddy_chr_set;
    if set.capacity != crate::capacity::expected_entries()
        || !pointer_shape(set.entries.as_ptr())
        || m.groups.len() > 32
    {
        return None;
    }
    let mapping: Vec<_> = m
        .trigger_speffect_to_buddy_map
        .iter()
        .map(|(a, b)| (*a, *b))
        .collect();
    let mut slots = HashMap::new();
    for index in crate::capacity::FIRST_SPIRIT_SLOT..set.capacity as usize {
        let ptr = unsafe { std::ptr::addr_of!((*set.entries.as_ptr().add(index)).chr_ins).read() };
        if let Some(ptr) = ptr
            && slots.insert(ptr.as_ptr() as usize, index as u16).is_some()
        {
            return None;
        }
    }
    let mut observations = Vec::new();
    for entry in m.groups.iter().filter(|e| e.first == owner) {
        if entry.second.len() > crate::capacity::MAX_GROUP_MEMBERS {
            return None;
        }
        for g in entry.second.iter() {
            if g.is_remote {
                return None;
            }
            let effect = mapping.iter().find(|(_, b)| *b == g.buddy_param_id)?.0;
            let (family, _) = policy::unique_family(&mapping, effect)?;
            let ptr = g.chr_ins.as_ptr();
            let slot = *slots.get(&(ptr as usize))?;
            let handle = registered_spirit(world, ptr).and_then(|actor| {
                (!actor.field_ins_handle.is_empty()
                    && world
                        .chr_ins_by_handle(&actor.field_ins_handle)
                        .is_some_and(|r| std::ptr::eq(r, ptr)))
                .then_some(handle_key(actor.field_ins_handle))
            });
            observations.push(crate::groups::Observation {
                slot,
                handle,
                family,
            });
        }
    }
    Some(observations)
}

fn refresh_duplicate_groups(force: bool) {
    if !config().duplicates {
        return;
    }
    let Ok(mut state) = DUPLICATE_GROUPS.try_lock() else {
        return;
    };
    let now = Instant::now();
    if !force
        && state
            .last
            .is_some_and(|t| now.duration_since(t) < Duration::from_millis(100))
    {
        return;
    }
    let _timer = timing::DUPLICATE_SAMPLE.start(config().debug);
    state.last = Some(now);
    state.book.invalidate();
    let current = (|| {
        if !enabled() {
            return None;
        }
        let world = unsafe { WorldChrMan::instance() }.ok()?;
        let pc = &world.main_player.as_ref()?.chr_ins;
        if pc.chr_flags1c5.death_flag() {
            return None;
        }
        let manager = unsafe {
            std::ptr::addr_of!(world.summon_buddy_manager)
                .cast::<*const SummonBuddyManager>()
                .read()
        };
        if !pointer_shape(manager) {
            return None;
        }
        Some((world, pc, manager))
    })();
    let Some((world, pc, manager)) = current else {
        state.context = None;
        state.book.reset();
        return;
    };
    let context = (
        world as *const _ as usize,
        manager as usize,
        handle_key(pc.field_ins_handle),
    );
    if state.context != Some(context) {
        state.book.reset();
        state.context = Some(context);
    }
    let m = unsafe { &*manager };
    let Some(observations) = duplicate_observations(world, m, pc.event_entity_id as i32) else {
        return;
    };
    // Dismissal is authoritative lifecycle evidence. Do not discard a reservation
    // just because no bodies have been observed yet or a fixed timeout elapsed.
    if m.disappear_requested {
        state.book.reset();
        return;
    }
    state
        .book
        .reconcile(&observations, m.request_summon_speffect_id != -1);
    if config().debug
        && state
            .report
            .is_none_or(|t| now.duration_since(t) >= Duration::from_secs(10))
    {
        state.report = Some(now);
        log(format!(
            "Duplicate group snapshot: total={} first16={:?} (id,family,members,pending); native_request={}",
            state.book.groups.len(),
            state
                .book
                .groups
                .iter()
                .take(16)
                .map(|g| (g.id, g.family, g.members.len(), g.is_pending()))
                .collect::<Vec<_>>(),
            m.request_summon_speffect_id
        ));
    }
}
fn duplicate_can_add(family: i32) -> bool {
    DUPLICATE_GROUPS
        .try_lock()
        .is_ok_and(|s| s.book.can_reserve(family, config().max_groups_per_ash))
}
fn duplicate_reserve(effect: i32) -> Option<u64> {
    refresh_duplicate_groups(true);
    let world = unsafe { WorldChrMan::instance() }.ok()?;
    let manager = unsafe {
        std::ptr::addr_of!(world.summon_buddy_manager)
            .cast::<*const SummonBuddyManager>()
            .read()
    };
    if !pointer_shape(manager) {
        return None;
    }
    let m = unsafe { &*manager };
    let mapping: Vec<_> = m
        .trigger_speffect_to_buddy_map
        .iter()
        .map(|(a, b)| (*a, *b))
        .collect();
    let (family, members) = policy::unique_family(&mapping, effect)?;
    let mut state = DUPLICATE_GROUPS.try_lock().ok()?;
    let id = state
        .book
        .reserve(family, members.len(), config().max_groups_per_ash)?;
    debug(format!(
        "Duplicate group reserved G{id} family={family} expected_members={}; native cost path",
        members.len()
    ));
    Some(id)
}
fn duplicate_cancel(id: u64) {
    if let Ok(mut s) = DUPLICATE_GROUPS.try_lock() {
        s.book.cancel(id);
    }
    debug(format!(
        "Duplicate group G{id}: native request rejected; reservation canceled"
    ));
}

fn duplicate_member_label(handle: FieldInsHandle) -> Option<(u64, usize)> {
    if !config().duplicates {
        return None;
    }
    DUPLICATE_GROUPS
        .try_lock()
        .ok()?
        .book
        .member_label(handle_key(handle))
}

fn duplicate_idle_native() -> bool {
    refresh_duplicate_groups(false);
    DUPLICATE_GROUPS.try_lock().is_ok_and(|s| s.book.is_idle())
}
