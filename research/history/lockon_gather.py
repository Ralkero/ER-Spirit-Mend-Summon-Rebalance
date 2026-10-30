exec(open('work/lockon_disasm.py',encoding='utf-8-sig').read().split('for name,rva,size')[0])
for name,rva,size in [('candidate_gather',0x717760,0x450),('insert_filter',0x716380,0x180),('target_flags',0x715a00,0x500)]:
 s='\n'.join(f'{i.address-base:x}: {i.bytes.hex():24} {i.mnemonic} {i.op_str}' for i in md.disasm(pe.get_data(rva,size),base+rva));(out/f'{name}.asm').write_text(s)
print((out/'candidate_gather.asm').read_text())
