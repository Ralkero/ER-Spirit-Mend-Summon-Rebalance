//! Capacity/search adaptation from ArialSenki MultipleSpiritAshes, MIT OR Apache-2.0.
//! Only the actor pool and search are changed; no area/range patches are included.
use fromsoftware_shared::program::Program;
use pelite::pe64::Pe;
use windows::Win32::System::{
    Diagnostics::Debug::FlushInstructionCache,
    Memory::{PAGE_EXECUTE_READ, PAGE_PROTECTION_FLAGS, VirtualProtect},
    Threading::GetCurrentProcess,
};
pub const ENTRIES: u32 = 520;
pub const SEAMLESS_ENTRIES: u32 = 1000;

/// Only the startup hash-pinned solo profile may use the observed larger pool.
pub fn expected_entries() -> u32 {
    if crate::runtime::accepted_seamless_solo_module() != 0 {
        SEAMLESS_ENTRIES
    } else {
        ENTRIES
    }
}

pub const FIRST_SPIRIT_SLOT: usize = 20;
pub const MAX_GROUP_MEMBERS: usize = (ENTRIES as usize) - FIRST_SPIRIT_SLOT;
pub type Patch = (usize, Vec<u8>, Vec<u8>);
const SLOT_LOOP: [u8; 57] = [
    0x41, 0x83, 0xf9, 0x0a, 0x7d, 0x0a, 0x43, 0x8d, 0x3c, 0x11, 0x49, 0x8d, 0x1c, 0x12, 0xeb, 0x0e,
    0x41, 0x8d, 0x7a, 0xf6, 0x41, 0x03, 0xf9, 0x49, 0x8d, 0x5a, 0xf6, 0x48, 0x03, 0xda, 0x48, 0x03,
    0xd9, 0x48, 0xc1, 0xe3, 0x04, 0x49, 0x03, 0xd8, 0x4c, 0x39, 0x3b, 0x74, 0x14, 0x41, 0xff, 0xc1,
    0x48, 0xff, 0xc1, 0x48, 0x83, 0xf9, 0x0a, 0x7c, 0xc7,
];
/// Prepared before the startup transaction. The constructor epilogue expands
/// only its freshly built empty registry, using the game's allocator/free pair.
/// No published registry is resized. The pinned Seamless solo profile instead
/// preserves its existing constructor and requires its exact 1000-entry pool.
pub fn prepare(
    p: &Program,
    guards: &mut Vec<crate::seamless_hooks::ReadGuard>,
) -> Result<Vec<Patch>, String> {
    let va = |rva| {
        p.rva_to_va(rva)
            .map(|x| x as usize)
            .map_err(|e| format!("capacity RVA: {e:?}"))
    };
    let mut patches = Vec::new();
    // One epilogue redirect avoids mixing old allocation sizes with new loop
    // bounds if a constructor was already running during patch installation.
    if let Some(patch) = prepare_constructor(p, guards)? {
        patches.push(patch);
    }
    patches.push((
        va(0x493492)?,
        vec![0x83, 0xc0, 2, 0x8d, 4, 0x80, 0x44, 0x8d, 0x14, 0],
        vec![0x41, 0xba, 0x14, 0, 0, 0, 0x0f, 0x1f, 0x40, 0],
    ));
    // Check surrounding allocation/iteration instructions, not just immediates.
    for (rva, signature) in [
        (
            0x495619,
            &[
                0x41, 0xb8, 8, 0, 0, 0, 0xff, 0x50, 0x50, 0x48, 0x89, 0x46, 0x18,
            ][..],
        ),
        (0x495630, &[0x89, 0x7e, 0x10, 0x33, 0xd2][..]),
        (
            0x49566b,
            &[0x48, 0x83, 0xc3, 0x10, 0x48, 0x83, 0xef, 1, 0x75, 0xdb][..],
        ),
        (0x4934f9, &[0x49, 0x8b, 0xc7, 0xe9, 0x9d, 1, 0, 0][..]),
    ] {
        if unsafe { std::slice::from_raw_parts(va(rva)? as *const u8, signature.len()) }
            != signature
        {
            return Err(format!("Capacity context conflict at {rva:x}"));
        }
    }
    let site = va(0x4934c0)?;
    let cave = unsafe { crate::hooks::near(site) }.ok_or("Cannot allocate slot-search stub")?;
    let mut body = vec![0x41, 0x8b, 0x46, 0x10, 0x83, 0xe8, 0x14];
    let loop_start = body.len();
    body.extend([
        0x41, 0x39, 0xc1, 0x43, 0x8d, 0x3c, 0x11, 0x7c, 2, 0x29, 0xc7, 0x48, 0x63, 0xdf, 0x48,
        0xc1, 0xe3, 4, 0x4c, 1, 0xc3, 0x4c, 0x39, 0x3b, 0x74, 0,
    ]);
    let found = body.len();
    body.extend([
        0x41, 0xff, 0xc1, 0x48, 0xff, 0xc1, 0x48, 0x39, 0xc1, 0x7c, 0,
    ]);
    let back = body.len();
    body[back - 1] = (loop_start as isize - back as isize) as u8;
    body.extend([0xff, 0x25, 0, 0, 0, 0]);
    body.extend(((site + 57) as u64).to_le_bytes());
    body[found - 1] = (body.len() - found) as u8;
    body.extend([0xff, 0x25, 0, 0, 0, 0]);
    body.extend(((site + 65) as u64).to_le_bytes());
    unsafe {
        std::ptr::copy_nonoverlapping(body.as_ptr(), cave as *mut u8, body.len());
    }
    let mut old = PAGE_PROTECTION_FLAGS(0);
    unsafe { VirtualProtect(cave as _, 0x1000, PAGE_EXECUTE_READ, &mut old) }
        .map_err(|e| format!("Search stub protection: {e}"))?;
    unsafe { FlushInstructionCache(GetCurrentProcess(), Some(cave as _), body.len()) }
        .map_err(|e| format!("Search stub cache: {e}"))?;
    let mut jump = vec![0x90; 57];
    jump[0] = 0xe9;
    let displacement =
        i32::try_from(cave as i128 - (site as i128 + 5)).map_err(|_| "Search stub range")?;
    jump[1..5].copy_from_slice(&displacement.to_le_bytes());
    patches.push((site, SLOT_LOOP.to_vec(), jump));
    Ok(patches)
}

const VANILLA_CONSTRUCTOR: &[u8] = &[
    0x48, 0x8b, 0x0d, 0x6f, 0x8d, 0x8f, 0x03, 0x48, 0x8b, 0x01, 0xba, 0x00, 0x05, 0x00, 0x00, 0x41,
    0xb8, 0x08, 0x00, 0x00, 0x00, 0xff, 0x50, 0x50, 0x48, 0x89, 0x46, 0x18, 0x48, 0x85, 0xc0, 0x74,
    0x4a, 0xbf, 0x50, 0x00, 0x00, 0x00, 0x89, 0x7e, 0x10, 0x33, 0xd2, 0x41, 0xb8, 0x00, 0x05, 0x00,
    0x00, 0x48, 0x8b, 0xc8, 0xe8, 0x4d, 0xac, 0x08, 0x02, 0x33, 0xdb, 0x66, 0x66, 0x66, 0x0f, 0x1f,
    0x84, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x8b, 0x4e, 0x18, 0x48, 0x03, 0xcb, 0x48, 0x89, 0x4c,
    0x24, 0x58, 0x48, 0x89, 0x4c, 0x24, 0x60, 0x74, 0x08, 0x33, 0xd2, 0xe8, 0xa6, 0xe1, 0xf2, 0xff,
    0x90, 0x48, 0x83, 0xc3, 0x10, 0x48, 0x83, 0xef, 0x01, 0x75, 0xdb, 0x48, 0x83, 0xc4, 0x30, 0x5f,
    0x5e, 0x5b, 0xc3,
];

fn constructor_signature(seamless: bool) -> Vec<u8> {
    let mut signature = VANILLA_CONSTRUCTOR.to_vec();
    if seamless {
        signature[11..15].copy_from_slice(&(SEAMLESS_ENTRIES * 16).to_le_bytes());
        signature[34..38].copy_from_slice(&SEAMLESS_ENTRIES.to_le_bytes());
        signature[45..49].copy_from_slice(&(SEAMLESS_ENTRIES * 16).to_le_bytes());
    }
    signature
}

// Adapted from upstream spirit_set_size; original notices retained in UPSTREAM.md.
fn prepare_constructor(
    p: &Program,
    guards: &mut Vec<crate::seamless_hooks::ReadGuard>,
) -> Result<Option<Patch>, String> {
    let seamless = crate::runtime::accepted_seamless_solo_module() != 0;
    let init = p
        .rva_to_va(0x49560a)
        .map_err(|e| format!("Constructor RVA: {e:?}"))? as usize;
    // Pin the complete constructor, including allocator and entry constructor.
    let expected = constructor_signature(seamless);
    guards.push(crate::seamless_hooks::capture_exact(init, &expected)?);
    if seamless {
        crate::runtime::log(format!(
            "Verified pinned Seamless registry constructor: preserving {SEAMLESS_ENTRIES} entries; local tracking limit {MAX_GROUP_MEMBERS}; no constructor patch"
        ));
        return Ok(None);
    }
    let site = init + 0x6b;
    let allocator = (init + 7)
        .checked_add_signed(i32::from_le_bytes(expected[3..7].try_into().unwrap()) as isize)
        .ok_or("Allocator displacement overflow")? as u64;
    let cave = unsafe { crate::hooks::near(site) }.ok_or("Cannot allocate constructor stub")?;
    let bytes = ENTRIES * 16;

    let mut body: Vec<u8> = vec![0x83, 0x7e, 0x10, 0x50, 0x75, 0x00]; // cmp dword [rsi+10h],80; jne done
    let mut to_done = vec![body.len()];
    body.extend_from_slice(&[0x48, 0x83, 0x7e, 0x18, 0x00, 0x74, 0x00]); // cmp qword [rsi+18h],0; je done
    to_done.push(body.len());
    body.extend_from_slice(&[0x48, 0xb9]); // mov rcx,&allocator
    body.extend_from_slice(&allocator.to_le_bytes());
    body.extend_from_slice(&[0x48, 0x8b, 0x09, 0x48, 0x8b, 0x01, 0xba]); // mov rcx,[rcx]; mov rax,[rcx]; mov edx,
    body.extend_from_slice(&bytes.to_le_bytes());
    body.extend_from_slice(&[
        0x41, 0xb8, 0x08, 0x00, 0x00, 0x00, // mov r8d,8
        0xff, 0x50, 0x50, // call [rax+50h]      allocate
        0x48, 0x85, 0xc0, 0x74, 0x00, // test rax,rax; je done
    ]);
    to_done.push(body.len());
    // mov rbx,rax; mov rdi,rax; xor eax,eax; mov ecx,
    body.extend_from_slice(&[0x48, 0x89, 0xc3, 0x48, 0x89, 0xc7, 0x31, 0xc0, 0xb9]);
    body.extend_from_slice(&(bytes / 8).to_le_bytes());
    body.extend_from_slice(&[
        0xf3, 0x48, 0xab, // rep stosq           zeroed entries
        0x48, 0x8b, 0x56, 0x18, // mov rdx,[rsi+18h]   the 80
        0x48, 0x89, 0x5e, 0x18, // mov [rsi+18h],rbx
        0xc7, 0x46, 0x10, // mov dword [rsi+10h],
    ]);
    body.extend_from_slice(&ENTRIES.to_le_bytes());
    body.extend_from_slice(&[0x48, 0xb9]); // mov rcx,&allocator
    body.extend_from_slice(&allocator.to_le_bytes());
    // mov rcx,[rcx]; mov rax,[rcx]; call [rax+68h]   free
    body.extend_from_slice(&[0x48, 0x8b, 0x09, 0x48, 0x8b, 0x01, 0xff, 0x50, 0x68]);
    for at in to_done {
        body[at - 1] = (body.len() - at) as u8;
    }
    body.extend_from_slice(&[0x48, 0x83, 0xc4, 0x30, 0x5f, 0x5e, 0x5b, 0xc3]); // done: the epilogue

    unsafe {
        std::ptr::copy_nonoverlapping(body.as_ptr(), cave as *mut u8, body.len());
    }
    let mut old = PAGE_PROTECTION_FLAGS(0);
    unsafe { VirtualProtect(cave as _, 0x1000, PAGE_EXECUTE_READ, &mut old) }
        .map_err(|e| format!("Constructor stub protection: {e}"))?;
    unsafe { FlushInstructionCache(GetCurrentProcess(), Some(cave as _), body.len()) }
        .map_err(|e| format!("Constructor stub cache: {e}"))?;
    let mut jump = vec![0x90; 8];
    jump[0] = 0xe9;
    let delta =
        i32::try_from(cave as i128 - (site as i128 + 5)).map_err(|_| "Constructor stub range")?;
    jump[1..5].copy_from_slice(&delta.to_le_bytes());
    Ok(Some((site, expected[0x6b..].to_vec(), jump)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_profiles_preserve_all_instructions() {
        let vanilla = constructor_signature(false);
        let seamless = constructor_signature(true);
        assert_eq!(vanilla, VANILLA_CONSTRUCTOR);
        assert_eq!(seamless.len(), 115);
        for (offset, expected) in [(11, 16000u32), (34, 1000), (45, 16000)] {
            assert_eq!(&seamless[offset..offset + 4], &expected.to_le_bytes());
        }
        for i in 0..115 {
            if !(11..15).contains(&i) && !(34..38).contains(&i) && !(45..49).contains(&i) {
                assert_eq!(vanilla[i], seamless[i]);
            }
        }
    }
}
