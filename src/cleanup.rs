// Narrow adaptation of upstream extras.rs::release_dead_groups.
// No HP reads from inactive actors, no persistent actor references, no query writes.
use eldenring::cs::{ChrCtrl, ChrCtrlModifier, SummonBuddyGroup};

static LAST_CLEANUP_SCAN: Mutex<Option<Instant>> = Mutex::new(None);
static LAST_CLEANUP_REPORT: Mutex<Option<(CleanupStats, Instant)>> = Mutex::new(None);
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
struct CleanupStats {
    candidates: usize,
    released: usize,
    unproven: usize,
    temporary: usize,
    native_excluded: usize,
    last_buddy: i32,
}

/// Return the exact inactive registry entry without touching actor storage.
fn inactive_entry(world: &WorldChrMan, chr: *const ChrIns) -> Option<usize> {
    let set = &world.summon_buddy_chr_set;
    if set.capacity != crate::capacity::ENTRIES
        || !pointer_shape(chr)
        || !pointer_shape(set.entries.as_ptr())
    {
        return None;
    }
    for i in 0..set.capacity as usize {
        let entry = unsafe { set.entries.as_ptr().add(i) };
        let ptr = unsafe { std::ptr::addr_of!((*entry).chr_ins).read() };
        if ptr.is_none_or(|p| p.as_ptr().cast_const() != chr) {
            continue;
        }
        let load = unsafe {
            std::ptr::addr_of!((*entry).chr_load_status)
                .cast::<u8>()
                .read()
        };
        let update = unsafe {
            std::ptr::addr_of!((*entry).chr_update_type)
                .cast::<u8>()
                .read()
        };
        return matches!((load, update), (2, 2) | (4, 2)).then_some(entry as usize);
    }
    None
}

/// Read the native summon-exclusion predicate's chain, with fresh OS page and
/// owner/backlink checks. No reference to inactive actors is handed to healing.
/// Pinned native RVA 3f4740 reads ChrIns+58 -> Ctrl+c8 -> Modifier+24 bit 0.
/// Buddy alive-check at 4b7ca5 uses this predicate for its owned entries.
unsafe fn exclusion_evidence(
    chr: *const ChrIns,
    entry: usize,
    pages: &mut PageChecks,
) -> Option<(bool, bool)> {
    if !pages.check(chr, false) {
        return None;
    }
    if unsafe { std::ptr::addr_of!((*chr).chr_set_entry).read().as_ptr() as usize } != entry {
        return None;
    }
    let ctrl = unsafe {
        std::ptr::addr_of!((*chr).chr_ctrl)
            .cast::<*const ChrCtrl>()
            .read()
    };
    if !pages.check(ctrl, false) || unsafe { (*ctrl).owner.as_ptr().cast_const() } != chr {
        return None;
    }
    let modifier = unsafe {
        std::ptr::addr_of!((*ctrl).modifier)
            .cast::<*const ChrCtrlModifier>()
            .read()
    };
    if !pages.check(modifier, false) || unsafe { (*modifier).owner.as_ptr().cast_const() } != ctrl {
        return None;
    }
    let flags = unsafe { modifier.cast::<u8>().add(0x24).cast::<u32>().read() };
    let temporary = unsafe { (*modifier).data.action_flags.temporary_death_state() };
    Some((flags & 1 != 0, temporary))
}

/// Game-task only, at most twice per second. Once marked, native cleanup owns
/// removal and subsequent scans skip the member. Unknown states fail closed.
fn release_inactive_groups() {
    if !config().multiple || !enabled() {
        return;
    }
    let now = Instant::now();
    let Ok(mut last) = LAST_CLEANUP_SCAN.try_lock() else {
        return;
    };
    if last.is_some_and(|t| now.duration_since(t) < Duration::from_millis(500)) {
        return;
    }
    *last = Some(now);
    drop(last);
    let _timer = timing::CLEANUP.start(config().debug);
    let Ok(world) = (unsafe { WorldChrMan::instance() }) else {
        return;
    };
    let Some(player) = world.main_player.as_ref() else {
        return;
    };
    if player.chr_ins.chr_flags1c5.death_flag() {
        return;
    }
    let manager = unsafe {
        std::ptr::addr_of!(world.summon_buddy_manager)
            .cast::<*mut SummonBuddyManager>()
            .read()
    };
    if !pointer_shape(manager) {
        return;
    }
    let m = unsafe { &*manager };
    if m.groups.len() > 32
        || m.disappear_requested
        || !m.player_has_alive_summon
        || !area_allowed(m)
        || m.request_summon_speffect_id != -1
    {
        return;
    }
    let mut pages = PageChecks::new();
    let mut stats = CleanupStats::default();
    // Raw group pointers exist only inside this synchronous game task.
    let candidates: Vec<*mut SummonBuddyGroup> = m
        .groups
        .iter()
        .filter(|entry| {
            entry.first == player.chr_ins.event_entity_id as i32
                && entry.second.len() <= crate::capacity::MAX_GROUP_MEMBERS
        })
        .flat_map(|entry| entry.second.iter())
        .filter(|g| {
            !g.is_remote
                && !g.warp_requested
                && !g.disappear_requested
                && inactive_entry(world, g.chr_ins.as_ptr()).is_some()
        })
        .map(|g| g as *const SummonBuddyGroup as *mut SummonBuddyGroup)
        .collect();
    for group in candidates {
        stats.candidates += 1;
        if !pages.check(group, true) {
            stats.unproven += 1;
            continue;
        }
        let g = unsafe { &*group };
        stats.last_buddy = g.buddy_param_id;
        if g.is_remote || g.warp_requested || g.disappear_requested {
            stats.unproven += 1;
            continue;
        }
        // Reject entries outside the manager's recognized Spirit Ash map.
        if !m
            .trigger_speffect_to_buddy_map
            .iter()
            .any(|(effect, buddy)| *effect > 0 && *buddy == g.buddy_param_id)
        {
            stats.unproven += 1;
            continue;
        }
        let chr = g.chr_ins.as_ptr();
        let Some(entry) = inactive_entry(world, chr) else {
            stats.unproven += 1;
            continue;
        };
        let Some((excluded, temporary)) = (unsafe { exclusion_evidence(chr, entry, &mut pages) })
        else {
            stats.unproven += 1;
            continue;
        };
        stats.native_excluded += usize::from(excluded);
        stats.temporary += usize::from(temporary);
        if !policy::release_inactive_member(excluded, temporary) {
            stats.unproven += 1;
            continue;
        }
        // Same game callback, validated current native owner and registry links.
        // Only removal metadata changes; no actor/HP/AI or other group's writes.
        unsafe {
            (*group).disappear_delay_sec = 2.0;
            (*group).disappear_requested = true;
        }
        stats.released += 1;
    }
    if !config().debug && stats.released == 0 {
        return;
    }
    if stats.candidates == 0 {
        return;
    }
    if let Ok(mut last) = LAST_CLEANUP_REPORT.try_lock() {
        if last.as_ref().is_none_or(|(old, time)| {
            *old != stats || now.duration_since(*time) >= Duration::from_secs(10)
        }) {
            log(format!(
                "Inactive group cleanup: {stats:?}; native exclusion and temporary-death guards; no health access"
            ));
            *last = Some((stats, now));
        }
    }
}

// Compile-time public layout checks supplement executable signatures.
const _: () = {
    assert!(std::mem::offset_of!(ChrIns, chr_ctrl) == 0x58);
    assert!(std::mem::offset_of!(ChrCtrl, modifier) == 0xc8);
    assert!(std::mem::offset_of!(ChrCtrlModifier, data) == 8);
    assert!(std::mem::offset_of!(eldenring::cs::ChrCtrlModifierData, action_flags) == 0x10);
};
