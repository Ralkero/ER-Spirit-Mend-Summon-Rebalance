from pathlib import Path
import re,json
p=Path('outputs/SpiritAshResummon/evidence/0.4.1-lockon-20261029-233642.log');s=p.read_text(encoding='utf-8-sig');start=s.rfind('[+0ms] SpiritAshResummon 0.4.1');s=s[start:];lines=s.splitlines();metrics={}
for l in lines:
 if any(x in l for x in ['diagnostic startup','Target diagnostic','Lock-on sample:']):print(l)
 if 'Timing window_ms=' in l:
  for name,calls,total,mx in re.findall(r'(\w+)\[calls=(\d+),total_us=(\d+),max_us=(\d+)\]',l):
   d=metrics.setdefault(name,{'calls':0,'total_us':0,'max_us':0});d['calls']+=int(calls);d['total_us']+=int(total);d['max_us']=max(d['max_us'],int(mx))
print('TIMING',json.dumps({k:v for k,v in metrics.items() if k in ['task','target_sample','readable_probe','hud_sample']}))
print('ERRORS',json.dumps([l for l in lines if re.search(r'panic|disabled:|\berror\b|failed|conflicting_dll: true',l,re.I)]))
