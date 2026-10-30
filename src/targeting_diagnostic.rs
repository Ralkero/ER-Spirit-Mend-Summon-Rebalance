// Read-only selected-target telemetry. Included in runtime to share the current
// world's registered-Spirit validation; never hooks or writes camera state.
use eldenring::cs::{FieldInsHandle, PlayerIns};
use pelite::pe64::Pe;

static TARGET_DIAGNOSTIC_READY: AtomicBool = AtomicBool::new(false);
static TARGET_DIAGNOSTIC_STATE: Mutex<TargetDiagnosticState> = Mutex::new(TargetDiagnosticState {
    sampled: None,
    reported: None,
    last: None,
});
struct TargetDiagnosticState {
    sampled: Option<Instant>,
    reported: Option<Instant>,
    last: Option<TargetDiagnosticSnapshot>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct TargetSpirit {
    handle: FieldInsHandle,
    buddy: i32,
    npc: i32,
    team: u8,
    death_flag: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct TargetDiagnosticSnapshot {
    // Identity scalars only; never dereferenced after this callback.
    world: usize,
    player: usize,
    owner: u32,
    locked: bool,
    selected: FieldInsHandle,
    spirits: Vec<TargetSpirit>,
    skipped: usize,
}

fn initialize_target_diagnostic() {
    if !config().debug || !config().target_diagnostics {
        return;
    }
    let result = (|| -> Result<(), String> {
        if std::mem::offset_of!(PlayerIns, locked_on_enemy) != 0x6b0
            || std::mem::offset_of!(ChrIns, is_locked_on) != 0xc9
            || std::mem::size_of::<FieldInsHandle>() != 8
        {
            return Err("typed target-field layout mismatch".into());
        }
        let program = Program::current();
        // Native camera update clears the player handle, then copies the
        // selected actor's FieldInsHandle (+8) to PlayerIns+6b0. Its toggle
        // operation writes ChrIns+0xc9. Check exact instructions before reading.
        let checks: &[(u32, &[u8])] = &[
            (
                0x71851a,
                &[0x49, 0xc7, 0x85, 0xb0, 0x06, 0, 0, 0xff, 0xff, 0xff, 0xff],
            ),
            (
                0x71857e,
                &[0x48, 0x8b, 0x48, 0x08, 0x49, 0x89, 0x8d, 0xb0, 0x06, 0, 0],
            ),
            (0x3f74e0, &[0x88, 0x91, 0xc9, 0, 0, 0, 0x41, 0xb1, 0x01]),
        ];
        for (rva, bytes) in checks {
            let address = program
                .rva_to_va(*rva)
                .map_err(|e| format!("target RVA {rva:x}: {e:?}"))?;
            if unsafe { std::slice::from_raw_parts(address as *const u8, bytes.len()) } != *bytes {
                return Err(format!("target-field instruction mismatch at RVA {rva:x}"));
            }
        }
        Ok(())
    })();
    match result {
        Ok(()) => {
            TARGET_DIAGNOSTIC_READY.store(true, Ordering::Release);
            log(
                "Target diagnostic ready: typed PlayerIns.locked_on_enemy +6b0 and ChrIns.is_locked_on +c9; instruction checks passed; read-only, 5Hz; observer does not change target eligibility",
            );
        }
        Err(e) => log(format!(
            "Target diagnostic disabled: {e}; gameplay retained"
        )),
    }
}

fn target_diagnostic_snapshot() -> Option<TargetDiagnosticSnapshot> {
    let world = unsafe { WorldChrMan::instance() }.ok()?;
    let player = world.main_player.as_ref()?;
    let pc = &player.chr_ins;
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
    let m = unsafe { &*manager };
    if m.groups.len() > 32 {
        return None;
    }
    let mut spirits = Vec::new();
    let mut skipped = 0;
    for owner in m
        .groups
        .iter()
        .filter(|o| o.first == pc.event_entity_id as i32)
    {
        if owner.second.len() > crate::capacity::MAX_GROUP_MEMBERS {
            return None;
        }
        for g in owner.second.iter() {
            if g.is_remote
                || g.disappear_requested
                || g.warp_requested
                || !m
                    .trigger_speffect_to_buddy_map
                    .iter()
                    .any(|(e, b)| *e > 0 && *b == g.buddy_param_id)
            {
                skipped += 1;
                continue;
            }
            let pointer = g.chr_ins.as_ptr();
            let Some(actor) = registered_spirit(world, pointer) else {
                skipped += 1;
                continue;
            };
            if !world
                .chr_ins_by_handle(&actor.field_ins_handle)
                .is_some_and(|resolved| std::ptr::eq(resolved, pointer))
            {
                skipped += 1;
                continue;
            }
            if actor.field_ins_handle.is_empty() {
                skipped += 1;
                continue;
            }
            let sample = TargetSpirit {
                handle: actor.field_ins_handle,
                buddy: g.buddy_param_id,
                npc: actor.npc_param_id,
                team: actor.team_type,
                death_flag: actor.chr_flags1c5.death_flag(),
            };
            if !spirits
                .iter()
                .any(|s: &TargetSpirit| s.handle == sample.handle)
            {
                spirits.push(sample);
            }
        }
    }
    spirits.sort_by_key(|s| (s.handle.block_id.0, s.handle.selector.0));
    Some(TargetDiagnosticSnapshot {
        world: world as *const _ as usize,
        player: player as *const _ as usize,
        owner: pc.event_entity_id,
        locked: pc.is_locked_on,
        selected: player.locked_on_enemy,
        spirits,
        skipped,
    })
}

fn sample_target_diagnostic() {
    if !TARGET_DIAGNOSTIC_READY.load(Ordering::Acquire) {
        return;
    }
    let now = Instant::now();
    let Ok(mut state) = TARGET_DIAGNOSTIC_STATE.try_lock() else {
        return;
    };
    if state
        .sampled
        .is_some_and(|last| now.duration_since(last) < Duration::from_millis(200))
    {
        return;
    }
    state.sampled = Some(now);
    let _timer = timing::TARGET_SAMPLE.start(true);
    // Diagnostic faults disable only this observer, never alter target policy.
    let result = std::panic::catch_unwind(|| {
        if enabled() {
            target_diagnostic_snapshot()
        } else {
            None
        }
    });
    let snapshot = match result {
        Ok(s) => s,
        Err(_) => {
            TARGET_DIAGNOSTIC_READY.store(false, Ordering::Release);
            state.last = None;
            drop(state);
            log("Target diagnostic disabled: observer panic; no camera writes or hooks applied");
            return;
        }
    };
    if state.last == snapshot
        && state
            .reported
            .is_some_and(|t| now.duration_since(t) < Duration::from_secs(10))
    {
        return;
    }
    state.reported = Some(now);
    let message = if let Some(s) = &snapshot {
        let owned = s.spirits.iter().find(|v| v.handle == s.selected);
        let class = if !s.locked {
            "not_locked"
        } else if s.selected.is_empty() {
            "locked_without_handle"
        } else if owned.is_some() {
            "owned_spirit"
        } else {
            "other_or_unresolved"
        };
        // Do not dereference an arbitrary selected NPC/enemy: this observer
        // compares scalar handles only. It is not authorization for healing.
        let list: Vec<_> = s
            .spirits
            .iter()
            .take(16)
            .map(|v| {
                format!(
                    "{:08x}:{:08x}/buddy={}/npc={}/team={}/death={}",
                    v.handle.selector.0,
                    v.handle.block_id.0 as u32,
                    v.buddy,
                    v.npc,
                    v.team,
                    v.death_flag
                )
            })
            .collect();
        format!(
            "Lock-on sample: owner={} locked={} selected={:08x}:{:08x} class={} owned_buddy={:?} owned_registered={} skipped={} spirits_first16={:?}",
            s.owner,
            s.locked,
            s.selected.selector.0,
            s.selected.block_id.0 as u32,
            class,
            owned.map(|v| v.buddy),
            s.spirits.len(),
            s.skipped,
            list
        )
    } else {
        "Lock-on sample: unavailable (no live local player or adapter gate closed); identity reset"
            .into()
    };
    state.last = snapshot;
    drop(state);
    log(message);
}
