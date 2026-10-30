import pefile, capstone, pathlib, json
p=pathlib.Path(r'C:\Program Files (x86)\Steam\steamapps\common\ELDEN RING\Game\eldenring.exe')
pe=pefile.PE(str(p), fast_load=True)
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64)
targets={'state':(0x4b73d0,0x120),'summon':(0x4b8690,0xa1),'exit':(0x4b8240,0x16b),'cost':(0x3c0b10,0x190),'blocked':(0x4b78f0,0x280),'alive':(0x4b7bf0,0x150)}
for sec in pe.sections:
 if sec.Characteristics & 0x20000000:
  data=sec.get_data()
  for i,b in enumerate(data[:-5]):
   if b==0xe8 and sec.VirtualAddress+i+5+int.from_bytes(data[i+1:i+5],'little',signed=True)==0x4b8690:
    r=sec.VirtualAddress+i
    targets[f'summon_caller_{r:x}']=(r-80,190)
for name,(rva,size) in targets.items():
 text='\n'.join(f'{i.address:x}: {i.bytes.hex():24} {i.mnemonic} {i.op_str}' for i in md.disasm(pe.get_data(rva,size),pe.OPTIONAL_HEADER.ImageBase+rva))
 pathlib.Path('work/'+name+'.asm').write_text(text)
print(json.dumps({'image_base':hex(pe.OPTIONAL_HEADER.ImageBase),'targets':targets}))
