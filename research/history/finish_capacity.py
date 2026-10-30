from pathlib import Path
root=Path('outputs/SpiritAshResummon')
p=root/'src/capacity.rs'
s=p.read_text()
up=Path('work/MultipleSpiritAshes/src/engine_patches.rs').read_text()
body=up.split('    let bytes = BUDDY_SET_ENTRIES * 16;',1)[1].split('    unsafe { std::ptr::copy_nonoverlapping(body.as_ptr(), cave as *mut u8, body.len()) };',1)[0]
body=body.replace('BUDDY_SET_ENTRIES','ENTRIES')
s+='''
// Adapted from upstream spirit_set_size; original notices retained in UPSTREAM.md.
fn prepare_constructor(p: &Program) -> Result<Patch, String> {
    let init = p.rva_to_va(0x49560a).map_err(|e| format!("Constructor RVA: {e:?}"))? as usize;
    // Pin the complete constructor, including allocator and entry constructor.
    let expected: &[u8] = &[
'''
# derive pinned bytes only from independently inspected on-disk exe, never live process
import pefile
pe=pefile.PE(r'C:\Program Files (x86)\Steam\steamapps\common\ELDEN RING\Game\eldenring.exe', fast_load=True)
b=pe.get_data(0x49560a,0x73)
s+='        '+', '.join(f'0x{x:02x}' for x in b)+'\n    ];\n'
s+='''    if unsafe { std::slice::from_raw_parts(init as *const u8, expected.len()) } != expected {
        return Err("Summon registry constructor conflict".into());
    }
    let site = init + 0x6b;
    let allocator = (init + 7).checked_add_signed(i32::from_le_bytes(expected[3..7].try_into().unwrap()) as isize).ok_or("Allocator displacement overflow")? as u64;
    let cave = unsafe { crate::hooks::near(site) }.ok_or("Cannot allocate constructor stub")?;
    let bytes = ENTRIES * 16;
'''+body+'''
    unsafe { std::ptr::copy_nonoverlapping(body.as_ptr(), cave as *mut u8, body.len()); }
    let mut old = PAGE_PROTECTION_FLAGS(0);
    unsafe { VirtualProtect(cave as _, 0x1000, PAGE_EXECUTE_READ, &mut old) }.map_err(|e| format!("Constructor stub protection: {e}"))?;
    unsafe { FlushInstructionCache(GetCurrentProcess(), Some(cave as _), body.len()) }.map_err(|e| format!("Constructor stub cache: {e}"))?;
    let mut jump = vec![0x90;8];
    jump[0]=0xe9;
    let delta=i32::try_from(cave as i128-(site as i128+5)).map_err(|_| "Constructor stub range")?;
    jump[1..5].copy_from_slice(&delta.to_le_bytes());
    Ok((site, expected[0x6b..].to_vec(), jump))
}
'''
p.write_text(s)
p=root/'src/runtime.rs';s=p.read_text().replace('Startup task registered; awaiting signature validation before any modification','Startup task registered; awaiting gameplay readiness').replace('before task registration or patches.','before task registration.');p.write_text(s)
p=root/'src/hooks.rs';s=p.read_text().replace('All eight sites are checked before the first write. No function is overwritten.','All call sites and capacity patches are checked before the first write.');p.write_text(s)
