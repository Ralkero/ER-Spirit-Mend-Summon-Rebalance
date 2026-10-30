//! Narrow call-site hooks, adapted from ArialSenki's MIT/Apache-2.0 rel32 stubs.
//! All call sites and capacity patches are checked before the first write.
use fromsoftware_shared::program::Program;
use pelite::pe64::Pe;
use std::{
    ffi::c_void,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};
use windows::Win32::System::{
    Diagnostics::Debug::FlushInstructionCache,
    Memory::{
        MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
        PAGE_READWRITE, VirtualAlloc, VirtualProtect,
    },
    Threading::GetCurrentProcess,
};
pub static INSTALLED: AtomicBool = AtomicBool::new(false);
pub static CLEAR: AtomicUsize = AtomicUsize::new(0);
pub const INNER_STATE_SITE: u32 = 0x4b86a9;
pub const ELIGIBILITY_SITE: u32 = 0x690833;
pub const CLEAR_SITE: u32 = 0x4b8a90;
pub const CLEAR_RVA: u32 = 0x4bba80;
pub static READY: AtomicBool = AtomicBool::new(false);
pub static STATE: AtomicUsize = AtomicUsize::new(0);
pub static SUMMON: AtomicUsize = AtomicUsize::new(0);
pub const STATE_RVA: u32 = 0x4b73d0;
pub const SUMMON_RVA: u32 = 0x4b8690;
pub const QUERY_SITES: [u32; 3] = [0x7c4154, 0x847ed0, 0x848194];
pub const COST_SITES: [u32; 4] = [0x69063a, 0x6906dd, 0x3c0b79, 0x3c0c2f];
pub const SUMMON_SITE: u32 = 0x3fb268;

pub fn call_bytes(site: usize, target: usize) -> Option<[u8; 5]> {
    let delta = i32::try_from(target as i128 - (site as i128 + 5)).ok()?;
    let mut b = [0xe8, 0, 0, 0, 0];
    b[1..].copy_from_slice(&delta.to_le_bytes());
    Some(b)
}
unsafe fn write(address: usize, bytes: &[u8]) -> bool {
    let mut old = PAGE_PROTECTION_FLAGS(0);
    if unsafe { VirtualProtect(address as _, bytes.len(), PAGE_EXECUTE_READWRITE, &mut old) }
        .is_err()
    {
        return false;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), address as *mut u8, bytes.len());
    }
    let flushed =
        unsafe { FlushInstructionCache(GetCurrentProcess(), Some(address as _), bytes.len()) }
            .is_ok();
    let mut ignored = PAGE_PROTECTION_FLAGS(0);
    let restored = unsafe { VirtualProtect(address as _, bytes.len(), old, &mut ignored) }.is_ok();
    flushed && restored
}
pub(crate) unsafe fn near(site: usize) -> Option<usize> {
    for distance in (0x10000usize..0x70000000).step_by(0x10000) {
        for candidate in [site.checked_add(distance), site.checked_sub(distance)]
            .into_iter()
            .flatten()
        {
            let page = unsafe {
                VirtualAlloc(
                    Some((candidate & !0xffff) as *const c_void),
                    0x1000,
                    MEM_RESERVE | MEM_COMMIT,
                    PAGE_READWRITE,
                )
            };
            if !page.is_null() {
                return Some(page as usize);
            }
        }
    }
    None
}

pub(crate) unsafe fn seal_page(page: usize) -> Result<(), String> {
    let mut old = PAGE_PROTECTION_FLAGS(0);
    unsafe { VirtualProtect(page as _, 0x1000, PAGE_EXECUTE_READ, &mut old) }
        .map_err(|e| format!("HUD relay protection: {e}"))?;
    unsafe { FlushInstructionCache(GetCurrentProcess(), Some(page as _), 0x1000) }
        .map_err(|e| format!("HUD relay cache: {e}"))?;
    Ok(())
}

pub(crate) unsafe fn commit_optional(patches: &[(usize, Vec<u8>, Vec<u8>)]) -> Result<(), String> {
    let ranges: Vec<_> = patches.iter().map(|(p, b, _)| (*p, b.len())).collect();
    let mut peers = crate::threads::Threads::prepare()?;
    if let Err(e) = peers.freeze(&ranges) {
        peers.thaw();
        return Err(e.into());
    }
    for (p, old, new) in patches {
        if old.len() != new.len()
            || unsafe { std::slice::from_raw_parts(*p as *const u8, old.len()) } != old
        {
            peers.thaw();
            return Err("Optional patch conflict; no changes applied".into());
        }
    }
    for (i, (p, _, new)) in patches.iter().enumerate() {
        if !unsafe { write(*p, new) } {
            let mut restored = true;
            for (p, old, _) in patches[..=i].iter().rev() {
                restored &= unsafe { write(*p, old) };
            }
            peers.thaw();
            return Err(format!(
                "Optional patch failed; rollback={restored}; restart recommended"
            ));
        }
    }
    if !peers.thaw() {
        return Err("HUD thread resume failed; restart required".into());
    }
    Ok(())
}

/// Runs on the startup worker before system/task registration. READY remains false
/// until the game task verifies that no player was loaded before registration.
pub unsafe fn install(multiple: bool) -> Result<(), String> {
    let p = Program::current();
    let va = |rva| {
        p.rva_to_va(rva)
            .map(|v| v as usize)
            .map_err(|e| format!("RVA {rva:x}: {e:?}"))
    };
    let state = va(STATE_RVA)?;
    let summon = va(SUMMON_RVA)?;
    for (rva, expected) in [
        // Pinned WW 2.7.2.0 registry state evidence; checked, never patched.
        (0x3f2d42, &[0x80, 0x78, 0x08, 0x04, 0x75, 0x3a][..]),
        (
            0x3f8ce2,
            &[0x48, 0x8b, 0x43, 0x10, 0xc6, 0x40, 0x08, 0x04][..],
        ),
        (
            0x401439,
            &[0x48, 0x8b, 0x46, 0x10, 0xc6, 0x40, 0x09, 0x04][..],
        ),
        (
            STATE_RVA,
            &[0x48, 0x89, 0x5c, 0x24, 0x08, 0x48, 0x89, 0x74, 0x24, 0x10][..],
        ),
        (
            SUMMON_RVA,
            &[
                0x48, 0x89, 0x5c, 0x24, 0x08, 0x48, 0x89, 0x74, 0x24, 0x10, 0x57, 0x48, 0x83, 0xec,
                0x20,
            ][..],
        ),
        (
            0x4b86ae,
            &[
                0x83, 0xf8, 0x01, 0x76, 0x1b, 0x83, 0xf8, 0x02, 0x75, 0x04, 0xc6, 0x43, 0x28, 0x01,
            ][..],
        ),
    ] {
        if unsafe { std::slice::from_raw_parts(va(rva)? as *const u8, expected.len()) } != expected
        {
            return Err(format!(
                "Function signature mismatch at {rva:x}; no hooks installed"
            ));
        }
    }
    let mut specs = Vec::new();
    for rva in QUERY_SITES {
        specs.push((rva, state, super::runtime::query as *const () as usize));
    }
    for rva in COST_SITES {
        specs.push((rva, state, super::runtime::cost as *const () as usize));
    }
    specs.push((
        SUMMON_SITE,
        summon,
        super::runtime::summon as *const () as usize,
    ));
    if multiple {
        // Native owner-alive predicate excludes members through this exact
        // modifier chain. Checked only, never patched or called on stale actors.
        for (rva, expected) in [
            (
                0x3f4740,
                &[
                    0x48, 0x8b, 0x41, 0x58, 0x48, 0x8b, 0x88, 0xc8, 0, 0, 0, 0x8b, 0x41, 0x24,
                    0x83, 0xe0, 1, 0xc3,
                ][..],
            ),
            (
                0x4b7ca5,
                &[
                    0xe8, 0x96, 0xca, 0xf3, 0xff, 0x84, 0xc0, 0x75, 0x05, 0x38, 0x43, 0x31,
                ][..],
            ),
        ] {
            if unsafe { std::slice::from_raw_parts(va(rva)? as *const u8, expected.len()) }
                != expected
            {
                return Err(format!(
                    "Inactive cleanup predicate signature mismatch at {rva:x}; no hooks installed"
                ));
            }
        }
        // The native item requirement routine independently rejects state -1.
        // Its area, attribute, multiplayer and affordability branches stay native.
        let expected = [
            0x41, 0x8b, 0xd5, 0xe8, 0x98, 0x6b, 0xe2, 0xff, 0x83, 0xf8, 0xff, 0x41, 0x0f, 0x95,
            0xc5,
        ];
        if unsafe {
            std::slice::from_raw_parts(va(ELIGIBILITY_SITE - 3)? as *const u8, expected.len())
        } != expected
        {
            return Err("Item eligibility context signature mismatch".into());
        }
        specs.push((
            ELIGIBILITY_SITE,
            state,
            super::runtime::eligibility as *const () as usize,
        ));
        specs.push((
            INNER_STATE_SITE,
            state,
            super::runtime::inner_summon_state as *const () as usize,
        ));
        specs.push((
            CLEAR_SITE,
            va(CLEAR_RVA)?,
            super::runtime::clear_for_spawn as *const () as usize,
        ));
        let expected = [0xe8, 0xeb, 0x2f, 0, 0, 0x41, 0x83, 0xbf];
        if unsafe { std::slice::from_raw_parts(va(CLEAR_SITE)? as *const u8, expected.len()) }
            != expected
        {
            return Err("Spawn clear context signature mismatch".into());
        }
    }
    let mut changes = Vec::new();
    for (rva, original, callback) in specs {
        let site = va(rva)?;
        let expected = call_bytes(site, original).ok_or("Original out of range")?;
        if unsafe { std::slice::from_raw_parts(site as *const u8, 5) } != expected {
            return Err(format!("Call conflict at {rva:x}; no hooks installed"));
        }
        changes.push((site, expected, callback));
    }
    let page = unsafe { near(changes[0].0) }.ok_or("Cannot allocate rel32 stub page")?;
    let mut patches = Vec::new();
    for (i, (site, old, callback)) in changes.iter().enumerate() {
        let stub = page + i * 16;
        // jmp [rip+0] with an embedded 64-bit target. Preserves registers and the caller's ABI.
        let mut code = vec![0xff, 0x25, 0, 0, 0, 0];
        code.extend_from_slice(&(*callback as u64).to_le_bytes());
        unsafe {
            std::ptr::copy_nonoverlapping(code.as_ptr(), stub as *mut u8, code.len());
        }
        patches.push((
            *site,
            old.to_vec(),
            call_bytes(*site, stub).ok_or("Stub out of range")?.to_vec(),
        ));
    }
    let mut old = PAGE_PROTECTION_FLAGS(0);
    unsafe { VirtualProtect(page as _, 0x1000, PAGE_EXECUTE_READ, &mut old) }
        .map_err(|e| format!("Stub protection: {e}"))?;
    unsafe { FlushInstructionCache(GetCurrentProcess(), Some(page as _), 0x1000) }
        .map_err(|e| format!("Stub cache: {e}"))?;
    if multiple {
        patches.extend(crate::capacity::prepare(&p)?);
        CLEAR.store(va(CLEAR_RVA)?, Ordering::Release);
    }
    // Validate the entire variable-length transaction before freezing threads.
    for (site, old, new) in &patches {
        if old.len() != new.len()
            || unsafe { std::slice::from_raw_parts(*site as *const u8, old.len()) } != old
        {
            return Err(format!("Patch conflict at {:x}; no changes applied", site));
        }
    }
    let ranges: Vec<_> = patches
        .iter()
        .map(|(site, old, _)| (*site, old.len()))
        .collect();
    STATE.store(state, Ordering::Release);
    SUMMON.store(summon, Ordering::Release);
    let mut threads = crate::threads::Threads::prepare()?;
    if let Err(reason) = threads.freeze(&ranges) {
        threads.thaw();
        return Err(reason.into());
    }
    // Recheck while peer threads are stopped, before committing the prepared batch.
    if patches
        .iter()
        .any(|(site, old, _)| unsafe { std::slice::from_raw_parts(*site as *const u8, old.len()) } != old)
    {
        threads.thaw();
        return Err("Call sites changed during preparation; no patches applied".into());
    }
    for (i, (site, _, new)) in patches.iter().enumerate() {
        if !unsafe { write(*site, new) } {
            let mut rollback = true;
            for (site, old, _) in patches[..=i].iter().rev() {
                rollback &= unsafe { write(*site, old) };
            }
            threads.thaw();
            return Err(format!(
                "Patch failed; rollback={rollback}; runtime remains disabled"
            ));
        }
    }
    INSTALLED.store(true, Ordering::Release);
    if !threads.thaw() {
        INSTALLED.store(false, Ordering::Release);
        return Err("Peer thread resume failed; restart required".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn known_native_call_encoding() {
        assert_eq!(
            call_bytes(0x1403fb268, 0x1404b8690),
            Some([0xe8, 0x23, 0xd4, 0x0b, 0])
        );
    }
    #[test]
    fn rel32_range_checked() {
        assert!(call_bytes(0, usize::MAX).is_none());
    }
    #[test]
    fn sites_are_unique() {
        let mut sites = QUERY_SITES.to_vec();
        sites.extend(COST_SITES);
        sites.push(SUMMON_SITE);
        sites.extend([ELIGIBILITY_SITE, INNER_STATE_SITE, CLEAR_SITE]);
        let n = sites.len();
        sites.sort();
        sites.dedup();
        assert_eq!(n, sites.len());
    }
}
