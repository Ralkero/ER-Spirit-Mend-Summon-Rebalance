//! Bounded aggregate diagnostics. No logging or allocation inside a timed callback.
use std::{
    sync::{
        Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};

pub struct Metric {
    name: &'static str,
    calls: AtomicU64,
    nanos: AtomicU64,
    max: AtomicU64,
}
impl Metric {
    const fn new(name: &'static str) -> Self {
        Self {
            name,
            calls: AtomicU64::new(0),
            nanos: AtomicU64::new(0),
            max: AtomicU64::new(0),
        }
    }
    pub fn start(&'static self, enabled: bool) -> Timer {
        Timer {
            metric: self,
            started: enabled.then(Instant::now),
        }
    }
}
pub struct Timer {
    metric: &'static Metric,
    started: Option<Instant>,
}
impl Drop for Timer {
    fn drop(&mut self) {
        if let Some(start) = self.started {
            let nanos = start.elapsed().as_nanos().min(u64::MAX as u128) as u64;
            self.metric.calls.fetch_add(1, Ordering::Relaxed);
            self.metric.nanos.fetch_add(nanos, Ordering::Relaxed);
            self.metric.max.fetch_max(nanos, Ordering::Relaxed);
        }
    }
}
pub static FRIENDLY_RELATION: Metric = Metric::new("friendly_relation_adapter");
pub static TARGET_SAMPLE: Metric = Metric::new("target_sample");
pub static TASK: Metric = Metric::new("task");
pub static QUERY: Metric = Metric::new("query");
pub static COST: Metric = Metric::new("cost");
pub static SUMMON: Metric = Metric::new("summon");
pub static NATIVE_STATE: Metric = Metric::new("native_state");
pub static SNAPSHOT: Metric = Metric::new("snapshot");
pub static SNAPSHOT_IDENTITY: Metric = Metric::new("snapshot_identity");
pub static SNAPSHOT_IDLE: Metric = Metric::new("snapshot_idle");
pub static SNAPSHOT_FULL: Metric = Metric::new("snapshot_full");
pub static READABLE_PROBE: Metric = Metric::new("readable_probe");
pub static PAGE_CACHE_HIT: Metric = Metric::new("page_cache_hit");
pub static REGISTRY_LOOKUP: Metric = Metric::new("registry_lookup");
pub static REGISTRY_REJECT: Metric = Metric::new("registry_reject");
pub static HANDLE_REJECT: Metric = Metric::new("handle_reject");
pub static HEALTH_REJECT: Metric = Metric::new("health_reject");
pub static TARGET_VALIDATED: Metric = Metric::new("target_validated");
pub static SNAPSHOT_RESOLVED: Metric = Metric::new("snapshot_resolved");
pub static CONFLICT: Metric = Metric::new("conflict_check");
pub static STONES: Metric = Metric::new("stones");
pub static HUD_SAMPLE: Metric = Metric::new("hud_sample");
pub static CLEANUP: Metric = Metric::new("inactive_cleanup");
static LAST: Mutex<Option<Instant>> = Mutex::new(None);

// Counters and packed scalar samples only: no actor pointers retained, no extra
// actor reads, page probes, allocation or log writes inside registry traversal.
#[derive(Clone, Copy)]
pub enum RegistryResult {
    PointerShape,
    Capacity,
    StorageShape,
    NotFound,
    Inactive,
    UpdateState,
    Backlink,
    Accepted,
}
const REGISTRY_NAMES: [&str; 8] = [
    "pointer_shape",
    "capacity",
    "storage_shape",
    "not_found",
    "inactive",
    "update_state",
    "backlink",
    "accepted",
];
static REGISTRY_COUNTS: [AtomicU64; 8] = [const { AtomicU64::new(0) }; 8];
static REGISTRY_SAMPLES: [AtomicU64; 8] = [const { AtomicU64::new(0) }; 8];

pub fn registry_result(
    enabled: bool,
    result: RegistryResult,
    capacity: usize,
    occupied_seen: usize,
    states: Option<(u8, u8)>,
) {
    if !enabled {
        return;
    }
    let (load, update) = states.unwrap_or((255, 255));
    let sample = capacity.min(65535) as u64
        | ((occupied_seen.min(65535) as u64) << 16)
        | ((load as u64) << 32)
        | ((update as u64) << 40)
        | ((states.is_some() as u64) << 48);
    let index = result as usize;
    REGISTRY_SAMPLES[index].store(sample, Ordering::Relaxed);
    REGISTRY_COUNTS[index].fetch_add(1, Ordering::Relaxed);
}

pub fn report() {
    let Ok(mut last) = LAST.try_lock() else {
        return;
    };
    let now = Instant::now();
    let Some(previous) = *last else {
        *last = Some(now);
        return;
    };
    let elapsed = now.duration_since(previous);
    if elapsed < Duration::from_secs(10) {
        return;
    }
    *last = Some(now);
    drop(last);
    let mut message = format!(
        "Timing window_ms={} (nested wall times, not additive):",
        elapsed.as_millis()
    );
    use std::fmt::Write;
    for metric in [
        &TASK,
        &QUERY,
        &COST,
        &SUMMON,
        &NATIVE_STATE,
        &SNAPSHOT,
        &SNAPSHOT_IDENTITY,
        &SNAPSHOT_IDLE,
        &SNAPSHOT_FULL,
        &READABLE_PROBE,
        &PAGE_CACHE_HIT,
        &REGISTRY_LOOKUP,
        &REGISTRY_REJECT,
        &HANDLE_REJECT,
        &HEALTH_REJECT,
        &TARGET_VALIDATED,
        &SNAPSHOT_RESOLVED,
        &CONFLICT,
        &STONES,
        &HUD_SAMPLE,
        &TARGET_SAMPLE,
        &FRIENDLY_RELATION,
        &CLEANUP,
    ] {
        let calls = metric.calls.swap(0, Ordering::Relaxed);
        let nanos = metric.nanos.swap(0, Ordering::Relaxed);
        let max = metric.max.swap(0, Ordering::Relaxed);
        let _ = write!(
            message,
            " {}[calls={},total_us={},max_us={}]",
            metric.name,
            calls,
            nanos / 1000,
            max / 1000
        );
    }
    crate::runtime::log(message);
    let mut registry = String::new();
    for (index, name) in REGISTRY_NAMES.iter().enumerate() {
        let count = REGISTRY_COUNTS[index].swap(0, Ordering::Relaxed);
        if count == 0 {
            continue;
        }
        let sample = REGISTRY_SAMPLES[index].load(Ordering::Relaxed);
        let _ = write!(
            registry,
            " {name}[count={count},last_capacity={},last_occupied_seen={},matched_states={},load={},update={}]",
            sample & 65535,
            (sample >> 16) & 65535,
            (sample >> 48) & 1,
            (sample >> 32) & 255,
            (sample >> 40) & 255
        );
    }
    if !registry.is_empty() {
        crate::runtime::log(format!(
            "Registry diagnostics (last scalar sample per result; capacity clamped to 65535; matched_states=0 means no state read):{registry}"
        ));
    }
}
