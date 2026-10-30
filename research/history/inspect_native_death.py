exec(open('work/registry_cfg.py').read().split('seeds=')[0])
for name,r,n in [('native_dead',0x3f4740,0x100),('native_active',0x3f2ae0,0x100)]:
 text='\n'.join(f'{x.address:x}: {x.bytes.hex():24} {x.mnemonic} {x.op_str}' for x in md.disasm(pe.get_data(r,n),base+r))
 pathlib.Path('work/'+name+'.asm').write_text(text)
 print(name+'\n'+text)
