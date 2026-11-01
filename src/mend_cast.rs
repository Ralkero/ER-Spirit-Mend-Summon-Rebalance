// Included by spirit_mend.rs. All offsets/calls below are WW 2.7.2.0 only.
use eldenring::cs::{BlockId, BulletSpawnData, CSBulletIns, CSBulletManager, FieldInsSelector};

const MEND_MAGIC: i32 = 990001;
static MEND_CAST_ORIGINAL: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static MEND_BUILD_ORIGINAL: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static MEND_HIT_ORIGINAL: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[repr(C)]
struct MendActionPrefix {
    owner: *const ChrIns,
    behavior_id: i32,
    magic_id: i32,
    goods_id: i32,
    reference_type: u8,
    pad15: [u8; 3],
}

#[derive(Clone, Copy)]
struct MendRelease {
    owner: FieldInsHandle,
    target: FieldInsHandle,
}
#[derive(Clone, Copy)]
struct MendFlight {
    release: MendRelease,
    bullet: FieldInsHandle,
}
thread_local! {
    // Only live on the native call stack, never between frames or casts.
    static MEND_RELEASE: Cell<Option<MendRelease>> = const { Cell::new(None) };
    static MEND_FLIGHT: Cell<Option<MendFlight>> = const { Cell::new(None) };
}
struct MendCellScope<T: Copy + 'static> {
    key: &'static std::thread::LocalKey<Cell<Option<T>>>,
    prior: Option<T>,
}
impl<T: Copy> MendCellScope<T> {
    fn enter(key: &'static std::thread::LocalKey<Cell<Option<T>>>, value: Option<T>) -> Self {
        Self {
            key,
            prior: key.with(|c| c.replace(value)),
        }
    }
}
impl<T: Copy> Drop for MendCellScope<T> {
    fn drop(&mut self) {
        self.key.with(|c| c.set(self.prior));
    }
}

type MendCastFn = unsafe extern "system" fn(*const MendActionPrefix, usize, *mut i32) -> u8;
type MendBuildFn =
    unsafe extern "system" fn(*const MendActionPrefix, *mut BulletSpawnData) -> usize;
type MendHitFn = unsafe extern "system" fn(usize, *const ChrIns, *const u8, u8, u8) -> usize;

const _: () = {
    assert!(std::mem::offset_of!(MendActionPrefix, magic_id) == 0xc);
    assert!(std::mem::size_of::<BulletSpawnData>() == 0x110);
    assert!(std::mem::offset_of!(BulletSpawnData, target) == 0x20);
    assert!(std::mem::offset_of!(BulletSpawnData, magic_id) == 0xc);
    assert!(std::mem::size_of::<CSBulletIns>() == 0xe30);
};

fn mend_selected_release(source: *const ChrIns) -> Option<MendRelease> {
    if !MEND_READY.load(Ordering::Acquire) || !enabled() {
        return None;
    }
    let world = unsafe { WorldChrMan::instance() }.ok()?;
    let player = world.main_player.as_ref()?;
    if !std::ptr::eq(&player.chr_ins, source) || !player.chr_ins.is_locked_on {
        return None;
    }
    let actor = world.chr_ins_by_handle(&player.locked_on_enemy)?;
    let (target, _, _) = mend_target(actor, source)?;
    Some(MendRelease {
        owner: player.chr_ins.field_ins_handle,
        target,
    })
}

unsafe extern "system" fn mend_cast(
    action: *const MendActionPrefix,
    emitter: usize,
    error: *mut i32,
) -> u8 {
    let call: MendCastFn =
        unsafe { std::mem::transmute(MEND_CAST_ORIGINAL.load(Ordering::Acquire)) };
    // Pointer is supplied by the two verified native stack-descriptor call sites.
    if action.is_null() || unsafe { (*action).magic_id } != MEND_MAGIC {
        return unsafe { call(action, emitter, error) };
    }
    let result = std::panic::catch_unwind(|| {
        let release = mend_selected_release(unsafe { (*action).owner });
        let Some(release) = release else {
            if !error.is_null() {
                unsafe {
                    *error = 2;
                }
            }
            log(
                "Spirit Mend release rejected: no eligible injured selected owned Spirit, offline/asset gate, or feature disabled; projectile and native resource commit skipped",
            );
            return 0;
        };
        let _scope = MendCellScope::enter(&MEND_RELEASE, Some(release));
        let accepted = unsafe { call(action, emitter, error) };
        log(format!(
            "Spirit Mend release: target={:08x}:{:08x} native_spawn_accepted={accepted}; native cost follows only on success; target captured at release",
            release.target.selector.0, release.target.block_id.0
        ));
        accepted
    });
    result.unwrap_or_else(|_| {
        MEND_READY.store(false, Ordering::Release);
        if !error.is_null() {
            unsafe {
                *error = 2;
            }
        }
        log("Spirit Mend release adapter panic: disabled; custom cast rejected");
        0
    })
}

unsafe extern "system" fn mend_build(
    action: *const MendActionPrefix,
    spawn: *mut BulletSpawnData,
) -> usize {
    let call: MendBuildFn =
        unsafe { std::mem::transmute(MEND_BUILD_ORIGINAL.load(Ordering::Acquire)) };
    let result = unsafe { call(action, spawn) };
    if spawn.is_null() || unsafe { (*spawn).magic_id } != MEND_MAGIC {
        return result;
    }
    let outcome = std::panic::catch_unwind(|| {
        let release = MEND_RELEASE.with(Cell::get)?;
        let data = unsafe { &mut *spawn };
        if data.owner != release.owner
            || data.bullet_id != MEND_MARKER
            || !MEND_READY.load(Ordering::Acquire)
        {
            return None;
        }
        // A native constructor output, owned by this call stack. No persistent
        // actor or camera write; the engine builds its own type-2 target record.
        data.set_spirit_mend_target(release.target).then_some(())
    });
    if !matches!(outcome, Ok(Some(()))) {
        unsafe {
            (*spawn).bullet_id = -1;
        }
        log(
            "Spirit Mend constructor rejected: missing/mismatched release context; invalid bullet ID prevents native spawn/cost",
        );
    }
    result
}

fn mend_handle(bytes: &[u8]) -> Option<FieldInsHandle> {
    Some(FieldInsHandle {
        selector: FieldInsSelector(u32::from_le_bytes(bytes.get(..4)?.try_into().ok()?)),
        block_id: BlockId(i32::from_le_bytes(bytes.get(4..8)?.try_into().ok()?)),
    })
}

fn mend_bullet_flight(candidate: *const CSBulletIns, source: *const ChrIns) -> Option<MendFlight> {
    if !MEND_READY.load(Ordering::Acquire) || !enabled() || !pointer_shape(candidate) {
        return None;
    }
    let world = unsafe { WorldChrMan::instance() }.ok()?;
    let player = world.main_player.as_ref()?;
    if !std::ptr::eq(&player.chr_ins, source) {
        return None;
    }
    let manager = unsafe { CSBulletManager::instance() }.ok()?;
    let mut current = manager.bullets.head;
    // Do not dereference the hit's candidate until the engine's live registry
    // contains that exact pointer. No unbounded traversal or OS page probes.
    for _ in 0..256 {
        let node = current?;
        if !pointer_shape(node.as_ptr()) {
            return None;
        }
        let bullet = unsafe { node.as_ref() };
        if std::ptr::eq(bullet, candidate) {
            if config().debug {
                let launch = bullet.launch_context_bytes();
                log(format!(
                    "Spirit Mend projectile evidence: bullet={:?} fly_param={} launch_bullet={} launch_magic={} target_kind={} launch_owner={:?} launch_target={:?} selected_now={:?} locked_now={}",
                    bullet.field_ins_handle,
                    bullet.fly_state.base.param.param_id,
                    i32::from_le_bytes(launch[4..8].try_into().unwrap()),
                    i32::from_le_bytes(launch[0x1c..0x20].try_into().unwrap()),
                    launch[0x140],
                    mend_handle(&launch[0x10..0x18]),
                    mend_handle(&launch[0x148..0x150]),
                    player.locked_on_enemy,
                    player.chr_ins.is_locked_on
                ));
            }
            if bullet.field_ins_handle.is_empty()
                || bullet.field_ins_handle.selector.0 >> 28 != 3
                || bullet.targeting_owner.bullet.as_ptr() != node.as_ptr()
                || bullet.targeting_owner.owner_chr_handle != player.chr_ins.field_ins_handle
                || bullet.fly_state.base.param.param_id != MEND_MARKER
            {
                return None;
            }
            let launch = bullet.launch_context_bytes();
            if i32::from_le_bytes(launch[4..8].try_into().ok()?) != MEND_MARKER
                || i32::from_le_bytes(launch[0x1c..0x20].try_into().ok()?) != MEND_MAGIC
                || launch[0x140] != 2
            {
                return None;
            }
            let owner = mend_handle(&launch[0x10..0x18])?;
            let target = mend_handle(&launch[0x148..0x150])?;
            if owner != player.chr_ins.field_ins_handle || target.is_empty() {
                return None;
            }
            return Some(MendFlight {
                release: MendRelease { owner, target },
                bullet: bullet.field_ins_handle,
            });
        }
        if bullet.next_bullet == current {
            return None;
        }
        current = bullet.next_bullet;
    }
    None
}

unsafe extern "system" fn mend_hit(
    module: usize,
    source: *const ChrIns,
    hit: *const u8,
    flag: u8,
    flag5: u8,
) -> usize {
    let call: MendHitFn = unsafe { std::mem::transmute(MEND_HIT_ORIGINAL.load(Ordering::Acquire)) };
    // Verified attack packet: +40 is AtkParam ID; +1d8 is original FieldIns.
    // All ordinary attacks take this constant-time passthrough path.
    if hit.is_null() || unsafe { hit.add(0x40).cast::<i32>().read_unaligned() } != MEND_MARKER {
        let _scope = MendCellScope::enter(&MEND_FLIGHT, None);
        return unsafe { call(module, source, hit, flag, flag5) };
    }
    let flight = std::panic::catch_unwind(|| {
        let candidate = unsafe { hit.add(0x1d8).cast::<*const CSBulletIns>().read_unaligned() };
        mend_bullet_flight(candidate, source)
    })
    .unwrap_or(None);
    if flight.is_none() {
        log(
            "Spirit Mend flight rejected: impact packet has no validated owned projectile launch target",
        );
    }
    let _scope = MendCellScope::enter(&MEND_FLIGHT, flight);
    unsafe { call(module, source, hit, flag, flag5) }
}

// Scalar handles only. Prune only when a COMPLETE current native bullet list
// proves absence; elapsed time is not evidence that a projectile was destroyed.
static MEND_CONSUMED: Mutex<crate::mend_policy::ImpactLedger> =
    Mutex::new(crate::mend_policy::ImpactLedger::new());
fn handle_key(handle: FieldInsHandle) -> u64 {
    (u64::from(handle.block_id.0 as u32) << 32) | u64::from(handle.selector.0)
}
fn mend_live_handles() -> Option<Vec<u64>> {
    let manager = unsafe { CSBulletManager::instance() }.ok()?;
    let mut current = manager.bullets.head;
    let mut live = Vec::new();
    for _ in 0..256 {
        let Some(node) = current else {
            return Some(live);
        };
        if !pointer_shape(node.as_ptr()) {
            return None;
        }
        let bullet = unsafe { node.as_ref() };
        let key = handle_key(bullet.field_ins_handle);
        if live.contains(&key) {
            return None;
        }
        live.push(key);
        current = bullet.next_bullet;
    }
    current.is_none().then_some(live)
}
// Constant-time session observation, no bullet traversal or page probes per frame.
fn refresh_mend_session() {
    if !config().spirit_mend {
        return;
    }
    let context = if enabled() {
        unsafe { WorldChrMan::instance() }.ok().and_then(|world| {
            let pc = &world.main_player.as_ref()?.chr_ins;
            (!pc.chr_flags1c5.death_flag())
                .then_some((world as *const _ as usize, handle_key(pc.field_ins_handle)))
        })
    } else {
        None
    };
    if let Ok(mut ledger) = MEND_CONSUMED.try_lock() {
        ledger.enter_context(context);
    }
}
fn mend_claim_impact(receiver: *const ChrIns, source: *const ChrIns) -> Option<MendFlight> {
    let flight = MEND_FLIGHT.with(Cell::get)?;
    let world = unsafe { WorldChrMan::instance() }.ok()?;
    let player = world.main_player.as_ref()?;
    let actor = registered_spirit(world, receiver)?;
    if !std::ptr::eq(&player.chr_ins, source) {
        return None;
    }
    let live = mend_live_handles();
    let mut consumed = MEND_CONSUMED.try_lock().ok()?;
    consumed.enter_context(Some((
        world as *const _ as usize,
        handle_key(player.chr_ins.field_ins_handle),
    )));
    if let Some(live) = live {
        consumed.retain_live(&live);
    }
    consumed
        .claim(
            crate::mend_policy::FlightKey {
                bullet: handle_key(flight.bullet),
                owner: handle_key(flight.release.owner),
                target: handle_key(flight.release.target),
            },
            handle_key(player.chr_ins.field_ins_handle),
            handle_key(actor.field_ins_handle),
        )
        .then_some(flight)
}
