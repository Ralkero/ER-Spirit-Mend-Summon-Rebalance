## 2026-11-01: compatible solo main publication

The current runtime is 0.6.1-solo.5. User acceptance covers Wolves/Lhutel coexistence, friendly lock-on, normal FPS (solo.4), and corrected names (solo.5). Same-Ash duplicates are configured on but not yet reported gameplay-accepted. Spirit Mend invalid casts, flight tracking and healing in this profile remain pending, as does merchant migration installation/gameplay. Earlier observations are version-specific.

Publication preserves runtime/vendor bytes and changes only packaging/configuration/documentation around the accumulated development source. Fresh build evidence is in evidence/publication-0.6.1-solo.5.json. Tests and gameplay were not run. No installed profile/save files were changed.

## 2026-10-31: Academy bridge shop migration - staged

Implemented: move ShopLineupParam 100599 to 100699 in the reviewed solo overlay,
remove Kale's added menu route, and use the Isolated Merchant's native Purchase
range 100675..100699. Spell 990001 and price 300 remain unchanged.

Compilation: `dotnet build tools/SpiritMendShopMove -c Release` with the existing
SoulsFormatsProject override succeeded, zero warnings/errors. Asset generation
completed; decrypted readback verified 164 unrelated NPC scripts and 193 unrelated
regulation members byte-identical. All 1,324 shop rows retained their field values
and names, with only the authorized row identity moved. Original inputs unchanged.

Installer preview succeeded and identified 25 protected files. Game was running;
no installed assets, DLL, INI, game-root files or saves were changed. Unit tests
and gameplay were not executed. Shop usability/purchase, existing-spell retention,
and Bell Bearing behavior are not gameplay-validated.

Next acceptance after closed-game installation: visit the Isolated Merchant on
the Academy bridge; find Spirit Mend under Purchase for 300 runes; verify other
stock/Sell/Leave; check an existing or newly purchased spell remains usable; verify
Kale's ordinary shop remains and Study sorcery is gone. See
[tool and rollback procedure](tools/SpiritMendShopMove/README.md).

## 2026-10-31: same-Ash duplicate groups enabled in active solo profile

At the user's request, enabled the existing solo.5 duplicate-group implementation
in the active Mod Engine 2 INI: AllowDuplicateAshGroups=true, MaxGroupsPerAsh=3.
Game/launchers were absent before the atomic INI change. Only those two keys were
added; DLL, launcher and mod assets retained their hashes (26 protected files).
The prior INI and guarded rollback are in the profile backups directory.

This is configuration activation, not a new DLL or gameplay acceptance. No build
or tests were run. The installed DLL SHA256 remains
01D03C6ABCF74DCAD496BABBBA91192B0EEC391FED09536E2000AF78940DEF40.
Duplicate mode takes precedence over HealOnReuse: using an active Ash requests a
new full group through the existing paid native summon path. Spirit Mend remains
the targeted healing route. Dismissal-dialog repurposing is deferred as requested.

Next user test: summon Wolves twice on level ground, restoring FP between casts;
expect six Wolves with 55 FP spent per successful cast and the first group retained.
Check no dismissal prompt, no reuse healing, working lock-on and normal FPS. The
four-row HUD does not show all six actors. Third group reaches the configured cap;
fourth should reject without spending FP. Same-Ash duplication, costs, lifecycle
and performance in this Seamless solo profile remain pending user validation.

## 2026-10-30: solo.5 HUD naming accepted

**Gameplay-validated (user-operated):** Wolves and Lhutel simultaneously summoned;
normal names displayed. PID 32412 log confirms goods 232000 and 258000 resolved=true,
with all four rows shown as Lone Wolf Ashes #1/#2/#3 and Lhutel the Headless #1.
The latest session remains offline_allowed=true, with no Disabled/PANIC/FAULT
entries in the inspected session. Friendly candidates are admitted; a fresh
user-observed cycling/FPS result was not supplied for this build. No new healing,
cost, death/resummon, or connected-session acceptance is inferred.

## 2026-10-30: solo.4 gameplay evidence and solo.5 name correction

User confirms Wolves and Lhutel simultaneously active, normal FPS, and lock-on
cycling among Spirits. The wolf HUD displays fallback names, confirmed in logs.
Solo.5 adds a bounded loaded-FMG reader without changing gameplay hooks or assets.
Compilation succeeded; tests compiled only, not executed. Name rendering and
first-label timing need user validation. See [evidence and acceptance](docs/SEAMLESS-SOLO-HUD-NAMES.md).

## 2026-10-30: 0.6.1-solo.4 session correction

Solo.3 startup hooks and HUD movie patch succeeded in the user-operated run.
Gameplay stayed disabled because protocol became 6 in the solo session. Solo.4
adds a pinned-profile exception with fresh local-only membership/host validation
and explicit offline_allowed diagnostics. Compilation and Clippy completed;
regression tests compiled only, not executed. Live membership, activation and
combat behavior remain pending. See [session evidence and ordered acceptance](docs/SEAMLESS-SOLO-SESSION.md).

## 2026-10-30: 0.6.1-solo.3 staged registry correction

Confirmed solo.2 accepts both redirect guards but rejects the changed registry
constructor, leaving gameplay inactive. Solo.3 preserves the exact pinned
1000-entry constructor; compiled and prepared for user testing. Read-only audit:
57/58 checked spans match; optional HUD name lookup uses its safe fallback.
Automated tests compiled only, not executed. See the ordered acceptance procedure
in [SEAMLESS-SOLO-REGISTRY.md](docs/SEAMLESS-SOLO-REGISTRY.md). Not installed into
the running game; no gameplay acceptance claimed.

# 0.6.1-solo.2 - confirmed startup conflict, staged correction

- User reports native summon-dismissal prompt and no friendly lock-on. Log confirms no hooks installed: signature mismatch at RVA4b73d0.
- Read-only capture: 2 of 31 sampled entry/call sites differ; both resolve through inspected Seamless redirects. Handler and callback code fingerprints match the pinned binary. No game memory writes or controls performed.
- Original GameMan online-mode flag observed as0, connection capability1; this does not establish all session-gate values.
- Windows release build and `cargo fmt --all --check`: succeeded.
- Regression cases prepared; `cargo test --lib --locked --no-run --target x86_64-pc-windows-msvc`: compiled without executing tests.
- Normal Clippy completed with the same eight existing style warnings; none in the new redirect module.
- Corrected startup, solo gameplay and connected compatibility: not tested. Installed DLL unchanged while game is open. [Details](docs/SEAMLESS-SOLO-REDIRECTS.md).

# 0.6.1-solo.1 - static/build evidence

- Rust release build (`cargo build --lib --locked --release --target x86_64-pc-windows-msvc`) and formatting check succeeded.
- C# SpiritMendMerge build succeeded with zero warnings/errors.
- Initial additive merge correctly rejected occupied shop ID 100521. Resolved with free shop 100599 and a separate native Kalé menu; no existing shop item replaced.
- Final encrypted merge readback preserved 42,760 existing rows across six affected tables and 188 untouched regulation members; imported seven rows.
- Each of three English message bundles preserved 12,829 existing entries in changed FMGs; nine total new entries. Unrelated message members preserved.
- 164 unrelated NPC script members unchanged; removing the three added Kalé menu/state changes reproduced the original script serialization exactly.
- Automated tests, game launch, solo runtime, shop usability and connected replication: **not executed**. No testing requested from the user in this pass.
- [Solo scope and installed acquisition](docs/SEAMLESS-SOLO.md).

# Seamless probe 1 - compilation only (2026-10-30)

- Release build with `--features seamless-probe --locked`: succeeded.
- Default gameplay release build with `--locked`: succeeded; not installed into the active Seamless profile.
- Probe `cargo clippy --lib --locked --features seamless-probe --target x86_64-pc-windows-msvc -- -D warnings`: succeeded after correcting one new style warning.
- `cargo fmt --all --check`: succeeded. PowerShell installer AST parsed without errors.
- Initial probe compilation found four type/import errors, all corrected before the successful build.
- Automated tests and gameplay tests: **not executed**, reserved for the user.
- No connected-session or replication acceptance claimed. [Next user procedure](docs/SEAMLESS-COMPATIBILITY.md#user-operated-diagnostic-procedure).

# Duplicate prototype acceptance

0.6.0-dev.1 user procedure and known conservative failure modes: [docs/DUPLICATE-GROUPS.md](docs/DUPLICATE-GROUPS.md). Validate stabilization with duplicates disabled before enabling this separate feature. Prepared tests were compiled without execution.

# Development acceptance (not executed)

See [the current audit](docs/STABILIZATION.md). All test execution remains reserved for the user. Run prepared tests with `cargo test --lib --locked --target x86_64-pc-windows-msvc`; `--no-run` only compiles them.

With duplicate mode disabled and matching 0.5.1 assets, record staff/equipment, target, projectile, HP/FP before/after and FPS:

1. No-target cast: zero FP and no shard; animation may play.
2. Enemy-target cast: zero FP and no shard. Unrelated NPCs stay excluded.
3. Full-health owned Spirit: zero FP and no shard.
4. Injured owned Spirit: one charge, 25% max HP capped, only that target.
5. Ordinary staff: base5 FP; Lusat: previously observed8 FP. Record actual modifiers.
6. Unlock AFTER visible release: original target heals if hit.
7. Switch Spirit AFTER release: no redirection. Try successive shots at different targets.
8. Valid shot misses/hits terrain: normal charge, no refund or unrelated heal.
9. Target dies/unloads in flight: no revival or redirected heal; test opportunistically.
10. Ordinary offensive sorcery: normal damage, projectile and cost.

Wait ten seconds before collecting logs. `release rejected` alone does not prove zero FP. `native_spawn_accepted` is not proof of charging. `projectile evidence` compares original launch_target to selected_now. `impact` logs expected HP/native dispatch, not actual HP: compare later HUD samples and visible health/resource changes. Stop on wrong-target healing, duplicate charges, crashes or sustained FPS loss. Historical records below retain their version context.

---

# Publication compilation review

The public source copy compiled successfully for Windows x64; the portable asset tool also compiled with zero warnings/errors against the existing pinned patched SoulsFormatsNEXT dependency. Current PowerShell setup tools parsed successfully. No automated or gameplay tests were executed. Runtime/vendor files match the installed 0.5.1 source byte-for-byte. See evidence/publication-build-review.json and publication build transcripts. The new dependency download script was not executed; its syntax was inspected.

---

# 0.5.1 - compilation/static review; gameplay pending

2026-10-30: Windows x64 release DLL compiled using the locked dependencies. Final build evidence: `evidence/build-0.5.1.txt`. One initial compilation caught a signed BlockId mismatch; fixed with an i32 byte decode before the successful build. Static PE inspection confirmed all six original call targets, cost short-circuit branches, bullet launch-copy path and packet offsets against the inspected executable. `evidence/cast-static/hook-review.json` records the executable hash and original call bytes. Compile-time assertions cover relevant local layouts. These observations do not prove in-game behavior.

Automated tests: NOT RUN, per user instruction. Agent gameplay tests: NOT RUN. Game not launched by agent. New 0.5.1 behavior: NOT YET USER-VALIDATED. The prior 0.5.0 user results below remain evidence only for that release. Installation evidence separately records process checks, backup and file hashes; file verification is not gameplay validation.

User procedure: follow SPIRIT-MEND.md. First establish invalid casts leave FP unchanged (no target, enemy, full-health wolf), then check one paid injured-wolf heal, then unlock/switch after projectile release. Continue with quick successive casts to different wolves, a wall/missed shard, target death during flight if practical, rest/resummon and an ordinary sorcery. Leave the game open and report FP before/after, chosen target, visible projectile/HP behavior and FPS. Do not infer actual HP from `native_accepted` alone.

Known limits: preflight is at release, so a rejected cast may animate; staff modifiers affect costs; valid misses are not refunded. Native hit-packet registry lifetime and the two casting paths still require live acceptance. If a new hook/layout guard rejects, Spirit Mend is disabled with a diagnostic. No multi-target or duplicate same-Ash behavior is added.

---

## 0.5.0 Spirit Mend � first impact healing session confirmed (2026-10-30)

User successfully purchased Spirit Mend from Kale after the shop correction, memorized it and cast at injured locked-on wolves. User reports stable behavior, no FPS drops and no animation oddities, with Lusat's Glintstone Staff+10 and no other FP-increasing equipment. This session does not include a new numeric FPS measurement or agent-controlled gameplay.

Evidence: `evidence/0.5.0-spirit-mend-success-20261030-002802.log`, parsed review `evidence/0.5.0-spirit-mend-success-review.json`, PID27464. Native impact hooks installed at+158ms. One scheduler NotFound retry at+255ms recovered at+305ms; gameplay readiness at+406ms. No adapter panic, disablement, rejected impact or hook conflict in the captured session.

Three accepted impacts and independently sampled HP:

| Impact | Target | HP before | Max | Later observed HP |
| --- | --- | --- | --- | --- |
| +136694ms | wolf #2, handle17100015:ffffffff |392|500|500 at+143867ms |
| +140972ms | wolf #3, handle17100016:ffffffff |365|500|490 at+143867ms |
| +143905ms | wolf #1, handle17100014:ffffffff |381|500|500 at+153952ms |

The middle result establishes an exact125HP increase (25% of500) without full-health clamping; the other two establish the observed cap at500. Wolf#1 remained381HP after the first two effects until its own cast, supporting individual targeting rather than group healing. All three actors stayed registered; their later HUD values remained500/500/490. HP was observed through existing ten-second HUD snapshots, not by adding a live memory probe or new callback. The user report supplies the animation/travel stability observation; these samples do not independently timestamp the visual projectile collision.

FP timeline:295 ->287 ->279 ->271, each decrement preceding one of the three impact events. This differs from the5FP base because Lusat's staff carries SpEffect1934. Read-only extraction from the installed regulation confirms magicConsumptionRate=1.5 (evidence/0.5.0-lusat-cost-param.json). Thus8FP per cast is consistent with native rounding of5*1.5. No evidence of duplicate FP charging in this sequence; native cost commit internals and other cost modifiers remain untraced. The fixed125HP increase with Lusat's+10 supports the intended unscaled percentage in this setup; other catalysts remain untested.

Assessment: acquisition, memorization, native staff casting, custom impact dispatch, individual percentage healing/clamping and modifier-consistent resource consumption are supported by user observations plus log evidence. No corrective build is indicated for this successful path. Invalid-cast no-charge gating and cast-time target latching remain unimplemented, as already documented. Repeated casts to the same target, insufficientFP, interrupted casts, full-health/no-target/enemy casts, targets dying or changing in flight, other Ashes and more catalysts remain separate scenarios. Existing native miss/rejected-impact charging is still the prototype behavior. No game files, installed DLL/assets or saves were changed during this review, and no automated/gameplay tests were executed by the agent.

---

## 0.5.0 shop correction installed

The game exited during preparation. Installed the asset-only correction using work/install-0.5.0-shopfix.ps1; verified four asset hashes, backed up the prior assets/docs/log, and confirmed DLL and log unchanged. Manifest: evidence/install-0.5.0-shopfix.json. Only regulation content differs; all three message bundles are identical. No game launch or tests executed. User must relaunch and confirm Spirit Mend appears at Kale before testing a cast.

## 0.5.0 shop-entry correction � prepared 2026-10-30

User reports Spirit Mend absent from Kale's purchase inventory. Live0.5.0 log PID12832 confirms DLL startup and impact-hook installation at+192ms; this does NOT prove the custom regulation loaded. The profile references dist/mod, but there is no live custom-row diagnostic yet.

Found a concrete asset error: the builder used shop row100599. Paramdex labels divide merchants into smaller ranges: Kale100500..100520, next merchant starts100525, and100575 onward belongs to the Coastal Cave merchant. The fromsoftware-rs invoke-esd example opens100500..100524 (https://github.com/vswarte/fromsoftware-rs/blob/main/examples/invoke-esd/src/lib.rs). Thus100599 was outside the Kale range; prior assumption of100500..100599 was wrong. The installed talk ESD was not extracted/verified in this review, so the range evidence is source example plus shipped-param naming, not a live menu trace.

Builder corrected to collision-checked unused row100521. Regenerated assets are staged in development-0.5.0-shopfix; DLL/impact logic unchanged. User gameplay must confirm shop visibility after closed-game installation/relaunch. No automated or gameplay tests run. Do not interpret this correction as proof that asset loading has already succeeded.

---

## 0.5.0 installation � 2026-10-30

Installed the previously compiled DLL and isolated regulation/English message assets. Verified all 21 payload files by SHA256 and all 17 backed-up files. Existing INI settings retained, adding Sorcery.EnableSpiritMend=true. Usual Launch-Test.ps1 and its profile now load dist/mod; Launch-SpiritMend.ps1 delegates to it. Previous installation, log, both launchers and profile are backed up at `<WORKSPACE>\outputs\SpiritAshResummon\evidence\installed-0.4.2-before-0.5.0-20261030-001829`. Original game regulation and prior log are unchanged. See evidence/install-0.5.0.json for the file manifest. No game launched or automated/gameplay tests executed. Shop availability, native impact path, actual healing and costs await the user's first test.

---

# 0.5.0 development � compilation and asset generation only

Rust Windows release compilation succeeded (evidence/build-0.5.0.txt). Separate C# asset builder compiled and generated regulation/message assets from the installed regulation and extracted English messages. Staged hashes: evidence/staged-0.5.0.json. No automated tests or gameplay tests executed; no game launched; no installed0.4.2 payload changed. Initial library build had78 upstream warnings, final incremental asset-tool build zero warnings/errors. The first controlled user test and limitations are in SPIRIT-MEND.md. Native hit-hook traversal, friendly projectile collision, native healing/clamping, shop acquisition and spell cost are still pending gameplay evidence.

---

## 0.4.2 follow-up � grace cleanup and fresh target acquisition confirmed

User reports sustained 60 FPS with a PS5 DualSense controller during the lock-on test. These are user observations, not agent frame/input measurements. User then locked onto a wolf, rested at grace, stood, resummoned and locked on again without visible problems.

Current-session log (PID 27708) archived as `evidence/0.4.2-grace-lockon-20261029-235017.log`; review: `evidence/0.4.2-grace-lockon-review.json`. At +242076ms the old wolf 17100014:ffffffff is selected; at +242509ms lock is false and the selected handle is empty while all three entries are retiring. At +243773ms no old entries remain. Fresh summoning succeeds at +249493ms, costs 350 -> 295 FP, and three valid owned actors are registered by +249823ms. The player selects fresh wolf handles 17100018:ffffffff at +251674ms and 17100019:ffffffff at +253741ms, then unlocks at +254574ms. This confirms clearing the old selected handle across the reported grace transition and acquiring distinct fresh actor handles. No panic, disabled, failed, mismatch or error diagnostics in the archived session. A later cleanup at +256224..257446ms also clears all three actors with FP restored; its cause was not explicitly reported and is not required for the first sequence's result.

Assessment: the core mixed native targeting and this grace-cleanup/reacquisition scenario are validated by user operation plus telemetry. No repeat of these checks is needed now. Selected-target combat death, occlusion/range, unrelated friendly NPC exclusion and other Ash types remain separate coverage gaps. The next development step is the staff-cast Intelligence healing sorcery using the selected owned actor, with target validity and resource-cost commitment handled at cast completion; neither spell behavior nor duplicate same-Ash groups are validated by this targeting test. No installed files were changed and no tests were executed by the agent.

---

## 0.4.2 user-operated friendly lock-on � core behavior confirmed

2026-10-29, PID 27708. Archived current-session evidence: `evidence/0.4.2-friendly-lockon-20261029-234735.log`; parsed results: `evidence/0.4.2-friendly-lockon-review.json`. User confirms wolf lock-on, cycling among wolves and switching to an enemy. No gameplay or automated tests were performed by the agent.

Startup signature checks and the friendly camera call hook succeeded (+118ms); task readiness followed at +310ms. Three owned wolf handles were actually selected: 17100014:ffffffff at +47030ms, 17100015:ffffffff at +47864ms, 17100016:ffffffff at +48280ms, then the first wolf again at +48930ms. At +50397ms, unlock cleared the selected handle. Later samples show enemy handle 17300018:3c2a2500 at +59414ms -> wolf14 at +61281ms -> enemy at +61914ms -> wolf16 at +62131ms -> wolf15 at +63198ms -> enemy at +64047ms. The observer classifies the non-owned handle as other_or_unresolved; the user's report identifies it as an enemy. These are target-state observations correlated with the reported cycling, not raw input capture. Wolf16 remains selected in samples at +70714ms and +80814ms before the enemy is selected at +82264ms. Unlock at +86450ms again clears the handle.

Across captured complete timing windows: friendly_relation_adapter 424451 calls, 37338us total, 32us maximum (approximately 0.088us average across all callbacks, including native-allowed fast paths; not an owned-validation-only average). This measures adapter time after the original relation call, not total camera/frame cost. target_sample: 665 calls, 12573us total, 499us maximum. Zero readable_probe calls; no panic, disabled, failed, mismatch, retry or error diagnostics in the archived current session. Three wolves remain registered in the final samples; recorded summon cost is 350 -> 295 FP. FPS and input method were not reported and cannot be inferred from callback counters.

Assessment: native friendly acquisition, individual wolf cycling, mixed enemy/Spirit selection, sustained wolf selection and unlock are supported by this session. No correction is indicated by this log. Remaining checks: selected-Spirit death/disappearance and rest/loading cleanup, range/occlusion behavior, exclusion of unrelated friendly NPCs/Torrent, other Ash types, and downstream spell/attack behavior. The dedicated Intelligence healing sorcery and duplicate same-Ash groups remain unimplemented. Recommended next targeted check is resting at grace while locked onto a wolf, then reacquiring a fresh summon; this provides a simple actor-lifetime transition before adding spell targeting.

---

# 0.4.2 � friendly lock-on: build complete, gameplay pending

Release x64 compilation succeeded without warnings using `cargo build --lib --locked --release --target x86_64-pc-windows-msvc --target-dir ../../work/prototype-build`; output is `evidence/build-0.4.2.txt`. Source inspection checked the 0x7179d6 caller context against saved native disassembly and traced the relation predicate, candidate scoring and switch list. This is not runtime proof. No automated tests, game launches or gameplay tests were executed by the agent, per the user's instruction. The install manifest records hashes, preserved configuration/log and backup location.

First user-operated check:

1. Launch the usual offline test profile. Load at Gatefront and wait five seconds; note FPS.
2. Summon Lone Wolf Ashes once. With wolves close and visible, aim toward one and use normal lock-on. Hold any successful wolf selection for five seconds. Try cycling to the other wolves using your usual directional controls, holding each selection about five seconds.
3. With wolves and at least one live enemy visible and in normal lock-on range, cycle both directions. Try enemy -> wolf -> another wolf -> enemy. The order is native spatial ordering, not a fixed sequence. Do not attack the friendly target during this first selection check.
4. Unlock, wait five seconds, then attempt to reacquire a wolf. Wait another ten seconds for the aggregate log. Report whether wolf acquisition, wolf-to-wolf cycling and enemy-to-wolf/back cycling worked; include controller vs mouse/keyboard and FPS. Leave the game open for log review.

If wolves cannot be selected, perform one normal enemy lock and hold it five seconds, then stop this test and report the result. The candidate counters and observer distinguish partial admission from actual selection; repeated input will not fix an unmet native eligibility gate.

After the first check succeeds: verify target release when a selected Spirit dies/disappears, grace/title/death cleanup, distant or obstructed targets, and that unrelated friendly NPCs/Torrent do not become eligible. These are pending, as are any downstream attack/spell behavior while targeting a Spirit. Friendly lock-on itself does not grant a healing spell or duplicate same-Ash summoning.

---

## 0.4.1 user-operated target diagnostic � observed result

2026-10-29, PID11968. Snapshot `evidence/0.4.1-lockon-20261029-233642.log`; compact summary `evidence/0.4.1-lockon-review.json`. User reports targeting several enemies and holding for roughly five seconds. Diagnostic signature/layout checks passed at+58ms. Seven distinct non-owned target handles appeared with locked=true; a previously selected handle reappeared after switching away. Three registered wolf handles remained distinct from all selected handles. Unlock at+102987ms cleared the selected handle to ffffffff:ffffffff and set locked=false; re-lock and subsequent unlock were also captured.

Captured target_sample counters:634 samples,11305us total,499us maximum (about0.018ms average). No VirtualQuery readable_probe calls in the captured session. One initial CSTask NotFound retry recovered during startup; no observer-disabled or runtime panic observed. FPS/input method were not reported, so these callback timings are not a frame-rate measurement. Native target identities are intentionally not dereferenced; their classification as enemies is supplied by the user's action report. Later summon cleanup does not invalidate target-read correlation and its cause was not independently determined.

Result: enough evidence for the read-only selected-handle baseline; no repeat of this test is needed now. Next: narrow friendly candidate admission/maintain/cycle investigation. This is not proof that friendly targeting or the proposed sorcery works. No game changes or test execution by the agent during this review.

# 0.4.1 diagnostic validation (2026-10-29)

Release x64 compilation succeeded without warnings (`evidence/build-0.4.1.txt`). No automated tests, runtime injection, launch or gameplay tests executed by the agent. Exact native field-access evidence is saved under `evidence/lockon-static/score.asm` and `lock_toggle.asm`; live field behavior awaits the following user-operated test.

1. Launch the usual offline test profile, load at Gatefront, and wait5seconds with lock-on OFF. Note FPS.
2. Summon wolves once and wait5seconds. Existing one-group-per-Ash/healing rules remain; this build does not implement duplicates or the sorcery.
3. With at least two ordinary enemies in sight, lock onto one for5seconds. Cycle to another enemy using your usual controls and hold5seconds. Cycle back and hold5seconds; then unlock and wait5seconds. Avoid killing the targets during this sequence if practical.
4. Optional: stand near a wolf and attempt ordinary lock-on, then unlock. Friendly lock-on is NOT enabled yet; inability to select it is expected. Report whether lock-on behaved normally, which input method you used (controller or mouse/keyboard), and FPS. Leave the game open for log review.

Expected evidence: observer-ready startup; not_locked -> other_or_unresolved with nonempty handleA -> different handleB -> A -> not_locked, plus three registered owned Spirit handles that differ from those enemy selections. Off-target handle data may be empty/stale while locked=false; that alone is not an error. Entries whose states are loading/retiring can be skipped and are never healing candidates. Camera-update vs GameMan timing can yield transient mismatches; record correlation, do not claim healing eligibility from this sample alone.

After this baseline, the next implementation task is the narrowly scoped friendly admission/retain/cycle path, preserving native enemies, geometry/range checks, allegiance, friendly damage and AI. No friendly-target or spell functionality is claimed by this release.

---

## 0.4.0 first user-operated session: Church of Elleh boundary test

2026-10-29, PID 10064. Archived log: `evidence/0.4.0-elleh-boundary-20261029-231346.log`; compact review: `evidence/0.4.0-elleh-boundary-review.json`.

User confirms summoning wolves at Church of Elleh for 55 FP, walking to Gatefront and back without dismissal, and steady 60 FPS. User also confirms resting at a grace afterward, explaining later cleanup rather than an unexpected boundary dismissal. Log records all 304 monument rows patched, fallback selection across map tiles, first wolf request while native range=false, three slots after spawning, and FP350->295. A later fresh wolf summon also spends55. Both types coexist at five slots; five wolf heals each spend55 and one Militiaman heal spends44. No panic, disabled-adapter or error messages in captured0.4.0 session. Rest cause comes from user confirmation, not a distinct engine cause code.

Timing includes an occasional activation peak around20ms and inactive cleanup around11ms (fresh OS page checks), despite user reporting steady60FPS. No recurring per-frame page-probe pattern appeared in ordinary active/idle windows. This observation validates the reported route only; distant maps, cleared dungeon interiors, scripted hubs and HP-cost activations remain unverified. No gameplay or automated tests executed by the agent.

# 0.4.0 validation � unrestricted zones (2026-10-29)

- Windows x64 optimized release compilation succeeded with no warnings (`evidence/build-0.4.0.txt`). No automated tests, DLL loading, game launch, injection or gameplay tests executed by the agent.
- Source inspected against pinned MultipleSpiritAshes extras.rs monument rules, fallback and lib.rs player-position synchronization. Existing local native-state disassembly shows the current/used monument comparison; no new binary patches are introduced.
- 0.3.1 user report: everything working well, no issues with the individual-entity UI. This does not establish 0.4.0 runtime behavior.
- New unrestricted mode is untested. Resource handling, group identity, HUD and version hooks were not intentionally changed; their behavior still needs regression observation after this update.

## User-operated 0.4.0 checks (in order)

1. Use the existing offline test launch. Load directly at Church of Elleh, stand on ordinary level ground away from a grace interaction, wait 15 seconds. Record FPS. The gravestone icon is not an acceptance criterion in this mode.
2. With no active group and enough FP, summon Lone Wolf Ashes once. Expect three wolves near the player and exactly 55 FP spent. Wait 15 seconds; report whether they stay present and whether FPS stays near the previous 60.
3. Walk from the church toward Gatefront and back across the former zone boundary without resting. Expect the wolves to remain. Restore FP as needed, add Militiamen for 44 FP, verify coexistence and four individual bars.
4. Let wolves take damage, heal once for 55 FP; Militiamen must not heal. Full-health reuse and insufficient FP should remain free no-ops. Try resummoning wolves after all three permanently die while the other type survives.
5. After the first checks succeed, quit/continue directly in a formerly prohibited location and summon again. Test an already-cleared Groveside Cave arena and a more distant ordinary overworld area without a nearby monument. Verify spawn is near you. Record exact location and action if greyed out or if anything disappears.
6. Rest, die, and fast travel separately: existing native cleanup should remove old groups; subsequent use should make fresh groups without stale bars or duplicates. Test HP-cost ashes separately if available. Optional: a scripted hub may still block item use; report it rather than treating ordinary terrain success as all-map coverage.

No scene-wide, high-count, hub/cutscene, DLC, HP-cost or loaded-map transition compatibility claim is made. Natural terrain and navigation restrictions are still present. To revert zones, exit, set PreserveSummoningRestrictions=true and restart; backup installation can restore 0.3.1.

---

# Validation record and user-operated test guide

## 0.3.1 four-entity HUD: compile/static review only

Release compilation succeeded without warnings; see evidence/build-0.3.1.txt. No automated tests, game launch, prototype loading or gameplay actions were performed by the agent. The 0.3.0 gameplay observations below do not establish that the four-row update works in-game. Installed files remain 0.3.0 until separately authorized installation; evidence/staged-0.3.1.json records hashes and status.

Source inspection checks the same MAX_ROWS=4 drives the view selection, native injection/reset bookkeeping and movie plan. All four row names/template references are required before rewriting. Previous extracted-asset analysis already identified RemotePCTag0..3 as instances of sprite 310, with intact Name/HP children. No actor-health/cost mutation code was changed. Offsets, spacing and scale remain the user-approved values.

After installation, user test:

1. Load Gatefront and check idle FPS/HUD. Summon wolves: expect three compact rows, #1/#2/#3, each at that wolf's own full HP; normal 55 FP cost.
2. Let individual wolves take different damage. Their bars should differ rather than mirror an aggregate. Heal the Ash once: all living wolves heal under existing rules and cost 55 FP once.
3. Summon Militiamen too: expect four total rows despite five visible entities (three wolf rows and one Militiaman if wolves were tracked first). Normal second-summon cost is 44 FP; the hidden fifth member remains active and healable.
4. Let a displayed entity die: remaining survivors must retain their own HP, and a hidden living member should fill the available row. Reuse an Ash to ensure hidden members are still healed. Skeleton revival may reassign its cosmetic row number; no premature despawn or duplicate should occur.
5. Report FPS, any overlap/clipping with four rows, and an optional screenshot. Later note reload/rest/death behavior and NPC ally conflicts during ordinary gameplay.

No acceptance row is marked passed yet. Recovery remains restart with [HUD] EnableGroupHud=false. The legacy setting name now controls the individual-entity HUD.

## 0.3.0 combat/healing HUD: user confirms both Ash types

User confirms both tests worked and all summon health bars acted normally during combat and healing. Archived evidence: evidence/0.3.0-hud-healing-20261029-221612.log. Wolf heal at +275,521 ms changes 350->475, 435->500 and 361->486 (max 500 each). The next HUD snapshot shows the exact sum 1461/1500, x3; FP changes 295->240, a 55 FP charge. Militiaman +10 heal at +326,379 ms changes 2269->2709 and 2536->2709 (max 2709 each), correctly capped at full health.

The captured heals occur in separate single-group sequences following the earlier successful two-group display. Therefore this establishes correct combat/heal display for each type, plus the earlier simultaneous full-health display; it does not independently establish unchanged-other-bar behavior during simultaneous two-group healing. No new clarification or scripted test is required now: retain that narrower coverage boundary for later gameplay. The user performs testing; no agent-driven tests or installed changes were made. Placement remains as approved.

## 0.3.0 two-group HUD: first successful user observation

User reports average 60 FPS with wolves and Militiamen summoned and approves the bar placement. The supplied 2560x1440 screenshot (evidence/0.3.0-two-bars-user-screenshot.jpg) visibly shows three wolves and two Militiamen, two separate readable group rows labeled Lone Wolf Ashes x3 and Skeletal Militiaman Ashes x2, with full-looking bars. No overlap between these rows or duplicate vanilla Spirit panel is visible in that image. This does not establish all HUD layouts, menus or resolutions.

Archived log: evidence/0.3.0-two-bars-20261029-221202.log. Initial wolf summon at +49,958 ms has FP 350 -> 295 (55); Militiaman +10 summon at +54,792 ms leaves FP 251 (44 more) and five group entries. Repeated HUD snapshots show wolves 1500/1500 across three survivors, Militiamen 5418/5418 across two, movie_ready=true and native_occupied=0. Settled ten-second windows contain about 600 copy calls, consistent with the reported performance but not an independent FPS measurement. At +133,843 ms HUD sampling used 2,350 microseconds total over 92 samples, maximum 423 microseconds (0.423 ms); no readable-page probes occurred in that window. This measures the collector, not the full HUD rendering cost.

Placement and initial two-group display are confirmed for this observed case. Independent bar response to damage/healing, partial deaths, revival, NPC ally interaction and transitions remain untested in 0.3.0. Next user check: damage and heal one group, confirm only its bar rises and normal FP is charged, then repeat for the other group. No code, installed configuration or DLL was changed; no tests were executed by the agent.

## 0.3.0 HUD prototype - compiled, not installed or gameplay-tested

Source build: `cargo build --lib --locked --release --target x86_64-pc-windows-msvc --target-dir ../../work/prototype-build`; successful without warnings, recorded in evidence/build-0.3.0.txt. No unit tests or Preflight script were run. No prototype DLL was loaded, and the game was not launched or controlled. Static Python analysis read the extracted GFX and on-disk executable; it did not invoke engine functions or the new movie patcher. Exact evidence is in hud-layout-analysis.json and hud-native-analysis.json.

Installed DLL remains 0.2.2 (SHA-256 E655D3DD7FEED27CC9F6BC8602DA25EAECB1B5A2167DF3AB7AB3E51763EF8517). Stage manifest records the new build hash. The latest 0.2.2 resummon correction remains unconfirmed because the user moved to normal gameplay testing.

First user-operated sequence after closed-game installation of the staged build:

1. Launch the normal offline test profile. Load Gatefront, wait 15 seconds without summoning, check for missing/misplaced normal HUD and FPS changes. Leave the game open for log inspection if anything is wrong.
2. On flat ground summon Lone Wolf Ashes. Expect one named row with x3, full HP and the normal 55 FP cost. Wait 15 seconds and report FPS/placement. If no row appears, stop and preserve the log.
3. Restore FP and summon Militiamen. Expect a second named row with x2; wolves remain and the normal 44 FP cost applies. Report overlap, clipping or any additional stale vanilla Spirit panel. A screenshot would help position adjustment.
4. Damage only wolves, then heal them once. Only the wolf row should rise; Militiaman row should remain unchanged; cost is still 55 FP. Repeat for Militiamen at 44 FP.
5. Later during ordinary gameplay observe partial deaths, complete group death/resummon, skeleton revival, rest/reload/death and menu/HUD hiding. Group bars must follow live state without stale references. NPC ally tags and >2 groups are unvalidated edge cases.

If bars disappear or the HUD malfunctions, exit normally, set [HUD] EnableGroupHud=false in the installed INI, and relaunch. This restores the unmodified HUD on the next process; it does not hot-unpatch the current process. No further test is reported as passed from compilation or static evidence.

## User-directed testing transition - 2026-10-29

0.2.2 was installed with backup and hash verification (evidence/install-0.2.2.json). The user elected to evaluate it through ordinary gameplay and report issues instead of continuing the scripted sequence. This does not establish a passing resummon result. Investigation moved to a two-row HUD with one bar per Ash group; see HUD-INVESTIGATION.md. No new tests were executed.

## 0.2.2 correction: compilation only; user test pending

The user subsequently reports that wolves could not be resummoned while Militiamen remained active. Evidence: `evidence/0.2.1-resummon-blocked-20261029-211535.log`, PID 39660. Militiamen +10 (212010) spawn at +1,493,608 ms and wolves (232000) join at +1,497,897 ms; costs are 44 and 55 FP. At +1,561,235 ms the wolf native/query pair is (2,-1), with alive/range true, no cooldown, five entries, zero exiting and 251 FP. Registry rejection then records 4/2 and 2/2 states. No wolf summon callback occurs in this blocked sequence. This supports an adapter lifecycle rejection; the native summon routine never gets the request. Entry counts alone do not establish how many Spirits were visibly alive.

0.2.2 adds guarded inactive-member cleanup and an HP-zero replacement guard. Release DLL compilation succeeded with the locked MSVC target (`evidence/build-0.2.2.txt`). A new policy truth-table test was added as source only. No automated tests, Preflight execution, DLL loading, or gameplay tests were performed by the agent. Existing static disassembly supports the signature/field chain; runtime semantics remain unproven. The game was still running 0.2.1 during preparation, so installed files were not replaced.

Next user-operated test after closing the game and installing 0.2.2:

1. Load Gatefront, wait 15 seconds without summoning, and check FPS/startup.
2. Summon Militiamen and wolves on flat ground. Confirm five visible Spirits and normal 44/55 FP costs. Refill FP.
3. Let all three wolves die while keeping at least one Militiaman continuously alive. Stay in the permitted area; do not rest or reload. Use Wolf Ashes once on flat ground with at least 55 FP.
4. Report whether three fresh wolves appear, the old Militiaman remains, FP drops by exactly 55, and any delay or hitch. If blocked, wait 15 seconds without further input and preserve the log; report whether the item was greyed or the bell played.
5. Only after that succeeds, check partial wolf death heals survivors without replacement, and verify knocked-down Militiamen can revive normally rather than being prematurely removed. Do not mark these cases passed before observation.

This correction does not change the earlier wall-placement observations. Boss resummoning remains deferred/not attempted; HP-cost and high-count behavior remain untested. Historical sections and matrices below describe their original release states, not current acceptance.

## 0.2.1 reported wolf resummon with a surviving Militiaman: not yet corroborated

Clarification: the user confirms making the extra attempts from the top of a wall, where summon placement was blocked, and succeeding on flat ground. This is consistent with the native false returns and preserved placement checks; the exact internal rejection branch was not instrumented. The failed attempts consumed no observed FP, and the successful ground-level attempt consumed 55. The retry discrepancy is resolved without a mod correction. Survivor retention remains unverified because all old entries had cleared before the successful wolf summon; this specific scenario should be repeated on flat ground with a continuously living Militiaman. Keep failed-placement behavior separate from the survivor-retention acceptance test.

User reports all wolves died while a Militiaman survived, then one immediate wolf resummon for 55 FP, with the Militiaman retained and stable 60 FPS. Evidence: `evidence/0.2.1-wolf-resummon-with-militiaman-20261029-210536.log`, PID 39660 through +1,024,636 ms. The captured sequence does not establish that specific survival case; clarification about retries or a separate attempt was requested. Do not mark this acceptance scenario passed from the reported outcome alone.

The setup summons wolves at +919,428 ms and Militiamen at +923,584 ms, leaving five entries and 251 FP. At +975,466 ms the native alive/range flags become false and all five entries are exiting. At +978,953 ms a wolf activation reaches AddGroup, but the original native summon routine returns false; no request remains pending and FP stays 251. At +985,274 ms all old entries have cleared. Another wolf callback at +986,149 ms passes through the ordinary native path and also returns false, again without observed FP spending. At +995,505 ms the next wolf callback returns true from an empty owner list; three new entries appear and FP drops 251 -> 196, exactly 55.

At +1,006,729 ms a new Militiaman activation adds two entries while preserving the new wolves, with FP 196 -> 152 (44). This is another supported different-type addition, not proof that an old Militiaman survived the preceding wolf resummon. No wolf AddGroup success while another old type was retained appears in the inspected sequence. A later live reread through +1,065,599 ms adds no further summon callback matching the claimed scenario.

The two native false returns are recorded as unresolved spawn refusals, not an adapter query block, an automatic delayed replay, or a proven cleanup bug. One refusal occurs after the old entries are already gone, so retained old group entries alone cannot explain both. Exact native rejection reasons are not logged; no eligibility bypass or timer reset is justified from this evidence. No panic or disabled-adapter message occurs, and settled task windows remain about 60 per second, consistent with the user's performance report. No new code, installed DLL/config or game input was changed by the agent.

## 0.2.1 Militiaman-only repeated healing: successful observed case

User reports only Militiamen healed while wolves and Militiamen were active, all five stayed present, and FPS stayed at 60. After reviewing the log, the user confirms three healing activations. Evidence: `evidence/0.2.1-militiaman-heal-multiple-20261029-210130.log`, PID 39660 through +774,481 ms. The original 350 -> 306 report is superseded by the recorded sequence below; each activation costs the expected 44 FP.

The setup summons wolves at +686,064 ms and adds Militiamen at +690,513 ms. Starting from 350 FP, normal initial costs leave 251 FP (350 - 55 - 44). Five local entries exist before each subsequent heal, with no exit requested at the activation snapshots. Selected and native active Goods/effect are 212000.

| Heal callback | Member HP changes (max 365 each) | Sampled FP |
|---|---|---|
| +716,184 ms | 53 -> 144; 49 -> 140 | 251 -> 207 |
| +720,201 ms | 1 -> 92; 1 -> 92 | 207 -> 163 |
| +723,961 ms | 43 -> 134; 77 -> 168 | 163 -> 119 |

Every member gains floor(365 * 0.25) = 91 HP without a maximum-HP change. Each activation logs exactly two Militiaman targets and changed_members=2. No wolf HP write, AddGroup request or native spawn occurs during these three activations. Group entries remain five after each charge; the user's observations establish visible retention and type isolation beyond the entry counts. The callbacks span approximately 17-19 ms by timestamps, not direct frame/input latency. No panic or disabled-adapter message occurs; continuing task windows are about 60 tasks/second, consistent with the user's 60 FPS report.

Later 14 FP decreases have no corresponding Ash callback and are not attributed to this mod. All entries enter cleanup at +760,345 ms and clear by +770,426 ms; the cause of this later cleanup was not supplied for this test and is not classified as either a defect or a passed lifecycle scenario. Registry transition rejections after the heals do not establish failed healing. No code, configuration, installed DLL or game inputs were changed by the agent.

This supports repeated paid Militiaman-only healing while wolves coexist, complementing the earlier wolf-only test. It does not establish revival behavior merely because one activation targets 1-HP skeletons. The next independent test is complete death of one type while the other remains alive: resummon both types, allow all wolves to die while at least one Militiaman remains alive, then activate Wolves once without resting/reloading. Expect three fresh wolves at full HP, retained Militiamen and one 55 FP charge. If both types die first, do not classify that attempt as this test.

## 0.2.1 wolf-only healing with two types active: successful observed case

User reports using Lone Wolf Ashes while wolves and Militiamen were active: only wolves healed, all five stayed present, and FPS stayed at 60. Evidence: `evidence/0.2.1-wolf-heal-multiple-20261029-205629.log`, PID 39660 through +475,143 ms. The initial report gave FP 91 -> 36; the user subsequently confirmed increasing maximum FP. The actual recorded charge is 350 -> 295, still exactly 55 FP. Preserve these actual values instead of repeating the original absolute FP report.

At +421,793 ms, the selected effect/Goods is 232000 (wolves), while native active Goods/effect is 212000 (Militiamen). Five local entries are present with none exiting. At +421,811 ms three wolf targets with max HP 500 receive 392 -> 500, 90 -> 215 and 418 -> 500. Each gain is 125 or capped at 500; changed_members=3. No Militiaman HP write, AddGroup request or native spawn is logged for this activation. At +422,054 ms, FP is 295 and five non-exiting entries remain. This specifically exercises healing an earlier type instead of the native last-active type. User observations corroborate target isolation and no visible duplication/dismissal.

The callback window records 18,486 us for the heal, including 16,941 us of page probes. About 60 game tasks/second and the user's stable 60 FPS support no sustained performance regression in this case; they do not exclude a single-frame hitch. No panic or disabled-adapter message occurs. Later registry 4/2 and 2/2 rejections concern transitioning actors and are not evidence of an invalid healing write.

At +457,007 ms all five entries enter cleanup, then clear by +463,691 ms. The user clarified that they stayed at Gatefront but rested at a Site of Grace after healing. This resolves the cleanup context: do not record it as an unexpected disappearance or an area-exit test. No uncontrolled code or parameter change is justified by this sequence.

Record wolf-only healing, maximum-HP capping and one observed native-sized FP charge as successful for this multi-type case. The capture also includes other Goods activations before the controlled wolf heal; those are not promoted to additional acceptance tests without user context. Militiaman-only healing, one-type death/resummon while another survives, upgrade-variant reuse, HP costs and higher-count behavior remain pending. No agent-operated gameplay or automated tests were run; installed DLL/config were unchanged.

Next user test: summon both groups again after resting. Let a living Militiaman take damage, keep wolves present, note current FP and use Skeletal Militiaman Ashes once with at least 44 FP. Expected: only surviving Militiamen heal, wolves stay unchanged, no extra members appear, and exactly 44 FP is spent. Record actual FP before/after (maximum FP is now higher), group counts and FPS, then leave the game open for log review. A wounded wolf makes cross-type isolation easier to observe, but the native trace/target writes also provide evidence.


## 0.2.1 coexistence: successful observed case

User summoned wolves, restored FP to 91 and activated Militiamen, then explicitly confirmed all three wolves and both Militiamen stayed visible at stable 60 FPS. Evidence: `evidence/0.2.1-second-ash-20261029-204950.log`, PID 39660 through +74,860 ms. The installed 0.2.1 session logs eleven hooks and readiness. No panic, disabled-adapter message or initialization failure appears.

First wolf summon: +33,040 ms native result=true, followed by three entries and sampled FP 91 -> 36. Refill to 91 appears at +36,104 ms. The new eligibility diagnostic reports Goods 212000, native -1, adapter 0. At +39,540 ms the Militiamen effect callback runs with native active state 2, three existing entries, range=true and no cooldown. At +39,551 ms AddGroup returns native true; at +39,562 ms the exact-request guard suppresses owner-group clearing. At +39,677 ms there are five non-exiting local entries and FP=47, a 44 FP decrease. The existing wolves were retained according to the guard log and user observation, rather than inferred solely from entry count.

Settled active windows record approximately 600 game tasks per ten seconds, consistent with the user's stable 60 FPS report. The latest complete window has task max 596 us, query max 20 us and cost max 47 us, with no registry/handle/health rejection. Load/update 4/4 actor validation uses the 520-entry registry. These measurements support this small-group case only; they do not validate high-count performance. The AddGroup callback spans about 11 ms by logged timestamps, not a direct input-to-frame latency measurement.

Record first different-type coexistence and its normal observed FP charge as passed for this case. Cross-type healing isolation, repeated same-type reuse, resummoning one dead type while another survives, upgrades-as-same-type, more than ten bodies, HP costs and transition regressions remain pending. No automated or agent-operated gameplay tests were run; the agent only read/archived logs and updated source documentation.

Next user test: while both types are present, let wolves take damage, refill to 91 FP and use Lone Wolf Ashes once. Prefer having a wounded living member of each type to make healing isolation visible. Expected: only surviving wolves heal by 25% of their own maximum HP (capped), no new wolves or dismissal, Militiamen remain unchanged, and FP drops by 55 (91 -> 36). Wait 15 seconds, report HP/count/FP/FPS observations and leave the game open for log review. Test Militiaman reuse separately afterward.

## 0.2.0 second-type activation blocked; 0.2.1 correction staged

User reports Militiaman Ashes greyed out after summoning wolves. Evidence: `evidence/0.2.0-second-ash-blocked-20261029-204428.log`, PID 26904. Wolf summon at +134,843 ms returns native success, produces three local entries and sampled FP changes 91 -> 36. Refill to 91 is logged at +138,124 ms. At +166,976 ms queued Goods=212000 (Militiamen), active Goods/effect=232010 (upgraded wolves), three non-exiting entries, alive/in-range=true, cooldown zero, and FP=91. Latest sampled menu query is native -1/adapter 0; no second summon callback follows. This rules out lack of FP or adapter menu rejection in the sampled state; it does not by itself identify every native predicate.

The expanded registry reports 520 entries and accepts load/update 4/4 after brief 3/4 loading rejections. Settled active windows record 600 tasks per about ten seconds, with no panic/disabled message. This supports the first-summon path, not coexistence. Static inspection found the untouched Goods-specific state call at 0x690833 feeding item-usability rejection independently of the hooked queries/costs. 0.2.1 routes that call through the same validated per-family policy and adds separate bounded eligibility diagnostics. No item-limit/area/price parameters were changed.

0.2.1 release compilation is recorded in `evidence/build-0.2.1.txt`. Automated/gameplay tests have NOT been executed. The update is staged in `pending-0.2.1`; installation requires the game to be closed. Current-session log and installed 0.2.0 remain preserved. After installation, repeat the controlled wolves/refill-to-91/Militiamen test; report greying, counts, FP and FPS. Expected outcome remains five Spirits, no dismissal, and a 44 FP second charge. Do not mark the correction successful before the user test.


## 0.2.0 startup and idle: successful observed session

User loaded the test character at Gatefront, waited at least 15 seconds without summoning and reported stable 60 FPS. Evidence: `evidence/0.2.0-idle-20261029-204111.log`, PID 26904, through +75,115 ms. Ten call sites and multi-type pool configuration are logged at +61 ms; game-task readiness follows at +496 ms. The temporary scheduler NotFound resolves on the second attempt.

Loaded-player task execution continues past the exact 520-entry capacity guard, supporting successful registry construction; the startup pool message alone only describes intended configuration. Offline mode/no multiplayer session and no conflicting DLL are logged. Resummon flags are updated for 304 BuddyStone rows. Latest native/adapter query is (0,0), with zero groups and 91 FP. No summon or heal callback occurs. No panic, disabled-adapter message or validation rejection is logged.

Two settled windows record 600 game tasks per approximately ten seconds, consistent with the user's 60 FPS report, not an independent frame-time measurement. Latest task total is 14,614 us/max 420 us; query max 1 us and cost max 9 us. No actor traversal or healing is exercised. Coexistence, active performance, costs and healing remain untested in 0.2.0.

Next user test: summon wolves, refill FP to 91, then summon Militiamen while wolves remain alive. Expect three wolves and two Militiamen together, second cost 44 FP (91 -> 47), no dismissal. Wait 15 seconds and report counts/FP/FPS. Stop on any first-summon slowdown or issue. The agent only inspected/archived logs and updated source documentation; no game inputs or installed files changed.


## 0.2.0 multi-type prototype � pending user validation

The user moved on from boss resummoning after another boss died too quickly. That case remains NOT ATTEMPTED. The original different-Ash rejection requirement is replaced by one group per unique Ash type. Earlier 0.1.12 results remain historical and do not validate 0.2.0.

Release compilation for x86_64-pc-windows-msvc succeeded (`evidence/build-0.2.0.txt`). Static source/instruction inspection identified the native spawn clear call, internal state call, constructor and slot search. REA deep decompilation was unavailable; local pefile/Capstone inspection supplied the instruction evidence. No automated tests, DLL execution probes or gameplay tests were run for this update, per user instructions. New pure-policy test cases are supplied but unexecuted.

### First user-run session

1. Launch the existing offline test profile. Load Gatefront and wait 15 seconds without summoning; report FPS and leave the game open for startup-log review. Stop here if there is a crash, slowdown or disabled-adapter message.
2. Summon Lone Wolf Ashes once. Expect three wolves and 55 FP spent. Refill to 91 FP.
3. While wolves remain alive, use Skeletal Militiaman Ashes once. Expect two Militiamen alongside the wolves, 44 FP spent (91 -> 47), no dismissal and no duplicate wolves. Wait 15 seconds, report counts/FPS/FP, and request log review.
4. After the coexistence log is reviewed, injure both types and refill FP. Use Wolves once: only surviving wolves should heal, costing 55 FP. Refill and use Militiamen once: only their surviving members should heal, costing 44 FP. Record counts and HP/cost observations separately.

### Follow-up cases (not yet run)

- Repeat either Ash at full health: no charge, dismissal or additional members.
- Attempt a heal or a new type with insufficient resources: no effect/charge. HP-cost Ashes require separate testing.
- Kill one member of one type: heal its survivors without reviving it or affecting the other type.
- Let an entire type die while another survives: resummon only the dead type, normal cost once, others retained. Repeat with each type and verify no delay/duplicates.
- Try a different upgrade of an already-active Ash if available: heals its existing group; does not create a second group or change current stats.
- Add more distinct types in stages, including more than ten total bodies to exercise expanded slots. Monitor counts, FPS and logs; do not jump straight to a stress test.
- Leave the summoning area, reload and die with several types active, as separate cases: normal cleanup, fresh groups on eligible return and no orphaned entities. Boss cleanup must also be retested.
- Test interrupted casting, rapid item switching and partially loading/retiring groups. Unknown transitions must not spawn duplicates.

There is a 500-body pool safety ceiling, not infinite engine capacity. High-count stability, native delayed-spawn behavior, item-limit compatibility and exact-once cost ordering remain unproven. Boss resummoning remains deferred at user request.

---


## Boss-test clarification: Beastman defeated before resummoning was attempted

User identifies the encounter as Beastman of Farum Azula in Groveside Cave and corrects the earlier report: healing was tested, but the boss died too quickly and resummoning was forgotten. This resolves the discrepancy with `evidence/0.1.12-boss-20261029-200709.log`: there was no claimed successful post-death summon to locate. Record boss-fight healing and its costs as supported by the two logged heal commits; mark in-fight resummoning NOT ATTEMPTED, not passed or failed. The later native rejection is consistent with the user's boss-defeat context, although the log itself does not identify the boss or record its defeat event.

Next focused boss test should use another unfinished encounter that normally allows Spirit Ashes. Initial summon, keep at least 55 FP available, allow the entire wolf group to die while the boss remains alive, then activate once without resting/reloading. Skip the already-validated healing step for this attempt if it makes the timing easier. Expected: three fresh wolves, one 55 FP charge and no extra delay or duplication. No need to repeat the completed Beastman fight. HP-based costs remain a separate pending validation category. No code, installed DLL/config or gameplay changed by the agent.

## 0.1.12 boss report: healing confirmed; reported resummon not yet reconciled with log

User reports boss-fight healing and delay-free resummoning after all wolves died, each activation costing 55 FP, with no duplicates/issues. Archived session: `evidence/0.1.12-boss-20261029-200709.log`, PID 3476 through +2,564,013 ms. Boss identity and whether the reported resummon preceded boss defeat were requested because the inspected sequence does not yet contain the claimed post-death summon. Boss context is user-provided; no boss event/identity is logged.

Observed initial wolf summon: +2,493,349 ms, native true, sampled FP 91 -> 36. First heal: callback +2,509,901 ms, HP writes +2,509,919 ms: 179/500 -> 304/500, 352/500 -> 477/500, full-health 500/500 unchanged; changed_members=2; FP 91 -> 36. Second heal: callback +2,527,223 ms, writes +2,527,241 ms: 9/500 -> 134/500, 477/500 -> 500/500, 429/500 -> 500/500; changed_members=3; FP 91 -> 36. Each gain is 125 or capped at 500. Neither heal invokes native spawning. User reports no visible duplicates or delay; callbacks span approximately 18 ms each by timestamp.

Subsequent sampled FP decreases to 12 and then 0 without another Spirit Ash callback; do not attribute that spending to this mod. The group then becomes inactive with three exiting entries at +2,551,568 ms, clearing to zero by +2,561,680 ms. Native/adapter remains (-1,-1), and no post-death summon appears through the archived capture. A subsequent live reread through +2,588,656 ms still shows zero group entries, FP now refilled to 91, native/adapter (-1,-1), and no resummon callback. The game process remains PID 3476. No panic or disabled-adapter message is logged.

Record the two heals, capping and their observed 55 FP charges as supported. Preserve the user's resummon report separately, but do not mark in-fight resummoning passed from this log until the timing/context discrepancy is resolved. Native prohibition after a boss ends would be expected, but boss completion is not established by this trace. Do not force eligibility or change code based on this ambiguity. No installed files or game inputs were changed by the agent.

## 0.1.12 player-death cleanup: successful observed cycle

User reports dying with wolves active, respawning, and summoning three fresh wolves at Gatefront for 55 FP without issues. Evidence: `evidence/0.1.12-player-death-20261029-200330.log`, PID 3476 through +2,343,799 ms. The trace becomes unavailable at +2,308,492 ms during the user-described death/load transition. At +2,310,880 ms the player has 91 FP, no queued item, requested/active goods=-1, no pending effect, alive=false and zero local group slots. Readiness subsequently becomes native/adapter (0,0).

The fresh wolf callback at +2,336,313 ms starts with no old group; native return=true takes 15 microseconds. At +2,336,576 ms, three non-exiting local entries are present with alive=true, range=true and FP=36: one 55 FP debit. The full callback measures 697 microseconds. The subsequent window records successful actor validation after brief loading-state rejections, with no handle/health failures. No panic, disabled-adapter message, ambiguous activation or false native result appears in the archive. Death itself is identified by the user's report; the scalar trace is not a player-death event recorder.

This supports the tested player-death lifecycle reset without stale-group blocking. Together with prior runs, observed cases now cover group healing, partial-group healing, healing cap, repeated paid healing, full-group resummoning (including cleanup overlap), insufficient FP, full-health no-op, different-Ash rejection, area exit/re-entry, title reload and player death. These are specific observed cases, not blanket guarantees or completed acceptance for every Ash/version. Boss-encounter behavior, HP-cost Ashes including insufficient HP, interruption/resource transaction edge cases, skeleton transition timing and HUD-icon behavior remain unresolved or untested.

Next meaningful gameplay validation: in an unfinished boss encounter that normally permits Spirit Ashes, summon wolves, refill FP, heal surviving wounded wolves once, then resummon once after the entire group dies while the fight is still active. Each successful activation should cost 55 FP, with no extra group, unexpected delay or crash. Record what can be completed; boss victory before group death does not count as a successful in-fight resummon test. No further build change is required for the currently observed cases.

## 0.1.12 title-screen reload cleanup: successful observed cycle

User reports quitting normally to the title screen with wolves active, continuing, finding old wolves gone, and summoning three fresh wolves for 55 FP without issues. Evidence: `evidence/0.1.12-title-reload-20261029-200115.log`, PID 3476 through +2,213,679 ms. The matching transition has no summon trace at +2,175,983 ms; player_loaded=false is recorded, then true at +2,187,967 ms. The +2,188,149 ms sample has no queued item, requested/active goods=-1, no pending effect, alive=false, zero local slots and FP=36. Native/query readiness reaches (0,0) after load. FP refill reaches 91 at +2,196,901 ms.

At +2,199,554 ms a fresh wolf callback starts with no retained group and native state 0. Native return=true follows in 17 microseconds. The +2,199,735 ms sample shows three non-exiting slots, alive=true, range=true, and FP=36, consistent with one 55 FP charge. Full callback time is 633 microseconds. Brief initial registry load-state rejections during spawning clear; the later +2,213,679 ms window has no registry/handle/health failures and approximately 60 game tasks/second. No panic, adapter-disabled message, ambiguous-activation rejection or false native return is present in the archived session.

This supports one successful title/Continue lifecycle reset and fresh summon. It does not validate player-death cleanup, boss transitions, HP-cost Ashes or every loading scenario. User ran all gameplay; the agent archived evidence and updated documentation only. Installed DLL/config remain unchanged.

Next isolated user test: on the test character, die with wolves active, respawn, return to a permitted summoning area if necessary, and activate Lone Wolf Ashes once with sufficient FP. Expected: old group gone, three fresh wolves, one normal 55 FP charge and no stale-group block. Leave the game open for log review. Do not record this as passed before the user performs it.

## 0.1.12 area exit, prohibited-area rejection and re-entry: successful observed cycle

User reports walking from Gatefront toward Church of Elleh, wolves disappearing en route, an unsuccessful use at Church with 91 FP and no cost, then resummoning after returning to an allowed area. Evidence: `evidence/0.1.12-area-transition-20261029-195844.log`, PID 3476 through +2,063,578 ms. The log has no map coordinates, so location attribution is supplied by the user.

The corresponding cycle shows an active wolf group's manager range flag becoming false at +1,988,343 ms; alive=false with three exiting entries at +1,993,201 ms; and zero local slots at +1,998,848 ms. Native/adapter remains (-1,-1), preserving native rejection. FP refills to 91 at +2,011,811 ms while the rejection remains. No activation callback or charge occurs in this interval. At +2,016,211 ms readiness becomes (0,0); the next wolf summon callback at +2,020,464 ms returns true in 15 microseconds. At +2,020,628 ms, three non-exiting entries are present, alive=true, range=true and FP=36 (one 55 FP charge). Later metrics continue validating active actors at around 60 game tasks/second with no steady-state registry/handle/health failures. No panic, disabled adapter or false native return appears in the archive.

This supports preservation of the tested area restriction, normal exit cleanup and summoning after re-entry without a mod-induced lockout. It does not prove every boss/area boundary or HUD-icon rendering. An earlier brief no-player sample/reset around +1,916,955 ms is not classified as a passed death/reload test without a user description of that event. No installed files or game inputs were changed by the agent.

Next isolated user test: with wolves active in the allowed area, quit to the title screen through the normal game menu and Continue on the test character. Verify the old group is gone. Ensure at least 55 FP (refill if needed), then summon Lone Wolf Ashes once. Expected: one fresh group of three wolves, one normal 55 FP charge, and no stale-group block or crash. Leave the game open for log review. This is a new test request, not a recorded result.

## HUD-icon clarification and revised area test

User reports not seeing the gravestone HUD icon at Gatefront during current testing, despite successful summoning there. Latest changed-state sample remains +1,559,023 ms with active wolf group, alive=true, manager in_range=true, FP=91 and native/adapter (2,-1); subsequent heartbeat continues. These fields do not expose HUD rendering. Source inspection confirms this standalone DLL has no HUD movie module or icon-specific patch, unlike upstream. This does not rule out indirect effects of eligibility or BuddyStone state changes on the native HUD.

Vanilla guidance describes the icon as a summon-availability indicator (Bandai Namco starter guide), and GameSkinny's Spirit Ash guide explicitly describes it disappearing after summoning. Thus an absent icon while companions are active is not adequate evidence that the area is prohibited, and the previous instruction to use icon disappearance as the sole test boundary was inappropriate for repeat-use testing. Its behavior before a fresh summon or after all companions die under this DLL remains unverified; do not declare the reported persistent absence fixed or definitively normal.

Revised area test: walk from Gatefront to the Church of Elleh location previously observed to prohibit summoning, without resting or fast traveling. Observe existing Spirit disappearance en route, then attempt Lone Wolf Ashes once with enough FP at the church and report FP/behavior for native-state log review. Do not require HUD-icon visibility to proceed. No changes to game/DLL/config made for this observation.

## 0.1.12 full-health reuse: successful observed rejection

User reports all surviving wolves at full HP, one Lone Wolf Ash activation attempted with 91 FP, no effect, wolves remained present and FP stayed 91. Evidence: `evidence/0.1.12-full-health-20261029-195041.log`, PID 3476 through +1,573,188 ms. A preceding normal wolf summon at +1,552,163 ms succeeds and costs 55 FP; refill reaches 91 at +1,556,422 ms. At +1,559,023 ms queued/active goods both equal 232000, alive=true, in-range=true, cooldown=0, three local slots with none exiting, FP=91 and native/adapter state (2,-1). No new summon callback, heal commit or native spawn follows. Later metrics show successful registry/snapshot validation with no registry/handle/health rejection and about 60 game tasks/second. No panic or disabled-adapter message appears.

This supports the user's full-health no-op/no-cost observation. Per-member HP values are not printed for rejected queries, so full HP is user-observed; input timing is not directly recorded. The native active state was rejected rather than passed through to dismissal. This is one observed case, not proof of all races or transition paths. No installed files or game inputs were changed by the agent.

Next user test: leave the permitted summoning area until the HUD gravestone icon disappears. With enough FP, attempt Lone Wolf Ashes once while the icon remains absent. Expected: no new summon or healing and no FP deduction; any normal disappearance of existing Spirits on leaving the area is allowed. Report icon state, FP before/after and behavior. Do not label this passed until observed.

## 0.1.12 different-Ash rejection: successful observed case

User reports trying Skeletal Militiaman Ashes with 91 FP while wolves remained alive: no action, wolves stayed present, and no FP was consumed. Evidence: `evidence/0.1.12-different-ash-20261029-194930.log`, PID 3476. At +1,507,150 ms sampled FP rises to 91. At +1,509,500 ms queued goods is 212000 (Militiamen), while active goods/effect remain 232000 (wolves), alive=true, in-range=true, cooldown=0, local_slots=3, exiting_slots=0 and FP=91. Native/adapter item-query pair is (-1,-1): native rejection is preserved. No subsequent summon callback, heal commit, new native spawn, or lower-FP sample appears through capture. The earlier attempt with queued Militiamen at 36 FP is not used as evidence for affordability-independent rejection. No panic or disabled adapter is logged.

User observation and the matching queued/active item evidence support one successful different-Ash rejection with sufficient FP, no charge and no replacement/dismissal of the wolves. The log is a sampled state trace, not a button-input recorder or proof of all possible Ash combinations. No new build or gameplay changes were made.

Next user test: full-health reuse. With all surviving wolves at full health, still in the permitted area and at least 55 FP available (preferably 91), select Lone Wolf Ashes and attempt one reuse. Expected: no HP/stat change, no dismissal or new wolves, and no FP deduction. Avoid damage during this attempt, since one wounded survivor legitimately makes it a paid heal. Record FP before/after and observed behavior.

## 0.1.12 insufficient-FP reuse: successful observed rejection

User reports attempting Lone Wolf Ashes once with 36 FP and an injured survivor: nothing happened and no FP was consumed. Evidence: `evidence/0.1.12-insufficient-fp-20261029-194752.log`, PID 3476 through +1,402,997 ms. The preceding normal summon at +1,368,701 ms returned true and sampled FP changed 91 -> 36. At +1,373,588 ms the adapter reports native/query (2,0), with the living group in range, cooldown 0 and FP still 36. This shows the healing-eligible state was exposed to native affordability handling, rather than merely blocked by a dead-member registry failure. No further summon callback, heal commit, native summon return or FP-changing timeline sample appears before the capture ends. Subsequent windows retain validated live actors and around 60 game tasks/second with no registry/handle/health rejections. No panic or disabled-adapter message appears.

The user observation plus logging supports one successful insufficient-FP healing rejection, with no resource charge or mod-side healing/spawn. Button presses and the native affordability branch are not directly logged, so the log alone cannot prove an attempted input or the precise internal rejection instruction. This does not validate HP-cost Ashes or all interrupted/racing activations. The agent only read/archived evidence and recorded this result; no game inputs or installed files changed.

Next user test on unchanged 0.1.12: while at least one wolf remains alive, refill FP to 91, select Skeletal Militiaman Ashes and attempt one use. Expected: no Militiamen, no dismissal or healing of the wolves, and no FP deduction. Sufficient FP is essential to distinguish different-Ash rejection from the already-tested affordability block. Report the result and leave the game open for log inspection.

## 0.1.12 Lone Wolf partial-group healing: successful observed case

User reports one wolf died, then one Ash activation healed the remaining wolves with no delay. Evidence: `evidence/0.1.12-wolf-heal-20261029-194602.log`, same PID 3476. At +1,242,316 ms the callback enters native state 2 with active/queued goods 232000, alive=true, in-range=true, cooldown=0 and FP=91. At +1,242,337 ms exactly two validated targets are written: 14/500 -> 139/500 and 49/500 -> 174/500. Each gains 125 HP, exactly 25% of its maximum. One heal-activation record reports changed_members=2. No native spawn call is logged for this activation. The next sample at +1,242,469 ms records FP=36, a 55 FP decrease. The user's no-delay report corroborates responsiveness; log timestamps do not independently measure button-to-animation latency. Retained local_slots=3 includes the dead member's entry and must not be interpreted as three living wolves.

This is a successful observed survivor-only healing case with one native-cost-sized charge and no mod-side spawn or dead-member HP write. It does not prove every corpse-unloading or skeleton-revival transition works. No panic, disabled adapter, false native return or ambiguous activation message appears in the archived session. Afterward, the group eventually transitions to no alive summon and cleanup; the log does not establish the cause of that later loss. No further code change is justified by this test alone. All gameplay was performed by the user; the agent read/archived the log and updated this record.

Suggested next isolated test: insufficient FP. With Lone Wolf Ashes active and at least one wounded living wolf, attempt one reuse while below the normal 55 FP cost (a fresh summon from 91 leaves 36). Expected: no healing, dismissal or additional wolf and no FP charge. Do not refill until after the attempt. Report the observation for log review; do not mark the case passed in advance.

## Clarification: skeletal revival makes partial-death reproduction inconclusive

The user cannot reliably distinguish the short intended partial-death test window: Skeletal Militiamen revive after HP depletion unless hit again, enemies often ignore them during revival, and both can be lost before reuse. Therefore the earlier wording calling this a confirmed partial-group healing bug was too definite. The observed registry rejection and successful later full-group resummon remain valid evidence, but a continuously living, wounded survivor at the attempted-heal moment has not been established. No partial-healing or no-revival result is marked passed, and no additional gameplay patch follows solely from this ambiguous observation.

Next controlled user test on unchanged 0.1.12: use Lone Wolf Ashes if owned (three wolves, listed normal cost 55 FP; https://eldenring.wiki.gg/wiki/Lone_Wolf_Ashes). Refill FP immediately after the initial summon, then attempt one reuse when one wolf is gone and at least one wounded survivor remains, while still within the allowed summoning area. Expected: survivors heal, no replacement for the dead wolf, and one 55 FP charge. Record any input-to-bell delay, member count, FP and whether a survivor stayed alive throughout. If every member dies before activation completes, record that attempt as full-group resummoning, not partial-group healing. No need to force repeated ambiguous skeleton trials.

## 0.1.12 follow-up: partial-group delay and first cleanup-overlap commit

User reports one permanently dead member and one survivor when attempting activation, with sufficient FP and about 4-5 seconds before the bell animation. Archived PID 3476 session: `evidence/0.1.12-partial-group-20261029-193045.log`. Outcome clarification was requested; do not assume a survivor heal or a revived dead member from the user's use of the word resummon.

At +299,351 ms, native/adapter state changes to (2,-1) with alive=true, cooldown=0 and FP=47. The +302,146 ms aggregate reports 953 registry update-state rejections with sampled load/update 4/2, alongside accepted 4/4 actors. Source snapshot aborts at registered_spirit(...)? before the dead-member health check whenever a member is no longer in 4/4. This supports the remaining mixed-group validation block, but the scalar timeline does not timestamp input or identify each member's death.

At +301,896 ms, engine manager reports alive=false and both entries exiting. At +302,413 ms, query result is (0,0). Callback begins +303,493 ms; +303,505 ms records Cleanup resummon admitted with two retained entries, then native true in 47 microseconds. Entire callback measured 11.994 ms, mostly the 10.866 ms page probe. At +303,705 ms, two non-exiting replacement entries are alive and FP is 3 (44 deducted). Continued validation of two actors through +382 s gives about 78 seconds of replacement retention. This is the first recorded successful cleanup-overlap commit; it supports the 0.1.12 full-group correction in this run. No heal commit appears anywhere in this session. The engine reported no survivor by commit time; it does not establish what the user saw at initial input or prove no individual revival across every case.

Mixed-group healing remains unresolved. A future correction must distinguish a positively identified dead/retiring member from ambiguous registry failure, skip only that member, and fully validate surviving heal targets. Do not simply admit all 4/2 actors for health writes or interpret every failed lookup as death. Installed files and running game were not modified during this inspection.

## 0.1.12 user run: immediate resummon observed, cleanup-overlap commit not exercised

User reports no delay on the death/resummon test. Session PID 3476 is archived separately at `evidence/0.1.12-resummon-20261029-192744.log` (through +202,089 ms). Initialization installed eight call sites and changed 304 BuddyStone rows; scheduler retry succeeded on attempt two. Two native summon callbacks returned true, each with 15 microseconds inside the native routine: initial summon at +65,475 ms and resummon at +162,383 ms. Sampled FP was 91 -> 47 for the first summon and 47 -> 3 for resummoning, consistent with one 44 FP charge each. At +162,463 ms (80 ms after resummon callback), two local slots were present with alive=true and no exiting entries. Later metrics through +202 s continue validating both members with roughly 60 game tasks/second and no steady-state registry/handle/health failures. No panic, disabled message, ambiguous-activation rejection or false native return appears. Game-task cadence is not an independent FPS capture; full HP was not measured by the spawn log.

Important limit: zero `Cleanup resummon admitted` messages. Old entries reached zero at +150,015 ms, around 12.368 seconds before the successful resummon callback. During retention, the new query path did preserve native readiness in samples such as +140,523 ms and +143,845 ms with two exiting slots. Some other samples still show (0,-1), so not every intermediate state qualifies. The user-observed immediate resummon is supported, but the new cleanup-overlap commit and replacement survival during old-entry removal remain untested. No new build is justified by this run alone.

Next targeted reproduction on the installed build: arrange at least 44 FP before the last member permanently dies, remain inside the permitted area, and use the same Ash once as soon as the final member is permanently gone. Avoid waiting to refill FP after death. Observe for 20 seconds without another Ash input, noting delay, FP, member count and whether replacements remain present. The user controls all gameplay; the agent only read/archived logs and recorded findings.

## 0.1.12 cleanup-readiness correction

Windows x64 release compilation succeeded with locked dependencies (`evidence/build-0.1.12.txt`). No automated tests or game were launched by the agent. Prior test results do not validate this correction.

The adapter now retains native state 0 during old-group cleanup only with resummoning enabled, no alive-summon flag, no manager dismissal, no pending summon effect, finite expired cooldown, exact active item/effect match, and a nonempty bounded local group whose every entry is exiting, nonremote, nonwarping and in the matching BuddyParam family. Each pointer must still match a current summon registry entry with raw load/update 2/2 or 4/2. Unknown, absent, loading or active entries do not qualify. No unloading actor is dereferenced and no health write is authorized by this exception. The actual effect argument is rechecked at commit. Engine-owned flags, timers, lists and resource values are not changed by this correction.

These registry pairs were observed during permanent-death cleanup; they are not a universal death predicate. The exception relies jointly on native readiness, the manager alive flag, exit metadata and registry state. Runtime testing must establish that native spawning while cleanup entries remain produces exactly one new group, preserves costs and does not cause delayed cleanup to remove the replacement. Out-of-range dismissal, travel/death teardown and partial-group cases remain unvalidated. A missing/unknown registry entry still blocks until native cleanup removes the group.

Next user test: launch the same offline profile, summon Skeletal Militiamen at Gatefront, and let both die permanently. Restore FP using a flask if necessary. As soon as both are permanently gone, use the same Ash exactly once, without resting, traveling or reloading. Observe for 20 seconds without another Ash input. Record the delay from bell to spawn, FP before/after, number of members, full health, FPS, and whether the new group remains present. Repeat one full-death/resummon cycle if stable. Leave the game open for log review. Look for `Cleanup resummon admitted` followed by the native result; lack of this message means the new path was not exercised. Subsequent regression tests should cover healing, one surviving member, different-Ash rejection and leaving the summoning area.

## Latest user run and diagnostic update

**Latest result (0.1.11, PID 23056): unnecessary adapter blocking during old-group cleanup identified.** User reports immediate resummoning in one attempt and approximately five seconds in another, both full-health groups with 44 FP charged at appearance and stable 60 FPS. Archived evidence: `evidence/0.1.11-resummon-20261029-191755.log`. The session contains six native summon entries/returns, all result=true and 14–17 microseconds native elapsed time; individual reported button presses are not timestamped. A clear sampled interval at +475,748 ms shows no alive summon, two local slots both exiting, cooldown 0, and native/adapter item-query results (0,-1). At +485,836 ms, about 10.088 seconds later, slots reach zero and results become (0,0). Callback arrives at +486,889 ms; native return is true and the next +487,152 ms sample has two active slots and FP 47→3. Similar ten-second exit-slot retention appears around +141–151 s and +382–392 s, while other cleanups are shorter. This establishes that our unresolved-entry validation overrides a native allowed result while old entries await removal. Remaining input buffering and animation scheduling cannot be fully reconstructed from the latest queued-goods field, which persists between uses; do not equate it to a newly pressed button or explain the user's exact five-second timing from it. The manager disappear-delay field remains 10 in every sample, so it is not evidence of a measured countdown. No panic/disabled message is recorded, and stable later windows retain two validated targets at roughly 60 tasks/second. Next fix should distinguish a confirmed ended local group from a living or ambiguous group and preserve native restrictions/cost handling, without merely treating every failed actor lookup as safe or resetting cleanup timers.

**0.1.11 installed:** confirmed game exit, backed up prior dist files, verified backup DLL/configuration hashes, installed the staged DLL and symbols, and verified DLL SHA256 `2E31FA09FC9C96472652BB74E0591B8DFBA505A52134FEA4D52068396D0E8316`. Configuration is unchanged with logging enabled. Installation details: `evidence/install-0.1.11.json`. No game or test suite launched by the agent. Next user reproduction: launch the same offline profile; summon Militiamen at Gatefront; let both permanently die; without resting/travel/reload, restore FP with a flask if needed, note FP, and activate the same Ash exactly once. Observe up to 20 seconds without another Ash input, noting bell animation, FP charge, spawn delay, member count/full HP and FPS. Leave the game open for log review whether or not the group appears.

**Resummon-delay clarification / staged 0.1.11:** user confirms bell animation immediately on input, with the 44 FP charge delayed until the new Spirits appeared about ten seconds later, without another input. This distinguishes animation onset from completion, but not whether the wait is inside the native request lifecycle or caused by mod eligibility rejection during cleanup. The current DLL does not implement a deferred spawn queue. Version 0.1.11 compiles (`evidence/build-0.1.11.txt`) and adds changed-state samples at 250 ms cadence plus callback begin/native return diagnostics, without changing gameplay gates or timers. Samples include pending/queued item IDs, manager flags, group-slot counts, FP, and latest item-query native/adapter result pair. Pair observations may be stale if no new query occurs; i32::MIN means none observed. They are not button timestamps or proof of charge causality. Debug observer overhead and runtime behavior are untested. Staged only; 0.1.10 remains installed. After installing with the game closed, repeat the permanent-group-death and single resummon attempt, noting bell timing and when FP changes; leave the game open for timeline review.

**Latest resummon observation (0.1.10, PID 28448): successful full group after permanent death without resting, with an unexplained approximately ten-second delay.** User reports both Militiamen permanently died, one Ash use eventually spawned two full-health members without another input, charged 44 FP, and retained 60 FPS. Evidence: `evidence/0.1.10-resummon-delay-20261029-185311.log`. In the +621,471 ms ten-second window the adapter rejects 3,236 registry lookups; samples show load/update=2/2 and 4/2. The following window (+631,473 ms) includes 242 idle snapshots, one summon callback taking only 30 microseconds including the native routine, 342 transient load=3/update=4 rejections and subsequent 4/4 acceptance. Stable later windows validate two members at approximately 60 tasks/second. No new heal, ambiguous activation, panic or disabled message appears around this resummon. This excludes a ten-second synchronous stall inside the observed summon callback; it does not establish where the input-to-appearance delay occurred. Old-group cleanup/readiness rejection is a plausible contributor, but aggregate ten-second windows lack input, FP deduction, animation and actual entity-creation timestamps. Clarify whether bell animation and FP deduction happened immediately on input or when the new group appeared before choosing a fix. Do not treat delayed responsiveness as fully validated, and do not mark unrelated death/transition cases passed.

**Additional healing clarification:** user confirmed the extra 0.1.10 heals after the first engagement were intentional and each consumed 44 FP. The earlier four-commits-versus-one-described-input discrepancy is resolved; no duplicate-trigger claim is supported by that log.

**Latest result (0.1.10, PID 28448): reduced healing callback time with correct HP gains/capping in the inspected session.** User reports both members healed and stayed present, steady 60 FPS without a perceived hitch, and 91 starting FP → 47 after summoning → 3 after one heal (44 FP each). Archived evidence: `evidence/0.1.10-heal-20261029-184942.log`. The log contains four heal commits: +334,738 ms (123→214 and 257→348), +346,048 ms (214→305 and 348→365), +373,052 ms (305→365 and 365→365, changed_members=1), and +419,666/667 ms (333→365 and 327→365), all max HP 365. Recorded gains are +91 or capped at 365; an already-full member is unchanged. These are computed post-write log values, with the user visually corroborating healing. Callbacks took 23.434, 25.568, 23.815 and 18.976 ms, versus prior 55.035–65.317 ms samples. Each used 4–5 OS page queries and 7–8 cache hits. This supports an observed timing improvement, not a guaranteed latency bound or controlled benchmark. No panic/disabled entry or stable-state registry/handle/health failure appears. Startup also exercised the readiness retry: NotFound on attempt 1, then ready on attempt 2 after 249 ms. The user described one heal; clarify whether the other three commits were additional intentional uses before attributing all four to distinct inputs or asserting their costs. The log also has two non-heal summon callbacks; their gameplay context is unconfirmed, so death/resummon acceptance remains pending.

**0.1.10 installed:** game exit was confirmed before replacement. The previous DLL, symbols, configuration and log were backed up under the path in `evidence/install-0.1.10.json`. Installed DLL matches the staged build SHA256 `B06F4A9443C3BC281307613EF00D27833D42230219A31654857203174D99947E`; configuration hash is unchanged and debug logging remains enabled. No game or test suite was launched by the agent. Next user test: use the same offline launcher, summon the Militiamen at Gatefront, restore FP with a flask if needed, let at least one member take damage while both survive, record FP and use the same Ash exactly once. Wait 15 seconds and report FP before/after, healing, member count, FPS/hitch observations. Leave the game open for log comparison against 0.1.9.

**Healing clarification and staged 0.1.10:** user confirmed all three 0.1.9 healing activations were intentional and each consumed 44 player FP; no hitch was noticed. Combined with the recorded +91 HP to each member per activation, this supports repeat group healing and normal FP cost in this run. It does not cover HP-cost Ashes, insufficient resources, clipping to full HP, partial group death or resummoning. Version 0.1.10 compiles (`evidence/build-0.1.10.txt`) and is staged separately to reduce duplicate VirtualQuery calls. No tests were executed; new boundary/protection test source is not a passed result. Snapshot region metadata is discarded before a fresh write-phase cache is created; nothing persists across callbacks. The existing 0.1.9 DLL remains installed while the game is open. After installation, repeat one damaged-group heal with FP recorded before/after, then leave the game open for callback timing and page-cache review. Compare `summon` callback wall time against the 0.1.9 55.035/61.989/65.317 ms samples; probe counts are not directly comparable because 0.1.10 also instruments formerly uncounted second write probes. Do not claim a performance improvement until this run is observed.

**Latest healing observation (0.1.9, PID 7100): correct integer HP gains recorded for both members; activation count and cost need clarification.** User reports restoring FP, damaging the Militiamen, using the same Ash once, and seeing approximately one-quarter HP restored. Archived log: `evidence/0.1.9-heal-20261029-183449.log`. It actually contains three separate heal commits: +199,319 ms (201→292 and 111→202), +203,389 ms (147→238 and 7→98), and +266,426/427 ms (251→342 and 219→310). All have max HP 365 and gain 91, floor(365*0.25), with changed_members=2. These entries log computed write values after the write loop, not independent subsequent readback; the user's visual observation corroborates healing. No cap boundary was exercised by these values. Do not classify the three commits as either intended repeated casts or duplicate callbacks until the user clarifies their inputs. Another native summon callback appears in the +232,818 ms window, with an idle-to-active transition; its reason is not logged, so do not mark death/resummoning passed. Healing callbacks took 55.035, 61.989 and 65.317 ms; page probes account for most of that time, indicating remaining per-cast latency despite stable between-cast performance. No panic/disabled entry was found. Player FP before/after each heal is not in this log and was not reported; normal healing cost remains unverified. Next step: clarify number of Ash uses, player FP expenditure and whether a hitch was visible before further testing or changes.

**Latest result (0.1.9, PID 7100): both actors pass complete query-time validation, with stable performance.** User reports both Militiamen spawned, stable 60 FPS after 15 seconds, and FP decreased from 77/91 to 33/91 (44 FP). Evidence: `evidence/0.1.9-active-20261029-183144.log`. At +102,747 ms, 1,800 full snapshot attempts completed, with 3,600 registry acceptances and 3,600 validated targets: two distinct targets per completed snapshot. Registry, handle and health rejection counts are zero in this stable window. Full snapshots total 11,870 microseconds over 10,007 ms, while 600 recurring tasks ran. During initial spawn the log recorded 45 rejected lookups with the last sample load=3/update=4, followed by stable acceptance at 4/4; transitional rejection is not treated as a failed stable-state check. No panic or disabled entry was recorded in the inspected session. One initial summon callback took 28 microseconds. This validates the complete read-only actor path in this run, not healing commit, repeated cost handling or death/resummon. Next: restore FP with a Cerulean flask (currently only 33, below the 44 cost), let at least one member take damage while both survive, then activate the same Ash once. Record FP before/after, HP bar changes, any dismissal/extra spawn, and any hitch; leave game open for log review. Do not rest to restore FP during this active-group test.

**0.1.9 update:** completed static verification against the installed executable; see `evidence/registry-state-0.1.9.md` and `.asm`. Native load checks require entry byte +8 = 4, a load-completion branch writes 4, and update-enabled logic writes entry byte +9 = 4. The adapter now admits only 4/4 and checks actor-to-entry back-pointer equality; ownership remains a separate check. Release compilation succeeded (`evidence/build-0.1.9.txt`); no automated or gameplay tests were run. Installation/rollback details are recorded in `evidence/install-0.1.9.json` upon installation. Next user step: launch the same offline test profile, load at Gatefront, confirm the monument icon, record FP, summon Skeletal Militiamen once, wait 15 seconds with them active, and report FPS, member count and FP before/after. Leave the game open; do not reuse the Ash until the complete actor validation path has been reviewed.

**Latest result (0.1.8, PID 3860, active summon): both Militiamen appeared, stable 60 FPS, normal 44 FP cost, per user.** The active mod logs one summon callback (25 microseconds) at +94,026 ms. Subsequent full windows maintain 596–598 tasks per roughly ten seconds. At +104,035 ms, 1,788 full snapshot attempts take 8,814 microseconds total with zero page probes. Every registry lookup is rejected as inactive: capacity 80, pointer membership found after two occupied entries, raw load=4 and update=4. This identifies the failing predicate: current code requires load=2 and update=0. It does not prove the binding's state names accurately describe live Spirit lifecycle semantics, nor that accepting 4/4 is safe during teardown. The aggregate sample concerns the first matched group member; processing returns on rejection, so neither all members nor downstream handle/health checks are validated. No panic or disabled entry is recorded in the inspected current session. First-summon cost and performance are confirmed for this user run; healing remains blocked and death/resummon is untested. Next development step: verify registry state semantics/lifetime handling before changing the state predicate. Do not ask the user to attempt healing with this build.

**Latest result (0.1.8, PID 3860): startup and idle check succeeded in this user run.** User loaded the test character at Gatefront, waited 15 seconds without summoning, and reported stable 60 FPS. Scheduler lookup succeeded on the first attempt after 17 ms; the startup task registered at +217 ms, all eight hook sites validated/installed at +644 ms, and 304 BuddyStone rows were patched at +24,716 ms. Loaded-character offline/session gates are satisfied. At +44,002 ms, 600 tasks ran in 10,002 ms; 1,200 snapshots all took the idle path, totaling 479 microseconds, with zero page probes, registry lookups or summons. No panic or disabled entry appears in the inspected current session. This run does not exercise scheduler retries or active-Spirit validation. Next user step: confirm the monument icon, record FP, summon Skeletal Militiamen once, wait 15 seconds with both active, then report FPS and FP before/after; leave the game open and do not reuse the Ash yet.

**0.1.8 update:** compiled successfully (`evidence/build-0.1.8.txt`), without running automated or gameplay tests. Source inspection of the pinned from-singleton 3.0.1 confirms that a null instance and an undiscovered singleton both become FromStatic::NotFound, while the upstream scheduler helper retries only Null. A local bounded retry now handles both lookup failures and reports exact errors. This explains a possible early-exit path; the 0.1.7 log cannot distinguish the original cause because it discarded the error. Startup recovery still needs runtime validation. Installation and backup are recorded in `evidence/install-0.1.8.json` upon installation. Next user step: launch the same offline test profile, load the test character at Gatefront and wait 15 seconds without summoning; request startup-log review before spending FP on another registry test.

**Latest result (0.1.7): disabled at startup, gameplay diagnostics not exercised.** User reports both Skeletal Militiamen appeared, stable 60 FPS after 15 seconds, and FP changed from 77/91 to 33/91 (44 FP). The matching PID 2092 session logs `SpiritAshResummon 0.1.7 diagnostic startup` then `Disabled: task scheduler unavailable` at +263 ms, with no task registration, hook installation or registry reports. Evidence: `evidence/0.1.7-startup-disabled-20261029-181942.log` (also contains the previous 0.1.6 session because logging appends). These gameplay observations describe a session with this mod disabled; they do not validate 0.1.7 active processing, healing, or resummoning. Source inspection shows the upstream scheduler wait helper immediately maps a missing singleton lookup to an error, whereas only a null instance is retried. The exact underlying error was not retained in this log, so a startup discovery race is a hypothesis, not proven. Next update must retain detailed startup errors and address readiness before another registry test.

**0.1.7 diagnostic update:** release compilation succeeded (`evidence/build-0.1.7.txt`); no automated or gameplay tests were run for this version. The user subsequently confirmed stable 60 FPS for the 0.1.6 active-Spirit run below. Version 0.1.7 retains its validation predicates and adds aggregate rejection reasons plus scalar registry/state samples. Installation details and rollback backup are recorded in `evidence/install-0.1.7.json` upon installation. Next user check: launch the same offline test profile, load at Gatefront with the monument icon visible, record FP, summon Skeletal Militiamen once, and wait 15 seconds while they remain active. Report FPS, FP before/after, and whether both appeared. Leave the game open for log review; do not reuse the Ash yet.

**Latest result (0.1.6): first summon observed; active actor validation rejects the summoned group.** The user summoned Skeletal Militiamen at Gatefront and waited 15 seconds, leaving the game running. Archived evidence: `evidence/0.1.6-active-20261029-181436.log`. The +1,845,095 ms window records one summon callback (27 microseconds). The next full 10,015 ms window records 600 task callbacks and 1,800 full snapshot attempts totaling 10,182 microseconds, with no readable page probes. However, all 1,800 registry lookups are rejected. This is evidence of low overhead on the early-rejection path, not validation of complete active-actor processing or healing. The aggregate counter does not distinguish registry membership, capacity/pointer checks, or active/local state failures. No panic or healing entry is recorded. FPS and normal resource deduction have not been reported for this summon. Next: diagnose the precise registry rejection without weakening actor validation; defer healing acceptance until targets resolve correctly.

0.1.6 was installed with the game closed; see `evidence/install-0.1.6.json`. At the user's explicit request, the agent used computer use to launch through the existing offline loader and load the character at Gatefront, observing about 60 FPS while idle. Keyboard input through the tool failed, so no summon was performed by the agent; the test was paused at the user's request. The user performed the subsequent summon above. No automated test suite was rerun.

**Latest result: first summon observed; active-Spirit performance failed in 0.1.5.** At Gatefront Ruins with the monument icon visible, the user summoned Skeletal Militiamen once and reported about 10 FPS. Archived evidence: `evidence/0.1.5-active-10fps-20261029-173836.log`. One summon callback completes in 29 microseconds. The +324,375 ms window records 92 tasks in 10,019 ms, with 276 full snapshots taking 8,864,910 microseconds; their 2,760 readable probes take 8,857,805 microseconds. This directly localizes almost all snapshot delay to page probes. No panic or healing is recorded. Initial summon cost has not been reported and remains unverified.

0.1.6 compiled successfully (`evidence/build-0.1.6.txt`) and was initially staged in `pending-0.1.6`, then installed as recorded above. Repeated queries replace page probes with current engine-registry membership, active/local state, handle and module-ownership validation. Commit still has fresh page/write checks. This relies on serialized native callbacks and the pinned engine layouts, and needs validation of active actors, registry states, teardown and transitions. Registry rejection is logged as an aggregate counter.

**Earlier greyed-item report:** the user was outside Church of Elleh without a monument icon, with Skeletal Militiaman Ashes costing 44 FP and 91 FP available. At Gatefront Ruins the icon appeared and the first summon succeeded. The unavailable-item report is therefore consistent with the normal location restriction; no eligibility bypass was added. Archived earlier log: `evidence/0.1.5-greyed-ash-20261029-173520.log`.

**Latest result (0.1.5): idle performance check passed in this user-operated run.** The user reports stable 60 FPS after loading the same character and waiting 20 seconds without summoning. Archived evidence: `evidence/0.1.5-idle-60fps-20261029-173313.log`. The +54,151 ms window records 600 game tasks in 10,000 ms; all 1,800 snapshots take the idle path and consume only 650 microseconds total (about 0.0011 ms per frame). There are zero full snapshots, readable probes or summon callbacks. Eight hooks and 304 parameter edits are active, offline/session gates are satisfied, and no panic is recorded. This confirms the idle-path improvement for this run; active-Spirit performance and all gameplay mechanics remain untested. The agent only read and archived the user's log.

**Previous result (0.1.4):** user reports 30–35 FPS after loading the same character and waiting 20 seconds without summoning. Evidence is archived at `evidence/0.1.4-low-fps-20261029-172633.log`. In the +43,126 ms window, 334 game tasks correspond to 334 item queries and 668 cost queries. Their 1,002 snapshot calls consume 6,318,420 microseconds (about 18.9 ms per frame). Native state calls consume 358 microseconds total, and the recurring task consumes 12,432 microseconds total. Subsequent windows show the same concentration in snapshot. No summon callback or panic is recorded. These are nested wall-clock timings, not independently additive CPU samples.

0.1.5 was compiled, then installed with the game fully exited (`evidence/install-0.1.5.json`). It moves the manager page probe after the validated no-group idle shortcut, preserving health checks and the full inspection path. Additional timers distinguish manager identity, idle returns, full inspection and readable probes. The improvement after moving the probe strongly implicates that probe on the idle path, although its individual duration was not directly measured in 0.1.4. Build evidence: `evidence/build-0.1.5.txt`. No tests were executed by the agent.

The user reported approximately 9 FPS with 0.1.3 after loading a character without summoning. Its log confirms eight installed hooks, offline mode, no lobby/protocol session, and 304 modified BuddyStone rows. The game-task heartbeat drops to roughly nine callbacks/second after character load. No panic or log flood appears. Evidence: `evidence/0.1.3-low-fps-20261029-172054.log`. This is a failed performance acceptance check, not successful gameplay validation. The log does not identify the slow function.

0.1.4 removes repeated optional-DLL discovery from item queries, avoids expensive summon inspection when no local group exists, and records aggregate timings for the task, hooks, native state function, snapshot, DLL checks and parameter maintenance. This is a candidate mitigation plus diagnostics, not a verified performance fix. It compiled successfully (`evidence/build-0.1.4.txt`) and is staged in `pending-0.1.4`; no tests were run by the agent.

In the earlier 0.1.2 run, the exact blocker was `online_flags=(false,true)`: actual online mode false, general server connection capability true. 0.1.3 corrected this predicate and added a no-lobby/no-protocol-session requirement through CSSessionManager. Four pure gate tests are supplied but remain unexecuted.

The user reported a successful menu launch. The 0.1.0 log confirmed all eight hooks and 304 modified BuddyStone rows, then reported `Rust panic caught; gameplay adapter disabled`. There was no timestamp/context, so whether this occurred during startup or exit is unknown. This is partial initialization evidence, not a passed mod startup test.

0.1.1 adds parameter-ready checks and timestamped panic details, and was successfully compiled. No tests were run by the agent for this update. The previous DLL/config/log are preserved under evidence/startup-0.1.0-*. The supplied PDB improves panic backtraces.

**Next user action after 0.1.6 installation with the game closed:** return to Gatefront Ruins, confirm the monument icon, record FP, and summon Skeletal Militiamen once. Do not reuse the Ash yet. Observe FPS for 10–15 seconds while they remain active, then report FP before/after, FPS and summon behavior for log review. If severe slowdown returns, stop and close the game; the log remains available. Healing retains page/write checks and could still produce a per-cast hitch; that is a separate, unmeasured next stage. Timing samples use wall time, so scheduling stalls are included and nested totals are not additive.

**Status: experimental compiled prototype. Gameplay acceptance is pending.**

The user is performing further gameplay tests. The agent performed only the explicitly authorized 0.1.6 launch and idle character-load attempt described above; no combat or summoning input succeeded. No save files were directly edited. The existing co-op/mod-pack installation is unchanged.

## Checks already completed before the user took over testing

| Check | Result / evidence |
|---|---|
| Upstream standalone build | Passed, locked dependencies, Rust 1.99.1; evidence/upstream-build.log |
| Prototype automated tests | 26 passed, 0 failed, 0 ignored; evidence/tests.txt |
| Strict Clippy | Passed with `--all-targets -- -D warnings`; evidence/clippy.txt |
| Installed executable identification | File version 2.7.2.0; SHA-256 f9118fea2cfd8fa3901e8060bfadb1552cb2471771ded8a5b662c4e298ffb328 |
| Static PE preflight | 8/8 call destinations and 3/3 function signatures matched; evidence/game-preflight.json |
| Final release DLL compilation | Passed, x86_64-pc-windows-msvc; evidence/build.txt |

The automated tests and Clippy completed before the final diagnostic-only change that logs each member's before/after HP. The final source was compiled after that change; tests were not repeated, respecting the user's instruction to perform subsequent tests themselves. Formatting was applied. No result above establishes runtime hooking, actual resource spending, crash freedom or gameplay correctness.

The DLL probe utility is supplied but **has not been executed**. Unsupported-version admission is unit-tested as a pure version check; rejection in an actual unsupported game process is untested. Thread suspension/patch transaction, table restoration and offline guard are implemented but not live-tested.

## Important unresolved points

1. Native resource requirements/calculation are reused, but the debit-to-heal transaction has not been traced live. Interrupted casts, changes between cost calculation and effect execution, full-health transitions, and last-member death can expose incorrect charging/free-heal cases. There is no proven refund/debit handshake yet.
2. The adapter requires an authoritative queued Goods ID. Its availability at all native cost/effect call sites is unverified. If it is unavailable, a commit rejects safely; an earlier native debit may still have occurred.
3. The original requirement that both GameMan flags be false blocked the observed offline session. Version 0.1.3 instead requires `is_in_online_mode=false` and CSSessionManager lobby/protocol states both zero; the user's run observed those values. Missing/unknown state rejection and session transitions are not live-validated. Version 0.1.4 caches DLL conflicts between game tasks for item queries, with a fresh check at activation commit.
4. Direct HP writes use typed version-specific bindings and multiple checks, but foreign pointer validation is not a guarantee against concurrent lifetime changes. Rust panic recovery cannot catch an access violation.
5. All eight on-disk sites match, but runtime restoration, mod-loader timing and startup thread creation need testing. Hot unloading is unsupported.
6. Initial area monument state must be created after the runtime parameter edit. Loading the mod halfway through a session is refused. In-place parameter reloads and other summon/parameter mods are unsupported.
7. Only WW 2.7.2.0 is enabled. Upstream's other supported versions have not been ported to this prototype's extra hook.

## Stage 0 — prepare once

1. Exit Elden Ring completely. Keep Steam available for the existing Mod Engine 2 launcher.
2. Back up your save folder using your normal procedure. Use a disposable test character. The supplied launcher **does not isolate or redirect saves**. It does not load the co-op `.me2pack` profile, and may expose your ordinary single-player characters. Do not load a valuable character for testing.
3. Ensure the game's launch setting is offline. Use the established Mod Engine 2 offline launch procedure with EAC disabled. Never test this in protected online play.
4. Open `dist/SpiritAshResummon.ini` and change `EnableLogging = false` to `EnableLogging = true`.
5. Do not add the DLL to the existing co-op launcher/profile. The supplied test script generates a separate profile with only this DLL and no loose mods.

## Stage 1 — startup only; do this first

Open PowerShell in the extracted project/package folder and run:

```powershell
.\tools\Launch-Test.ps1
```

The script defaults to the existing loader/game paths on this PC. On another PC, supply `-Loader` and `-GameExe`. It creates only `config_spiritash_test.toml` within this package, then calls the loader. It does not change the existing loader profile, game files or save files.

1. Stop at the title/main menu. Do not select Continue yet.
2. Allow about 30 seconds for startup. Observe freezes, crash, unexpected online state or a loader error.
3. Read `dist/SpiritAshResummon.log`. Look for `All 8 call sites validated and installed for WW 2.7.2.0`. A `Disabled`, `mismatch`, `conflict`, `rollback`, `panic`, or `restart required` message is a failed startup test; stop and report it.
4. The `Resummon flag set to 0 ...` line may appear once the parameter repository and offline state are available. Do not proceed to mechanic tests unless it appears after loading the disposable character.
5. Exit normally and send the log plus whether the main menu opened. **Get this stage reviewed before moving on.** This is the first requested user test; nothing has already passed in the game.

## Stage 2 — single FP-cost Ash

After startup is reviewed, load the disposable character at a known valid Spirit Ash area. Use one unmodified Ash and remove resource-cost modifiers/temporary infinite-FP effects for the first measurements. Record Ash name, upgrade, normal cost, player FP/HP before/after each activation, and member count. Avoid combat damage to the player while measuring HP costs.

1. First activation: one normal summon/group, normal animation and one native cost.
2. Let the spirit take damage. Reuse the exact same Ash once. It should remain in place, keep its AI/equipment and heal without dismissal/duplication. Record FP before/after.
3. Inspect the `Heal member` log lines. Check `hp_after_written = min(max_hp, hp_before + floor(max_hp * 0.25))`. For 2,000 max HP, 900 must become 1,400 and 1,800 must become 2,000. The logged maximum must not change.
4. Repeat three separate activations. Each should have one cost, one group heal and no new entity. At full HP, try again: no heal, dismissal or resource loss is expected.
5. Let the spirit die fully and wait for its normal cleanup. Reuse the same Ash **without resting, reloading, fast traveling or dying**. Expect a fresh full-health summon with normal cost. Repeat death/resummon three times.
6. While it lives, select another Ash. Attempt use: no second group, no dismissal and no cost. Return to the original Ash and verify healing still works.

Stop at the first failure; preserve the log and record the exact action/order. A log that says a heal occurred is not by itself proof that the resource cost was correct.

## Stage 3 — affordability, HP-cost Ash and interruptions

- FP sufficient: native cost exactly once, including first summon and resummon.
- FP insufficient: attempt healing and fresh summoning; no health gain, summon or resource loss.
- FP exactly equal to the normal cost: compare with normal game eligibility; cost should not be duplicated.
- Use an HP-cost Ash such as Mimic Tear. Measure its normal first-summon HP cost, then compare a healing activation and resummon. Do not assume a hard-coded cost; use the actual item/native behavior.
- At or below the native HP affordability threshold, activation must be rejected without harming the player or healing the spirit. Avoid unrelated combat damage during measurement.
- Interrupt the casting animation with a hit/roll where the engine permits it. Record whether resources were spent and whether a heal occurred. Compare against normal casting behavior, and specifically flag charged-but-rejected heals.
- Start reuse as the last spirit member is about to die. It must never revive a dead actor or leave a partial duplicate group. This is a high-priority cost/lifetime edge case.
- Try inventory activation and quick-slot/pouch activation separately. Record if one route works and another fails.

## Stage 4 — multi-entity and upgrade coverage

Use Lone Wolf Ashes, then Demi-Human or Greatshield Soldier Ashes. Test at +0 and one upgraded level (ideally +10).

1. Count the normal initial group; damage multiple members by different amounts.
2. Reuse once: all survivors heal by 25% of **their own** maxima, capped. One group-wide cost only.
3. Let one member die, keeping at least one survivor. Reuse: heal survivors only; member count must not increase.
4. Let all members die and clean up. Reuse without resting: full fresh group, one normal cost.
5. Repeat while the player is near an unrelated friendly NPC. It must not receive healing or a changed stat. No other players are permitted in this test.

## Stage 5 — restrictions and transitions

- In a prohibited area, first summon/reuse remains unavailable. No new summon-anywhere behavior.
- Walk out of the permitted area; normal dismissal and range limits remain.
- In a boss encounter that allows Ashes, repeat heal and death/resummon. After the boss dies, normal cleanup/restrictions remain.
- Die as the player, reload, travel to another area and return. A new valid first summon must work with no stale group, old target or orphaned entities.
- Quit to title and reload the test character several times. Completely close/relaunch between configuration changes.
- With `HealOnReuse=false`, same-Ash use should return to native dismissal. With `AllowResummoning=false`, do not expect the rest restriction to be removed. Restore defaults afterward.
- Do not intentionally enter online mode or load a protected executable to test rejection. Version/signature negative tests should use an offline development harness or unsupported offline installation only.

## Acceptance matrix

Except for the limited observations noted below, gameplay rows remain **NOT RUN**. No full mechanic has passed acceptance. Pure unit coverage does not change that status.

| Scenario | Required observation |
|---|---|
| First activation | User observed Skeletal Militiamen spawn in 0.1.5; normal cost unverified and active FPS failed |
| Death and reuse | Fresh group without resting, normal cost |
| Living reuse | 25% maximum-HP heal, no replacement |
| Full-health reuse | No over-heal and preferably no cost (policy rejects it) |
| 10% / 90% HP | Approximately 35% / 100%; exact integers checked in log |
| Multiple survivors | Every survivor healed, one cost |
| Partial group death | No individual revival |
| Entire group dead | Fresh complete group at full HP |
| Insufficient FP / HP | No effect and no loss |
| Different Ash active | No additional summon or automatic dismissal |
| Prohibited area | No monument icon / greyed Ash near Church of Elleh, then successful summon with icon at Gatefront; broader restriction coverage untested |
| Repeated healing | No duplicates, stat changes or passive regeneration |
| Boss encounter | Works only where native eligibility remains |
| Death / reload / travel | Clean state and no stale target writes |
| Interrupted / last-member death cast | Correct charging and no invalid actor access |

## What to report

Send the current log, stage/step, Ash name and upgrade, before/after player FP or HP, survivor count, and what happened. A short clip is useful for animation, duplication and ordering problems. For a crash, retain the log and Windows application fault information; restart without this DLL instead of repeatedly loading the same failing test.

Next development priority is native cost/effect telemetry and any queue-routing correction revealed by Stage 2/3, followed by lifecycle stress tests. Production acceptance requires all matrix rows to pass; the current compile/static results alone do not satisfy it.

Installed with both game/protected-launcher processes absent; prior dist files backed up and backup/installed DLL hashes verified. INI unchanged. DLL SHA256: `F96C97C246C16A8F8240CCFCD613D404C1B4DF3C8DAFD436E97B574E8CAC63E6`. See `evidence/install-0.1.12.json`. No tests or game launched by the agent.

