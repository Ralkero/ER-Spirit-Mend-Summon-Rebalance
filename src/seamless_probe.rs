//! Connected-session investigation only. No gameplay hooks, actor writes,
//! packets, parameter changes, HUD changes, or gameplay code are compiled in.
//! Register a GameMan task to read SDK metadata on its owning game thread.
use eldenring::{
    cs::{CSSessionManager, CSTaskGroupIndex, CSTaskImp, GameMan, SummonBuddyManager, WorldChrMan},
    fd4::FD4TaskData,
    util::system::wait_for_system_init,
};
use fromsoftware_shared::{FromStatic, SharedTaskImpExt, program::Program};
use pelite::pe64::Pe;
use sha2::{Digest, Sha256};
use std::{
    ffi::{OsString, c_void},
    fs::{File, OpenOptions},
    io::{Read, Write},
    os::windows::ffi::OsStringExt,
    path::{Path, PathBuf},
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};
use windows::{
    Win32::{
        Foundation::HMODULE,
        System::{
            LibraryLoader::GetModuleFileNameW,
            Memory::{
                MEM_COMMIT, MEMORY_BASIC_INFORMATION, PAGE_GUARD, PAGE_NOACCESS, VirtualQuery,
            },
        },
    },
    core::{PCWSTR, w},
};

include!("seamless_probe_sites.rs");
const GAME_SHA: &str = "f9118fea2cfd8fa3901e8060bfadb1552cb2471771ded8a5b662c4e298ffb328";
const ERSC_SHA: &str = "0a093d8490967bf19c62f895fdf1fda21840282d2b2b371b9b4fc24a0cfe146e";
static DIRECTORY: OnceLock<PathBuf> = OnceLock::new();
static START: OnceLock<Instant> = OnceLock::new();
static STOP: AtomicBool = AtomicBool::new(false);
static STATE: Mutex<ProbeState> = Mutex::new(ProbeState {
    sample: None,
    report: None,
    sites: None,
    last: None,
    fingerprints: Vec::new(),
});
struct ProbeState {
    sample: Option<Instant>,
    report: Option<Instant>,
    sites: Option<Instant>,
    last: Option<String>,
    fingerprints: Vec<Vec<u8>>,
}
#[link(name = "kernel32")]
unsafe extern "system" {
    #[link_name = "GetModuleHandleW"]
    fn module_handle(name: PCWSTR) -> HMODULE;
}
fn module(name: PCWSTR) -> HMODULE {
    unsafe { module_handle(name) }
}
fn path(handle: Option<HMODULE>) -> Option<PathBuf> {
    let mut buffer = [0u16; 32768];
    let len = unsafe { GetModuleFileNameW(handle, &mut buffer) } as usize;
    (len > 0 && len < buffer.len()).then(|| PathBuf::from(OsString::from_wide(&buffer[..len])))
}
fn hash(path: &Path) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut digest = Sha256::new();
    let mut buffer = [0u8; 65536];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        digest.update(&buffer[..n]);
    }
    Ok(format!("{:x}", digest.finalize()))
}
fn log(message: impl std::fmt::Display) {
    if let Some(directory) = DIRECTORY.get()
        && let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(directory.join("SpiritMendCompatibilityProbe.log"))
    {
        let _ = writeln!(
            file,
            "[+{}ms] {message}",
            START.get_or_init(Instant::now).elapsed().as_millis()
        );
    }
}
fn protected_or_conflicting() -> bool {
    [
        w!("EasyAntiCheat_x64.dll"),
        w!("EasyAntiCheat_EOS.dll"),
        w!("SpiritAshResummon.dll"),
        w!("MultipleSpiritAshes.dll"),
        w!("NativeTweaks.dll"),
    ]
    .into_iter()
    .any(|name| !module(name).0.is_null())
}
// One bounded page query per metadata object at 1 Hz; never on actor lists.
fn readable<T>(pointer: *const T) -> bool {
    let address = pointer as usize;
    let Some(end) = address.checked_add(std::mem::size_of::<T>()) else {
        return false;
    };
    if address < 0x10000 || !address.is_multiple_of(std::mem::align_of::<T>()) {
        return false;
    }
    let mut info = MEMORY_BASIC_INFORMATION::default();
    (unsafe {
        VirtualQuery(
            Some(pointer.cast()),
            &mut info,
            std::mem::size_of_val(&info),
        ) != 0
    }) && info.State == MEM_COMMIT
        && info.Protect.0 & (PAGE_GUARD.0 | PAGE_NOACCESS.0) == 0
        && end <= (info.BaseAddress as usize).saturating_add(info.RegionSize)
}

pub fn start(handle: usize) {
    let Some(location) = path(Some(HMODULE(handle as *mut c_void))) else {
        return;
    };
    let Some(directory) = location.parent() else {
        return;
    };
    let _ = DIRECTORY.set(directory.to_owned());
    log("Seamless probe 1 startup; DIAGNOSTICS ONLY; no Spirit Mend gameplay enabled");
    let Some(game) = path(None) else {
        return;
    };
    if !game
        .file_name()
        .is_some_and(|n| n.eq_ignore_ascii_case("eldenring.exe"))
        || hash(&game).ok().as_deref() != Some(GAME_SHA)
    {
        log("STOP: unsupported executable hash; no game memory inspected");
        return;
    }
    // Mod Engine may load other DLLs after our startup thread begins.
    let deadline = Instant::now() + Duration::from_secs(30);
    let seamless = loop {
        if protected_or_conflicting() {
            log("STOP: protected environment or gameplay DLL conflict");
            return;
        }
        let handle = module(w!("ersc.dll"));
        if !handle.0.is_null() {
            break handle;
        }
        if Instant::now() >= deadline {
            log("STOP: Seamless DLL not loaded");
            return;
        }
        std::thread::sleep(Duration::from_millis(100));
    };
    if path(Some(seamless)).and_then(|p| hash(&p).ok()).as_deref() != Some(ERSC_SHA) {
        log("STOP: unsupported Seamless DLL hash");
        return;
    }
    log(format!(
        "Pinned executable and Seamless hashes accepted; pid={}",
        std::process::id()
    ));
    let program = Program::current();
    if let Err(e) = wait_for_system_init(&program, Duration::from_secs(30)) {
        log(format!("STOP: system initialization: {e:?}"));
        return;
    }
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        if let Ok(tasks) = unsafe { CSTaskImp::instance() } {
            std::mem::forget(
                tasks.run_recurring(tick as fn(&FD4TaskData), CSTaskGroupIndex::GameMan),
            );
            log(
                "Read-only GameMan task registered: metadata 1 Hz; hook bytes 0.2 Hz; heartbeat 15 sec",
            );
            return;
        }
        if Instant::now() >= deadline {
            log("STOP: scheduler unavailable");
            return;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn snapshot() -> String {
    let flags = unsafe { GameMan::instance() }
        .ok()
        .map(|g| (g.is_in_online_mode, g.server_connection_enabled));
    let session = unsafe { CSSessionManager::instance() }
        .ok()
        .map(|s| unsafe {
            // Unknown discriminants are logged as numbers, never materialized as enums.
            (
                std::ptr::addr_of!(s.lobby_state).cast::<u32>().read(),
                std::ptr::addr_of!(s.protocol_state).cast::<u32>().read(),
                s.players.len(),
            )
        });
    let Some(world) = (unsafe { WorldChrMan::instance() }).ok() else {
        return format!("flags={flags:?} session={session:?} world=absent");
    };
    let owner = world
        .main_player
        .as_ref()
        .map(|p| p.chr_ins.event_entity_id as i32);
    let player_handle = world
        .main_player
        .as_ref()
        .map(|p| p.chr_ins.field_ins_handle);
    let manager = unsafe {
        std::ptr::addr_of!(world.summon_buddy_manager)
            .cast::<*const SummonBuddyManager>()
            .read()
    };
    let prefix = format!(
        "flags={flags:?} session={session:?} owner={owner:?} player_handle={player_handle:?} capacity={}",
        world.summon_buddy_chr_set.capacity
    );
    if !readable(manager) {
        return format!("{prefix} manager=unavailable");
    }
    let manager = unsafe { &*manager };
    if manager.groups.len() > 32 {
        return format!("{prefix} groups=oversize_skipped");
    }
    let mut rows = Vec::new();
    let mut remaining = 128usize;
    for entry in manager.groups.iter().take(32) {
        if entry.second.len() > remaining {
            return format!("{prefix} members=oversize_skipped");
        }
        remaining -= entry.second.len();
        let mut local = 0usize;
        let mut remote = 0usize;
        let mut retiring = 0usize;
        let mut warping = 0usize;
        let mut buddies = Vec::new();
        // Only native group metadata; never dereference a group's actor pointer.
        for member in entry.second.iter().take(128) {
            if member.is_remote {
                remote += 1;
            } else {
                local += 1;
            }
            retiring += usize::from(member.disappear_requested);
            warping += usize::from(member.warp_requested);
            buddies.push(member.buddy_param_id);
        }
        rows.push((
            entry.first,
            Some(entry.first) == owner,
            local,
            remote,
            retiring,
            warping,
            buddies,
        ));
    }
    format!(
        "{prefix} requested_effect={} active_goods={} alive={} area={} groups={rows:?}",
        manager.request_summon_speffect_id,
        manager.active_summon_goods_id,
        manager.player_has_alive_summon,
        manager.is_within_activation_range
    )
}
fn inspect_sites(state: &mut ProbeState) {
    let p = Program::current();
    let mut fingerprints = Vec::new();
    for &(name, rva, expected) in SITES {
        let Ok(address) = p.rva_to_va(rva) else {
            log("STOP: hook RVA unavailable");
            STOP.store(true, Ordering::Release);
            return;
        };
        let pointer = address as *const [u8; 16];
        if !readable(pointer) {
            log("STOP: hook bytes unreadable");
            STOP.store(true, Ordering::Release);
            return;
        }
        fingerprints.push(unsafe { *pointer }.to_vec());
        let current = fingerprints.last().unwrap();
        let i = fingerprints.len() - 1;
        if state.fingerprints.get(i) != Some(current) {
            log(format!(
                "SITE {name} rva={rva:x} original={} bytes={current:02x?}",
                current.as_slice() == expected
            ));
        }
    }
    state.fingerprints = fingerprints;
}
fn tick(_: &FD4TaskData) {
    if STOP.load(Ordering::Acquire) {
        return;
    }
    let Ok(mut state) = STATE.try_lock() else {
        return;
    };
    let now = Instant::now();
    if state
        .sample
        .is_some_and(|t| now.duration_since(t) < Duration::from_secs(1))
    {
        return;
    }
    state.sample = Some(now);
    if protected_or_conflicting() || module(w!("ersc.dll")).0.is_null() {
        STOP.store(true, Ordering::Release);
        log("STOP: environment changed; sampling disabled");
        return;
    }
    // Rust panic containment is not an access-violation handler.
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let started = Instant::now();
        let current = snapshot();
        if state.last.as_ref() != Some(&current)
            || state
                .report
                .is_none_or(|t| now.duration_since(t) >= Duration::from_secs(15))
        {
            log(format!(
                "SAMPLE {current}; metadata_us={}",
                started.elapsed().as_micros()
            ));
            state.report = Some(now);
            state.last = Some(current);
        }
        if state
            .sites
            .is_none_or(|t| now.duration_since(t) >= Duration::from_secs(5))
        {
            inspect_sites(&mut state);
            state.sites = Some(now);
        }
    }));
    if result.is_err() {
        STOP.store(true, Ordering::Release);
        log("STOP: Rust panic in sample");
    }
}
