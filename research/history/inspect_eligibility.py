exec(open('work/registry_cfg.py').read().split('seeds=')[0])
md.skipdata=True
for name,r,n in [('eligibility',0x6907d7,0x4a0),('item79',0x690900,0x1000)]:
 pathlib.Path('work/'+name+'.asm').write_text('\n'.join(f'{x.address:x}: {x.bytes.hex():24} {x.mnemonic} {x.op_str}' for x in md.disasm(pe.get_data(r,n),base+r)))
