// Included in runtime. Native projectile release/preflight and impact dispatch.
include!("mend_cast.rs");
const MEND_MARKER: i32 = 99000100;
const MEND_HEAL: i32 = 99000101;
static MEND_ORIGINAL: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static MEND_READY: AtomicBool = AtomicBool::new(false);

type MendEffectFn = unsafe extern "system" fn(
    *const ChrIns,
    i32,
    *const ChrIns,
    u8,
    usize,
    usize,
    u8,
    u8,
    u8,
) -> u8;

fn initialize_spirit_mend() {
    if !config().spirit_mend {
        return;
    }
    let result = (|| -> Result<(), String> {
        let program = Program::current();
        let va = |rva| {
            program
                .rva_to_va(rva)
                .map(|v| v as usize)
                .map_err(|e| format!("Mend RVA {rva:x}: {e:?}"))
        };
        let target = va(0x3e8e70)?;
        let signature = [
            0x48, 0x8b, 0xc4, 0x57, 0x41, 0x54, 0x41, 0x55, 0x41, 0x56, 0x41, 0x57, 0x48, 0x83,
            0xec, 0x50,
        ];
        if unsafe { std::slice::from_raw_parts(target as *const u8, signature.len()) } != signature
        {
            return Err("native effect target signature mismatch".into());
        }
        let cast = va(0x3c0f80)?;
        let build = va(0x3c10c0)?;
        let hit = va(0x4489f0)?;
        for (rva, bytes) in [
            (
                0x3c0f80,
                &[0x40, 0x53, 0x56, 0x57, 0x48, 0x81, 0xec, 0x60, 0x01, 0, 0][..],
            ),
            (
                0x3c10c0,
                &[0x48, 0x89, 0x5c, 0x24, 0x08, 0x57, 0x48, 0x83, 0xec, 0x70][..],
            ),
            (
                0x4489f0,
                &[
                    0x4c, 0x8b, 0xdc, 0x55, 0x53, 0x56, 0x57, 0x41, 0x56, 0x41, 0x57,
                ][..],
            ),
            // Original FieldIns pointer in attack packet; native bullet init
            // copies the 660-byte launch context to CSBulletIns+4b0.
            (0x448c35, &[0x49, 0x8b, 0x9e, 0xd8, 0x01, 0, 0][..]),
            (0x39e908, &[0x48, 0x81, 0xc1, 0xb0, 0x04, 0, 0][..]),
            // Failed spawn returns before 3c0e70 (native cost application).
            (
                0x3c05e6,
                &[
                    0x84, 0xc0, 0x75, 0x06, 0x48, 0x83, 0xc4, 0x20, 0x5b, 0xc3, 0x48, 0x8b, 0xcb,
                    0xe8, 0x78, 0x08, 0, 0,
                ][..],
            ),
            (
                0x3c0621,
                &[
                    0x84, 0xc0, 0x75, 0x06, 0x48, 0x83, 0xc4, 0x20, 0x5b, 0xc3, 0x48, 0x8b, 0xcb,
                    0xe8, 0x3d, 0x08, 0, 0,
                ][..],
            ),
            (0x395212, &[0xc6, 0x03, 0x02, 0x48, 0x89, 0x43, 0x08][..]),
        ] {
            if unsafe { std::slice::from_raw_parts(va(rva)? as *const u8, bytes.len()) } != bytes {
                return Err(format!("Mend native layout/signature mismatch at {rva:x}"));
            }
        }
        // Transaction installs all six call-site adapters or none. No global
        // function-entry detours, stolen instructions, or executable-wide scans.
        let mut patches = Vec::new();
        for (rva, original, adapter) in [
            (0x4493d3, target, mend_impact as *const () as usize),
            (0x4494b8, target, mend_impact as *const () as usize),
            (0x3c05d6, cast, mend_cast as *const () as usize),
            (0x3c0611, cast, mend_cast as *const () as usize),
            (0x3c0ff0, build, mend_build as *const () as usize),
            (0x449ffa, hit, mend_hit as *const () as usize),
        ] {
            let site = va(rva)?;
            let expected = hooks::call_bytes(site, original).ok_or("Mend original call range")?;
            if unsafe { std::slice::from_raw_parts(site as *const u8, 5) } != expected {
                return Err(format!("Mend call conflict at {rva:x}"));
            }
            let page = unsafe { hooks::near(site) }.ok_or("Mend relay allocation")?;
            let mut stub = vec![0xff, 0x25, 0, 0, 0, 0];
            stub.extend_from_slice(&adapter.to_le_bytes());
            unsafe {
                std::ptr::copy_nonoverlapping(stub.as_ptr(), page as *mut u8, stub.len());
                hooks::seal_page(page)?;
            }
            patches.push((
                site,
                expected.to_vec(),
                hooks::call_bytes(site, page)
                    .ok_or("Mend relay range")?
                    .to_vec(),
            ));
        }
        MEND_ORIGINAL.store(target, Ordering::Release);
        MEND_CAST_ORIGINAL.store(cast, Ordering::Release);
        MEND_BUILD_ORIGINAL.store(build, Ordering::Release);
        MEND_HIT_ORIGINAL.store(hit, Ordering::Release);
        MEND_READY.store(true, Ordering::Release);
        let committed = unsafe { hooks::commit_optional(&patches) };
        if committed.is_err() {
            MEND_READY.store(false, Ordering::Release);
        }
        committed
    })();
    match result {
        Ok(()) => log(
            "Spirit Mend 0.5.1 hooks installed: invalid release skips native cost; per-projectile native launch target retained; marker 99000100 -> native heal 99000101; native spell FP modifiers retained",
        ),
        Err(e) => log(format!(
            "Spirit Mend disabled: {e}; impact marker remains inert"
        )),
    }
}

fn mend_target(
    receiver: *const ChrIns,
    source: *const ChrIns,
) -> Option<(FieldInsHandle, i32, i32)> {
    use eldenring::cs::SpEffectParam;
    if !MEND_READY.load(Ordering::Acquire) || !is_owned_living_target(source, receiver) {
        return None;
    }
    let world = unsafe { WorldChrMan::instance() }.ok()?;
    let actor = registered_spirit(world, receiver)?;
    let repo = unsafe { SoloParamRepository::instance() }.ok()?;
    if !param_ready::<SpEffectParam>(repo) {
        return None;
    }
    let marker = repo.get::<SpEffectParam>(MEND_MARKER as u32)?;
    let heal = repo.get::<SpEffectParam>(MEND_HEAL as u32)?;
    if marker.change_hp_rate() != 0.0
        || marker.change_hp_point() != 0
        || heal.change_hp_rate() != -25.0
        || heal.change_hp_point() != 0
        || heal.effect_endurance() != 0.0
        || heal.motion_interval() != 0.0
        || heal.state_info() != 0
    {
        return None;
    }
    let data = unsafe { health_ptr(actor, Inspection::Query, &mut PageChecks::new()) }?;
    let (hp, max) = unsafe { ((*data).hp, (*data).max_hp) };
    if !crate::mend_policy::eligible(true, Some(Health { hp, max })) {
        return None;
    }
    Some((actor.field_ins_handle, hp, max))
}

unsafe extern "system" fn mend_impact(
    receiver: *const ChrIns,
    effect: i32,
    source: *const ChrIns,
    no_sync: u8,
    position: usize,
    context: usize,
    flag6: u8,
    flag7: u8,
    flag8: u8,
) -> u8 {
    let original = MEND_ORIGINAL.load(Ordering::Acquire);
    if original == 0 {
        return 0;
    }
    let call: MendEffectFn = unsafe { std::mem::transmute(original) };
    if effect != MEND_MARKER {
        return unsafe {
            call(
                receiver, effect, source, no_sync, position, context, flag6, flag7, flag8,
            )
        };
    }
    let outcome = std::panic::catch_unwind(|| {
        let Some((handle, hp, max)) = mend_target(receiver, source) else {
            log(
                "Spirit Mend impact rejected: target/source/health/asset gate; no healing applied (valid released projectile costs are not refunded)",
            );
            return 0;
        };
        let Some(flight) = mend_claim_impact(receiver, source) else {
            log(
                "Spirit Mend impact rejected: wrong launch target, untracked projectile, or duplicate delivery; current lock-on is not used",
            );
            return 0;
        };
        let accepted = unsafe {
            call(
                receiver, MEND_HEAL, source, no_sync, position, context, flag6, flag7, flag8,
            )
        };
        log(format!(
            "Spirit Mend impact: target={:08x}:{:08x} hp_before={hp} max_hp={max} expected_capped_hp={} native_accepted={accepted} projectile={:08x}:{:08x}; original launch target matched independently of current lock-on; FP handled only by native spell cast; HP after must be observed separately",
            handle.selector.0,
            handle.block_id.0,
            (hp as i64 + (max as i64 / 4)).min(max as i64),
            flight.bullet.selector.0,
            flight.bullet.block_id.0
        ));
        accepted
    });
    match outcome {
        Ok(v) => v,
        Err(_) => {
            MEND_READY.store(false, Ordering::Release);
            log("Spirit Mend disabled after impact adapter panic; custom marker suppressed");
            0
        }
    }
}
