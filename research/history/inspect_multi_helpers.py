exec(open('work/registry_cfg.py').read().split('seeds=')[0])
for name,r,size in [('multi_origin_a',0x4ba6f0,0x500),('multi_origin_b',0x4b9f50,0x500),('multi_entry_init',0x3c3810,0x60)]:
 lines=[f'{x.address:x}: {x.bytes.hex():24} {x.mnemonic} {x.op_str}' for x in md.disasm(pe.get_data(r,size),base+r)]
 pathlib.Path('work/'+name+'.asm').write_text('\n'.join(lines))
