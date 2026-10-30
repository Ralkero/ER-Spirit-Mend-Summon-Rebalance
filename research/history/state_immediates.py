exec(open('work/registry_cfg.py').read().split('seeds=')[0])
import re
for s in pe.sections:
 if not s.Characteristics & 0x20000000:continue
 data=s.get_data()
 for m in re.finditer(b'\x04\x04',data):
  r=s.VirtualAddress+m.start()
  if r>0x1800000:continue
  ctx=pe.get_data(r-6,12)
  print(hex(r),ctx.hex())
