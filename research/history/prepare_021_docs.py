from pathlib import Path
r=Path('outputs/SpiritAshResummon')
def read(p):
 b=p.read_bytes()
 try:return b.decode('utf-8').replace('\r','')
 except UnicodeDecodeError:return b.decode('cp1252').replace('\r','')
p=r/'Cargo.toml';p.write_text(read(p).replace('version = "0.2.0"','version = "0.2.1"',1),encoding='utf-8')
p=r/'Cargo.lock';p.write_text(read(p).replace('name = "spirit-ash-resummon"\nversion = "0.2.0"','name = "spirit-ash-resummon"\nversion = "0.2.1"',1),encoding='utf-8')
p=r/'tools/Preflight.ps1';s=read(p).replace(',@(0x690833,0x4b73d0)','').replace("    @{rva=0x690830; hex='418bd5e8986be2ff83f8ff410f95c5'},\n",'').replace('@(0x4b8a90,0x4bba80)','@(0x4b8a90,0x4bba80),@(0x690833,0x4b73d0)').replace("$signatures=@(","$signatures=@(\n    @{rva=0x690830; hex='418bd5e8986be2ff83f8ff410f95c5'},");p.write_text(s,encoding='utf-8')
p=r/'README.md';s=read(p).replace('# SpiritAshResummon 0.2.0','# SpiritAshResummon 0.2.1',1);start=s.index('**0.2.0 compiles');end=s.index('\n\n',start);s=s[:start]+'''**0.2.1 is a staged, untested correction.** In 0.2.0, startup/idle and the first wolf summon succeeded, but Militiamen remained greyed out with 91 FP. One additional native eligibility call is now routed through the existing Ash policy. Compilation alone does not establish that the blockage is resolved. All gameplay and automated test execution is left to the user. Boss-fight resummoning remains deferred.\n'''+s[end:];s=s.replace('call_sites=10','call_sites=11');p.write_text(s,encoding='utf-8')
p=r/'IMPLEMENTATION.md';s=read(p);s=s.replace('# Implementation — 0.2.0','# Implementation — 0.2.1',1);idx=s.index('\n## Scope change');s=s[:idx]+'''
## 0.2.1: independent item-usability gate

The 0.2.0 test exposed an omitted eligibility call at RVA `0x690833`: the engine passes the Goods ID in EDX, calls GetBuddyState, compares EAX against -1 and stores non-rejection in R13B. At `0x69096e`, a false R13B joins the other native failure predicates. Static inspection establishes this independent gate; the log shows the already-hooked item query allowed a different type but no activation reached DoSummon. Runtime confirmation of this call's rejection is pending.

This call now uses a dedicated `eligibility` wrapper with the same `adapt_query` policy as existing item queries, and is installed only in multiple-type mode. Exact surrounding instructions and original call target are validated transactionally. Normal item attribute, resource, area and multiplayer conditions around it remain native. Eleven call redirects are installed in multi-type mode. No EquipParamGoods item-limit field is modified: upstream's broader `use_limit_summon_buddy` clearing was considered, but this specific omitted native-state check provides a smaller correction.

The existing 250 ms diagnostic trace now includes last eligibility Goods/native/adapter values, with atomic scalars only and no per-call file writes. -128 values mean no observation yet. This records the extra gate separately from menu queries. It does not prove final item usability or cost consumption. 0.2.1 remains untested and staged; the running 0.2.0 DLL is unchanged.
'''+s[idx:];s=s.replace('Ten call-site redirects and three','Eleven call-site redirects and three');p.write_text(s,encoding='utf-8')
p=r/'VALIDATION.md';s=read(p);head,rest=s.split('\n',1);s=head+'''

## 0.2.0 second-type activation blocked; 0.2.1 correction staged

User reports Militiaman Ashes greyed out after summoning wolves. Evidence: `evidence/0.2.0-second-ash-blocked-20261029-204428.log`, PID 26904. Wolf summon at +134,843 ms returns native success, produces three local entries and sampled FP changes 91 -> 36. Refill to 91 is logged at +138,124 ms. At +166,976 ms queued Goods=212000 (Militiamen), active Goods/effect=232010 (upgraded wolves), three non-exiting entries, alive/in-range=true, cooldown zero, and FP=91. Latest sampled menu query is native -1/adapter 0; no second summon callback follows. This rules out lack of FP or adapter menu rejection in the sampled state; it does not by itself identify every native predicate.

The expanded registry reports 520 entries and accepts load/update 4/4 after brief 3/4 loading rejections. Settled active windows record 600 tasks per about ten seconds, with no panic/disabled message. This supports the first-summon path, not coexistence. Static inspection found the untouched Goods-specific state call at 0x690833 feeding item-usability rejection independently of the hooked queries/costs. 0.2.1 routes that call through the same validated per-family policy and adds separate bounded eligibility diagnostics. No item-limit/area/price parameters were changed.

0.2.1 release compilation is recorded in `evidence/build-0.2.1.txt`. Automated/gameplay tests have NOT been executed. The update is staged in `pending-0.2.1`; installation requires the game to be closed. Current-session log and installed 0.2.0 remain preserved. After installation, repeat the controlled wolves/refill-to-91/Militiamen test; report greying, counts, FP and FPS. Expected outcome remains five Spirits, no dismissal, and a 44 FP second charge. Do not mark the correction successful before the user test.

'''+rest;p.write_text(s,encoding='utf-8')
p=r/'CHANGELOG.md';s=read(p).replace('# Changelog\n','# Changelog\n\n## 0.2.1 — 2026-10-29 (staged item-eligibility correction)\n\n- Record 0.2.0 second-Ash greying with sufficient FP while the adapter menu query allowed it.\n- Hook the separate native item-usability state call at 0x690833, only in multiple-type mode, using the existing per-family eligibility policy. Validate its context and original target.\n- Add bounded Goods/native/adapter eligibility trace fields. No item-limit, cost or area parameters changed.\n- Compile and stage the diagnostic DLL; no automated or gameplay tests executed, no running installation replaced.\n',1);p.write_text(s,encoding='utf-8')
(r/'evidence/item-eligibility-0.2.1.md').write_text('''# Item eligibility inspection

Pinned installed WW 2.7.2.0 executable; static pefile/Capstone inspection only. Native decompiler unavailable in this environment. No debugger attached, process memory modified or game inputs generated.

- 0x690830 `mov edx,r13d`: selected Goods ID.
- 0x690833 `call 0x4b73d0`: independent GetBuddyState.
- 0x690838 `cmp eax,-1`; 0x69083b `setne r13b`.
- 0x69096e `test r13b,r13b`; 0x690971 `je 0x690982`: joins native usability failure path.
- FP requirement comparison remains at 0x690964, separate from this state result.
- Existing 0.2.0 hooks did not include 0x690833; the menu path logged (-1,0), yet activation never reached the effect callback.

Correction adds one call-site redirect to the same existing per-family policy, preserving surrounding native predicates and rejecting unknown state. No runtime claim until user reproduction. Upstream item-limit clearing remains excluded.
''',encoding='utf-8')
