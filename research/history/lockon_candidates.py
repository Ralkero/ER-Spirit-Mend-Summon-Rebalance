exec(open('work/lockon_disasm.py',encoding='utf-8-sig').read().split('for name,rva,size')[0])
for name,rva,size in [('candidate_update',0x717b00,0x850),('target_resolve',0x7150b0,0x100),('lock_toggle',0x3f74e0,0x50),('candidate_helpers',0x715b00,0x3a0)]:
 s='\n'.join(f'{i.address-base:x}: {i.bytes.hex():24} {i.mnemonic} {i.op_str}' for i in md.disasm(pe.get_data(rva,size),base+rva));(out/f'{name}.asm').write_text(s)
print((out/'candidate_update.asm').read_text()[:27000])
