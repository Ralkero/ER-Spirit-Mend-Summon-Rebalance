exec(open('work/registry_cfg.py').read().split('seeds=')[0])
import hashlib,json
chunks=[('entry_load_gate',0x3f2d30,0x55),('entry_load_completion',0x3f8ce2,9),('entry_update_enabled',0x401439,0x1e),('entry_reset',0x3c3830,0x12),('entry_index',0x493d90,0xc),('actor_index',0x493e20,0xf),('actor_handle',0x493de0,0x32)]
lines=['SHA256 '+hashlib.sha256(pathlib.Path(pe.__file__).read_bytes()).hexdigest() if hasattr(pe,'__file__') else 'Pinned executable SHA256 f9118fea2cfd8fa3901e8060bfadb1552cb2471771ded8a5b662c4e298ffb328']
for name,r,size in chunks:
 lines.append(f'\n{name} RVA 0x{r:x}')
 lines.extend(f'{x.address:x}: {x.bytes.hex():24} {x.mnemonic} {x.op_str}' for x in md.disasm(pe.get_data(r,size),base+r))
pathlib.Path('outputs/SpiritAshResummon/evidence/registry-state-0.1.9.asm').write_text('\n'.join(lines))
print('\n'.join(lines[:30]))
