## 0.6.1-solo.5 main source publication - 2026-11-01

- Promote the reviewed Mod Engine 2 / pinned Seamless solo compatibility work to main.
- Include redirect/registry/session fixes and user-accepted HUD name correction.
- Make the compatible solo preset the build-package default, including duplicate groups (three per Ash).
- Preserve runtime code, spell balance, standalone options and all guards.
- Include the staged Academy merchant migration tools; no local installation implied.
- Consolidate current setup/status; retain historical documentation and explicit pending tests.

## Shop migration - prepared 2026-10-31 (DLL unchanged)

- Move Spirit Mend from Kale to the Isolated Merchant on the Raya Lucaria bridge.
- Use the merchant's normal Purchase inventory, preserving the 300-rune price.
- Relocate shop row 100599 to free row 100699 and restore Kale's vanilla dialogue.
- Add a hash-guarded asset builder and preview-first installer with backup/rollback.
- Preserve existing owned spells, all combat parameters and active configuration.
- Compilation and structural asset readback verified; installation/gameplay pending.

## 0.6.1-solo.5 - staged HUD name compatibility fix

- Read loaded FMG item names when the pinned solo profile redirects the public name wrapper.
- Verify layout fingerprints and bound all reads; preserve generic fallback on failure.
- Retain per-family name caching, four compact rows, and all gameplay behavior.
- Record user acceptance of solo.4 Wolves/Lhutel coexistence and friendly lock-on.

## 0.6.1-solo.4 - experimental solo session correction

- Accept observed protocol 6 only in the hash-pinned Seamless solo profile, with no lobby, offline game flag and verified local-only membership/host.
- Retain standalone offline rules and reject remote, unknown, or transitional session state.
- Validate player-vector bounds before reading entries; add scalar gate diagnostics and five pure regression tests (compiled, not executed).
- Preserve existing settings, assets, costs, healing formula, and native hooks.

## 0.6.1-solo.3 - staged, gameplay unvalidated

- Preserve the exact expanded registry constructor in the pinned Seamless solo profile.
- Require profile-specific runtime capacity; retain bounded local tracking and native free-slot checks.
- Revalidate constructor bytes within the startup transaction; retain strict standalone signatures.
- Prepare DLL-only update/rollback from solo.2, preserving active INI and assets.
- Document the remaining optional HUD name-lookup fallback and manual acceptance.

# 0.6.1-solo.2 - reviewed Seamless redirect compatibility

- Correct demonstrated startup rejection of the pinned Seamless summon-state and summon-request redirects.
- Preserve existing function entry chains; verify relay/closure shape, context identity, native prologue tails, handler/callback hashes and transaction-time stability.
- Retain every other signature check and all solo-session, ownership, resource and executable protections. No per-frame scanning or asset changes.
- Add prepared regression coverage and a guarded DLL-only updater. Compilation/static evidence only; installation deferred while the game is running.

# 0.6.1-solo.1 - opt-in Seamless solo profile

- Added AllowSeamlessSolo=false by default; exact binary hash exception when enabled, retaining idle/offline and remote-entry rejection and native hook signatures.
- Added reviewed additive asset merge preserving Clever/Terra rows, text and unrelated NPC scripts.
- Avoided Clever's shop collision with a separate Study sorcery menu at Kalé and shop row100599; spell IDs, 300-rune acquisition and all combat values retained.
- Added a guarded solo profile installer/rollback and separate configuration; diagnostic loader entry replaced without altering existing mods or saves.
- No gameplay or automated tests executed. Connected compatibility remains pending.

# Seamless probe 1 - local investigation (2026-10-30)

- Added a compile-time diagnostic entry point that excludes all gameplay and HUD hooks.
- Exact executable/Seamless hash guards, bounded session/owner metadata, and change-only prospective hook-site fingerprints.
- Separate ME2 diagnostic installer with configuration backup, package hashes, preservation checks and guarded rollback.
- Documented connected ownership/replication gaps, co-op party HUD conflict, and required Clever/Terra asset merge.
- Existing gameplay compatibility guards and healing formula unchanged; no live tests executed.

# 0.6.0-dev.1 - local experimental prototype

- Optional duplicate same-Ash groups (defaultoff), validated1..8 group cap (default3).
- Serialized reservations, full-handle/slot membership, partial-death retention and native costs/cleanup.
- Stable G-id/member labels in the existing four-row HUD;10Hz bookkeeping and bounded timing diagnostics.
- Existing heal-on-reuse retained when disabled; no installed INI or game files changed.
- Tests prepared/compiled, not run; gameplay acceptance pending.

# Unreleased stabilization

- Replace five-second impact-key expiry with complete-registry lifetime pruning.
- Add pure target/impact and supplied native-call fixture regression coverage.
- Add current audit and ordered user acceptance; historical records preserved.
- No new hooks, spell balance, asset or installed-file changes.

# 0.5.1 - 2026-10-30

- Added Spirit Mend target/health/ownership preflight before native projectile creation and resource commitment; no refunds or manual deductions.
- Capture the selected target at release into native projectile launch data; validate that original target at impact independently of current lock-on.
- Add live-bullet identity/lifetime checks, bounded duplicate-impact suppression, scoped callback state and projectile diagnostics.
- Preserve the corrected Kale shop assets, existing spell ID, native staff cost modifiers, animation, impact heal and prior summon features.
- Compile Windows x64 DLL and prepare a reversible installation. Gameplay and automated tests remain user-operated.

---

## 0.5.0 shop-entry correction (asset-only)

- Correct Spirit Mend shop row100599 to100521 inside the documented Kale100500..100524 shop range. Preserve spell, price, native costs, projectile, healing adapter and DLL.
- Correction is staged pending closed-game installation and user confirmation; do not repeat the cast test until the spell can be purchased.

## 0.5.0 development — Spirit Mend

- Clone Swift Glintstone Shard into a separate staff sorcery with its animation, visuals, icon and provisional12INT/5FP/one-slot tuning.
- Build a single non-damaging friendly projectile, inert impact marker and native25%-maxHP heal routed through an owned-Spirit validation callback.
- Add isolated regulation/English text builder, provisional Kale shop acquisition and separate development launcher. Preserve existing0.4.2 installation.
- Compile and stage; no tests executed. Cast preflight/no-charge rejection and cast-time target latch remain unfinished; keep the target selected until impact in this prototype.

## 0.4.2 — friendly Spirit lock-on prototype

- Add optional camera-only team-relation override for validated living local Spirit Ash actors, preserving native enemy decisions and directional selection/cycling code.
- Add exact call-context/prologue checks, transactional installation, independent feature failure fallback, ten-second admission counters and adapter timing.
- Keep native range/visibility gates, allegiance, AI, costs, healing and summon behavior unchanged.
- Compile Windows release; leave all automated and gameplay tests to the user. Native friendly acquisition/cycling remains pending runtime verification.

# 0.4.1 — selected-target diagnostic

- Added opt-in5Hz read-only native lock flag/handle telemetry for the planned targeted Intelligence sorcery.
- Validated typed offsets against exact native field-access instructions; no new camera hooks, patch bytes or OS page probes.
- Logs owned Spirit identity scalars, target classification and timing without resolving arbitrary target actors.
- Preserved0.4.0 gameplay, configuration defaults, capacity and four-bar HUD.
- Release compilation succeeded; all gameplay/automated testing remains user-operated.

---

# 0.4.0 — unrestricted summon-zone prototype

- User authorized removal of vanilla Spirit Ash zone restrictions after reporting the 0.3.1 UI working well.
- Added optional expanded monument range, no activation-region requirement, no return-distance dismissal and no monument boss-elimination target.
- Added cached nearest-monument fallback and current-player spawn-origin synchronization. Preserved rest/death/loading cleanup, costs, one group per Ash and four individual HUD bars.
- Existing PreserveSummoningRestrictions=false now selects unrestricted mode; true/default restores prior zone policy. RequireNormalResourceCost=false remains rejected.
- Preserved upstream notices and source backup. Release compiled; no automated/gameplay tests run.

---

# Changelog

## 0.3.1 - 2026-10-29 (four individual Spirit bars; untested)

- Replace aggregate Ash-group HP with separate current/max HP per validated living Spirit.
- Track row identity by family and native entity handle; label members with localized Ash name and #N.
- Increase the display cap to four, sharing the limit across row selection, injection/reset and movie layout. Overflow members remain active and fill freed rows.
- Extend the isolated compact template to RemotePCTag0..3; preserve approved scale, spacing and offsets.
- Retain native group-wide healing/cost behavior and the existing configuration key. No additional hooks or gameplay writes.
- Compile release DLL without warnings and stage separately. Automated/gameplay tests not run; installed 0.3.0 unchanged.

## 0.3.0 - 2026-10-29 (two-group HUD prototype, not gameplay-tested)

- Analyze the extracted original GFX placements and shared template. Inspect installed-executable HUD call/RTTI bytes without running game code.
- Add optional two-row HUD: local, uniquely associated Ash families; localized Goods names; survivor counts; sum(current HP)/sum(max HP) of survivors; two oldest tracked groups.
- Sample on the existing game task at most 10 Hz, with registry/backlink/handle/module ownership checks, no inactive HP reads and no VirtualQuery in the sampling/render paths. Render hook consumes scalar snapshots only, expires them after 500 ms and skips occupied native friendly tags.
- Adapt upstream Scaleform movie parsing and tag-copy interception with source attribution. Clone the shared template for rows 0/1, reposition only those rows, retain other templates, and suppress Buddy only in a complete supported movie rewrite. Pin the original asset fingerprint.
- Install two additional HUD hooks transactionally; refuse unknown detours and validate native signatures/accessors. Add optional position/spacing/scale settings and bounded diagnostics.
- Compile release DLL without warnings and stage separately. No automated/gameplay tests; no installed DLL/config changes. Runtime HUD recovery requires disabling the feature and restarting.

## 0.2.2 - 2026-10-29 (inactive-member cleanup; gameplay untested)

- Record the failed wolf resummon with another type active; distinguish the adapter registry rejection from the earlier wall-placement refusals.
- Adapt upstream dead-group release using inactive registry states, the pinned native summon-exclusion predicate, temporary-death exclusion, page checks and ownership/backlink checks. Never read inactive HP or force-clear all groups.
- Request native per-member disappearance with its two-second cleanup delay; inspect at most twice per second and log bounded scalar outcomes plus aggregate timing.
- Require retirement evidence before replacing an active-registry group whose HP has only reached zero, protecting revival transitions.
- Add executable predicate signatures, compile-time layout assertions and policy test source. Release compilation only; no automated or gameplay tests executed. Installed 0.2.1 remains unchanged until a closed-game installation.

## 0.2.1 — 2026-10-29 (item-eligibility correction)

- User clarified the two rejected wolf requests were attempted from atop a wall; the later successful summon was on flat ground. This is consistent with preserved native placement rejection and no-charge failures. Retry discrepancy resolved; surviving-old-type preservation still needs an eligible-ground test because the old groups had cleared before the successful request.

- Reported wolf resummon with a surviving Militiaman remains uncorroborated: the captured sequence has all old entries exiting/cleared, two native refusals without observed charge, then a successful 55-FP wolf summon from zero entries and a separate 44-FP Militiaman addition. Requested sequence clarification; no speculative gameplay change made.

- User confirms three Militiaman-only heals while wolves coexist, all five visible and stable 60 FPS. Logs show two 365-max-HP targets gaining 91 HP per activation, no spawning, and FP 251 -> 207 -> 163 -> 119 (44 per use). The initial reported absolute FP values are corrected by logged evidence and the confirmed activation count. Healing isolation now has observed cases in both directions; one-type resummoning remains pending.

- User-operated wolf-only heal while both types coexist: three 500-max-HP wolves receive 392 -> 500, 90 -> 215 and 418 -> 500; no spawn, five entries retained and FP 350 -> 295. User confirms only wolves healed and stable 60 FPS, clarifies increased maximum FP and a later Grace rest explaining cleanup. Other multi-type cases remain pending.

- Installed while the game was closed with a verified 0.2.0 backup. User then confirmed three wolves and two Militiamen coexisting at stable 60 FPS. Logs show eligibility (-1 -> 0), successful native AddGroup, preserved prior groups, five entries and 91 -> 47 FP. Multi-type healing and other regressions remain pending.

- Record 0.2.0 second-Ash greying with sufficient FP while the adapter menu query allowed it.
- Hook the separate native item-usability state call at 0x690833, only in multiple-type mode, using the existing per-family eligibility policy. Validate its context and original target.
- Add bounded Goods/native/adapter eligibility trace fields. No item-limit, cost or area parameters changed.
- Compile and stage the diagnostic DLL; no automated or gameplay tests executed, no running installation replaced.

## 0.2.0 — 2026-10-29 (multiple distinct Ash types; active behavior untested)

- User-operated startup/idle: stable 60 FPS, ten hooks installed, loaded-player capacity guard passed, offline gates ready, no logged errors or activations. Coexistence/healing tests remain pending.

- User-authorized scope change: one group per Ash type, with different types allowed together; upgrade variants share a type.
- Match healing targets to the selected Ash family independently of the native last-active item. Keep existing health/cost safeguards.
- Add a scoped internal native-state hook and exact-request spawn-clear guard to retain other groups. No timer resets or delayed request replay.
- Adapt upstream constructor/slot-search expansion to 500 Spirit body slots, with signature checks and capacity rejection.
- Install the startup patch transaction before registry construction; keep gameplay gated on game-task readiness.
- Add optional AllowMultipleAshTypes setting and policy test source. Compile release DLL; automated and in-game tests not executed.
- Boss resummoning remains not attempted and is deferred at user request.


## 0.1.12 â€” 2026-10-29 (cleanup-readiness correction)

- Boss report clarified: Beastman of Farum Azula in Groveside Cave was defeated before resummoning was attempted. Two paid boss-fight heals are supported; in-fight resummoning is not attempted, rather than a failed or passed test. The earlier log/report discrepancy is resolved.

- User-described boss run logs two correct capped group heals, each with sampled 55 FP spending. Reported post-death resummon is not present in the inspected sequence and remains unverified pending context clarification; no eligibility change made.

- User-operated player-death cycle: no old group after respawn, three fresh wolves for one 55 FP charge, no logged panic or persistent stale-group rejection. Boss and HP-cost cases remain pending.

- User-operated title/Continue test: unloaded player followed by cleared group/item state, then three fresh wolves for one 55 FP charge. No logged panic or persistent validation failure. Recorded as a successful observed reload cycle; player death remains a separate pending test.

- User-operated Gatefront/Church of Elleh area cycle: wolves cleaned up on exit, native prohibited-area rejection preserved at 91 FP with no activation/charge, then three wolves resummoned on return for 55 FP. Locations are user-reported; matching range/group/native-state transitions are archived.

- User-operated full-health wolf reuse with 91 FP: same queued/active Ash, native/adapter (2,-1), validated active group, no activation callback or cost. User confirms full HP and wolves stayed present. Recorded as successful observed full-health no-op behavior.

- User-operated different-Ash attempt with 91 FP: queued Militiamen (212000), active wolves (232000), native/adapter (-1,-1), FP unchanged and no new activation callback. User confirms wolves stayed present. Recorded as a successful observed different-Ash rejection.

- User-operated insufficient-FP reuse with an injured wolf and 36 FP produced no effect or charge. Log shows healing-eligible native/query (2,0), no subsequent activation callback or HP write, and no further sampled FP change. Recorded as a successful observed FP-affordability case; HP-based costs remain untested.

- Subsequent user-operated Lone Wolf test: after one member died, one reuse healed two surviving 500-max-HP wolves by 125 HP each (14 -> 139 and 49 -> 174), with sampled FP 91 -> 36 and no native spawn call for that activation. User reports no delay. This supports the observed partial-group case; other lifecycle transitions remain unvalidated.

- Subsequent user run reported immediate resummoning; logged FP 47 -> 3, two replacement slots and about 40 seconds of continued member validation without logged errors. The actual spawn occurred after old entries were removed; no cleanup-overlap commit was logged. The query exception was observed, but its commit path still needs a prompt post-death activation test. See VALIDATION.md and the archived PID 3476 session.

- Preserve native summon readiness for a same-Ash group awaiting cleanup only when every retained local entry is exiting and matches the observed non-updating registry states (2/2 or 4/2), with no live-summon flag, pending summon request, manager dismissal or cooldown. Unknown and active states remain rejected.
- Inspect registry metadata without dereferencing retiring actors. Keep the existing 4/4 lifetime gate and health-write safeguards for healing. No timer resets, group deletion, forced readiness, extra spawning function or manual FP/HP deduction.
- Log each admitted cleanup-overlap activation and retain timeline/native-result diagnostics. Verify the actual effect argument before native commit.
- Windows release compilation succeeded; no automated or gameplay tests executed by the agent. Response time, exact new-group count and survival through old-group cleanup remain pending user validation.

## 0.1.11 â€” 2026-10-29 (staged resummon timeline diagnostics)

- Subsequently installed after confirmed game exit, with verified backup and installed DLL hashes and unchanged configuration. See `evidence/install-0.1.11.json`. User reproduction remains pending.

- User clarified that the bell animation started on the first use, but FP was charged only when both Spirits appeared about ten seconds later, without another input. The preceding mod callback log is insufficient to locate that asynchronous wait.
- Add debug-only changed-state timeline samples at most every 250 ms: queued/requested goods, request/active effects, local group-slot and exiting-slot counts, alive/range/disappear flags, rounded cooldown/disappear timers, player FP, and the most recently observed native/adapter item-query result pair.
- Add exact callback entry and native-return timestamps/result/elapsed time. Timeline samples retain scalar values only, inspect no Spirit actor pointers, add no OS page probes, and make no new game function calls. Player FP uses the existing local-player ownership checks. Sampling may miss short transitions and is not a button-input or resource-transaction hook.
- No gameplay-rule or timer changes. Compilation succeeded; no tests executed. Staged separately while 0.1.10 remains installed.

## 0.1.10 â€” 2026-10-29 (staged healing callback optimization)

- Subsequent user run: healing callbacks measured 18.976â€“25.568 ms with 4â€“5 page queries and 7â€“8 cache hits, down from the earlier 55â€“65 ms samples. Correct integer HP gain and capping were recorded; user reports 44 FP per initial summon/heal and steady 60 FPS with no perceived hitch. Four logged heal commits versus one described use require input-count clarification; do not infer all costs or duplicate-trigger behavior from this log alone. Scheduler readiness retry also succeeded after an initial NotFound.

- Subsequently installed with the game exited; previous dist files backed up and DLL/configuration hashes verified. See `evidence/install-0.1.10.json`. Runtime verification is pending.

- User confirmed the three 0.1.9 heal activations were intentional and each charged 44 FP. Both targets gained floor(365*0.25)=91 HP per activation. No hitch was perceived, although callback timing was 55â€“65 ms.
- Replace repeated OS queries with stack-only region metadata scoped to a single validation pass (16 regions, uncached fallback when full). Snapshot and final write validation use independent caches. Neither protection results nor actor addresses persist between callbacks.
- One fresh VirtualQuery supplies both readability and writability for each distinct region in the final write pass, replacing the old two queries per target. Preserve full object-range containment, commit/protection/alignment checks, unchanged HP/max checks and all entity ownership gates.
- `readable_probe` now counts/times every actual VirtualQuery including write checks; previously the second write query was outside that metric. Add `page_cache_hit`. Keep callback wall time as the comparable before/after measure.
- Windows release compilation succeeded. Added region-boundary/protection regression test source but did not execute tests. Staged separately; performance improvement and gameplay remain unverified for this build.

## 0.1.9 â€” 2026-10-29 (verified registry state profile)

- Static inspection of the installed, hash-matched executable corroborates the live 4/4 load/update pair. The old assumption Active=2 / Local=0 from upstream enum names was wrong for this use. See `evidence/registry-state-0.1.9.md` and its instruction excerpts for exact evidence and limits.
- Require load=4/update=4 for the pinned build, and add startup byte-signature checks at the native state gate/writers. No new code locations are patched. Network ownership remains checked separately through offline/session and local group/is_remote guards.
- Add actor-to-registry back-pointer equality. Preserve handle, module ownership, health and transition checks. Add aggregate downstream handle/health rejection, target validation and full snapshot completion metrics.
- Release compilation succeeded. No automated or gameplay tests were run for 0.1.9. This update needs user validation of full actor processing before healing tests.

## 0.1.8 â€” 2026-10-29 (bounded scheduler readiness retry)

- 0.1.7 disabled itself at +263 ms before any task registration or hook installation. Its user's 60 FPS and 44 FP first-summon observations therefore do not validate the mod.
- Replace the upstream scheduler wait helper with a local startup-worker retry loop. The pinned from-singleton 3.0.1 returns None for both a missing singleton and a null instance; FromStatic maps either to NotFound, which the upstream helper treats as immediately fatal.
- Retry lookup errors at 50 ms intervals for up to 30 seconds, logging the actual error at most once per five seconds plus final success/timeout. No task registration or gameplay patch happens until lookup succeeds. Executable, signature and environment gates remain in place.
- Retain 0.1.7 registry diagnostics and all gameplay validation predicates. Windows release compilation succeeded; no automated or in-game tests were run for 0.1.8.

## 0.1.7 â€” 2026-10-29 (registry rejection diagnostics)

- User confirmed stable 60 FPS with Skeletal Militiamen active in 0.1.6. Its log rejects every attempted registry lookup, so healing remains blocked and the complete validated actor path is not yet performance-tested.
- Split registry outcomes into pointer shape, capacity, storage shape, missing membership, inactive state, non-local state and accepted. Record bounded scalar samples of capacity, occupied slots examined, and matched load/update bytes. If both state predicates fail, the inactive outcome takes precedence and both bytes remain visible.
- Emit aggregate results once per ten seconds with debug logging; no per-lookup logging, allocation, additional actor dereference, page probe, or actor-pointer cache. Validation rules and gameplay behavior are unchanged.
- Windows release compilation succeeded. Automated and in-game tests were not run for 0.1.7; the user will perform the next summon check.

## 0.1.6 â€” 2026-10-29 (staged active-Spirit performance fix)

- User successfully summoned Skeletal Militiamen at Gatefront Ruins with the monument icon visible, then reported approximately 10 FPS. The 0.1.5 log records one summon callback and 8,857,805 microseconds in 2,760 readable-page probes during a 10,019 ms window. Nearly all snapshot time is in those probes.
- Split read-only query inspection from activation-commit inspection. Item queries now resolve each group actor through the current engine-owned summon ChrSet before dereferencing it, requiring active/local load state, a matching engine handle, pointer alignment, module-owner back-pointer and the existing group/family/HP checks. Registry capacity is bounded at 1,024; no actor or protection-result cache persists between callbacks.
- Retain fresh OS page checks for the commit snapshot and writable/unchanged-health checks before any heal writes. Per-cast latency remains to be measured.
- Unresolved enabled query/cost snapshots now reject rather than falling through to possible native dismissal or a second summon. Added registry lookup/rejection timing counters.
- Windows release compilation succeeded; no automated or in-game tests were executed by the agent. DLL and symbols staged separately; installed 0.1.5 remains unchanged.

## 0.1.5 â€” 2026-10-29 (staged idle-path follow-up)

- Subsequent installation and user-operated idle run: stable 60 FPS reported, corroborated by 600 tasks per ten seconds. Snapshot time fell to 650 microseconds total per ten seconds, with all 1,800 snapshots taking the idle shortcut. No panic recorded. Active-Spirit performance and summon/heal/cost behavior remain untested.
- User-operated 0.1.4 run improved from approximately 9 FPS to 30â€“35 FPS, still below the user's normal performance. The log localizes 6.3â€“6.4 seconds per ten-second window to snapshot inspection (three calls/frame, approximately 19â€“20 ms/frame); native state and recurring task costs are small.
- Move the manager's VirtualQuery-based page check after the empty-local-group shortcut. Before any manager field access, require a non-null aligned pointer matching WorldChrMan's current engine-owned manager, already used by the immediately preceding native state call. No manager/actor pointer is cached across callbacks.
- Preserve all full-path and health-write checks. Add snapshot identity/idle/full and readable-probe timings to distinguish path selection from probe overhead.
- Release compilation succeeded; no tests run by the agent. Staged separately with symbols. The suspected page-probe cost and restored FPS remain unverified until the next user run.

## 0.1.4 â€” 2026-10-29 (staged performance diagnostic)

- User reported about 9 FPS in 0.1.3 after loading a character without summoning. The task heartbeat corroborates roughly 9 updates/second; no panic or log flood was recorded. The responsible function is not yet measured.
- Replaced allocated optional-DLL name/error construction with constant UTF-16 names and a raw Win32 presence predicate. Refresh conflict status once per game task and again at activation commit; item queries use the task's atomic status. Actual online/session state is still read for each enabled query.
- Skip health probes, Goods lookup, effect-map allocation and actor traversal when the validated manager has no local group entries, no alive-summon flag and no native active state. Existing groups still receive full validation.
- Added debug-only aggregate callback and section timings, emitted once per ten seconds. Nested elapsed times include native state calls and must not be added together.
- Release DLL compiled successfully. No automated or in-game tests executed by the agent. Staged separately; no installed DLL replacement in this turn.

## 0.1.3 â€” 2026-10-29 (staged gate correction)

- The user's 0.1.2 run identified the exact blocking predicate: player loaded, parameters ready, no DLL conflict, online mode false, but general server-connection capability true.
- Corrected the gate to require actual offline mode plus CSSessionManager lobby/protocol states both None (raw values 0), with missing or unknown state rejected. The general capability flag is retained in diagnostics but not used as active-session evidence.
- Added four source-level gate regression tests for the user to run; the agent did not execute them. Release compilation only.
- Staged separately while the game remains open. No game network settings or active process memory were changed.

## 0.1.2 â€” 2026-10-29 (staged diagnostic)

- User reports loading the disposable character without summoning. The 0.1.1 log still contains only hook installation; no panic and no BuddyStone edit is recorded.
- Added activation-gate state reporting: player loaded, both online flags, conflict presence, parameter readiness, config and captured-row count. Debug mode emits a ten-second callback heartbeat to distinguish inactive gates from a task that stopped running.
- Startup identifies package version and process ID. No activation restrictions were relaxed.
- Compiled only; no tests run by the agent. Staged separately while the game remains open.
## 0.1.1 â€” 2026-10-29

- User-operated startup reached the menu; the 0.1.0 log confirmed eight hooks and 304 BuddyStone edits, followed by a caught panic. Its cause and timing were not recorded, so startup acceptance remains pending.
- Added parameter-resource readiness checks before BuddyStone/Goods access, covering a potential missing-resource panic during initialization or teardown. This is a candidate fix, not a confirmed diagnosis.
- Added elapsed timestamps, panic message/location/backtrace and callback context. Included symbols in the diagnostic package.
- Rebuilt only; no further automated or game tests were run by the agent. Preserved the previous DLL, INI and log under evidence.

## 0.1.0 â€” 2026-10-29

- Inspected and built MultipleSpiritAshes 0.2.4 at pinned commit; retained licensing and vendored bindings.
- Created standalone Rust DLL and optional strict INI configuration.
- Implemented narrow runtime BuddyStone rest-flag change with conditional restoration.
- Implemented same-item healing, all-member group mapping, health clamping and dead-member exclusion.
- Routed repeat-use costs through native requirement/calculation hooks; documented unresolved debit/commit timing.
- Added exact-version admission, signature checks, conflict/offline gates, peer-thread patch transaction and diagnostics.
- Added automated policy/layout/config tests, read-only PE preflight, non-game DLL refusal probe, reproducible build scripts and manual validation matrix.
- No gameplay tests executed and no game/mod-loader/save installation changed.
