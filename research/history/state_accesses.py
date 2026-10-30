exec(open('work/registry_cfg.py').read().split('seeds=')[0])
import re
hits=[]
for s in pe.sections:
 if not s.Characteristics & 0x20000000:continue
 data=s.get_data()
 for m in re.finditer(rb'[\x80\xc6][\x40-\x7f][\x08\x09][\x00-\x06]',data):
  r=s.VirtualAddress+m.start()
  if 0x380000<r<0x700000:
   x=next(md.disasm(pe.get_data(r,15),base+r,count=1),None)
   if x: hits.append((r,x.mnemonic,x.op_str))
print('\n'.join(f'{r:x}: {mn} {op}' for r,mn,op in hits))

