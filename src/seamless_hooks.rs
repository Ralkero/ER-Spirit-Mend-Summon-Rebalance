//! Accept only the two observed redirects in the pinned Seamless solo build.
//! We never replace these function entries or invoke private Seamless callbacks:
//! our original-call pointers still enter the game's existing Seamless chain.
use sha2::{Digest, Sha256};
use std::ffi::c_void;
use windows::Win32::System::Memory::{
    MEM_COMMIT, MEMORY_BASIC_INFORMATION, PAGE_GUARD, PAGE_NOACCESS, VirtualQuery,
};

#[link(name = "kernel32")]
unsafe extern "system" {
    #[link_name = "ReadProcessMemory"]
    fn read_process_memory(
        process: *mut c_void,
        address: *const c_void,
        output: *mut c_void,
        size: usize,
        copied: *mut usize,
    ) -> i32;
}

pub struct ReadGuard {
    address: usize,
    expected: Vec<u8>,
    scratch: Vec<u8>,
}
impl ReadGuard {
    /// No allocations, calls into game code, or pointer dereferences. This also
    /// runs with peer threads frozen immediately before the patch transaction.
    pub fn matches(&mut self) -> bool {
        let mut copied = 0;
        let ok = unsafe {
            read_process_memory(
                -1isize as *mut c_void,
                self.address as *const c_void,
                self.scratch.as_mut_ptr().cast(),
                self.scratch.len(),
                &mut copied,
            )
        };
        ok != 0 && copied == self.scratch.len() && self.expected == self.scratch
    }
}

fn capture(address: usize, length: usize, code: bool) -> Result<ReadGuard, String> {
    if length == 0 || length > 256 || address < 0x10000 {
        return Err("Invalid Seamless redirect read range".into());
    }
    let end = address
        .checked_add(length)
        .ok_or("Seamless range overflow")?;
    let mut info = MEMORY_BASIC_INFORMATION::default();
    if unsafe {
        VirtualQuery(
            Some(address as *const c_void),
            &mut info,
            std::mem::size_of_val(&info),
        )
    } == 0
        || info.State != MEM_COMMIT
        || info.Protect.0 & (PAGE_GUARD.0 | PAGE_NOACCESS.0) != 0
        || (code && info.Protect.0 & 0xf0 == 0)
        || end > (info.BaseAddress as usize).saturating_add(info.RegionSize)
    {
        return Err("Seamless redirect memory mapping unavailable".into());
    }
    let mut guard = ReadGuard {
        address,
        expected: vec![0; length],
        scratch: vec![0; length],
    };
    let mut copied = 0;
    if unsafe {
        read_process_memory(
            -1isize as *mut c_void,
            address as *const c_void,
            guard.expected.as_mut_ptr().cast(),
            length,
            &mut copied,
        )
    } == 0
        || copied != length
    {
        return Err("Cannot read Seamless redirect".into());
    }
    Ok(guard)
}
/// Capture an exact instruction profile for revalidation during the transaction.
pub(crate) fn capture_exact(address: usize, expected: &[u8]) -> Result<ReadGuard, String> {
    let guard = capture(address, expected.len(), true)?;
    if guard.expected != expected {
        return Err("Summon registry constructor conflict".into());
    }
    Ok(guard)
}

fn pointer(bytes: &[u8]) -> Option<usize> {
    Some(u64::from_le_bytes(bytes.try_into().ok()?) as usize)
}
fn rel32_target(site: usize, bytes: &[u8]) -> Option<usize> {
    if bytes.len() < 5 || bytes[0] != 0xe9 {
        return None;
    }
    site.checked_add(5)?
        .checked_add_signed(i32::from_le_bytes(bytes[1..5].try_into().ok()?) as isize)
}
fn bridge_target(bytes: &[u8]) -> Option<usize> {
    if bytes.get(..6) == Some(&[0xff, 0x25, 0, 0, 0, 0]) {
        pointer(bytes.get(6..14)?)
    } else if bytes.get(..2) == Some(&[0x48, 0xb8]) && bytes.get(10..12) == Some(&[0xff, 0xe0]) {
        pointer(bytes.get(2..10)?)
    } else {
        None
    }
}
fn closure(bytes: &[u8]) -> Option<(usize, usize)> {
    if bytes.get(..2) != Some(&[0x48, 0xb8])
        || bytes.get(10..17) != Some(&[0x66, 0x48, 0x0f, 0x6e, 0xe8, 0x49, 0xbb])
        || bytes.get(25..28) != Some(&[0x41, 0xff, 0xe3])
    {
        return None;
    }
    Some((pointer(bytes.get(2..10)?)?, pointer(bytes.get(17..25)?)?))
}

/// Startup-only allowlist. Acceptance requires the hash-pinned DLL exception,
/// untouched remainder of the native prologue, observed relay/closure shapes,
/// exact context identity fields, and full handler/callback code fingerprints.
pub fn validate_entry(rva: u32, site: usize, original: &[u8]) -> Result<Vec<ReadGuard>, String> {
    let module = crate::runtime::accepted_seamless_solo_module();
    if module == 0 {
        return Err("No hash-verified Seamless solo exception".into());
    }
    let (handler, handler_len, handler_hash, callback, callback_len, callback_hash) = match rva {
        0x4b73d0 => (
            0x96ed0,
            99,
            "284f12a5ace0ef8192dfb86ed25f07025f7612e1ba4aca6250df95d702258dcd",
            0x8b4f0,
            141,
            "22287a11b829e16259577973b2c67ca65ab0c8a0f6e24561cb600f1533224a2f",
        ),
        0x4b8690 => (
            0x96480,
            101,
            "7b8aa999303193257098f782f9d93ab2273c8a9fafae0a902a5cd4a537419aa5",
            0x8b4c0,
            46,
            "a638a466305fd5d4f01b3abb0d8d12f78ce58b5058d6d67ce43124f7cee21787",
        ),
        _ => return Err("Not a reviewed Seamless entry".into()),
    };
    let head = capture(site, original.len(), true)?;
    if original.len() < 5 || head.expected[5..] != original[5..] {
        return Err("Native prologue tail changed beyond Seamless's rel32 redirect".into());
    }
    let relay_address =
        rel32_target(site, &head.expected).ok_or("Unrecognized Seamless entry jump")?;
    let relay = capture(relay_address, 14, true)?;
    let closure_address = bridge_target(&relay.expected).ok_or("Unrecognized Seamless relay")?;
    if closure_address == relay_address || closure_address == site {
        return Err("Cyclic Seamless relay".into());
    }
    let thunk = capture(closure_address, 28, true)?;
    let (context, destination) = closure(&thunk.expected).ok_or("Unrecognized Seamless closure")?;
    if destination != module.checked_add(handler).ok_or("Handler overflow")? {
        return Err("Seamless redirect points to an unreviewed handler".into());
    }
    let mut guards = vec![head, relay, thunk];
    for (offset, expected) in [
        (0, module + 0x1dce50),
        (16, module + callback),
        (56, context),
    ] {
        let field = capture(
            context.checked_add(offset).ok_or("Context overflow")?,
            8,
            false,
        )?;
        if pointer(&field.expected) != Some(expected) {
            return Err("Seamless closure context identity mismatch".into());
        }
        guards.push(field);
    }
    for (offset, len, hash) in [
        (handler, handler_len, handler_hash),
        (callback, callback_len, callback_hash),
    ] {
        let body = capture(module + offset, len, true)?;
        if format!("{:x}", Sha256::digest(&body.expected)) != hash {
            return Err("Seamless native handler/callback fingerprint mismatch".into());
        }
        guards.push(body);
    }
    crate::runtime::log(format!(
        "Verified pinned Seamless redirect at {rva:x}; original function entry retained; {} read guards",
        guards.len()
    ));
    Ok(guards)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rel32_bounds_and_opcode_are_checked() {
        assert_eq!(rel32_target(100, &[0xe9, 5, 0, 0, 0]), Some(110));
        assert_eq!(
            rel32_target(100, &[0xe9, 0xfb, 0xff, 0xff, 0xff]),
            Some(100)
        );
        assert_eq!(rel32_target(usize::MAX, &[0xe9, 0, 0, 0, 0]), None);
        assert_eq!(rel32_target(0, &[0xe9, 0xf0, 0xff, 0xff, 0xff]), None);
        assert_eq!(rel32_target(100, &[0xe8, 0, 0, 0, 0]), None);
    }
    #[test]
    fn only_observed_bridge_encodings_are_accepted() {
        let mut indirect = vec![0xff, 0x25, 0, 0, 0, 0];
        indirect.extend(0x12345678u64.to_le_bytes());
        assert_eq!(bridge_target(&indirect), Some(0x12345678));
        indirect[2] = 1;
        assert_eq!(bridge_target(&indirect), None);
        let mut absolute = vec![0x48, 0xb8];
        absolute.extend(0x12345678u64.to_le_bytes());
        absolute.extend([0xff, 0xe0]);
        assert_eq!(bridge_target(&absolute), Some(0x12345678));
        absolute[11] = 0xd0;
        assert_eq!(bridge_target(&absolute), None);
        assert_eq!(bridge_target(&[0xff, 0x25]), None);
    }
    #[test]
    fn closure_requires_xmm5_context_and_r11_tail_jump() {
        let mut bytes = vec![0x48, 0xb8];
        bytes.extend(0x12345678u64.to_le_bytes());
        bytes.extend([0x66, 0x48, 0x0f, 0x6e, 0xe8, 0x49, 0xbb]);
        bytes.extend(0x87654321u64.to_le_bytes());
        bytes.extend([0x41, 0xff, 0xe3]);
        assert_eq!(closure(&bytes), Some((0x12345678, 0x87654321)));
        bytes[14] = 0xe0;
        assert_eq!(closure(&bytes), None);
    }
}
