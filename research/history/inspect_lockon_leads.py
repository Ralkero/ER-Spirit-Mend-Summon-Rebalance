from pathlib import Path
import hashlib,json,pefile
p=Path(r'C:\Program Files (x86)\Steam\steamapps\common\ELDEN RING\Game\eldenring.exe');b=p.read_bytes();pe=pefile.PE(data=b,fast_load=True);pe.parse_data_directories(directories=[pefile.DIRECTORY_ENTRY['IMAGE_DIRECTORY_ENTRY_EXCEPTION']]);functions=[(e.struct.BeginAddress,e.struct.EndAddress) for e in getattr(pe,'DIRECTORY_ENTRY_EXCEPTION',[])]
patterns={'camera_range':'c7 83 30 29 00 00 00 00 70 41','candidate_scoring':'f3 0f 10 43 64 f3 0f 10 4b 60','target_switch':'48 89 5c 24 20 55 56 41 57 48 8d 6c 24 90','candidate_post_acquire':'a8 20 74 10 80 be'}
out={'exe_sha256':hashlib.sha256(b).hexdigest(),'reference_repo':'SchuhBaum/FreeLockOnCamera','commit':'cc3317740b266c12484d22ed7f44bd137603b977','analysis':'Offline byte patterns only; candidate names inferred from reference source, not proof of friendly eligibility or current-target handle. No patches applied.','patterns':{}}
for name,hexstr in patterns.items():
 needle=bytes.fromhex(hexstr);matches=[]
 for sec in pe.sections:
  if not sec.Characteristics & 0x20000000:continue
  data=sec.get_data();i=data.find(needle)
  while i>=0:
   rva=sec.VirtualAddress+i;fn=next(((a,z) for a,z in functions if a<=rva<z),None);matches.append({'rva':hex(rva),'function':[hex(v) for v in fn] if fn else None});i=data.find(needle,i+1)
 out['patterns'][name]={'bytes':hexstr,'matches':matches}
Path('outputs/SpiritAshResummon/evidence/targeted-healing-lockon-leads.json').write_text(json.dumps(out,indent=2));print(json.dumps(out,indent=2))
