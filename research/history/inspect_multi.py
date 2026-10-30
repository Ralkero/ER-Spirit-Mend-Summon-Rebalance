exec(open('work/registry_cfg.py').read().split('seeds=')[0])
import re
text=pathlib.Path('work/MultipleSpiritAshes/src/engine_patches.rs').read_text()
pat=text.split('const BUDDY_SET_INIT: &str = "',1)[1].split('";',1)[0].replace('\\','')
pat=pat.split()
code_sec=next(s for s in pe.sections if s.Name.rstrip(b'\0')==b'.text')
code=code_sec.get_data(); start=code_sec.VirtualAddress
regex=b''.join(b'.' if t=='?' else re.escape(bytes.fromhex(t)) for t in pat)
hits=list(re.finditer(regex,code,re.DOTALL)); print('init_hits',[(hex(start+h.start())) for h in hits])
for h in hits:
 r=start+h.start(); lines=[f'{x.address:x}: {x.bytes.hex():24} {x.mnemonic} {x.op_str}' for x in md.disasm(pe.get_data(r,len(pat)),base+r)]; pathlib.Path('work/multi_init.asm').write_text('\n'.join(lines)); print('\n'.join(lines))
for name,r,size in [('multi_spawn',0x4b8738,0x6d0),('multi_clear',0x4bba80,0x170),('multi_exit',0x4b8240,0x200),('multi_blocked',0x4b78f0,0x300)]:
 md.skipdata=True
 lines=[f'{x.address:x}: {x.bytes.hex():24} {x.mnemonic} {x.op_str}' for x in md.disasm(pe.get_data(r,size),base+r)]
 pathlib.Path('work/'+name+'.asm').write_text('\n'.join(lines))
pat=text.split('const SLOT_START: &str = "',1)[1].split('";',1)[0]
needle=bytes.fromhex(pat); print('slot_start',hex(start+code.index(needle)))
pat=text.split('const SLOT_LOOP: [u8; 57] = [',1)[1].split('];',1)[0]
needle=bytes(int(t,16) for t in re.findall('0x([0-9a-f]{2})',pat)); pos=start+code.index(needle)
print('slot_loop',hex(pos)); pathlib.Path('work/multi_slots.asm').write_text('\n'.join(f'{x.address:x}: {x.bytes.hex():24} {x.mnemonic} {x.op_str}' for x in md.disasm(pe.get_data(pos-90,200),base+pos-90)))
