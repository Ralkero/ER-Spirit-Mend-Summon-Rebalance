from pathlib import Path
import pefile, capstone, sys, struct
p=pefile.PE(r'C:\Program Files (x86)\Steam\steamapps\common\ELDEN RING\Game\eldenring.exe',fast_load=True)
p.parse_data_directories(directories=[3])
entry=p.OPTIONAL_HEADER.DATA_DIRECTORY[3]
functions=list(struct.iter_unpack('<III',p.get_data(entry.VirtualAddress,entry.Size)))
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64)
base=p.OPTIONAL_HEADER.ImageBase
out=Path('outputs/SpiritAshResummon/evidence/cast-static')
for arg in sys.argv[1:]:
 parts=arg.split(':');mode,addr=parts[:2];rva=int(addr,16)
 if mode in ['f','r']:
  fn=next((e for e in functions if e[0]<=rva<e[1]),None)
  if fn is None and mode=='f': print('no function',arg);continue
  start,end=(rva,rva+int(parts[2],16)) if mode=='r' else fn[:2]
  lines=[f'function {start:x}..{end:x}']+[f'{i.address-base:x}: {i.mnemonic} {i.op_str}' for i in md.disasm(p.get_data(start,end-start),base+start)]
  (out/f'{rva:x}.asm').write_text('\n'.join(lines));print('\n'.join(lines))
 elif mode=='x':
  for s in p.sections:
   if not s.Characteristics&0x20000000:continue
   b=s.get_data()
   for j in range(len(b)-5):
    if b[j]==0xe8 and s.VirtualAddress+j+5+struct.unpack_from('<i',b,j+1)[0]==rva:
     a=s.VirtualAddress+j
     fn=next((e.struct for e in p.DIRECTORY_ENTRY_EXCEPTION if e.struct.BeginAddress<=a<e.struct.EndAddress),None)
     print(f'call {a:x}, fn {fn.BeginAddress:x}' if fn else f'call {a:x}')
