import pefile, pathlib, re, struct
p=pathlib.Path(r'C:\Program Files (x86)\Steam\steamapps\common\ELDEN RING\Game\eldenring.exe')
b=p.read_bytes(); pe=pefile.PE(data=b,fast_load=True)
for m in re.finditer(rb'[^\x00]{0,100}ChrSet[^\x00]{0,180}', b):
 print(hex(pe.get_rva_from_offset(m.start())),repr(m.group()))
