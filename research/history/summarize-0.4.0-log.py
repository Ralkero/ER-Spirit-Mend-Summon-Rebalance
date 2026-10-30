from pathlib import Path
import re,json
p=Path('outputs/SpiritAshResummon/evidence/0.4.0-elleh-boundary-20261029-231346.log');s=p.read_text(encoding='utf-8');s=s[s.rfind('[+0ms] SpiritAshResummon 0.4.0'):];lines=s.splitlines()
for l in lines:
 if 'Heal member' in l or ('Summon timeline sample' in l and any(f'player_fp: Some({v})' in l for v in [295,251,196,141,97,42,157,102])):
  print(l)
metrics={}
for l in lines:
 if 'Timing window_ms=' not in l:continue
 for name,calls,total,mx in re.findall(r'(\w+)\[calls=(\d+),total_us=(\d+),max_us=(\d+)\]',l):
  d=metrics.setdefault(name,{'calls':0,'total_us':0,'max_us':0});d['calls']+=int(calls);d['total_us']+=int(total);d['max_us']=max(d['max_us'],int(mx))
print('METRICS',json.dumps({k:metrics[k] for k in ['task','query','cost','summon','hud_sample','inactive_cleanup','readable_probe']}))
print('FAULTS',[l for l in lines if re.search(r'panic|Disabled:|error|failed|conflict.*true',l,re.I)])
