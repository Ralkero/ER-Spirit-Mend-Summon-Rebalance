import pefile, pathlib, re, struct, capstone, json
p=pathlib.Path(r'C:\Program Files (x86)\Steam\steamapps\common\ELDEN RING\Game\eldenring.exe')
b=p.read_bytes(); pe=pefile.PE(data=b,fast_load=True); base=pe.OPTIONAL_HEADER.ImageBase
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64)
td=0x3c87138
for m in re.finditer(re.escape(struct.pack('<I',td)),b):
 off=m.start()-12
 if off<0: continue
 vals=struct.unpack_from('<6I',b,off)
 if vals[0]!=1 or vals[5]!=pe.get_rva_from_offset(off): continue
 col=base+vals[5]; print('COL',hex(col),vals)
 for ref in re.finditer(re.escape(struct.pack('<Q',col)),b):
  vt=pe.get_rva_from_offset(ref.start()+8)
  print('VTABLE',hex(vt))
  out=[]
  for i in range(14):
   f=struct.unpack_from('<Q',b,ref.start()+8+i*8)[0]
   out.append(f'{i}: {f:x}')
   out.extend(f'{x.address:x}: {x.bytes.hex():24} {x.mnemonic} {x.op_str}' for x in md.disasm(pe.get_data(f-base,180),f))
  pathlib.Path('work/registry_vtable.asm').write_text('\n'.join(out))
  print('\n'.join(out[:100]))
