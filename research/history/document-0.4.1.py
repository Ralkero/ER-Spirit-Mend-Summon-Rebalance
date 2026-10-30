from pathlib import Path
root=Path('outputs/SpiritAshResummon')
updates={
'README.md':'''# SpiritAshResummon 0.4.1 — read-only lock-on diagnostic

This build retains0.4.0 gameplay and adds an optional selected-target observer for the planned Intelligence healing sorcery. It does not enable duplicate same-Ash summons, friendly lock-on or spell casting. `[Debug] EnableTargetDiagnostics=true` together with EnableLogging=true enables5Hz sampling and change/10-second heartbeat logs. Defaults are off; the local diagnostic package enables both.

The observer reads typed PlayerIns.locked_on_enemy and ChrIns.is_locked_on after exact native instruction/layout checks, then compares the selected scalar handle against current validated local Spirits. No arbitrary enemy pointer is dereferenced, no camera hook or game-state write is added, no health read is added and no per-sample VirtualQuery is used. A diagnostic signature mismatch disables only the observer. See the current procedure at the top of VALIDATION.md. Existing gameplay details below remain applicable unless explicitly superseded.

---

''',
'IMPLEMENTATION.md':'''# Implementation — 0.4.1 target observer

`src/targeting_diagnostic.rs` is included in runtime. Config adds target_diagnostics (defaultfalse), read from Debug.EnableTargetDiagnostics. Initialization happens after existing version/conflict/startup-hook checks. It verifies typed offsets PlayerIns.locked_on_enemy=0x6b0, ChrIns.is_locked_on=0xc9, handle size8 and exact native instructions at RVA0x71851a (clear selected handle),0x71857e (copy selected actor+8 handle to player+6b0),0x3f74e0 (write lock flag). No new patches are installed.

The existing GameMan task samples at most5Hz. A live local player and existing enabled/offline gates are required. It copies the selected handle/lock flag and bounded owner-group metadata; only registry-active, backlink/handle-validated local Spirits are inspected for handle/Buddy/NPC/team/death-flag scalars. Disappearing/warping/remote/unrecognized entries are excluded. Selected unknown actors are never resolved or dereferenced. Labels are not_locked, locked_without_handle, owned_spirit, other_or_unresolved. These labels are telemetry, NOT an authorization predicate for a future heal. Full HP/range/LOS/action validation will be necessary for healing.

Only scalar snapshots survive the callback. World/player scalar identity prevents stale comparisons; missing/dead player resets the snapshot. Changes and10s heartbeats are logged, with at most16 Spirit descriptions per line and full count. Timing metric target_sample includes the sampler/log cost. It does not collect the complete native camera candidate list or change its admission/cycling rules. Existing camera candidate analysis remains provisional. Source static analysis found list nodes with next+0x80, target record+0x88 and flags+0x90; selected bit0x20 and eligibility bit0x10 participate in scoring/cycling, but interpreting those bits does not prove where friendly actors are rejected. Do not bypass these flags globally.

Build compiled, no automated or gameplay tests run. Runtime checks execute only when the user launches the diagnostic. Reference disassembly is under evidence/lockon-static, generated offline by workspace work/lockon_disasm.py and work/lockon_candidates.py. No reference camera-mod source was copied into shipping code.

---

''',
'VALIDATION.md':'''# 0.4.1 diagnostic validation (2026-10-29)

Release x64 compilation succeeded without warnings (`evidence/build-0.4.1.txt`). No automated tests, runtime injection, launch or gameplay tests executed by the agent. Exact native field-access evidence is saved under `evidence/lockon-static/score.asm` and `lock_toggle.asm`; live field behavior awaits the following user-operated test.

1. Launch the usual offline test profile, load at Gatefront, and wait5seconds with lock-on OFF. Note FPS.
2. Summon wolves once and wait5seconds. Existing one-group-per-Ash/healing rules remain; this build does not implement duplicates or the sorcery.
3. With at least two ordinary enemies in sight, lock onto one for5seconds. Cycle to another enemy using your usual controls and hold5seconds. Cycle back and hold5seconds; then unlock and wait5seconds. Avoid killing the targets during this sequence if practical.
4. Optional: stand near a wolf and attempt ordinary lock-on, then unlock. Friendly lock-on is NOT enabled yet; inability to select it is expected. Report whether lock-on behaved normally, which input method you used (controller or mouse/keyboard), and FPS. Leave the game open for log review.

Expected evidence: observer-ready startup; not_locked -> other_or_unresolved with nonempty handleA -> different handleB -> A -> not_locked, plus three registered owned Spirit handles that differ from those enemy selections. Off-target handle data may be empty/stale while locked=false; that alone is not an error. Entries whose states are loading/retiring can be skipped and are never healing candidates. Camera-update vs GameMan timing can yield transient mismatches; record correlation, do not claim healing eligibility from this sample alone.

After this baseline, the next implementation task is the narrowly scoped friendly admission/retain/cycle path, preserving native enemies, geometry/range checks, allegiance, friendly damage and AI. No friendly-target or spell functionality is claimed by this release.

---

''',
'CHANGELOG.md':'''# 0.4.1 — selected-target diagnostic

- Added opt-in5Hz read-only native lock flag/handle telemetry for the planned targeted Intelligence sorcery.
- Validated typed offsets against exact native field-access instructions; no new camera hooks, patch bytes or OS page probes.
- Logs owned Spirit identity scalars, target classification and timing without resolving arbitrary target actors.
- Preserved0.4.0 gameplay, configuration defaults, capacity and four-bar HUD.
- Release compilation succeeded; all gameplay/automated testing remains user-operated.

---

'''
}
for name,head in updates.items():
 p=root/name;s=p.read_text(encoding='utf-8-sig');p.write_text(head+s,encoding='utf-8',newline='\n')
p=root/'TARGETED-HEALING-INVESTIGATION.md';s=p.read_text(encoding='utf-8-sig');p.write_text('''# 0.4.1 implementation progress

Selected-target storage is now established at source/static level: PlayerIns.locked_on_enemy is a typed FieldInsHandle at+0x6b0, with native writes matching the type, and ChrIns.is_locked_on at+0xc9 is written by the native toggle routine. This resolves the earlier unknown about reading the selected actor. A compiled read-only diagnostic is now available for user correlation of lock/cycle events. Friendly candidate admission/retention, spell transactions and duplicate summon mode remain future work. No executable camera patches or game tests were performed.

---

'''+s,encoding='utf-8',newline='\n')
p=root/'START-HERE.txt';s=p.read_text(encoding='utf-8-sig');p.write_text('''0.4.1: READ-ONLY LOCK-ON DIAGNOSTIC
Follow the first procedure in VALIDATION.md: unlock, summon wolves, lock enemy A, cycle B, cycle A, unlock. Hold each state5seconds.
Friendly lock-on, duplicates and the healing sorcery are not enabled in this diagnostic. Current0.4.0 gameplay is retained.

'''+s,encoding='utf-8',newline='\n')
