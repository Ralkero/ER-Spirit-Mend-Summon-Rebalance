exec(open('work/registry_cfg.py').read().split('seeds=')[0])
for name,r,size in [('load_states',0x3f8a80,0x500),('update_states',0x401340,0x220),('active_check',0x3f2d00,0x100)]:
 md.skipdata=True
 lines=[f'{x.address:x}: {x.bytes.hex():24} {x.mnemonic} {x.op_str}' for x in md.disasm(pe.get_data(r,size),base+r)]
 pathlib.Path('work/'+name+'.asm').write_text('\n'.join(lines))
 print(name+'\n'+'\n'.join(lines))
