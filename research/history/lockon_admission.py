exec(open('work/lockon_disasm.py',encoding='utf-8-sig').read().split('for name,rva,size')[0])
for name,rva,size in [('admission_predicate',0x51b8b0,0x480),('target_init',0x714c00,0x300)]:
 s='\n'.join(f'{i.address-base:x}: {i.bytes.hex():24} {i.mnemonic} {i.op_str}' for i in md.disasm(pe.get_data(rva,size),base+rva));(out/f'{name}.asm').write_text(s)
print((out/'admission_predicate.asm').read_text())
