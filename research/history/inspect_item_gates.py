exec(open('work/registry_cfg.py').read().split('seeds=')[0])
md.skipdata=True
for site in [0x7c4154,0x847ed0,0x848194,0x69063a,0x6906dd]:
 start=site-180
 lines=[f'{x.address:x}: {x.bytes.hex():24} {x.mnemonic} {x.op_str}' for x in md.disasm(pe.get_data(start,600),base+start)]
 pathlib.Path(f'work/item_gate_{site:x}.asm').write_text('\n'.join(lines))
