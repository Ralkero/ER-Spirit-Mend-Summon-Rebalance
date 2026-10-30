from pathlib import Path
r=Path('outputs/SpiritAshResummon')
def read(p):
    b=p.read_bytes()
    try: return b.decode('utf-8')
    except UnicodeDecodeError: return b.decode('cp1252')
p=r/'VALIDATION.md'
note='''
## 0.2.0 startup and idle: successful observed session

User loaded the test character at Gatefront, waited at least 15 seconds without summoning and reported stable 60 FPS. Evidence: `evidence/0.2.0-idle-20261029-204111.log`, PID 26904, through +75,115 ms. Ten call sites and multi-type pool configuration are logged at +61 ms; game-task readiness follows at +496 ms. The temporary scheduler NotFound resolves on the second attempt.

Loaded-player task execution continues past the exact 520-entry capacity guard, supporting successful registry construction; the startup pool message alone only describes intended configuration. Offline mode/no multiplayer session and no conflicting DLL are logged. Resummon flags are updated for 304 BuddyStone rows. Latest native/adapter query is (0,0), with zero groups and 91 FP. No summon or heal callback occurs. No panic, disabled-adapter message or validation rejection is logged.

Two settled windows record 600 game tasks per approximately ten seconds, consistent with the user's 60 FPS report, not an independent frame-time measurement. Latest task total is 14,614 us/max 420 us; query max 1 us and cost max 9 us. No actor traversal or healing is exercised. Coexistence, active performance, costs and healing remain untested in 0.2.0.

Next user test: summon wolves, refill FP to 91, then summon Militiamen while wolves remain alive. Expect three wolves and two Militiamen together, second cost 44 FP (91 -> 47), no dismissal. Wait 15 seconds and report counts/FP/FPS. Stop on any first-summon slowdown or issue. The agent only inspected/archived logs and updated source documentation; no game inputs or installed files changed.

'''
s=read(p);head,rest=s.split('\n',1);p.write_text(head+'\n'+note+rest,encoding='utf-8')
p=r/'README.md';s=read(p).replace('**0.2.0 compiles but has not been tested in game.**','**0.2.0 compiles and its user-operated startup/idle test reports stable 60 FPS with matching readiness logs. Active multi-type behavior remains untested.**');p.write_text(s,encoding='utf-8')
p=r/'CHANGELOG.md';s=read(p).replace('## 0.2.0 — 2026-10-29 (multiple distinct Ash types; untested prototype)','## 0.2.0 — 2026-10-29 (multiple distinct Ash types; active behavior untested)\n\n- User-operated startup/idle: stable 60 FPS, ten hooks installed, loaded-player capacity guard passed, offline gates ready, no logged errors or activations. Coexistence/healing tests remain pending.');p.write_text(s,encoding='utf-8')
