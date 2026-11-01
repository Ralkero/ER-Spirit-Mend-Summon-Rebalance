// Included in runtime. Explicit solo-only exception, not connected compatibility.
static SEAMLESS_SOLO_MODULE: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

pub(crate) fn accepted_seamless_solo_module() -> usize {
    SEAMLESS_SOLO_MODULE.load(Ordering::Acquire)
}

fn seamless_conflict() -> bool {
    let module = unsafe { module_handle_raw(windows::core::w!("ersc.dll")) }.0 as usize;
    let accepted = SEAMLESS_SOLO_MODULE.load(Ordering::Acquire);
    if accepted == 0 { module != 0 } else { module != accepted }
}

fn initialize_seamless_solo() -> Result<(), String> {
    let module = unsafe { module_handle_raw(windows::core::w!("ersc.dll")) };
    if module.0.is_null() || !config().seamless_solo {
        return Ok(()); // The ordinary conflict guard still rejects ersc by default.
    }
    // Once on startup worker, never in a game callback. Hash both files before
    // granting the exception. All original live hook signatures are still required.
    use sha2::{Digest, Sha256};
    use std::io::Read;
    for (handle, expected, name) in [
        (None, "f9118fea2cfd8fa3901e8060bfadb1552cb2471771ded8a5b662c4e298ffb328", "game"),
        (Some(module), "0a093d8490967bf19c62f895fdf1fda21840282d2b2b371b9b4fc24a0cfe146e", "Seamless"),
    ] {
        let mut path = [0u16; 32768];
        let len = unsafe { GetModuleFileNameW(handle, &mut path) } as usize;
        if len == 0 || len >= path.len() { return Err(format!("Cannot identify {name} file")); }
        let mut file = std::fs::File::open(PathBuf::from(OsString::from_wide(&path[..len])))
            .map_err(|e| format!("Cannot read {name}: {e}"))?;
        let mut digest = Sha256::new();
        let mut buffer = [0u8; 65536];
        loop {
            let n = file.read(&mut buffer).map_err(|e| format!("Cannot hash {name}: {e}"))?;
            if n == 0 { break; }
            digest.update(&buffer[..n]);
        }
        if format!("{:x}", digest.finalize()) != expected {
            return Err(format!("Unsupported {name} SHA256 for AllowSeamlessSolo"));
        }
    }
    SEAMLESS_SOLO_MODULE.store(module.0 as usize, Ordering::Release);
    log("Seamless SOLO exception: pinned files accepted; original hook checks, offline flag and verified solo membership remain required; only lobby=0 with protocol=0 or 6 accepted; remote or unknown session membership rejects gameplay");
    Ok(())
}

// Offsets corroborated by pinned Seamless code at RVAs 26ed9/26edd,
// 26f00/26f0c and 9077b..907a5: vector start/end, 0x100 stride, local flag.
const _: () = {
    use eldenring::cs::{SessionManagerPlayerEntry, SessionManagerPlayerEntryBase};
    assert!(std::mem::offset_of!(CSSessionManager, players) == 0x70);
    assert!(std::mem::offset_of!(CSSessionManager, host_player) == 0x90);
    assert!(std::mem::size_of::<SessionManagerPlayerEntry>() == 0x100);
    assert!(std::mem::offset_of!(SessionManagerPlayerEntry, is_local_player) == 0xe9);
    assert!(std::mem::offset_of!(SessionManagerPlayerEntryBase, steam_id) == 0x10);
};

fn seamless_solo_members(session: &CSSessionManager) -> Option<crate::session_policy::SoloMembers> {
    use crate::session_policy::{SoloMembers, solo_vector_count};
    use eldenring::cs::SessionManagerPlayerEntry;
    // Same native callback lifetime as the previous player-list guard. No stored
    // actor/list pointers, OS probes, allocations, or high-frequency logging.
    // Validate the header before SDK len()/slice construction could use it.
    let header = std::ptr::addr_of!(session.players).cast::<usize>();
    let (first, last, end) = unsafe {
        (header.add(1).read(), header.add(2).read(), header.add(3).read())
    };
    let count = solo_vector_count(first, last, end)?;
    if count > 1 {
        return Some(SoloMembers { count, all_local: false, host_is_local_or_empty: false });
    }
    let host_id = unsafe { session.host_player.as_non_null().cast::<u64>().as_ptr().add(2).read() };
    if count == 0 {
        return Some(SoloMembers { count, all_local: true, host_is_local_or_empty: host_id == 0 });
    }
    // Raw u8 rejects malformed native bools without materializing an invalid bool.
    let entry = first as *const SessionManagerPlayerEntry;
    let local = unsafe { std::ptr::addr_of!((*entry).is_local_player).cast::<u8>().read() };
    let local_id = unsafe { std::ptr::addr_of!((*entry).base.steam_id).read() };
    Some(SoloMembers {
        count,
        all_local: local == 1,
        host_is_local_or_empty: host_id == 0 || (local_id != 0 && host_id == local_id),
    })
}
