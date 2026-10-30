//! Experimental Windows adapter. Every actor is resolved anew within the activation.
use crate::{
    config::Config,
    hooks,
    policy::{self, Action, Health},
    timing,
};
use eldenring::{
    cs::{
        BuddyParam, BuddyStoneParam, CSChrDataModule, CSSessionManager, CSTaskGroupIndex,
        CSTaskImp, ChrIns, ChrInsModuleContainer, EquipParamGoods, GameMan, ItemCategory,
        SoloParam, SoloParamRepository, SummonBuddyManager, WorldChrMan,
    },
    fd4::FD4TaskData,
    util::system::wait_for_system_init,
};
use fromsoftware_shared::{
    FromStatic, GameVersion, LANG_ID_EN, SharedTaskImpExt, program::Program,
};
use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
    ffi::{OsString, c_void},
    fs::{self, OpenOptions},
    io::Write,
    os::windows::ffi::OsStringExt,
    path::PathBuf,
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};
use windows::{
    Win32::{
        Foundation::HMODULE,
        System::{
            LibraryLoader::GetModuleFileNameW,
            Memory::{
                MEM_COMMIT, MEMORY_BASIC_INFORMATION, PAGE_EXECUTE_READWRITE,
                PAGE_EXECUTE_WRITECOPY, PAGE_GUARD, PAGE_NOACCESS, PAGE_READWRITE, PAGE_WRITECOPY,
                VirtualQuery,
            },
        },
    },
    core::PCWSTR,
};

static CONFIG: OnceLock<Config> = OnceLock::new();
static DIRECTORY: OnceLock<PathBuf> = OnceLock::new();
static STARTED: OnceLock<Instant> = OnceLock::new();
static ATTEMPTED: AtomicBool = AtomicBool::new(false);
static FAULT: AtomicBool = AtomicBool::new(false);
// Refreshed by the game task; activation commit also checks the loader directly.
// No game actor or singleton pointer is cached here.
static CONFLICTING_DLL: AtomicBool = AtomicBool::new(true);
#[derive(Clone, Copy)]
struct StoneOriginal {
    address: usize,
    flag: u32,
    range: u16,
    return_range: i16,
    region: u32,
    eliminate: u32,
}
type StoneOriginals = (usize, HashMap<u32, StoneOriginal>);
static ANYWHERE_READY: AtomicBool = AtomicBool::new(false);
static STONES: Mutex<Option<StoneOriginals>> = Mutex::new(None);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct GateStatus {
    player_loaded: Option<bool>,
    online_flags: Option<(bool, bool)>,
    session_states: Option<(u32, u32)>,
    conflicting_dll: bool,
    buddy_param_ready: Option<bool>,
    resummon_config: bool,
    captured_rows: Option<usize>,
}
static LAST_GATE: Mutex<Option<(GateStatus, Instant, u64)>> = Mutex::new(None);
pub(crate) fn config() -> &'static Config {
    CONFIG.get_or_init(Config::default)
}
pub fn log(message: impl std::fmt::Display) {
    if let Some(dir) = DIRECTORY.get()
        && let Ok(mut f) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(dir.join("SpiritAshResummon.log"))
    {
        let elapsed = STARTED.get_or_init(Instant::now).elapsed().as_millis();
        let _ = writeln!(f, "[+{elapsed}ms] {message}");
    }
}
fn debug(message: impl std::fmt::Display) {
    if config().debug {
        log(message);
    }
}
#[derive(Clone, Copy)]
struct Supported;
impl GameVersion for Supported {
    const NAME: &'static str = "elden ring";
    fn from_lang_version(lang: u16, v: &str) -> Option<Self> {
        (lang == LANG_ID_EN && v == "2.7.2.0").then_some(Self)
    }
}
#[link(name = "kernel32")]
unsafe extern "system" {
    // A missing optional DLL is an ordinary predicate result. Avoid constructing
    // a windows::Error (and associated error information) for each expected miss.
    #[link_name = "GetModuleHandleW"]
    fn module_handle_raw(name: PCWSTR) -> HMODULE;
}
fn loaded(name: PCWSTR) -> bool {
    !unsafe { module_handle_raw(name) }.0.is_null()
}
fn conflicts() -> bool {
    let _timer = timing::CONFLICT.start(config().debug);
    [
        windows::core::w!("MultipleSpiritAshes.dll"),
        windows::core::w!("NativeTweaks.dll"),
        windows::core::w!("ersc.dll"),
        windows::core::w!("EasyAntiCheat_x64.dll"),
        windows::core::w!("EasyAntiCheat_EOS.dll"),
    ]
    .into_iter()
    .any(loaded)
}
pub fn start(module: usize) {
    let mut buf = [0u16; 32768];
    let n = unsafe { GetModuleFileNameW(Some(HMODULE(module as *mut c_void)), &mut buf) } as usize;
    if n == 0 || n >= buf.len() {
        return;
    }
    let path = PathBuf::from(OsString::from_wide(&buf[..n]));
    let Some(dir) = path.parent() else {
        return;
    };
    let _ = DIRECTORY.set(dir.to_path_buf());
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        log(format!(
            "PANIC DETAIL: {info}\n{}",
            std::backtrace::Backtrace::force_capture()
        ));
        previous_hook(info);
    }));
    log(format!(
        "SpiritAshResummon {} diagnostic startup pid={}",
        env!("CARGO_PKG_VERSION"),
        std::process::id()
    ));
    let c = match fs::read_to_string(dir.join("SpiritAshResummon.ini")) {
        Ok(s) => Config::parse(&s),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Config::default()),
        Err(e) => Err(e.to_string()),
    };
    let c = match c {
        Ok(c) => c,
        Err(e) => {
            log(format!("Disabled: {e}"));
            return;
        }
    };
    let _ = CONFIG.set(c);
    let mut host = [0u16; 32768];
    let n = unsafe { GetModuleFileNameW(None, &mut host) } as usize;
    if n == 0
        || !PathBuf::from(OsString::from_wide(&host[..n]))
            .file_name()
            .is_some_and(|n| n.eq_ignore_ascii_case("eldenring.exe"))
    {
        log("Disabled: host is not eldenring.exe; no game memory accessed");
        return;
    }
    let p = Program::current();
    let Program::Mapping(image) = p else {
        return;
    };
    if Supported::detect(&image).is_err() {
        log("Disabled: unsupported executable; only WW 2.7.2.0 is enabled");
        return;
    }
    if conflicts() {
        log("Disabled: conflicting DLL or protected environment detected");
        return;
    }
    // Capacity must be patched before the engine constructs its summon registry.
    // Gameplay callbacks pass through until the game task publishes READY.
    if let Err(error) = unsafe { hooks::install(config().multiple) } {
        log(format!("Disabled: startup patch transaction: {error}"));
        return;
    }
    log(format!(
        "Startup patches installed: call_sites={}, multi_type={}, actor_pool={}; awaiting game task readiness",
        if config().multiple { 11 } else { 8 },
        config().multiple,
        if config().multiple {
            crate::capacity::ENTRIES
        } else {
            80
        }
    ));
    initialize_target_diagnostic();
    initialize_friendly_lockon();
    initialize_spirit_mend();
    if config().hud {
        match unsafe { crate::hud::install() } {
            Ok(()) => log(
                "HUD hooks installed: four individual Spirit rows; waiting for movie load and live snapshot",
            ),
            Err(e) => log(format!("HUD disabled; gameplay hooks retained: {e}")),
        }
    }
    if let Err(error) = wait_for_system_init(&p, Duration::from_secs(30)) {
        log(format!("Disabled: system initialization failed: {error:?}"));
        return;
    }
    let Some(tasks) = wait_for_scheduler(Duration::from_secs(30)) else {
        return;
    };
    std::mem::forget(tasks.run_recurring(tick as fn(&FD4TaskData), CSTaskGroupIndex::GameMan));
    log("Startup task registered; awaiting gameplay readiness");
}

/// Runs only on the DLL startup worker, before task registration.
/// from-singleton 3.0.1 maps both an undiscovered singleton and a discovered
/// null instance to None; FromStatic consequently reports NotFound for both.
/// The upstream wait helper retries only Null and can therefore exit early.
/// Retry either lookup error within a bounded deadline; never guess an address.
fn wait_for_scheduler(timeout: Duration) -> Option<&'static CSTaskImp> {
    let started = Instant::now();
    let mut attempts = 0u32;
    let mut next_report = Duration::ZERO;
    loop {
        if attempts > 0 && started.elapsed() >= timeout {
            log(format!(
                "Disabled: task scheduler lookup deadline reached after {} ms, attempts={attempts}; see preceding lookup error",
                started.elapsed().as_millis()
            ));
            return None;
        }
        attempts += 1;
        // CSTaskImp explicitly permits instance lookup from a worker thread.
        match unsafe { CSTaskImp::instance() } {
            Ok(tasks) => {
                log(format!(
                    "Task scheduler ready after {} ms, attempts={attempts}",
                    started.elapsed().as_millis()
                ));
                return Some(tasks);
            }
            Err(error) => {
                let elapsed = started.elapsed();
                if elapsed >= timeout {
                    log(format!(
                        "Disabled: task scheduler unavailable after {} ms, attempts={attempts}, error={error:?}",
                        elapsed.as_millis()
                    ));
                    return None;
                }
                if elapsed >= next_report {
                    log(format!(
                        "Waiting for task scheduler: elapsed_ms={}, attempts={attempts}, error={error:?}",
                        elapsed.as_millis()
                    ));
                    next_report = elapsed + Duration::from_secs(5);
                }
                std::thread::sleep(Duration::from_millis(50).min(timeout - elapsed));
            }
        }
    }
}

fn offline() -> bool {
    policy::offline_allowed(
        unsafe { GameMan::instance() }
            .ok()
            .map(|g| g.is_in_online_mode),
        session_states(),
        CONFLICTING_DLL.load(Ordering::Acquire),
    )
}
fn session_states() -> Option<(u32, u32)> {
    let session = unsafe { CSSessionManager::instance() }.ok()?;
    // Read raw values so unexpected enum discriminants reject instead of being interpreted.
    Some(unsafe {
        (
            std::ptr::addr_of!(session.lobby_state).cast::<u32>().read(),
            std::ptr::addr_of!(session.protocol_state)
                .cast::<u32>()
                .read(),
        )
    })
}
fn tick(_: &FD4TaskData) {
    let _timer = timing::TASK.start(config().debug);
    guarded("GameMan task", (), tick_inner);
}
fn tick_inner() {
    if FAULT.load(Ordering::Acquire) {
        update_stones(false);
        return;
    }
    if !ATTEMPTED.swap(true, Ordering::AcqRel) {
        if unsafe { WorldChrMan::instance() }.is_ok_and(|w| w.main_player.is_some()) {
            log("Disabled: late load with an active player; restart from title screen");
            return;
        }
        if !hooks::INSTALLED.load(Ordering::Acquire) {
            return;
        }
        hooks::READY.store(true, Ordering::Release);
        log("Game task ready; gameplay adapter enabled after startup validation");
    }
    if !hooks::READY.load(Ordering::Acquire) {
        return;
    }
    CONFLICTING_DLL.store(conflicts(), Ordering::Release);
    refresh_pending_spawn();
    if config().multiple {
        if let Ok(world) = unsafe { WorldChrMan::instance() } {
            if world.main_player.is_some()
                && world.summon_buddy_chr_set.capacity != crate::capacity::ENTRIES
            {
                FAULT.store(true, Ordering::Release);
                log(format!(
                    "Disabled: summon registry has {} entries, expected {}; constructor was too early or another mod changed it. Restart required",
                    world.summon_buddy_chr_set.capacity,
                    crate::capacity::ENTRIES
                ));
                return;
            }
        }
    }
    report_gate_status();
    update_stones(offline() && (config().resummon || config().anywhere));
    prepare_anywhere(std::ptr::null_mut());
    release_inactive_groups();
    refresh_hud();
    sample_target_diagnostic();
    report_friendly_lockon();
    if config().debug {
        report_summon_trace();
        timing::report();
    }
}

/// Observational scalar samples only; no actor addresses or pending requests
/// are retained, and no engine state is changed by this diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SummonTrace {
    last_item_query_native_and_adapter: (i32, i32),
    last_eligibility_goods_native_adapter: (i32, i32, i32),
    queued_goods: Option<i32>,
    requested_goods: i32,
    request_effect: i32,
    active_goods: i32,
    active_effect: u32,
    alive_flag: bool,
    in_range: bool,
    disappear_requested: bool,
    cooldown_seconds_ceil: Option<i32>,
    disappear_seconds_ceil: Option<i32>,
    local_slots: usize,
    exiting_slots: usize,
    player_fp: Option<i32>,
}
fn summon_trace() -> Option<SummonTrace> {
    if !enabled() {
        return None;
    }
    let world = unsafe { WorldChrMan::instance() }.ok()?;
    let player = world.main_player.as_ref()?;
    let pc = &player.chr_ins;
    let manager_ptr = unsafe {
        std::ptr::addr_of!(world.summon_buddy_manager)
            .cast::<*const SummonBuddyManager>()
            .read()
    };
    if !pointer_shape(manager_ptr) {
        return None;
    }
    let manager = unsafe { &*manager_ptr };
    if manager.groups.len() > 32 {
        return None;
    }
    let mut local_slots = 0;
    let mut exiting_slots = 0;
    for entry in manager.groups.iter() {
        if entry.first != pc.event_entity_id as i32 {
            continue;
        }
        if entry.second.len() > crate::capacity::MAX_GROUP_MEMBERS {
            return None;
        }
        for group in entry.second.iter() {
            local_slots += 1;
            exiting_slots += usize::from(group.disappear_requested);
        }
    }
    let mut pages = PageChecks::new();
    let player_fp = unsafe { health_ptr(pc, Inspection::Query, &mut pages) }.and_then(|data| {
        let (fp, max) = unsafe { ((*data).fp, (*data).max_fp) };
        (fp >= 0 && max >= fp).then_some(fp)
    });
    let seconds = |value: f32| value.is_finite().then(|| value.ceil() as i32);
    Some(SummonTrace {
        last_item_query_native_and_adapter: {
            let packed = LAST_ITEM_QUERY.load(Ordering::Relaxed);
            ((packed >> 32) as u32 as i32, packed as u32 as i32)
        },
        last_eligibility_goods_native_adapter: {
            let packed = LAST_ELIGIBILITY_QUERY.load(Ordering::Relaxed);
            (
                (packed >> 32) as u32 as i32,
                (packed >> 8) as u8 as i8 as i32,
                packed as u8 as i8 as i32,
            )
        },
        queued_goods: pc
            .tae_queued_use_item
            .as_valid()
            .filter(|item| item.category() == ItemCategory::Goods)
            .map(|item| item.param_id() as i32),
        requested_goods: manager.requested_summon_goods_id,
        request_effect: manager.request_summon_speffect_id,
        active_goods: manager.active_summon_goods_id,
        active_effect: manager.active_summon_speffect_id,
        alive_flag: manager.player_has_alive_summon,
        in_range: manager.is_within_activation_range,
        disappear_requested: manager.disappear_requested,
        cooldown_seconds_ceil: seconds(manager.item_use_cooldown_timer),
        disappear_seconds_ceil: seconds(manager.buddy_disappear_delay_sec),
        local_slots,
        exiting_slots,
        player_fp,
    })
}
type TraceHistory = (Option<Instant>, Option<SummonTrace>);
// i32::MIN in both halves means no enabled item query has been observed.
static LAST_ITEM_QUERY: AtomicU64 = AtomicU64::new(0x8000_0000_8000_0000);
// i32::MIN / -128 / -128 means no item eligibility observation yet.
static LAST_ELIGIBILITY_QUERY: AtomicU64 = AtomicU64::new(0x8000_0000_0000_8080);
static LAST_SUMMON_TRACE: Mutex<TraceHistory> = Mutex::new((None, None));
fn report_summon_trace() {
    let Ok(mut history) = LAST_SUMMON_TRACE.try_lock() else {
        return;
    };
    let now = Instant::now();
    if history
        .0
        .is_some_and(|last| now.duration_since(last) < Duration::from_millis(250))
    {
        return;
    }
    history.0 = Some(now);
    let sample = summon_trace();
    if sample != history.1 {
        log(format!(
            "Summon timeline sample (250ms cadence, timers rounded up): {sample:?}"
        ));
        history.1 = sample;
    }
}
fn report_gate_status() {
    let status = GateStatus {
        player_loaded: unsafe { WorldChrMan::instance() }
            .ok()
            .map(|w| w.main_player.is_some()),
        online_flags: unsafe { GameMan::instance() }
            .ok()
            .map(|g| (g.is_in_online_mode, g.server_connection_enabled)),
        session_states: session_states(),
        conflicting_dll: CONFLICTING_DLL.load(Ordering::Acquire),
        buddy_param_ready: unsafe { SoloParamRepository::instance() }
            .ok()
            .map(param_ready::<BuddyStoneParam>),
        resummon_config: config().resummon,
        captured_rows: STONES
            .lock()
            .ok()
            .map(|s| s.as_ref().map_or(0, |(_, rows)| rows.len())),
    };
    let Ok(mut last) = LAST_GATE.lock() else {
        return;
    };
    let (changed, due, count) = last
        .as_ref()
        .map_or((true, true, 1), |(old, emitted, count)| {
            (
                *old != status,
                emitted.elapsed() >= Duration::from_secs(10),
                count.saturating_add(1),
            )
        });
    if changed || (config().debug && due) {
        log(format!(
            "Activation gates tick={count}: {status:?}; online_flags=(online_mode, connection_capability); require online_mode=false and session_states=(lobby=0, protocol=0); connection_capability is diagnostic only"
        ));
        *last = Some((status, Instant::now(), count));
    } else if let Some((_, _, previous_count)) = last.as_mut() {
        *previous_count = count;
    }
}
fn update_stones(enable: bool) {
    let _timer = timing::STONES.start(config().debug);
    let Ok(repo) = (unsafe { SoloParamRepository::instance_mut() }) else {
        ANYWHERE_READY.store(false, Ordering::Release);
        return;
    };
    // The repository singleton can outlive its loaded parameter resources.
    // get/rows assume a present res cap and panic otherwise (including teardown).
    if !param_ready::<BuddyStoneParam>(repo) {
        ANYWHERE_READY.store(false, Ordering::Release);
        return;
    }
    let identity = repo as *mut _ as usize;
    let Ok(mut state) = STONES.lock() else {
        return;
    };
    if state.as_ref().is_some_and(|(id, _)| *id != identity) {
        *state = None;
    }
    if !enable {
        ANYWHERE_READY.store(false, Ordering::Release);
        if let Ok(mut cache) = ANYWHERE_CACHE.lock() {
            *cache = None;
        }
        if let Some((_, originals)) = state.take() {
            for (id, old) in originals {
                if let Some(row) = repo.get_mut::<BuddyStoneParam>(id)
                    && row as *mut _ as usize == old.address
                {
                    // Restore only values still equal to this mod's writes.
                    if config().resummon && row.summoned_event_flag_id() == 0 {
                        row.set_summoned_event_flag_id(old.flag);
                    }
                    if config().anywhere {
                        if row.activate_range() == u16::MAX {
                            row.set_activate_range(old.range);
                        }
                        if row.overwrite_return_range() == -1 {
                            row.set_overwrite_return_range(old.return_range);
                        }
                        if row.overwrite_activate_region_entity_id() == 0 {
                            row.set_overwrite_activate_region_entity_id(old.region);
                        }
                        if row.eliminate_target_entity_id() == 0 {
                            row.set_eliminate_target_entity_id(old.eliminate);
                        }
                    }
                }
            }
        }
        return;
    }
    if let Some((_, originals)) = state.as_ref() {
        // A singleton can survive a parameter-table replacement. Check an anchor row.
        if originals.iter().next().is_some_and(|(id, old)| {
            repo.get::<BuddyStoneParam>(*id)
                .is_some_and(|row| row as *const _ as usize == old.address)
        }) {
            ANYWHERE_READY.store(config().anywhere, Ordering::Release);
            return;
        }
        *state = None;
    }
    ANYWHERE_READY.store(false, Ordering::Release);
    if let Ok(mut cache) = ANYWHERE_CACHE.lock() {
        *cache = None;
    }
    let mut originals = HashMap::new();
    for (id, row) in repo.rows_mut::<BuddyStoneParam>() {
        originals.insert(
            id,
            StoneOriginal {
                address: row as *mut _ as usize,
                flag: row.summoned_event_flag_id(),
                range: row.activate_range(),
                return_range: row.overwrite_return_range(),
                region: row.overwrite_activate_region_entity_id(),
                eliminate: row.eliminate_target_entity_id(),
            },
        );
        if config().resummon {
            row.set_summoned_event_flag_id(0);
        }
        if config().anywhere {
            row.set_activate_range(u16::MAX);
            row.set_overwrite_return_range(-1);
            row.set_overwrite_activate_region_entity_id(0);
            row.set_eliminate_target_entity_id(0);
        }
    }
    if !originals.is_empty() {
        log(format!(
            "BuddyStone rules applied: rows={} resummon={} summon_anywhere={} keep_after_boss={}",
            originals.len(),
            config().resummon,
            config().anywhere,
            config().anywhere
        ));
        *state = Some((identity, originals));
        ANYWHERE_READY.store(config().anywhere, Ordering::Release);
    }
}

include!("anywhere.rs");

fn param_ready<P: SoloParam>(repo: &SoloParamRepository) -> bool {
    repo.solo_param_holders
        .get(P::INDEX as usize)
        .and_then(|holder| holder.get_res_cap(0))
        .is_some()
}

fn pointer_shape<T>(p: *const T) -> bool {
    !p.is_null() && (p as usize).is_multiple_of(std::mem::align_of::<T>())
}
#[derive(Clone, Copy)]
struct CheckedRegion {
    start: usize,
    end: usize,
    readable: bool,
    writable: bool,
}
impl CheckedRegion {
    fn from_info(m: &MEMORY_BASIC_INFORMATION) -> Option<Self> {
        let start = m.BaseAddress as usize;
        let end = start.checked_add(m.RegionSize)?;
        let readable = m.State == MEM_COMMIT && m.Protect.0 & (PAGE_GUARD.0 | PAGE_NOACCESS.0) == 0;
        let writable = readable
            && [
                PAGE_READWRITE,
                PAGE_WRITECOPY,
                PAGE_EXECUTE_READWRITE,
                PAGE_EXECUTE_WRITECOPY,
            ]
            .contains(&m.Protect);
        Some(Self {
            start,
            end,
            readable,
            writable,
        })
    }
    fn admits(&self, start: usize, end: usize, write: bool) -> bool {
        start >= self.start
            && start < self.end
            && end >= start
            && end <= self.end
            && self.readable
            && (!write || self.writable)
    }
}

/// Stack-only region metadata for one synchronous validation pass. Never static
/// or returned with a Snapshot. The write pass creates a NEW cache after all
/// native lookups, so snapshot protection results cannot authorize HP writes.
/// The same serialized-callback/lifetime assumption as health_ptr still applies.
struct PageChecks {
    regions: [Option<CheckedRegion>; 16],
    used: usize,
}
impl PageChecks {
    fn new() -> Self {
        Self {
            regions: [None; 16],
            used: 0,
        }
    }
    fn check<T>(&mut self, p: *const T, write: bool) -> bool {
        if !pointer_shape(p) {
            return false;
        }
        let start = p as usize;
        let Some(end) = start.checked_add(std::mem::size_of::<T>()) else {
            return false;
        };
        for region in self.regions[..self.used].iter().flatten() {
            if start >= region.start && start < region.end {
                let _hit = timing::PAGE_CACHE_HIT.start(config().debug);
                return region.admits(start, end, write);
            }
        }
        let mut m = MEMORY_BASIC_INFORMATION::default();
        let read = {
            // This now measures every actual VirtualQuery, including write checks.
            let _probe = timing::READABLE_PROBE.start(config().debug);
            unsafe { VirtualQuery(Some(p.cast()), &mut m, std::mem::size_of_val(&m)) }
        };
        if read != std::mem::size_of_val(&m) {
            return false;
        }
        let Some(region) = CheckedRegion::from_info(&m) else {
            return false;
        };
        let allowed = region.admits(start, end, write);
        if self.used < self.regions.len() {
            self.regions[self.used] = Some(region);
            self.used += 1;
        }
        // A full cache falls back to an OS query, never to unchecked acceptance.
        allowed
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Inspection {
    Query,
    Commit,
}

fn inspect_pointer<T>(p: *const T, inspection: Inspection, pages: &mut PageChecks) -> bool {
    pointer_shape(p) && (inspection == Inspection::Query || pages.check(p, false))
}

/// Query mode requires a current engine-owned actor: WorldChrMan's local player
/// or an entry admitted by the pinned load/update profile, in this same callback.
/// Its module ownership is checked below. Commit adds fresh OS page checks.
/// Neither mode makes the game-thread/lifetime assumption safe across callbacks.
unsafe fn health_ptr(
    chr: *const ChrIns,
    inspection: Inspection,
    pages: &mut PageChecks,
) -> Option<*mut CSChrDataModule> {
    if !inspect_pointer(chr, inspection, pages) {
        return None;
    }
    let modules = unsafe {
        std::ptr::addr_of!((*chr).modules)
            .cast::<*const ChrInsModuleContainer>()
            .read()
    };
    if !inspect_pointer(modules, inspection, pages) {
        return None;
    }
    let data = unsafe {
        std::ptr::addr_of!((*modules).data)
            .cast::<*mut CSChrDataModule>()
            .read()
    };
    if !inspect_pointer(data, inspection, pages)
        || unsafe { (*data).owner.as_ptr() } != chr.cast_mut()
    {
        return None;
    }
    Some(data)
}

/// Resolve group pointers by membership before dereferencing them. Inspect only
/// the current, engine-owned summon registry (which can also contain Torrent).
/// Local group ownership and Ash-family checks still select targets afterward.
fn registered_spirit(world: &WorldChrMan, pointer: *const ChrIns) -> Option<&ChrIns> {
    let _timer = timing::REGISTRY_LOOKUP.start(config().debug);
    let resolved = registered_spirit_inner(world, pointer);
    if resolved.is_none() {
        let _rejected = timing::REGISTRY_REJECT.start(config().debug);
    }
    resolved
}

fn registered_spirit_inner(world: &WorldChrMan, pointer: *const ChrIns) -> Option<&ChrIns> {
    use timing::RegistryResult;
    let debug = config().debug;
    if !pointer_shape(pointer) {
        timing::registry_result(debug, RegistryResult::PointerShape, 0, 0, None);
        return None;
    }
    let set = &world.summon_buddy_chr_set;
    let capacity = set.capacity as usize;
    if set.capacity > 1024 {
        timing::registry_result(debug, RegistryResult::Capacity, capacity, 0, None);
        return None;
    }
    if !pointer_shape(set.entries.as_ptr()) {
        timing::registry_result(debug, RegistryResult::StorageShape, capacity, 0, None);
        return None;
    }
    let mut occupied_seen = 0;
    for index in 0..set.capacity as usize {
        // The registry storage is borrowed from this callback's WorldChrMan;
        // do not construct references to arbitrary group or inactive actor pointers.
        let entry = unsafe { set.entries.as_ptr().add(index) };
        let actor = unsafe { std::ptr::addr_of!((*entry).chr_ins).read() };
        occupied_seen += usize::from(actor.is_some());
        if actor.is_none_or(|p| p.as_ptr().cast_const() != pointer) {
            continue;
        }
        // Raw discriminants reject unknown states without interpreting invalid enums.
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
        // The upstream enum names do not describe this executable's lifecycle.
        // Native load gate at RVA 3f2d42 requires 4; completion writes 4 at
        // 3f8ce6, and update-enabled logic writes 4 at 40143d. See retained
        // registry-state-0.1.9 evidence. This is NOT a network ownership test.
        // Offline session and local group/is_remote checks remain independent.
        let active = load == 4;
        let updating = update == 4;
        let result = if !active {
            RegistryResult::Inactive
        } else if !updating {
            RegistryResult::UpdateState
        } else if unsafe { std::ptr::addr_of!((*pointer).chr_set_entry).read().as_ptr() }
            .cast_const()
            != entry
        {
            RegistryResult::Backlink
        } else {
            RegistryResult::Accepted
        };
        timing::registry_result(debug, result, capacity, occupied_seen, Some((load, update)));
        return matches!(result, RegistryResult::Accepted).then(|| unsafe { &*pointer });
    }
    timing::registry_result(
        debug,
        RegistryResult::NotFound,
        capacity,
        occupied_seen,
        None,
    );
    None
}
struct Snapshot {
    action: Action,
    goods: i32,
    targets: Vec<(*mut CSChrDataModule, Health)>,
    retiring_slots: usize,
}

/// Inspect registry metadata only. Never dereference an unloading actor or
/// admit it as a healing target. Unknown/missing membership remains ambiguous.
fn retiring_registry_member(world: &WorldChrMan, pointer: *const ChrIns) -> bool {
    let set = &world.summon_buddy_chr_set;
    if !pointer_shape(pointer) || set.capacity > 1024 || !pointer_shape(set.entries.as_ptr()) {
        return false;
    }
    for index in 0..set.capacity as usize {
        let entry = unsafe { set.entries.as_ptr().add(index) };
        let actor = unsafe { std::ptr::addr_of!((*entry).chr_ins).read() };
        if actor.is_none_or(|p| p.as_ptr().cast_const() != pointer) {
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
        // Observed during permanent-death cleanup on the pinned executable.
        // This pair alone is NOT proof of death: caller requires native ready,
        // no alive summon, and every local member explicitly exiting.
        return matches!((load, update), (2, 2) | (4, 2));
    }
    false
}

/// Borrowed pointers live only inside one synchronous game callback and never enter static state.
unsafe fn snapshot_single(
    manager: *mut SummonBuddyManager,
    goods: i32,
    native: i32,
    inspection: Inspection,
) -> Option<Snapshot> {
    let _timer = timing::SNAPSHOT.start(config().debug);
    let identity_timer = timing::SNAPSHOT_IDENTITY.start(config().debug);
    if manager.is_null()
        || !(manager as usize).is_multiple_of(std::mem::align_of::<SummonBuddyManager>())
    {
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
    drop(identity_timer);
    // The native state routine just used this manager on the same game callback,
    // and its address matches the current engine-owned WorldChrMan member.
    // Inspect only that borrowed manager for the idle shortcut; do not cache it.
    // Commit adds page probes below. Read-only queries use registry ownership.
    let player = world.main_player.as_ref()?;
    let pc = &player.chr_ins;
    let m = unsafe { &*manager };
    if m.groups.len() > 32 {
        return None;
    }
    // With neither a native alive flag nor any local group entries, there is
    // nothing to heal or block. Preserve the native result without health probes,
    // goods lookup, effect-map allocation or actor traversal. Never use this
    // shortcut for the native active/dismissal state, or for a retained group.
    if native != 2
        && !m.player_has_alive_summon
        && !m
            .groups
            .iter()
            .any(|entry| entry.first == pc.event_entity_id as i32 && !entry.second.is_empty())
    {
        let _idle = timing::SNAPSHOT_IDLE.start(config().debug);
        return Some(Snapshot {
            action: Action::Vanilla,
            goods,
            targets: Vec::new(),
            retiring_slots: 0,
        });
    }
    let _active = timing::SNAPSHOT_FULL.start(config().debug);
    let mut pages = PageChecks::new();
    if !inspect_pointer(manager, inspection, &mut pages) {
        return None;
    }
    let pdata = unsafe { health_ptr(pc, inspection, &mut pages) }?;
    if unsafe { (*pdata).hp } <= 0 || pc.chr_flags1c5.death_flag() {
        return None;
    }
    let queued = pc
        .tae_queued_use_item
        .as_valid()
        .filter(|i| i.category() == ItemCategory::Goods)
        .map(|i| i.param_id() as i32);
    let goods = if goods >= 0 { goods } else { queued? };
    let repo = unsafe { SoloParamRepository::instance() }.ok()?;
    if !param_ready::<EquipParamGoods>(repo) {
        return None;
    }
    let item = repo.get::<EquipParamGoods>(goods as u32)?;
    if item.ref_category() != 2 {
        return None;
    }
    let effect = item.ref_id_default();
    let mapping: Vec<_> = m
        .trigger_speffect_to_buddy_map
        .iter()
        .map(|(a, b)| (*a, *b))
        .collect();
    let family = policy::family(&mapping, effect);
    if family.is_empty() {
        return None;
    }
    // Do not apply live-actor health validation to retired cleanup entries.
    // Preserve the native ready result, never manufacture one. The engine owns
    // spawning, cost, area eligibility and cleanup; no timer/list/flag is reset.
    if config().resummon
        && native == 0
        && !m.player_has_alive_summon
        && !m.disappear_requested
        && m.request_summon_speffect_id == -1
        && m.item_use_cooldown_timer.is_finite()
        && m.item_use_cooldown_timer <= 0.0
        && goods == m.active_summon_goods_id
        && effect as u32 == m.active_summon_speffect_id
    {
        let mut slots = 0;
        let mut all_retiring = true;
        for entry in m.groups.iter() {
            if entry.first != pc.event_entity_id as i32 {
                continue;
            }
            if entry.second.len() > crate::capacity::MAX_GROUP_MEMBERS {
                return None;
            }
            for group in entry.second.iter() {
                slots += 1;
                all_retiring &= group.disappear_requested
                    && !group.is_remote
                    && !group.warp_requested
                    && family.contains(&group.buddy_param_id)
                    && retiring_registry_member(world, group.chr_ins.as_ptr());
            }
        }
        if slots > 0 && all_retiring {
            return Some(Snapshot {
                action: Action::Vanilla,
                goods,
                targets: Vec::new(),
                retiring_slots: slots,
            });
        }
    }
    let mut any_live = false;
    let mut foreign = false;
    let mut targets = Vec::new();
    let mut seen = HashSet::new();
    if m.groups.len() > 32 {
        return None;
    }
    for entry in m.groups.iter() {
        if entry.first != pc.event_entity_id as i32 {
            continue;
        }
        if entry.second.len() > crate::capacity::MAX_GROUP_MEMBERS {
            return None;
        }
        for g in entry.second.iter() {
            let chr = g.chr_ins.as_ptr();
            let actor = registered_spirit(world, chr)?;
            // Engine handle lookup protects against a recycled or stale group pointer.
            if !world
                .chr_ins_by_handle(&actor.field_ins_handle)
                .is_some_and(|resolved| std::ptr::eq(resolved, chr))
            {
                let _rejected = timing::HANDLE_REJECT.start(config().debug);
                return None;
            }
            let Some(data) = (unsafe { health_ptr(actor, inspection, &mut pages) }) else {
                let _rejected = timing::HEALTH_REJECT.start(config().debug);
                return None;
            };
            let h = unsafe {
                Health {
                    hp: (*data).hp,
                    max: (*data).max_hp,
                }
            };
            if h.hp <= 0 || actor.chr_flags1c5.death_flag() {
                continue;
            }
            any_live = true;
            if g.is_remote
                || g.disappear_requested
                || g.warp_requested
                || !family.contains(&g.buddy_param_id)
            {
                foreign = true;
                continue;
            }
            if h.max <= 0 || h.hp > h.max {
                return None;
            }
            if seen.insert(chr as usize) {
                targets.push((data, h));
                let _validated = timing::TARGET_VALIDATED.start(config().debug);
            }
        }
    }
    let same = goods == m.active_summon_goods_id
        && effect as u32 == m.active_summon_speffect_id
        && !foreign;
    let eligible = area_allowed(m) && !m.disappear_requested && m.item_use_cooldown_timer <= 0.0;
    let wounded = targets
        .iter()
        .any(|(_, h)| policy::healed(*h, config().percent).is_some_and(|hp| hp > h.hp));
    let action = policy::action(native, any_live, same, eligible, wounded, config().heal);
    let _resolved = timing::SNAPSHOT_RESOLVED.start(config().debug);
    Some(Snapshot {
        action,
        goods,
        targets,
        retiring_slots: 0,
    })
}
unsafe fn native_state(m: *mut SummonBuddyManager, goods: i32) -> i32 {
    let _timer = timing::NATIVE_STATE.start(config().debug);
    let f: unsafe extern "system" fn(*mut SummonBuddyManager, i32) -> i32 =
        unsafe { std::mem::transmute(hooks::STATE.load(Ordering::Acquire)) };
    unsafe { f(m, goods) }
}
fn enabled() -> bool {
    hooks::READY.load(Ordering::Acquire) && !FAULT.load(Ordering::Acquire) && offline()
}

unsafe fn query_inner(m: *mut SummonBuddyManager, goods: i32) -> i32 {
    let native = unsafe { native_state(m, goods) };
    let result = unsafe { adapt_query(m, goods, native) };
    if config().debug {
        LAST_ITEM_QUERY.store(
            ((native as u32 as u64) << 32) | result as u32 as u64,
            Ordering::Relaxed,
        );
    }
    result
}
unsafe fn adapt_query(m: *mut SummonBuddyManager, goods: i32, native: i32) -> i32 {
    if !enabled() {
        return native;
    }
    match unsafe { snapshot(m, goods, native, Inspection::Query) }.map(|s| s.action) {
        Some(Action::Heal | Action::AddGroup) => 0,
        Some(Action::Reject) => -1,
        Some(Action::Vanilla) => native,
        // An unresolved active group must not fall through to native dismissal
        // or allow an unvalidated spawn. The no-group idle path returns Vanilla.
        None => -1,
    }
}
unsafe fn cost_inner(m: *mut SummonBuddyManager, goods: i32) -> i32 {
    let native = unsafe { native_state(m, goods) };
    if !enabled() {
        return native;
    }
    // A native request already accepted by this mod must not turn into the
    // free-dismissal cost branch while the engine finishes its item transaction.
    if config().multiple && matches!(native, 0..=2) && pending_spawn_cost(m, goods) {
        return 0;
    }
    match unsafe { snapshot(m, goods, native, Inspection::Query) }.map(|s| s.action) {
        // The engine checks affordability and computes/deducts its own normal FP/HP cost.
        // No custom subtraction is performed anywhere in this DLL.
        Some(Action::Heal | Action::AddGroup) => 0,
        Some(Action::Reject) => 2,
        Some(Action::Vanilla) => native,
        None => 2,
    }
}
unsafe fn summon_inner(m: *mut SummonBuddyManager, effect: i32) -> bool {
    let original: unsafe extern "system" fn(*mut SummonBuddyManager, i32) -> bool =
        unsafe { std::mem::transmute(hooks::SUMMON.load(Ordering::Acquire)) };
    // Keep conflict detection fresh at an actual activation, even between tasks.
    CONFLICTING_DLL.store(conflicts(), Ordering::Release);
    if !enabled() {
        return unsafe { original(m, effect) };
    }
    let native = unsafe { native_state(m, -1) };
    if config().debug {
        log(format!(
            "Summon callback begin effect={effect} native_state={native} timeline={:?}",
            summon_trace()
        ));
    }
    let Some(s) = (unsafe { snapshot(m, -1, native, Inspection::Commit) }) else {
        // This hook is exclusively the main player's summon-effect call. An unresolved
        // request must not fall through to dismissal or spawn on a possibly stale flag.
        debug("Rejected ambiguous Spirit Ash activation");
        return false;
    };
    match s.action {
        Action::AddGroup => unsafe { add_group(m, effect, s.goods, original) },
        Action::Vanilla => {
            if s.retiring_slots > 0 {
                // Recheck the actual effect argument as well as the queued item
                // before passing a cleanup-overlap activation to the engine.
                let manager = unsafe { &*m };
                if effect as u32 != manager.active_summon_speffect_id {
                    debug("Rejected cleanup resummon: effect argument changed");
                    return false;
                }
                debug(format!(
                    "Cleanup resummon admitted goods={} retiring_slots={}; native ready, no alive summon, all local members exiting and registry update=2",
                    s.goods, s.retiring_slots
                ));
            }
            sync_anywhere_origin(m);
            let started = Instant::now();
            let result = unsafe { original(m, effect) };
            if config().debug {
                let elapsed_us = started.elapsed().as_micros();
                log(format!(
                    "Native summon returned effect={effect} result={result} elapsed_us={elapsed_us} timeline={:?}",
                    summon_trace()
                ));
            }
            result
        }
        Action::Reject => {
            debug(format!("Rejected active activation goods={}", s.goods));
            false
        }
        Action::Heal => {
            let Ok(repo) = (unsafe { SoloParamRepository::instance() }) else {
                return false;
            };
            if !param_ready::<EquipParamGoods>(repo) {
                return false;
            }
            if !repo
                .get::<EquipParamGoods>(s.goods as u32)
                .is_some_and(|r| r.ref_id_default() == effect)
            {
                return false;
            }
            // Fresh OS metadata for this write pass, independent of snapshot's
            // dropped cache. One query supplies both readability and writability.
            // All targets validated before the first write. No native calls or
            // allocations between this validation loop and the HP writes.
            let mut write_pages = PageChecks::new();
            for (data, h) in &s.targets {
                if !write_pages.check(*data, true)
                    || unsafe { (**data).hp } != h.hp
                    || unsafe { (**data).max_hp } != h.max
                {
                    return false;
                }
            }
            let mut changed = 0;
            for (data, h) in &s.targets {
                if let Some(new) = policy::healed(*h, config().percent)
                    && new > h.hp
                {
                    unsafe {
                        (**data).hp = new;
                    };
                    changed += 1;
                }
            }
            // Emit diagnostics only after all writes, never between group-member writes.
            if config().debug {
                for (member, (_, h)) in s.targets.iter().enumerate() {
                    log(format!(
                        "Heal member goods={} member={} hp_before={} max_hp={} hp_after_written={}",
                        s.goods,
                        member,
                        h.hp,
                        h.max,
                        policy::healed(*h, config().percent).unwrap_or(h.hp)
                    ));
                }
            }
            debug(format!(
                "Heal activation goods={} changed_members={changed}; native cost path",
                s.goods
            ));
            changed > 0
        }
    }
}

fn guarded<T>(context: &str, fallback: T, f: impl FnOnce() -> T) -> T {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(value) => value,
        Err(payload) => {
            FAULT.store(true, Ordering::Release);
            let message = payload
                .downcast_ref::<&str>()
                .copied()
                .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
                .unwrap_or("non-string panic payload");
            log(format!(
                "Rust panic caught in {context}: {message}; gameplay adapter disabled. Restart the game."
            ));
            fallback
        }
    }
}
pub unsafe extern "system" fn query(m: *mut SummonBuddyManager, goods: i32) -> i32 {
    let _timer = timing::QUERY.start(config().debug);
    guarded("item query", -1, || {
        prepare_anywhere(m);
        unsafe { query_inner(m, goods) }
    })
}
pub unsafe extern "system" fn eligibility(m: *mut SummonBuddyManager, goods: i32) -> i32 {
    let _timer = timing::QUERY.start(config().debug);
    guarded("item eligibility", -1, || {
        prepare_anywhere(m);
        let native = unsafe { native_state(m, goods) };
        let result = unsafe { adapt_query(m, goods, native) };
        if config().debug {
            LAST_ELIGIBILITY_QUERY.store(
                ((goods as u32 as u64) << 32) | ((native as u8 as u64) << 8) | result as u8 as u64,
                Ordering::Relaxed,
            );
        }
        result
    })
}
pub unsafe extern "system" fn cost(m: *mut SummonBuddyManager, goods: i32) -> i32 {
    let _timer = timing::COST.start(config().debug);
    guarded("cost query", 2, || {
        prepare_anywhere(m);
        unsafe { cost_inner(m, goods) }
    })
}
pub unsafe extern "system" fn summon(m: *mut SummonBuddyManager, effect: i32) -> bool {
    let _timer = timing::SUMMON.start(config().debug);
    guarded("summon effect", false, || {
        prepare_anywhere(m);
        unsafe { summon_inner(m, effect) }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn version_allowlist_is_exact() {
        assert!(Supported::from_lang_version(LANG_ID_EN, "2.7.2.0").is_some());
        for v in ["2.7.1.0", "2.7.2.1", "2.7.3.0", "1.17.2", ""] {
            assert!(Supported::from_lang_version(LANG_ID_EN, v).is_none());
        }
        assert!(Supported::from_lang_version(0, "2.7.2.0").is_none());
    }
    #[test]
    fn null_entity_is_rejected() {
        assert!(
            unsafe { health_ptr(std::ptr::null(), Inspection::Query, &mut PageChecks::new()) }
                .is_none()
        );
        assert!(
            unsafe { health_ptr(std::ptr::null(), Inspection::Commit, &mut PageChecks::new()) }
                .is_none()
        );
    }
    #[test]
    fn checked_region_rejects_cross_boundary_and_read_only_writes() {
        let region = CheckedRegion {
            start: 0x1000,
            end: 0x2000,
            readable: true,
            writable: false,
        };
        assert!(region.admits(0x1ffc, 0x2000, false));
        assert!(!region.admits(0x1ffc, 0x2001, false));
        assert!(!region.admits(0x0fff, 0x1003, false));
        assert!(!region.admits(0x2000, 0x2000, false));
        assert!(!region.admits(0x1000, 0x0fff, false));
        assert!(!region.admits(0x1000, 0x1004, true));
    }
    #[test]
    fn checked_region_rejects_guard_uncommitted_and_overflow() {
        let mut m = MEMORY_BASIC_INFORMATION {
            BaseAddress: 0x1000usize as _,
            RegionSize: 0x1000,
            State: MEM_COMMIT,
            Protect: PAGE_READWRITE,
            ..Default::default()
        };
        assert!(
            CheckedRegion::from_info(&m)
                .unwrap()
                .admits(0x1000, 0x1004, true)
        );
        m.Protect = PAGE_READWRITE | PAGE_GUARD;
        assert!(
            !CheckedRegion::from_info(&m)
                .unwrap()
                .admits(0x1000, 0x1004, true)
        );
        m.Protect = PAGE_NOACCESS;
        assert!(
            !CheckedRegion::from_info(&m)
                .unwrap()
                .admits(0x1000, 0x1004, false)
        );
        m.Protect = PAGE_READWRITE;
        m.State = Default::default();
        assert!(
            !CheckedRegion::from_info(&m)
                .unwrap()
                .admits(0x1000, 0x1004, false)
        );
        m.BaseAddress = (usize::MAX - 1) as _;
        assert!(CheckedRegion::from_info(&m).is_none());
    }
    #[test]
    fn upstream_layout_offsets_are_pinned() {
        use std::mem::{offset_of, size_of};
        assert_eq!(size_of::<SummonBuddyManager>(), 0x110);
        assert_eq!(offset_of!(SummonBuddyManager, groups), 0x70);
        assert_eq!(offset_of!(SummonBuddyManager, active_summon_goods_id), 0xe4);
        assert_eq!(offset_of!(ChrIns, modules), 0x190);
        assert_eq!(offset_of!(CSChrDataModule, hp), 0x138);
        assert_eq!(offset_of!(CSChrDataModule, max_hp), 0x13c);
    }
}
include!("multi.rs");
include!("cleanup.rs");

include!("hud_snapshot.rs");

include!("targeting_diagnostic.rs");
include!("friendly_lockon.rs");
include!("spirit_mend.rs");
