from pathlib import Path
p=Path('outputs/SpiritAshResummon/src/hooks.rs')
s=p.read_text()
s=s.replace('pub static READY:', 'pub static INSTALLED: AtomicBool = AtomicBool::new(false);\npub static CLEAR: AtomicUsize = AtomicUsize::new(0);\npub const INNER_STATE_SITE: u32 = 0x4b86a9;\npub const CLEAR_SITE: u32 = 0x4b8a90;\npub const CLEAR_RVA: u32 = 0x4bba80;\npub static READY:')
s=s.replace('unsafe fn near(', 'pub(crate) unsafe fn near(')
s=s.replace('pub unsafe fn install() -> Result<(), String> {', 'pub unsafe fn install(multiple: bool) -> Result<(), String> {')
s=s.replace('/// Must run during startup on a game task, before a player exists.', '/// Runs on the startup worker before system/task registration. READY remains false\n/// until the game task verifies that no player was loaded before registration.')
s=s.replace('    let mut changes = Vec::new();', '''    if multiple {
        specs.push((INNER_STATE_SITE, state, super::runtime::inner_summon_state as *const () as usize));
        specs.push((CLEAR_SITE, va(CLEAR_RVA)?, super::runtime::clear_for_spawn as *const () as usize));
        let expected = [0xe8, 0xeb, 0x2f, 0, 0, 0x41, 0x83, 0xbf];
        if unsafe { std::slice::from_raw_parts(va(CLEAR_SITE)? as *const u8, expected.len()) } != expected {
            return Err("Spawn clear context signature mismatch".into());
        }
    }
    let mut changes = Vec::new();''')
s=s.replace('            *old,\n            call_bytes(*site, stub).ok_or("Stub out of range")?,','            old.to_vec(),\n            call_bytes(*site, stub).ok_or("Stub out of range")?.to_vec(),')
s=s.replace('    STATE.store(state, Ordering::Release);', '''    if multiple {
        patches.extend(crate::capacity::prepare(&p)?);
        CLEAR.store(va(CLEAR_RVA)?, Ordering::Release);
    }
    // Validate the entire variable-length transaction before freezing threads.
    for (site, old, new) in &patches {
        if old.len() != new.len() || unsafe { std::slice::from_raw_parts(*site as *const u8, old.len()) } != old {
            return Err(format!("Patch conflict at {:x}; no changes applied", site));
        }
    }
    let ranges: Vec<_> = patches.iter().map(|(site, old, _)| (*site, old.len())).collect();
    STATE.store(state, Ordering::Release);''')
s=s.replace('threads.freeze(&patches)', 'threads.freeze(&ranges)')
s=s.replace('std::slice::from_raw_parts(*site as *const u8, 5)', 'std::slice::from_raw_parts(*site as *const u8, old.len())')
s=s.replace('    READY.store(true, Ordering::Release);', '    INSTALLED.store(true, Ordering::Release);')
s=s.replace('        READY.store(false, Ordering::Release);','        INSTALLED.store(false, Ordering::Release);')
p.write_text(s)
# Copy the independently verified upstream search-loop byte sequence verbatim.
u=Path('work/MultipleSpiritAshes/src/engine_patches.rs').read_text()
loop=u.split('const SLOT_LOOP: [u8; 57] = [',1)[1].split('];',1)[0]
c=r'''//! Capacity/search adaptation from ArialSenki MultipleSpiritAshes, MIT OR Apache-2.0.
//! Only the actor pool and search are changed; no area/range patches are included.
use fromsoftware_shared::program::Program;
use windows::Win32::System::{Memory::{VirtualProtect, PAGE_EXECUTE_READ, PAGE_PROTECTION_FLAGS}, Diagnostics::Debug::FlushInstructionCache, Threading::GetCurrentProcess};
pub const ENTRIES: u32 = 520;
pub const FIRST_SPIRIT_SLOT: usize = 20;
pub const MAX_GROUP_MEMBERS: usize = (ENTRIES as usize) - FIRST_SPIRIT_SLOT;
pub type Patch = (usize, Vec<u8>, Vec<u8>);
const SLOT_LOOP: [u8; 57] = [LOOP_BYTES];
/// Prepared before the startup transaction. Constructor changes retain native
/// allocation, zeroing, entry construction and eventual destruction. No live
/// registry is resized. If already constructed, runtime capacity check refuses.
pub fn prepare(p: &Program) -> Result<Vec<Patch>, String> {
    let va = |rva| p.rva_to_va(rva).map(|x| x as usize).map_err(|e| format!("capacity RVA: {e:?}"));
    let mut patches = Vec::new();
    let bytes = ENTRIES * 16;
    let mut allocate = vec![0xba]; allocate.extend(bytes.to_le_bytes());
    let mut count = vec![0xbf]; count.extend(ENTRIES.to_le_bytes());
    let mut zero = vec![0x41,0xb8]; zero.extend(bytes.to_le_bytes());
    patches.push((va(0x495614)?, vec![0xba,0,5,0,0], allocate));
    patches.push((va(0x49562b)?, vec![0xbf,0x50,0,0,0], count));
    patches.push((va(0x495635)?, vec![0x41,0xb8,0,5,0,0], zero));
    patches.push((va(0x493492)?, vec![0x83,0xc0,2,0x8d,4,0x80,0x44,0x8d,0x14,0], vec![0x41,0xba,0x14,0,0,0,0x0f,0x1f,0x40,0]));
    // Check surrounding allocation/iteration instructions, not just immediates.
    for (rva, signature) in [
        (0x495619, &[0x41,0xb8,8,0,0,0,0xff,0x50,0x50,0x48,0x89,0x46,0x18][..]),
        (0x495630, &[0x89,0x7e,0x10,0x33,0xd2][..]),
        (0x49566b, &[0x48,0x83,0xc3,0x10,0x48,0x83,0xef,1,0x75,0xdb][..]),
        (0x4934f9, &[0x49,0x8b,0xc7,0xe9,0x9d,1,0,0][..]),
    ] {
        if unsafe { std::slice::from_raw_parts(va(rva)? as *const u8, signature.len()) } != signature {
            return Err(format!("Capacity context conflict at {rva:x}"));
        }
    }
    let site=va(0x4934c0)?;
    let cave=unsafe { crate::hooks::near(site) }.ok_or("Cannot allocate slot-search stub")?;
    let mut body=vec![0x41,0x8b,0x46,0x10,0x83,0xe8,0x14];
    let loop_start=body.len();
    body.extend([0x41,0x39,0xc1,0x43,0x8d,0x3c,0x11,0x7c,2,0x29,0xc7,0x48,0x63,0xdf,0x48,0xc1,0xe3,4,0x4c,1,0xc3,0x4c,0x39,0x3b,0x74,0]);
    let found=body.len();
    body.extend([0x41,0xff,0xc1,0x48,0xff,0xc1,0x48,0x39,0xc1,0x7c,0]);
    let back=body.len(); body[back-1]=(loop_start as isize-back as isize) as u8;
    body.extend([0xff,0x25,0,0,0,0]); body.extend(((site+57) as u64).to_le_bytes());
    body[found-1]=(body.len()-found) as u8;
    body.extend([0xff,0x25,0,0,0,0]); body.extend(((site+65) as u64).to_le_bytes());
    unsafe { std::ptr::copy_nonoverlapping(body.as_ptr(),cave as *mut u8,body.len()); }
    let mut old=PAGE_PROTECTION_FLAGS(0);
    unsafe { VirtualProtect(cave as _,0x1000,PAGE_EXECUTE_READ,&mut old) }.map_err(|e|format!("Search stub protection: {e}"))?;
    unsafe { FlushInstructionCache(GetCurrentProcess(),Some(cave as _),body.len()) }.map_err(|e|format!("Search stub cache: {e}"))?;
    let mut jump=vec![0x90;57]; jump[0]=0xe9;
    let displacement=i32::try_from(cave as i128-(site as i128+5)).map_err(|_|"Search stub range")?;
    jump[1..5].copy_from_slice(&displacement.to_le_bytes());
    patches.push((site,SLOT_LOOP.to_vec(),jump));
    Ok(patches)
}
'''.replace('LOOP_BYTES',loop)
Path('outputs/SpiritAshResummon/src/capacity.rs').write_text(c)
for name in ['Cargo.toml','Cargo.lock']:
 p=Path('outputs/SpiritAshResummon')/name;p.write_text(p.read_text().replace('version = "0.1.12"','version = "0.2.0"'))
