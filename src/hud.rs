// Independent HUD adapter adapted from ArialSenki's hud.rs (MIT OR Apache-2.0).
use eldenring::cs::{CSFeManImp, ChrFriendTagEntry, FieldInsHandle, MsgRepositoryImp};
use fromsoftware_shared::{FromStatic, program::Program};
use pelite::pe64::Pe;
use std::{
    ffi::c_void,
    sync::{
        Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};
pub static INSTALLED: AtomicBool = AtomicBool::new(false);
pub static MOVIE_READY: AtomicBool = AtomicBool::new(false);
static ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static LOOKUP: AtomicUsize = AtomicUsize::new(0);
static TABLE_NAMES: AtomicBool = AtomicBool::new(false);
static COPY_COUNT: AtomicUsize = AtomicUsize::new(0);
static BUSY_COUNT: AtomicUsize = AtomicUsize::new(0);
static FAULT: AtomicBool = AtomicBool::new(false);
pub const MAX_ROWS: usize = 4;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MemberKey {
    pub base: i32,
    pub handle: FieldInsHandle,
}
#[derive(Clone, Debug)]
pub struct Member {
    pub key: MemberKey,
    pub instance: Option<(u64, usize)>,
    pub hp: u32,
    pub max: u32,
}
struct Row {
    name: String,
    hp: u32,
    max: u32,
}
struct State {
    identity: (usize, u32),
    order: Vec<(MemberKey, usize)>,
    names: std::collections::HashMap<i32, String>,
    rows: Vec<Row>,
    updated: Option<Instant>,
    last_report: Option<Instant>,
}
static STATE: Mutex<Option<State>> = Mutex::new(None);
pub fn clear() {
    if let Ok(mut s) = STATE.try_lock() {
        *s = None;
    }
}
pub fn publish(identity: (usize, u32), members: Vec<Member>, mut name: impl FnMut(i32) -> String) {
    let Ok(mut lock) = STATE.try_lock() else {
        return;
    };
    let s = lock.get_or_insert_with(|| State {
        identity,
        order: Vec::new(),
        names: Default::default(),
        rows: Vec::new(),
        updated: None,
        last_report: None,
    });
    if s.identity != identity {
        s.identity = identity;
        s.order.clear();
        s.names.clear();
        s.rows.clear();
    }
    // Engine handles are scalar identity values, never dereferenced here.
    // Retain ordering and member numbers for still-valid living entities.
    s.order
        .retain(|(key, _)| members.iter().any(|m| m.key == *key));
    for m in &members {
        if !s.order.iter().any(|(key, _)| *key == m.key) {
            let ordinal = s
                .order
                .iter()
                .filter(|(key, _)| key.base == m.key.base)
                .map(|(_, n)| *n)
                .max()
                .unwrap_or(0)
                + 1;
            s.order.push((m.key, ordinal));
        }
    }
    s.names
        .retain(|base, _| members.iter().any(|m| m.key.base == *base));
    s.rows.clear();
    for (key, ordinal) in s.order.iter().take(MAX_ROWS) {
        let Some(m) = members.iter().find(|m| m.key == *key) else {
            continue;
        };
        if m.hp == 0 || m.max == 0 || m.hp > m.max {
            continue;
        }
        let label = s.names.entry(key.base).or_insert_with(|| name(key.base));
        s.rows.push(Row {
            name: match m.instance {
                Some((group, member)) => format!("{label} G{group} #{member}"),
                None => format!("{} #{}", label, ordinal),
            },
            hp: m.hp,
            max: m.max,
        });
    }
    let now = Instant::now();
    s.updated = Some(now);
    if crate::runtime::config().debug
        && s.last_report
            .is_none_or(|t| now.duration_since(t) >= Duration::from_secs(10))
    {
        crate::runtime::log(format!(
            "HUD entity snapshot: members={members:?}; shown={:?}; movie_ready={}; copies={}; native_occupied={}",
            s.rows
                .iter()
                .map(|r| (&r.name, r.hp, r.max))
                .collect::<Vec<_>>(),
            MOVIE_READY.load(Ordering::Acquire),
            COPY_COUNT.swap(0, Ordering::Relaxed),
            BUSY_COUNT.swap(0, Ordering::Relaxed)
        ));
        s.last_report = Some(now);
    }
}
pub fn goods_name(id: i32) -> Option<String> {
    let address = LOOKUP.load(Ordering::Acquire);
    if id <= 0 {
        return None;
    }
    let repo = unsafe { MsgRepositoryImp::instance_mut().ok()? } as *mut _ as *mut c_void;
    if address == 0 {
        let name = TABLE_NAMES
            .load(Ordering::Acquire)
            .then(|| crate::hud_names::native_lookup(repo as usize, id))
            .flatten();
        if crate::runtime::config().debug {
            crate::runtime::log(format!(
                "HUD loaded-table name: goods={id} resolved={}",
                name.is_some()
            ));
        }
        return name;
    }
    let call: unsafe extern "system" fn(*mut c_void, u32, u32, i32) -> *const u16 =
        unsafe { std::mem::transmute(address) };
    for category in [111, 10, 319, 419] {
        let ptr = unsafe { call(repo, 0, category, id) };
        if ptr.is_null() {
            continue;
        }
        let mut len = 0;
        while len < 128 && unsafe { *ptr.add(len) } != 0 {
            len += 1;
        }
        if len == 0 || len == 128 {
            continue;
        }
        let s = String::from_utf16_lossy(unsafe { std::slice::from_raw_parts(ptr, len) });
        if !s.starts_with('?') && s != "NULL" && !s.chars().all(|c| c.is_ascii_digit()) {
            return Some(s);
        }
    }
    None
}
fn free(e: &ChrFriendTagEntry) -> bool {
    e.field_ins_handle.is_empty() && !e.is_visible && !e.is_not_on_screen && !e.is_debug_summon
}
fn empty_handle(h: &mut FieldInsHandle) {
    unsafe { (h as *mut FieldInsHandle).cast::<u64>().write(u64::MAX) }
}
fn reset(e: &mut ChrFriendTagEntry) {
    e.is_visible = false;
    e.is_line_of_sight_blocked = false;
    e.is_not_on_screen = false;
    e.role_name_color = 0;
    empty_handle(&mut e.field_ins_handle);
    let _ = e.name_string.assign_str("");
    let _ = e.role_string.assign_str("");
    e.hp = 0;
    e.max_hp = 0;
    e.hp_max_uncapped = 0;
    e.max_recoverable_hp = 0;
    e.last_damage_time_delta = 0.0;
    e.voice_chat_state = 0;
    e.team_type = 0;
    e.is_down_scaled = false;
    e.has_rune_arc = false;
    e.is_debug_summon = false;
}
unsafe extern "system" fn copy(fe: *mut CSFeManImp, delta: f32) {
    let address = ORIGINAL.load(Ordering::Acquire);
    if address == 0 {
        return;
    }
    let call: unsafe extern "system" fn(*mut CSFeManImp, f32) =
        unsafe { std::mem::transmute(address) };
    let mut written = [false; MAX_ROWS];
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if fe.is_null()
            || !INSTALLED.load(Ordering::Acquire)
            || !MOVIE_READY.load(Ordering::Acquire)
            || FAULT.load(Ordering::Acquire)
        {
            return;
        }
        let Ok(s) = STATE.try_lock() else { return };
        let Some(s) = s.as_ref() else { return };
        if s.updated
            .is_none_or(|t| t.elapsed() > Duration::from_millis(500))
        {
            return;
        }
        let entries = unsafe { &mut (*fe).friendly_chr_tag_displays };
        // Never overwrite real friendly entries. Native offscreen stacking is
        // not isolated from those entries, so skip this frame if any are occupied.
        if entries.iter().any(|e| !free(e)) {
            BUSY_COUNT.fetch_add(1, Ordering::Relaxed);
            return;
        }
        for (i, row) in s.rows.iter().take(MAX_ROWS).enumerate() {
            let e = &mut entries[i];
            written[i] = true;
            if e.name_string.assign_str(&row.name).is_err() || e.role_string.assign_str("").is_err()
            {
                reset(e);
                continue;
            }
            e.is_visible = false;
            e.is_line_of_sight_blocked = false;
            e.is_not_on_screen = true;
            e.role_name_color = 1;
            empty_handle(&mut e.field_ins_handle);
            e.hp = row.hp;
            e.max_hp = row.max;
            e.hp_max_uncapped = row.max;
            e.max_recoverable_hp = row.hp;
            e.last_damage_time_delta = 0.0;
            e.voice_chat_state = 0;
            e.team_type = 0;
            e.is_down_scaled = false;
            e.has_rune_arc = false;
            e.is_debug_summon = false;
        }
        COPY_COUNT.fetch_add(1, Ordering::Relaxed);
    }));
    if result.is_err() {
        FAULT.store(true, Ordering::Release);
    }
    unsafe { call(fe, delta) };
    if !fe.is_null() {
        for (i, w) in written.iter().enumerate() {
            if *w {
                reset(unsafe { &mut (*fe).friendly_chr_tag_displays[i] });
            }
        }
    }
}
pub unsafe fn install() -> Result<(), String> {
    let p = Program::current();
    let va = |rva| {
        p.rva_to_va(rva)
            .map(|n| n as usize)
            .map_err(|e| format!("HUD RVA: {e:?}"))
    };
    let site = va(0x772fbd)?;
    let target = va(0x775480)?;
    let expected = crate::hooks::call_bytes(site, target).ok_or("HUD call range")?;
    if unsafe { std::slice::from_raw_parts(site as *const u8, 5) } != expected {
        return Err("HUD tag-copy call conflict".into());
    }
    let signature = [
        0x48, 0x8b, 0xc4, 0x55, 0x56, 0x57, 0x41, 0x54, 0x41, 0x55, 0x41, 0x56, 0x41, 0x57, 0x48,
        0x8d,
    ];
    if unsafe { std::slice::from_raw_parts(target as *const u8, signature.len()) } != signature {
        return Err("HUD copy target signature mismatch".into());
    }
    let lookup = va(0x2671370)?;
    let code = unsafe { std::slice::from_raw_parts(lookup as *const u8, 10) };
    if code[..4] == [0x3b, 0x51, 0x10, 0x73] && code[5..10] == [0x44, 0x3b, 0x41, 0x14, 0x73] {
        LOOKUP.store(lookup, Ordering::Release);
    } else if crate::runtime::accepted_seamless_solo_module() != 0 {
        let wrapper = unsafe { std::slice::from_raw_parts(lookup as *const u8, 49) };
        let leaf = unsafe { std::slice::from_raw_parts(va(0x2671c40)? as *const u8, 120) };
        if crate::hud_names::layout_matches(wrapper, leaf) {
            TABLE_NAMES.store(true, Ordering::Release);
            crate::runtime::log(
                "HUD names: verified loaded-table reader enabled; redirected name function is not called",
            );
        }
    }
    let movie = crate::hud_movie::prepare()?;
    let page = unsafe { crate::hooks::near(site) }.ok_or("HUD copy relay allocation")?;
    let mut stub = vec![0xff, 0x25, 0, 0, 0, 0];
    stub.extend_from_slice(&(copy as *const () as usize).to_le_bytes());
    unsafe {
        std::ptr::copy_nonoverlapping(stub.as_ptr(), page as *mut u8, stub.len());
        crate::hooks::seal_page(page)?;
    }
    ORIGINAL.store(target, Ordering::Release);
    INSTALLED.store(true, Ordering::Release);
    let result = unsafe {
        crate::hooks::commit_optional(&[
            movie,
            (
                site,
                expected.to_vec(),
                crate::hooks::call_bytes(site, page)
                    .ok_or("HUD copy relay range")?
                    .to_vec(),
            ),
        ])
    };
    if result.is_err() {
        INSTALLED.store(false, Ordering::Release);
    }
    result
}
