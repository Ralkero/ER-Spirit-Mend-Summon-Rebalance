// Included in runtime. Only the camera's team-relation call is intercepted.
// Native actor lifetime and serialized camera callback assumptions apply, as
// with the existing query adapter. No actor/team/camera state is written here.
static FRIENDLY_ORIGINAL: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static FRIENDLY_READY: AtomicBool = AtomicBool::new(false);
static FRIENDLY_PANIC: AtomicBool = AtomicBool::new(false);
static FRIENDLY_NATIVE: AtomicU64 = AtomicU64::new(0);
static FRIENDLY_REJECTED: AtomicU64 = AtomicU64::new(0);
static FRIENDLY_ADMITTED: AtomicU64 = AtomicU64::new(0);
static FRIENDLY_REPORT: Mutex<Option<Instant>> = Mutex::new(None);

fn initialize_friendly_lockon() {
    if !config().friendly_lockon {
        return;
    }
    match unsafe { install_friendly_lockon() } {
        Ok(()) => log(
            "Friendly Spirit lock-on installed: camera relation call RVA 7179d6; local living Ash actors only; native range, visibility, scoring and cycling retained",
        ),
        Err(e) => log(format!(
            "Friendly Spirit lock-on disabled: {e}; native targeting retained"
        )),
    }
}

unsafe fn install_friendly_lockon() -> Result<(), String> {
    let p = Program::current();
    let va = |rva| {
        p.rva_to_va(rva)
            .map(|v| v as usize)
            .map_err(|e| format!("friendly lock-on RVA {rva:x}: {e:?}"))
    };
    // Verified against WW 2.7.2.0. Context includes the candidate/viewer argument
    // assignments and the conditional rejection, not just an E8 instruction.
    let checks: &[(u32, &[u8])] = &[
        (
            0x7179d0,
            &[
                0x48, 0x8b, 0xd0, 0x49, 0x8b, 0xcf, 0xe8, 0xd5, 0x3e, 0xe0, 0xff, 0x85, 0xc0, 0x0f,
                0x84, 0x7d, 0x01, 0, 0,
            ],
        ),
        (
            0x51b8b0,
            &[
                0x40, 0x56, 0x48, 0x83, 0xec, 0x30, 0x48, 0x8b, 0xf2, 0x48, 0x85, 0xc9, 0x74, 0x71,
                0x48, 0x85, 0xd2, 0x74, 0x6c,
            ],
        ),
    ];
    for (rva, bytes) in checks {
        if unsafe { std::slice::from_raw_parts(va(*rva)? as *const u8, bytes.len()) } != *bytes {
            return Err(format!("camera instruction conflict at RVA {rva:x}"));
        }
    }
    let site = va(0x7179d6)?;
    let target = va(0x51b8b0)?;
    let expected = hooks::call_bytes(site, target).ok_or("original call range")?;
    let page = unsafe { hooks::near(site) }.ok_or("friendly relay allocation")?;
    let mut stub = vec![0xff, 0x25, 0, 0, 0, 0];
    stub.extend_from_slice(&(friendly_relation as *const () as usize).to_le_bytes());
    unsafe {
        std::ptr::copy_nonoverlapping(stub.as_ptr(), page as *mut u8, stub.len());
        hooks::seal_page(page)?;
    }
    let replacement = hooks::call_bytes(site, page).ok_or("friendly relay range")?;
    FRIENDLY_ORIGINAL.store(target, Ordering::Release);
    FRIENDLY_READY.store(true, Ordering::Release);
    let result =
        unsafe { hooks::commit_optional(&[(site, expected.to_vec(), replacement.to_vec())]) };
    if result.is_err() {
        FRIENDLY_READY.store(false, Ordering::Release);
    }
    result
}

unsafe extern "system" fn friendly_relation(
    viewer: *const ChrIns,
    candidate: *const ChrIns,
) -> i32 {
    let original = FRIENDLY_ORIGINAL.load(Ordering::Acquire);
    if original == 0 {
        return 0;
    }
    let call: unsafe extern "system" fn(*const ChrIns, *const ChrIns) -> i32 =
        unsafe { std::mem::transmute(original) };
    let native = unsafe { call(viewer, candidate) };
    let debug = config().debug;
    let _timer = timing::FRIENDLY_RELATION.start(debug);
    if native != 0 {
        if debug {
            FRIENDLY_NATIVE.fetch_add(1, Ordering::Relaxed);
        }
        return native;
    }
    if !FRIENDLY_READY.load(Ordering::Acquire) {
        return native;
    }
    let admitted = std::panic::catch_unwind(|| is_owned_living_target(viewer, candidate));
    match admitted {
        Ok(true) => {
            if debug {
                FRIENDLY_ADMITTED.fetch_add(1, Ordering::Relaxed);
            }
            1
        }
        Ok(false) => {
            if debug {
                FRIENDLY_REJECTED.fetch_add(1, Ordering::Relaxed);
            }
            native
        }
        Err(_) => {
            // Disable this feature only. Report from the task, never this hot callback.
            FRIENDLY_READY.store(false, Ordering::Release);
            FRIENDLY_PANIC.store(true, Ordering::Release);
            native
        }
    }
}

fn is_owned_living_target(viewer: *const ChrIns, candidate: *const ChrIns) -> bool {
    if viewer.is_null() || candidate.is_null() || viewer == candidate || !enabled() {
        return false;
    }
    let Ok(world) = (unsafe { WorldChrMan::instance() }) else {
        return false;
    };
    let Some(player) = world.main_player.as_ref() else {
        return false;
    };
    let pc = &player.chr_ins;
    if !std::ptr::eq(pc, viewer) || pc.chr_flags1c5.death_flag() {
        return false;
    }
    let manager = unsafe {
        std::ptr::addr_of!(world.summon_buddy_manager)
            .cast::<*const SummonBuddyManager>()
            .read()
    };
    if !pointer_shape(manager) {
        return false;
    }
    let m = unsafe { &*manager };
    if m.disappear_requested || m.groups.len() > 32 {
        return false;
    }
    for owner in m
        .groups
        .iter()
        .filter(|o| o.first == pc.event_entity_id as i32)
    {
        if owner.second.len() > crate::capacity::MAX_GROUP_MEMBERS {
            return false;
        }
        for g in owner.second.iter() {
            // Compare engine group identity before reading any candidate field.
            if g.chr_ins.as_ptr().cast_const() != candidate {
                continue;
            }
            if g.is_remote
                || g.disappear_requested
                || g.warp_requested
                || !m
                    .trigger_speffect_to_buddy_map
                    .iter()
                    .any(|(e, b)| *e > 0 && *b == g.buddy_param_id)
            {
                return false;
            }
            let Some(actor) = registered_spirit(world, candidate) else {
                return false;
            };
            if actor.chr_flags1c5.death_flag()
                || actor.field_ins_handle.is_empty()
                || !world
                    .chr_ins_by_handle(&actor.field_ins_handle)
                    .is_some_and(|resolved| std::ptr::eq(resolved, candidate))
            {
                return false;
            }
            let Some(data) =
                (unsafe { health_ptr(actor, Inspection::Query, &mut PageChecks::new()) })
            else {
                return false;
            };
            // Query validation uses typed owner backlinks, not per-frame OS probes.
            return unsafe { (*data).max_hp > 0 && (*data).hp > 0 && (*data).hp <= (*data).max_hp };
        }
    }
    false
}

fn report_friendly_lockon() {
    if FRIENDLY_PANIC.swap(false, Ordering::AcqRel) {
        log(
            "Friendly Spirit lock-on disabled after callback panic; native relation results retained",
        );
    }
    if !config().debug || !config().friendly_lockon {
        return;
    }
    let Ok(mut last) = FRIENDLY_REPORT.try_lock() else {
        return;
    };
    let now = Instant::now();
    if last.is_some_and(|t| now.duration_since(t) < Duration::from_secs(10)) {
        return;
    }
    *last = Some(now);
    drop(last);
    log(format!(
        "Friendly lock-on candidates: ready={} native_allowed={} owned_admitted={} rejected={} (calls, not unique actors; selection reported separately)",
        FRIENDLY_READY.load(Ordering::Acquire),
        FRIENDLY_NATIVE.swap(0, Ordering::Relaxed),
        FRIENDLY_ADMITTED.swap(0, Ordering::Relaxed),
        FRIENDLY_REJECTED.swap(0, Ordering::Relaxed)
    ));
}
