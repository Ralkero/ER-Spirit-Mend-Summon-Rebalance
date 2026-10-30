from pathlib import Path
r=Path('outputs/SpiritAshResummon')
def read(p):
 b=p.read_bytes()
 try:s=b.decode('utf-8')
 except UnicodeDecodeError:s=b.decode('cp1252')
 return s.replace('\r','')
p=r/'VALIDATION.md';s=read(p);head,rest=s.split('\n',1)
note='''

## 0.2.1 coexistence: successful observed case

User summoned wolves, restored FP to 91 and activated Militiamen, then explicitly confirmed all three wolves and both Militiamen stayed visible at stable 60 FPS. Evidence: `evidence/0.2.1-second-ash-20261029-204950.log`, PID 39660 through +74,860 ms. The installed 0.2.1 session logs eleven hooks and readiness. No panic, disabled-adapter message or initialization failure appears.

First wolf summon: +33,040 ms native result=true, followed by three entries and sampled FP 91 -> 36. Refill to 91 appears at +36,104 ms. The new eligibility diagnostic reports Goods 212000, native -1, adapter 0. At +39,540 ms the Militiamen effect callback runs with native active state 2, three existing entries, range=true and no cooldown. At +39,551 ms AddGroup returns native true; at +39,562 ms the exact-request guard suppresses owner-group clearing. At +39,677 ms there are five non-exiting local entries and FP=47, a 44 FP decrease. The existing wolves were retained according to the guard log and user observation, rather than inferred solely from entry count.

Settled active windows record approximately 600 game tasks per ten seconds, consistent with the user's stable 60 FPS report. The latest complete window has task max 596 us, query max 20 us and cost max 47 us, with no registry/handle/health rejection. Load/update 4/4 actor validation uses the 520-entry registry. These measurements support this small-group case only; they do not validate high-count performance. The AddGroup callback spans about 11 ms by logged timestamps, not a direct input-to-frame latency measurement.

Record first different-type coexistence and its normal observed FP charge as passed for this case. Cross-type healing isolation, repeated same-type reuse, resummoning one dead type while another survives, upgrades-as-same-type, more than ten bodies, HP costs and transition regressions remain pending. No automated or agent-operated gameplay tests were run; the agent only read/archived logs and updated source documentation.

Next user test: while both types are present, let wolves take damage, refill to 91 FP and use Lone Wolf Ashes once. Prefer having a wounded living member of each type to make healing isolation visible. Expected: only surviving wolves heal by 25% of their own maximum HP (capped), no new wolves or dismissal, Militiamen remain unchanged, and FP drops by 55 (91 -> 36). Wait 15 seconds, report HP/count/FP/FPS observations and leave the game open for log review. Test Militiaman reuse separately afterward.
'''
p.write_text(head+'\n'+note+rest,encoding='utf-8',newline='\n')
p=r/'README.md';s=read(p);start=s.index('**0.2.1 is a staged');end=s.index('\n\n',start)
s=s[:start]+'''**0.2.1 has one successful user-operated coexistence test:** three wolves and two Militiamen remained present at stable 60 FPS, and the second summon spent 44 FP. Logs confirm the added eligibility hook, native request and group-preservation path. Multi-type healing, repeated resummoning and other regressions still require testing. All gameplay and automated test execution is left to the user; boss-fight resummoning remains deferred.'''+s[end:];p.write_text(s,encoding='utf-8',newline='\n')
p=r/'CHANGELOG.md';s=read(p).replace('## 0.2.1 — 2026-10-29 (staged item-eligibility correction)','## 0.2.1 — 2026-10-29 (item-eligibility correction)\n\n- Installed while the game was closed with a verified 0.2.0 backup. User then confirmed three wolves and two Militiamen coexisting at stable 60 FPS. Logs show eligibility (-1 -> 0), successful native AddGroup, preserved prior groups, five entries and 91 -> 47 FP. Multi-type healing and other regressions remain pending.',1);p.write_text(s,encoding='utf-8',newline='\n')
