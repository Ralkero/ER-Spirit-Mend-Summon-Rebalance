from pathlib import Path
import pefile,struct,json,hashlib
p=Path(r'C:\Program Files (x86)\Steam\steamapps\common\ELDEN RING\Game\eldenring.exe');b=p.read_bytes();pe=pefile.PE(data=b,fast_load=True);base=pe.OPTIONAL_HEADER.ImageBase
sections=[(s.Name.rstrip(b'\0'),s.VirtualAddress,s.get_data()) for s in pe.sections]
def read(rva,n):return pe.get_data(rva,n)
def vt(name):
 desc=None
 for _,rva,data in sections:
  off=data.find(name+b'\0')
  if off>=16:desc=rva+off-16;break
 if desc is None:return None
 _,rva,data=next(s for s in sections if s[0]==b'.rdata')
 locator=None
 for off in range(0,len(data)-24,4):
  v=struct.unpack_from('<6I',data,off)
  if v[0]==1 and v[1]==0 and v[3]==desc and v[5]==rva+off:locator=rva+off;break
 if locator is None:return None
 needle=struct.pack('<Q',base+locator)
 off=data.find(needle)
 while off>=0:
  if off%8==0:return rva+off+8
  off=data.find(needle,off+1)
items={}
for name in [b'.?AVCSScaleformFileOpener@CS@@',b'.?AVMemoryFile@Scaleform@@']:
 v=vt(name);entries=[]
 if v:
  for i in range(7):
   target=struct.unpack('<Q',read(v+i*8,8))[0]-base
   entries.append({'slot':i,'rva':hex(target),'bytes':read(target,16).hex()})
 items[name.decode()]={'vtable':hex(v) if v else None,'entries':entries}
c=read(0x772fbd,5)
out={'exe_sha256':hashlib.sha256(b).hexdigest(),'copy_call':{'rva':'0x772fbd','bytes':c.hex(),'target':hex(0x772fbd+5+struct.unpack('<i',c[1:])[0])},'copy_prologue':read(0x775480,16).hex(),'message_lookup':read(0x2671370,16).hex(),'rtti':items,'static_only':True}
Path('outputs/SpiritAshResummon/evidence/hud-native-analysis.json').write_text(json.dumps(out,indent=2));print(json.dumps(out,indent=2))
