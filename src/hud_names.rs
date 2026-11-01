//! Bounded read-only lookup in the game's already-loaded FMG tables.
//! Used only when the public name wrapper is redirected in the pinned solo profile.
//! No native function calls, new hooks, retained native pointers, or asset files.
use sha2::{Digest, Sha256};

pub fn layout_matches(wrapper: &[u8], leaf: &[u8]) -> bool {
    wrapper.len() == 49
        && wrapper[0] == 0xe9
        && leaf.len() == 120
        && format!("{:x}", Sha256::digest(&wrapper[5..]))
            == "f23aeb8c9c504a07b8f316cb251e6ffa885c9c20e4d3f77d91e34ebd1235ba96"
        && format!("{:x}", Sha256::digest(leaf))
            == "e81100b62772d9b841937d8572d8dd746ba7d29110ca86d2969f52d1f9183b8d"
}
fn block<const N: usize>(
    read: &mut impl FnMut(usize, &mut [u8]) -> bool,
    at: usize,
) -> Option<[u8; N]> {
    if at < 0x10000 || at.checked_add(N)? > 0x0000_8000_0000_0000 {
        return None;
    }
    let mut b = [0; N];
    read(at, &mut b).then_some(b)
}
fn u32_at(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes(b[off..off + 4].try_into().unwrap())
}
fn u64_at(b: &[u8], off: usize) -> usize {
    u64::from_le_bytes(b[off..off + 8].try_into().unwrap()) as usize
}

fn text(read: &mut impl FnMut(usize, &mut [u8]) -> bool, base: usize, id: u32) -> Option<String> {
    let h = block::<40>(read, base)?;
    // FMG v2, little-endian Unicode; the engine relocates only the offsets-table pointer.
    if h[..4] != [0, 0, 2, 0] || h[8..12] != [1, 0, 0, 0] || u32_at(&h, 20) != 0xff {
        return None;
    }
    let size = u32_at(&h, 4) as usize;
    let groups = u32_at(&h, 12) as usize;
    let strings = u32_at(&h, 16) as usize;
    if !(40..=16 * 1024 * 1024).contains(&size)
        || groups == 0
        || groups > 65536
        || strings == 0
        || strings > 100000
    {
        return None;
    }
    let end = base.checked_add(size)?;
    let groups_end = base.checked_add(40 + groups * 16)?;
    let offsets = u64_at(&h, 24);
    if groups_end > end || offsets < groups_end || offsets.checked_add(strings * 8)? > end {
        return None;
    }
    let mut lo = 0usize;
    let mut hi = groups;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let g = block::<16>(read, base.checked_add(40 + mid * 16)?)?;
        let first = u32_at(&g, 4);
        let last = u32_at(&g, 8);
        if first > last {
            return None;
        }
        if id < first {
            hi = mid;
            continue;
        }
        if id > last {
            lo = mid + 1;
            continue;
        }
        let index = (u32_at(&g, 0) as usize).checked_add((id - first) as usize)?;
        if index >= strings {
            return None;
        }
        let relative = u64_at(&block::<8>(read, offsets.checked_add(index * 8)?)?, 0);
        let string_start = base.checked_add(relative)?;
        if relative == 0
            || string_start < offsets.checked_add(strings * 8)?
            || string_start >= end
            || !relative.is_multiple_of(2)
        {
            return None;
        }
        let mut bytes = [0u8; 256];
        let n = (end - string_start).min(bytes.len()) & !1;
        if n < 2 || !read(string_start, &mut bytes[..n]) {
            return None;
        }
        let units: Vec<u16> = bytes[..n]
            .as_chunks::<2>()
            .0
            .iter()
            .map(|p| u16::from_le_bytes([p[0], p[1]]))
            .collect();
        let length = units.iter().position(|u| *u == 0)?;
        if length == 0 {
            return None;
        }
        let name = String::from_utf16(&units[..length]).ok()?;
        return (!name.starts_with('?')
            && name != "NULL"
            && !name.chars().all(|c| c.is_ascii_digit()))
        .then_some(name);
    }
    None
}

pub fn lookup(
    mut read: impl FnMut(usize, &mut [u8]) -> bool,
    repo: usize,
    id: i32,
) -> Option<String> {
    if id <= 0 {
        return None;
    }
    let h = block::<24>(&mut read, repo)?;
    let languages = u32_at(&h, 16);
    let categories = u32_at(&h, 20);
    if languages == 0 || languages > 64 || categories == 0 || categories > 2048 {
        return None;
    }
    let tables = u64_at(&h, 8);
    // Same active-language slot 0 as the original wrapper's caller.
    let table = u64_at(&block::<8>(&mut read, tables)?, 0);
    for category in [111usize, 10, 319, 419] {
        if category >= categories as usize {
            continue;
        }
        let at = table.checked_add(category * 8)?;
        let Some(pointer) = block::<8>(&mut read, at) else {
            continue;
        };
        if let Some(name) = text(&mut read, u64_at(&pointer, 0), id as u32) {
            return Some(name);
        }
    }
    None
}

#[cfg(windows)]
pub fn native_lookup(repo: usize, id: i32) -> Option<String> {
    use std::ffi::c_void;
    #[link(name = "kernel32")]
    unsafe extern "system" {
        #[link_name = "ReadProcessMemory"]
        fn rpm(
            process: *mut c_void,
            address: *const c_void,
            buffer: *mut c_void,
            size: usize,
            copied: *mut usize,
        ) -> i32;
    }
    lookup(
        |at, output| {
            let mut copied = 0;
            let ok = unsafe {
                rpm(
                    -1isize as *mut c_void,
                    at as *const c_void,
                    output.as_mut_ptr().cast(),
                    output.len(),
                    &mut copied,
                )
            };
            ok != 0 && copied == output.len()
        },
        repo,
        id,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    const BASE: usize = 0x10000;
    fn fixture() -> Vec<u8> {
        let mut b = vec![0; 128];
        b[..4].copy_from_slice(&[0, 0, 2, 0]);
        b[4..8].copy_from_slice(&128u32.to_le_bytes());
        b[8] = 1;
        b[12..16].copy_from_slice(&1u32.to_le_bytes());
        b[16..20].copy_from_slice(&1u32.to_le_bytes());
        b[20..24].copy_from_slice(&255u32.to_le_bytes());
        b[24..32].copy_from_slice(&((BASE + 56) as u64).to_le_bytes());
        b[44..48].copy_from_slice(&232000u32.to_le_bytes());
        b[48..52].copy_from_slice(&232000u32.to_le_bytes());
        b[56..64].copy_from_slice(&64u64.to_le_bytes());
        for (i, u) in "Lone Wolf Ashes".encode_utf16().chain([0]).enumerate() {
            b[64 + i * 2..66 + i * 2].copy_from_slice(&u.to_le_bytes());
        }
        b
    }
    fn get(b: &[u8], id: u32) -> Option<String> {
        text(
            &mut |at, out| {
                let Some(offset) = at.checked_sub(BASE) else {
                    return false;
                };
                let Some(src) = b.get(offset..offset + out.len()) else {
                    return false;
                };
                out.copy_from_slice(src);
                true
            },
            BASE,
            id,
        )
    }
    #[test]
    fn copied_fmg_resolves_exact_id_and_rejects_missing() {
        let b = fixture();
        assert_eq!(get(&b, 232000).as_deref(), Some("Lone Wolf Ashes"));
        assert_eq!(get(&b, 232001), None);
        assert_eq!(get(&b, 231999), None);
    }
    #[test]
    fn malformed_tables_and_unreadable_memory_reject() {
        for off in [2, 8, 12, 20, 24, 40, 56] {
            let mut b = fixture();
            b[off..off + 4].fill(255);
            assert_eq!(get(&b, 232000), None);
        }
        assert_eq!(get(&fixture()[..63], 232000), None);
        assert_eq!(lookup(|_, _| false, BASE, 232000), None);
    }
    #[test]
    fn unterminated_or_invalid_utf16_rejects() {
        let mut b = fixture();
        for pair in b[64..].chunks_exact_mut(2) {
            pair.copy_from_slice(&65u16.to_le_bytes());
        }
        assert_eq!(get(&b, 232000), None);
        b[64..66].copy_from_slice(&0xd800u16.to_le_bytes());
        b[66..68].fill(0);
        assert_eq!(get(&b, 232000), None);
    }
}
