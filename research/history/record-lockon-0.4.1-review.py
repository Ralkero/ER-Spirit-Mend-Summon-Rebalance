from pathlib import Path
import re,json
root=Path('outputs/SpiritAshResummon');p=root/'evidence/0.4.1-lockon-20261029-233642.log';s=p.read_text(encoding='utf-8-sig');s=s[s.rfind('[+0ms] SpiritAshResummon 0.4.1'):];lines=s.splitlines()
selected=sorted(set(re.findall(r'locked=true selected=([0-9a-f]+:[0-9a-f]+)',s)))
summary={'version':'0.4.1','pid':11968,'archive':p.name,'user_report':'Targeted several enemies, attempted approximately five seconds per target.','diagnostic_ready':True,'distinct_non_owned_selected_handles':selected,'owned_spirit_handles':['17100014:ffffffff','17100015:ffffffff','17100016:ffffffff'],'unlock_cleared_handle_observed':True,'target_sample_calls':634,'target_sample_total_us':11305,'target_sample_max_us':499,'readable_probe_calls':0,'startup_note':'One scheduler NotFound retry at407ms, followed by successful initialization.','limitations':['No FPS or input-device report; no independent frame-time claim.','Enemy identity is based on user report; observer intentionally labels unmatched handles other_or_unresolved.','No owned-Spirit targeting observed or enabled; this does not validate friendly eligibility/cycling.','No automated or agent-operated gameplay tests.']}
(root/'evidence/0.4.1-lockon-review.json').write_text(json.dumps(summary,indent=2),encoding='utf-8')
p=root/'VALIDATION.md';old=p.read_text(encoding='utf-8-sig');note='''## 0.4.1 user-operated target diagnostic — observed result

2026-10-29, PID11968. Snapshot `evidence/0.4.1-lockon-20261029-233642.log`; compact summary `evidence/0.4.1-lockon-review.json`. User reports targeting several enemies and holding for roughly five seconds. Diagnostic signature/layout checks passed at+58ms. Seven distinct non-owned target handles appeared with locked=true; a previously selected handle reappeared after switching away. Three registered wolf handles remained distinct from all selected handles. Unlock at+102987ms cleared the selected handle to ffffffff:ffffffff and set locked=false; re-lock and subsequent unlock were also captured.

Captured target_sample counters:634 samples,11305us total,499us maximum (about0.018ms average). No VirtualQuery readable_probe calls in the captured session. One initial CSTask NotFound retry recovered during startup; no observer-disabled or runtime panic observed. FPS/input method were not reported, so these callback timings are not a frame-rate measurement. Native target identities are intentionally not dereferenced; their classification as enemies is supplied by the user's action report. Later summon cleanup does not invalidate target-read correlation and its cause was not independently determined.

Result: enough evidence for the read-only selected-handle baseline; no repeat of this test is needed now. Next: narrow friendly candidate admission/maintain/cycle investigation. This is not proof that friendly targeting or the proposed sorcery works. No game changes or test execution by the agent during this review.

''';p.write_text(note+old,encoding='utf-8',newline='\n')
print(json.dumps(summary,indent=2))
