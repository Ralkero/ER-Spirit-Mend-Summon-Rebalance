from pathlib import Path
import re,json
root=Path('outputs/SpiritAshResummon');archive=root/'evidence/0.4.0-elleh-boundary-20261029-231346.log';s=archive.read_text();s=s[s.rfind('[+0ms] SpiritAshResummon 0.4.0'):];lines=s.splitlines();metrics={}
for l in lines:
 if 'Timing window_ms=' in l:
  for name,calls,total,mx in re.findall(r'(\w+)\[calls=(\d+),total_us=(\d+),max_us=(\d+)\]',l):
   d=metrics.setdefault(name,{'calls':0,'total_us':0,'max_us':0});d['calls']+=int(calls);d['total_us']+=int(total);d['max_us']=max(d['max_us'],int(mx))
faults=[l for l in lines if re.search(r'panic|Disabled:|\berror\b|failed|conflicting_dll: true',l,re.I)]
activations=[]
for i,l in enumerate(lines):
 if 'Summon callback begin' not in l:continue
 effect=int(re.search(r'effect=(\d+)',l)[1]);before=int(re.search(r'player_fp: Some\((\d+)\)',l)[1]);after=next((int(m[1]) for n in lines[i+1:] if 'Summon timeline sample' in n and (m:=re.search(r'player_fp: Some\((\d+)\)',n))),None)
 activations.append({'at_ms':int(re.search(r'\[\+(\d+)ms',l)[1]),'effect':effect,'fp_before':before,'fp_next_sample':after,'delta':before-after if after is not None else None})
result={'session_pid':10064,'version':'0.4.0','archive':str(archive),'user_report':'Church of Elleh summon 55 FP; Gatefront and back without dismissal; 60 FPS; rested afterward explains later cleanup','activations':activations,'faults':faults,'timings':{k:metrics[k] for k in ['task','query','cost','summon','hud_sample','inactive_cleanup','readable_probe']},'scope':'User-operated observation and log review only; no agent-run tests. Other maps unverified.'}
(root/'evidence/0.4.0-elleh-boundary-review.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
p=root/'VALIDATION.md';text=p.read_text(encoding='utf-8-sig');note='''## 0.4.0 first user-operated session: Church of Elleh boundary test

2026-10-29, PID 10064. Archived log: `evidence/0.4.0-elleh-boundary-20261029-231346.log`; compact review: `evidence/0.4.0-elleh-boundary-review.json`.

User confirms summoning wolves at Church of Elleh for 55 FP, walking to Gatefront and back without dismissal, and steady 60 FPS. User also confirms resting at a grace afterward, explaining later cleanup rather than an unexpected boundary dismissal. Log records all 304 monument rows patched, fallback selection across map tiles, first wolf request while native range=false, three slots after spawning, and FP350->295. A later fresh wolf summon also spends55. Both types coexist at five slots; five wolf heals each spend55 and one Militiaman heal spends44. No panic, disabled-adapter or error messages in captured0.4.0 session. Rest cause comes from user confirmation, not a distinct engine cause code.

Timing includes an occasional activation peak around20ms and inactive cleanup around11ms (fresh OS page checks), despite user reporting steady60FPS. No recurring per-frame page-probe pattern appeared in ordinary active/idle windows. This observation validates the reported route only; distant maps, cleared dungeon interiors, scripted hubs and HP-cost activations remain unverified. No gameplay or automated tests executed by the agent.

''';p.write_text(note+text,encoding='utf-8')
