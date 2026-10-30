# 0.5.1 - native release gate and per-projectile target

`src/mend_cast.rs` is included by `spirit_mend.rs`. All addresses below are RVAs in the inspected WW 2.7.2.0 executable (App 1.17.2), SHA256 F9118FEA2CFD8FA3901E8060BFADB1552CB2471771DED8A5B662C4E298FFB328. Static evidence is in `evidence/cast-static`; these are pefile/Capstone observations, not REA decompilation or runtime traces.

- Calls 3c05d6/3c0611 invoke projectile action 3c0f80. In both callers, a false result returns before resource application 3c0e70. The action prefix contains owner ChrIns at +0, magic ID at +c. The wrapper affects only Magic 990001. It validates offline state, current player, locked-on owned living injured Spirit and healing assets, then calls the native function. Rejection returns false, with no manual FP subtraction or refund. Native intelligence/equipment/FP eligibility and staff modifiers remain in the game. The release check occurs after animation start, not at button press.
- Scoped thread-local release handles are available only during that native call. At 3c0ff0 -> 3c10c0, the normal builder finishes first. Its fresh BulletSpawnData output is checked for magic/owner/bullet IDs and an empty extended target. The explicit target at +20 is set, the borrowed alternate auto-target pointer is cleared, and the forced free-aim bit is cleared for this locked-on custom spell. No camera/actor state is rewritten. Unexpected context makes the bullet ID invalid, preventing spawn/cost.
- Native 38e750 selects the explicit handle; 3950c0 constructs a type-2 target with the full handle at +8, then stores it in launch context +140. 39e90f copies the 660-byte launch context to CSBulletIns+4b0. Each native projectile therefore carries its own original target, separate from mutable homing state at +d30. There is no global last-target variable or between-frame actor-pointer cache.
- Call 449ffa -> 4489f0 supplies the attack packet. For custom attack ID +40 = 99000100, its original FieldIns pointer at +1d8 is compared against the bounded current CSBulletManager list before dereference. Checks include bullet type/full handle, owner backlink, fly parameter, launch bullet/magic IDs, original target kind/handle and current player ownership. A scoped value carries those scalar identities through the existing effect calls at 4493d3/4494b8.
- Impact receiver must match the original launch target and still pass owned/living/injured/asset checks. Current camera selection is diagnostic only. A bounded 64-entry, five-second scalar handle ledger prevents duplicate delivery from the same projectile. No borrowed pointers survive a callback. Native registry lifetime and generation-bearing handles are checked anew; engine destruction/load removes the projectile. Expired delivery keys are pruned on the next impact.
- The existing native 3e8e70 operation applies SpEffect 99000101 (instant -25 percent HP-rate, zero flat HP, duration/interval/stateInfo zero). A valid release that later misses, meets an obstruction, loses its target, or arrives after another heal still spends its cost. It cannot transfer its heal to a new selection or a different intersecting ally.
- All six call-site adapters install as one existing transaction, with original-call checks, callee signatures, native cost-branch/context signatures and compile-time layout checks. A mismatch disables Spirit Mend hooks. Without hooks, assets remain inert and native casts may still cost FP; no invalid-cast guarantee applies to a disabled/unsupported installation.

Vendored bindings add four public BulletSpawnData fields, a narrowly scoped explicit-target method, and a read-only CSBulletIns launch-byte accessor. Licenses are retained. Ordinary casts/effects are forwarded. No per-frame scans, VirtualQuery loops, native target/team writes, custom costs, passive healing or new gameplay tests are introduced.

---

# 0.5.0 development — native projectile impact healing

See [SPIRIT-MEND.md](SPIRIT-MEND.md) for the custom parameter chain, scoped hit-effect callback, native percentage healing, ABI/signature evidence, isolated asset build and remaining cast-preflight/target-latch work. The implementation is compiled, not runtime-verified. Earlier sections below describe prior releases.

---

# Implementation — 0.4.2 friendly camera admission

`src/friendly_lockon.rs`, included in runtime, intercepts only the camera candidate-gather loop's team-relation call at RVA 0x7179d6, originally calling 0x51b8b0. Offline Capstone analysis of WW 2.7.2.0 shows the original predicate queries the team relation table; it does not perform geometry or health checks. The caller passes viewer R15 in RCX and candidate RAX in RDX. A zero return skips the candidate. The callback first invokes this predicate and preserves every nonzero return. It overrides zero only for a currently validated, living local Ash actor. The global relation predicate itself remains unpatched.

Validation requires existing READY/fault/offline/session gates, viewer identity equal to the current main player, living player, current summon-manager owner groups, recognized positive effect-to-Buddy mapping, exact candidate pointer membership, nonremote/nonwarping/nondisappearing entries, active registry state/backlink, full handle re-resolution, nonempty handle, no death flag and typed data-module ownership with 0 < HP <= MaxHP. Actor pointers are borrowed only for this callback, never stored across frames. Query validation adds no VirtualQuery, heap allocation or log write in the normal callback path. This relies on native actor lifetime during camera traversal, like existing engine callbacks; catch_unwind handles Rust panic, not access violations.

A nearby RX relay redirects the one call through the existing transactional optional patch writer. Exact 19-byte caller context plus target prologue are checked after existing executable/version/conflict gates. Failed installation disables only this feature; a caught adapter panic falls back to the original result and reports through the game task. No candidate flags, selected handle, team, AI or HP are written. The availability flag before this call, range/cone tests after it, downstream visibility/scoring and directional candidate-list cycling remain native. Whether Spirits pass all other gates is intentionally left to the first gameplay test; this prototype does not force those flags.

Aggregate ten-second counters distinguish native_allowed, owned_admitted and rejected predicate calls (not unique actors). `friendly_relation_adapter` measures only adapter time after the original predicate. The existing 5Hz selected-handle observer independently reports owned_spirit vs other_or_unresolved selection. Counters alone cannot prove successful selection or cycling. A nonzero owned_admitted count with no owned selection points to a remaining downstream gate; no admissions with known live groups may indicate upstream exclusion or validation rejection.

Evidence: `evidence/lockon-static/candidate_gather.asm`, `admission_predicate.asm`, `score.asm`, `switch.asm`. The offline disassembler is an ordinary Capstone fallback, not a REA decompilation result. No FreeLockOnCamera code is copied; its camera/scoring/range changes are not imported. Config adds Targeting.EnableFriendlySpiritLockOn, default false, enabled only in the prepared local prototype. Costs, summons, UI, and planned spell logic are unchanged.

---

# Implementation — 0.4.1 target observer

`src/targeting_diagnostic.rs` is included in runtime. Config adds target_diagnostics (defaultfalse), read from Debug.EnableTargetDiagnostics. Initialization happens after existing version/conflict/startup-hook checks. It verifies typed offsets PlayerIns.locked_on_enemy=0x6b0, ChrIns.is_locked_on=0xc9, handle size8 and exact native instructions at RVA0x71851a (clear selected handle),0x71857e (copy selected actor+8 handle to player+6b0),0x3f74e0 (write lock flag). No new patches are installed.

The existing GameMan task samples at most5Hz. A live local player and existing enabled/offline gates are required. It copies the selected handle/lock flag and bounded owner-group metadata; only registry-active, backlink/handle-validated local Spirits are inspected for handle/Buddy/NPC/team/death-flag scalars. Disappearing/warping/remote/unrecognized entries are excluded. Selected unknown actors are never resolved or dereferenced. Labels are not_locked, locked_without_handle, owned_spirit, other_or_unresolved. These labels are telemetry, NOT an authorization predicate for a future heal. Full HP/range/LOS/action validation will be necessary for healing.

Only scalar snapshots survive the callback. World/player scalar identity prevents stale comparisons; missing/dead player resets the snapshot. Changes and10s heartbeats are logged, with at most16 Spirit descriptions per line and full count. Timing metric target_sample includes the sampler/log cost. It does not collect the complete native camera candidate list or change its admission/cycling rules. Existing camera candidate analysis remains provisional. Source static analysis found list nodes with next+0x80, target record+0x88 and flags+0x90; selected bit0x20 and eligibility bit0x10 participate in scoring/cycling, but interpreting those bits does not prove where friendly actors are rejected. Do not bypass these flags globally.

Build compiled, no automated or gameplay tests run. Runtime checks execute only when the user launches the diagnostic. Reference disassembly is under evidence/lockon-static, generated offline by workspace work/lockon_disasm.py and work/lockon_candidates.py. No reference camera-mod source was copied into shipping code.

---

# Implementation — 0.4.0

## 0.4.0: user-authorized unrestricted zones

The user explicitly superseded the original vanilla-area restriction after accepting the four-row HUD. `Config.anywhere` is the inverse of the existing `PreserveSummoningRestrictions` setting. Costs remain mandatory. Default/no-INI behavior preserves zones; the prepared local package sets preservation false.

`runtime.rs::update_stones` now captures row address plus five originals per BuddyStoneParam. Anywhere mode applies activate_range=65535, overwrite_return_range=-1, overwrite_activate_region_entity_id=0 and eliminate_target_entity_id=0; the independent resummon switch still controls summoned_event_flag_id=0. These are the rules from upstream extras.rs::update_buddy_stone_rules. No disk parameter files change. Disable/fault restoration compares each current field to our written value and validates the row address. Table readiness/repository/anchor checks gate the readiness flag and reset the tile cache. Parameters loaded before a transition can retain native cached data until restart/reload; live setting toggles are not supported.

`anywhere.rs` adapts upstream stone_tile/nearest_stone/apply_stone_fallback. It prefers native nonzero monuments; when none is loaded it supplies an actual captured parameter row ID. Selection is cached by current world/player/manager identity and map tile, including 2x2/4x4 overworld blocks. Same-area nearest IDs are preferred; if an area has none, a deterministic real row is used. No entity pointer is manufactured. This cross-area fallback extends the upstream last-stone fallback and especially needs gameplay validation in dungeons/hubs. Cache entries contain scalar identities, not dereferenceable retained actors.

The helper runs in the existing game task and at the outer item query/cost/eligibility/activation callbacks, before actor snapshots. This covers native monument searches between tasks without adding hooks or rewriting native state return codes. It requires the current world manager, live player, valid finite block coordinates, applied params and existing offline/version/conflict gates. Adapter range checks admit the configured anywhere mode; native collision, action/cooldown and lifecycle checks still apply. It does not blindly clear disappearance flags, revive entities or erase groups. The used-monument lock is released only when no local native entries remain, no live-summon flag or pending request exists and cooldown is over, avoiding the upstream raw-HP scan of reviving skeletons.

At accepted fresh summon calls and pending-request task updates, the spawn origin is synchronized to the current player's finite Havok position using typed module pointers. This prevents a remote fallback monument supplying the location. Native spawn placement still decides whether terrain is suitable. There are no new per-frame VirtualQuery probes, no DLL hook addresses added, and no changes to healing/cost calculation or HUD files. Existing guards/version signatures remain pinned to WW 2.7.2.0. Engine-owned pointer access retains the project's serialized game-callback lifetime assumption; arbitrary concurrent game-state mutation is not supported.

All older claims below about excluding area-rule changes describe versions through 0.3.1. The 0.4.0 section takes precedence. Native scene scripts may still prohibit item use or remove entities; those are untested, not globally bypassed. Rest, death, map unload and scripted cleanup are intentionally preserved.

## 0.3.1: individual entities, four rows

The user superseded the group-total display with one bar per Spirit, maximum four. `hud_snapshot.rs` keeps the existing ownership/family/registry/handle/module validation and 10 Hz cadence, but now publishes a Member per valid living entity, containing its copied FieldInsHandle, family base ID and individual HP/max. No actor pointer or summed HP is stored in the HUD snapshot. Invalid/dead/retiring members are skipped individually; they no longer suppress healthy peers' rows.

`hud.rs` keys ordering by (Ash family, full native FieldInsHandle). It retains continuously observed members' ordinals and appends new members with the next currently unused ordinal within that family. Names are localized Ash names suffixed #N. Ordinals are UI identifiers, not NPC slots or permanent spawn IDs; a temporarily unobservable or reviving actor can return with another ordinal/order. Metadata is removed when members disappear; stale handles are never used to retrieve health. World/player changes and empty state clear or prune the view as before.

MAX_ROWS=4 is shared by row selection, copy-hook injection bookkeeping and movie layout. RemotePCTag0..3 are reassigned to the isolated sprite and repositioned; tags 4/5 retain their original template/positions. All four required placements must match before replacement. Scale, spacing, original asset fingerprint, native hook addresses, sampling/expiry, occupied-tag suppression and recovery behavior are unchanged. There are no new hooks or gameplay writes. The first four tracked living entities appear, with overflow taking slots as entries disappear. Example: three wolves followed by two Militiamen initially show three wolf rows and one Militiaman row; the fifth entity is not despawned or denied healing.

The prior 0.3.0 sections below describe the previous aggregate implementation. For 0.3.1, use individual HP semantics and four-row limits above. Full handle identity is validated afresh each sample; native handle recycling between samples can reuse a cosmetic identity, but cannot cause a stale HP read because render never resolves cached handles.

## 0.3.0 HUD implementation

`hud.rs` owns a two-row scalar snapshot and the native tag-copy adapter. `hud_snapshot.rs` is included in runtime.rs so it can reuse engine-owned actor validation. `hud_movie.rs` adapts the upstream GFX parser and MemoryFile loader interception. `hooks.rs` provides an optional frozen-thread patch transaction and RX relay sealing, separate from gameplay installation. Configuration is optional and defaults HUD off; pending-0.3.0 enables it explicitly.

Static file analysis found Buddy at (23,561), sprite 482; RemotePCTag0 at (196.7,286), RemotePCTag1 at (196.95,380), both sprite 310. That sprite contains HP, Name, RoleName and multiplayer icon clips. Full observations: evidence/hud-layout-analysis.json. The GFX declared length is 1,634,757 bytes, with 11 trailing archive-padding bytes in the extracted file. Pinning uses FNV-1a64 0x71f7ecb979e78040 over the declared movie, allowing padding differences. This is an accidental-compatibility check, not a cryptographic authenticity guarantee. The source asset's SHA-256 is recorded in evidence/uxm-hud-extraction.json.

The movie rewrite duplicates the first two rows' shared sprite under a conservatively checked unused ID, points only those two placements to it, hides unused icons in that clone, scales and repositions the rows, and hides Buddy. Other template instances keep the original sprite. All required layout steps must succeed before replacement bytes are returned. Existing replacement movies are refused by fingerprint. Patched buffers are retained for process lifetime with at most four cache entries. No game assets are included in the package, and no file on E: is changed by the DLL.

Native evidence: tag-copy call 0x772fbd targets 0x775480; message lookup 0x2671370; OpenFile RTTI vtable 0x2bc0748 slot 1 targets 0xd6e970; MemoryFile vtable 0x2baa020 has verified valid/tell/length accessors at slots 2/4/6. See evidence/hud-native-analysis.json. The two HUD hooks use prepared relays with original function addresses published before commit. Peers are frozen while instructions are rechecked and written; rollback is attempted on failure. OpenFile detours from other mods are refused rather than chained. HUD setup errors do not undo existing gameplay hooks.

The collector checks readiness/offline state, player lifetime, local owner, family uniqueness, current registry 4/4 state, actor backlink, native handle resolution and module ownership. It uses the existing Query read contract on the game task; no actor pointer leaves that synchronous pass. This does not establish safety under concurrent engine mutation beyond the existing task/lifetime assumption. Unknown members suppress a complete row rather than fabricate a total; inactive exiting members are skipped without dereferencing actor HP. HP-zero members are excluded from totals, with group identity retained while entries remain, allowing revival to reappear in subsequent snapshots. No actor health, resources or AI are modified by HUD code.

Totals are saturated u32 sums of valid survivors, capped at summed max HP. Death can raise the displayed percentage by removing a member from the denominator; the xN count makes the loss visible. Group order uses first observed local-list order; if multiple new groups appear between samples, it is not guaranteed to reconstruct exact historical spawn order. The first two tracked groups are displayed; there is no third-group cycling in this prototype. Stable records contain IDs/strings/scalars only. Name lookup uses a real Goods row with matching trigger and native message categories, cached per world/player snapshot identity. Unknown names fall back to Spirit Ash <id>.

Snapshots refresh at most every 100 ms and expire in the copy hook after 500 ms. The native copy runs every frame; injection uses nonblocking locking, no actor traversal and no file I/O. DLString assignment may allocate through the engine. The original copy routine always runs, then injected free entries are reset. Any occupied friendly slot prevents injection rather than overwriting NPC/player data. Because the loaded tag positions are changed, NPC display placement is not fully isolated; this remains a prototype limitation. The cloned sprite isolates icon-template changes for tags 2–5 but cannot undo positions for a native user of tags 0/1.

Timing includes hud_sample plus 10-second scalar summary logs (shown names/HP/count, copy count, occupied-tag count). No measured performance claim is made. After successful movie replacement, a later row failure can leave the vanilla panel hidden; automatic runtime re-show is not implemented. Restart with HUD disabled is the recovery path. A runtime panic boundary does not catch access violations. Actual rendering, menus, transitions, skeleton revival, NPC-tag coexistence and performance are pending user validation.

## 0.2.2: inactive members retained alongside another type

The 0.2.1 adapter required active registry states 4/4 unless a group already had `disappear_requested`. After wolves died with another type present, their entries remained 4/2 or 2/2 without that flag, causing snapshot rejection before DoSummon. Suppressing the spawn-specific owner-wide clear makes explicit dead-member release necessary in this observed case. Earlier native placement refusals on a wall are a separate issue.

`src/cleanup.rs`, included by runtime.rs, adapts upstream `extras.rs::release_dead_groups` more conservatively. On the game task at most every 500 ms, it examines only local owner groups with recognized BuddyParam IDs, no warp/remote/disappearance request, and exact inactive registry membership. Fresh page checks and actor -> registry, controller -> actor, and modifier -> controller backlinks precede reads. It never reads inactive actor health or caches actor pointers between callbacks.

Static native evidence: RVA 0x3f4740 reads ChrIns+0x58 -> ChrCtrl+0xc8 -> modifier+0x24 bit 0. The owner-alive predicate calls it at 0x4b7ca5 and excludes true results. Both instruction sequences are startup signature requirements, not additional hooks. Public chain offsets have compile-time assertions. This is a native summon-exclusion predicate, not an independently proven universal permanent-death flag. Release additionally requires inactive 2/2 or 4/2 and a false typed `temporary_death_state` flag; uncertain evidence leaves the entry unchanged and the request blocked.

For an eligible entry only, the mod writes its native `disappear_delay_sec=2.0` and `disappear_requested=true`; engine cleanup owns removal. No HP, stats, manager timers, resource values, or living actor fields are changed. Existing snapshot logic can then skip that inactive exiting member; it does not itself wait two seconds or replay a rejected cast. A selected family with any valid survivor still heals rather than spawns. Active-registry HP-zero/death-flag members prevent a replacement group until retirement is established or they are removed, avoiding HP-only assumptions about skeleton revival.

This path runs only in multiple-type mode while ready/offline, a native summon remains alive, the player remains in range, and no summon/dismissal is pending. Living entries cause no new page probes. Inactive candidates can incur page-query overhead; `inactive_cleanup` timing and release/unproven/temporary counters make this visible during user tests. The 500 ms throttle limits probe frequency but does not guarantee hitch-free large groups. Native cost routing and the eleven call hooks are unchanged.

0.2.2 is compiled and staged, not installed or gameplay validated. In particular, the modifier evidence may be unavailable after unload, in which case the correction deliberately fails closed. Native death/revival behavior and interaction with partially surviving groups require user validation. Older sections below describe historical build states.

## 0.2.1: independent item-usability gate

The 0.2.0 test exposed an omitted eligibility call at RVA `0x690833`: the engine passes the Goods ID in EDX, calls GetBuddyState, compares EAX against -1 and stores non-rejection in R13B. At `0x69096e`, a false R13B joins the other native failure predicates. Static inspection establishes this independent gate; the log shows the already-hooked item query allowed a different type but no activation reached DoSummon. Runtime confirmation of this call's rejection is pending.

This call now uses a dedicated `eligibility` wrapper with the same `adapt_query` policy as existing item queries, and is installed only in multiple-type mode. Exact surrounding instructions and original call target are validated transactionally. Normal item attribute, resource, area and multiplayer conditions around it remain native. Eleven call redirects are installed in multi-type mode. No EquipParamGoods item-limit field is modified: upstream's broader `use_limit_summon_buddy` clearing was considered, but this specific omitted native-state check provides a smaller correction.

The existing 250 ms diagnostic trace now includes last eligibility Goods/native/adapter values, with atomic scalars only and no per-call file writes. -128 values mean no observation yet. This records the extra gate separately from menu queries. It does not prove final item usability or cost consumption. 0.2.1 remains untested and staged; the running 0.2.0 DLL is unchanged.

## Scope change and status

The user requested coexistence of different Ash types on 2026-10-29, superseding the original single-type restriction. This build is compiled, not runtime validated. The historical sections below record earlier architecture and diagnostics; this section defines current multi-type behavior.

## Group identity and healing

`src/multi.rs` is included by `runtime.rs` and reuses its ownership, registry, handle, health-module and commit-time page checks. The selected Goods row supplies the trigger effect. `policy::unique_family` resolves exactly one base effect within upgrade offsets 0–10, gathers all linked BuddyParam IDs and rejects ambiguous/overlapping families. Only local-owner entries with those IDs can become healing targets. Upgrade levels share one type identity; unrelated groups never become targets. Existing characters retain their original stats and upgrade effects.

A surviving selected member selects Heal (or Reject when everyone is full/HealOnReuse is off). No surviving selected member selects AddGroup. Unknown lifecycle states reject rather than count as death. Exiting entries in the observed 2/2 or 4/2 registry states can be ignored without dereferencing them; these entries are never healed. A missing/unknown registry entry remains ambiguous and rejects. Skeleton revival transitions are still a known testing gap.

Healing retains the 0.1.10/0.1.12 typed HP implementation: `min(max, current + floor(max * percentage / 100))`. Fresh page/write and unchanged HP/max checks precede all writes. No actor pointers persist across callbacks. Reuse never invokes native spawning. Matching a type no longer depends on the game's single last-active-Goods field.

## Native activation and resource costs

The original eight item/requirement/cost/effect hooks remain. Native state queried with Goods=-1 supplies active-group readiness; the selected Goods-specific -1 can mean merely a different type from the game's last summon. The new policy may replace that Goods-specific rejection only after independently identifying the selected family, validating native active state 0/1/2, normal activation range, absence of manager dismissal/pending request and expiration of the native cooldown. Native active state -1 is never changed to ready. Item-area, multiplayer and requirement functions remain unpatched.

For AddGroup, the mod calls native DoSummon once. A thread-local scope applies only during that synchronous call. Its internal state call at RVA `0x4b86a9` can report ready instead of active/dismissal for the validated request. It does not broadly change GetBuddyState or schedule a second request. Native origin/scaling/spawn checks remain in DoSummon.

A bounded pending token stores only world/manager identity addresses and owner/Goods/effect values, never actor references. The spawn-specific clear call at `0x4b8a90` normally invokes `0x4bba80` to erase every owner group before adding the new one. For the exact accepted pending request, this one clear call is suppressed, preserving other groups. Other cleanup call sites are untouched; disappearance flags/timers are never reset. The token is discarded when current world/player/manager/request no longer matches. No delayed replay or synthetic request is implemented.

Cost wrappers expose normal paid state for a validated Heal/AddGroup. While the exact accepted AddGroup is still pending, the matching cost query also exposes paid state so it cannot become a free-dismissal calculation midway through consumption. The engine owns affordability and deduction; there is no manual FP/HP subtraction or custom price. Exact-once timing, interrupted activations and HP-cost Ashes require user-run validation in 0.2.0.

## Actor capacity and startup transaction

`src/capacity.rs` adapts upstream `engine_patches.rs::spirit_set_size` and `spirit_slots`. The constructor epilogue at `0x495675` expands only the just-constructed empty 80-entry registry to 520 zeroed entries through the same native allocator/free pair. The entry initializer at `0x3c3810` was inspected: zeroed storage gives its empty pointer/status/flag state. The original allocation and initialization body is unchanged, avoiding mixed allocation/loop sizes during installation. The complete pinned constructor bytes are checked first. Allocation failure leaves the old set; the gameplay adapter refuses an unexpected capacity.

The slot-start sequence at `0x493492` begins at 20. The bounded search at `0x4934c0` uses actual capacity and wraps once, retaining native full-pool/found-slot exits. New groups preflight available slots using all family BuddyParam rows and their mount requirements; no existing group is evicted. There is no per-type-count cap, but memory remains finite: 500 Spirit body slots. No stress/performance claim is made for large counts.

Startup checks WW file version 2.7.2.0, known conflicting DLLs and all original bytes. Eleven call-site redirects and three capacity/search redirects are prepared as one transaction, then peer threads are suspended, byte ranges rechecked, writes committed or rolled back, and peers resumed. Thread overlap checks now cover variable-length sites. Stubs are RX before installation. Hooks are installed early enough for registry construction, but gameplay callbacks pass through until the game task publishes readiness. A late-loaded player or wrong registry capacity disables gameplay adaptation; restart is required. No live registry resize, hot unload or online compatibility is supported. A disabled adapter does not uninstall already-applied code patches; exit restores the process.

## Files and compatibility

New: `src/multi.rs`, `src/capacity.rs`. Updated: `runtime.rs`, `hooks.rs`, `threads.rs`, `policy.rs`, `config.rs`, `lib.rs`, Cargo manifests/lock, configuration, build/preflight tooling and documentation. `AllowMultipleAshTypes=true` is optional and defaults on. False keeps the legacy single-type policy and eight hooks, without capacity or spawn-retention patches.

Upstream unrestricted-area/item/range patches, item-limit parameter clearing, low-FP/free dismissal and group flag/timer resets are deliberately excluded. Existing native item restrictions may expose additional compatibility gaps; unknown modded parameter families fail closed. Version signatures and source inspection cannot establish live safety. Loading the DLL with another summon-system mod, Seamless Co-op or protected online play is unsupported. MIT/Apache and dependency notices remain in the package.

---

# Historical architecture through 0.1.12

## Status and foundation

The prototype is a fresh cdylib with a small gameplay policy and a native adapter. It does not link the upstream MultipleSpiritAshes gameplay library. Only its vendored engine bindings and narrowly adapted hook/parameter techniques are reused. Upstream's standalone source compiled successfully with its locked dependencies before development started.

Milestones 1â€“4 have source implementations; milestone 5 has a build, packaging, diagnostics and static/unit validation. Runtime acceptance remains open for all gameplay milestones. This is not evidence of a working in-game release.

## Module responsibilities

| File | Responsibility |
|---|---|
| src/lib.rs | Loader entry; schedules initialization outside loader lock |
| src/config.rs | Optional strict INI parsing and invariant validation |
| src/policy.rs | Maximum-HP arithmetic, action policy and all-member trigger-family mapping |
| src/hooks.rs | Version-specific call-site validation, stub preparation and transactional patching |
| src/threads.rs | Startup peer-thread suspension/context validation and resumption |
| src/runtime.rs | Offline checks, parameter lifecycle, native item routing, validated group healing |
| src/timing.rs | Bounded aggregate wall-time diagnostics emitted every ten seconds |
| tools/Preflight.ps1 | Read-only installed-executable checks |
| src/bin/dll_probe.rs | Real Windows DLL load into a non-game host to test refusal |

## Resummoning

Upstream `extras.rs::update_buddy_stone_rules` explicitly sets `BuddyStoneParam.summoned_event_flag_id` to zero for resummoning. Its other modifications to activate range, return range, activation region and boss elimination target are independent and are excluded here.

At a GameMan task, after hooks are ready and offline checks pass, the adapter captures row IDs, addresses and original flag values, then sets only that flag to zero. Restoration uses matching row addresses and restores only a value still equal to the mod's zero. A representative row detects table replacement. Parameter hot reload/in-place table replacement is not a supported compatibility guarantee.

The DLL does not clear event flags through the save/event manager. After all members die, native manager cleanup and normal DoSummon create the fresh group, charge its native cost, and apply its normal upgrade effects. Dead members are not selectively respawned. No group pointers are retained, so reloads/deaths require no custom actor tracking reset.

## Hook sites and activation

Only WW 2.7.2.0 is admitted. The installed file's static signatures were independently checked. Addresses are image-relative:

| Purpose | RVAs |
|---|---|
| Native GetBuddyState | 0x4b73d0 |
| Three menu queries | 0x7c4154, 0x847ed0, 0x848194 |
| Native FP/HP requirements | 0x69063a, 0x6906dd |
| Native cost calculation | 0x3c0b79, 0x3c0c2f |
| Native DoSummon | 0x4b8690 |
| Main-player summon-effect call | 0x3fb268 |

The first seven sites are from upstream. Static analysis of the installed executable located the last call and confirmed `RCX=manager`, `EDX=effect`, boolean result in AL. The native DoSummon path calls GetBuddyState(manager, -1), sets `disappear_requested` for state 2, and otherwise records the summon request after native checks. This DLL intercepts its main-player call, preserving the original function for fresh summons.

UI/requirement/cost wrappers call the original state routine first. They map only a validated, matching, wounded living group in native state 2 to state 0. Native state -1/cooldown is never bypassed. UI rejection yields -1; cost rejection yields 2 (the native free dismissal/no-cost branch). A successful heal returns true without invoking DoSummon, setting a spawn request, or setting the dismissal flag. A different Ash or ambiguous active request is rejected.

The same goods ID and active trigger effect must match; upgrades are not ignored. Negative native goods arguments are resolved only from the player's valid queued Goods item. No quick-slot selection guess is used for committing a heal. **Whether that queue is populated at every cost and effect call must be checked live.** An ambiguous commit is rejected rather than risking a duplicate or dismissal.

## Healing choice and safeguards

As of 0.1.6, read-only item queries and actual activation commits use different inspection modes. Query mode first matches the manager to current WorldChrMan. Group pointers are compared against the current summon ChrSet's bounded entry table before any actor dereference; only active/local entries (raw load state 2 and update type 0) are accepted. The engine handle must then resolve to the same actor. Player/modules are borrowed from the current engine-owned actor graph, with null/alignment and health-module owner checks. Group owner, exact Ash family, HP validity, range and cooldown checks remain. This depends on the game's serialized callback/lifetime guarantees and exact supported layout; it is not protection against arbitrary concurrent corruption.

Commit mode re-resolves the group and retains fresh VirtualQuery checks for manager, player, actors and modules, then checks write permission and unchanged HP/max HP before writes. No cached actor pointer or cached page-protection result crosses a callback. Query mode cannot write health. An unresolved enabled snapshot rejects the UI/cost action; it no longer falls through to native dismissal. This distinction removes the demonstrated repeated page-query bottleneck while retaining commit safeguards. Commit latency, registry-state correctness during transitions and actual healing behavior remain unverified.

Approach A was investigated: `ChrInsExt::apply_speffect` exists, and the [Paramdex SpEffect definition](https://github.com/soulsmods/Paramdex/blob/master/ER/Defs/SpEffect.xml) describes `effectEndurance=0` as instantaneous and `changeHpRate` as a percentage of maximum HP. A negative rate is the candidate healing direction. No neutral, conflict-free carrier row was validated against the installed regulation or runtime, and the bindings supply no row insertion API. Reusing an existing global effect could affect unrelated actors or retain asynchronous behavior.

Approach B was investigated in the vendored CS APIs/RVA tables; no native health setter was exposed. REA's deep decompiler provider was unavailable. This bounded search does not establish that no native function exists.

Approach C is therefore the prototype implementation: write only the typed `CSChrDataModule.hp` integer after validating the engine-owned actor and health module. It performs `min(max_hp, hp + floor(max_hp * percentage / 100))` using widened arithmetic. Maximum HP and all other stats remain unchanged. No effect row is modified and no regeneration is introduced.

Validation per activation includes:

- Main WorldChrMan manager identity and alive local player.
- Goods effect category and exact active goods/trigger match.
- Local-player owner key in the manager's group map.
- All BuddyParam IDs from the trigger's chaining-map family, including shared-trigger entries and upgrade offsets 0â€“10.
- No remote, disappearing, warping or foreign-family living members.
- Non-null, aligned, committed readable actor/module memory; module owner back-pointer.
- Engine handle resolution returns the exact current actor pointer.
- Positive HP, no death flag, positive maximum HP and HP not exceeding it.
- Writable committed health memory and unchanged HP/max HP immediately before committing all writes.

Targets are deduplicated and never retained across callbacks. Unknown/oversized lists fail closed. Dead entries are skipped and never revived. All targets are collected before the first write; no native engine function is called between health writes. The assumptions about serialized game activation and actor lifetime still require runtime stress testing; Rust panic guards do not catch access violations or prove foreign memory valid.

## Resource costs and unresolved transaction boundary

Upstream established that native state 2 zeros FP/HP costs, so repeat-use healing cannot simply intercept dismissal. The four item cost/requirement hooks restore state 0 for a valid heal; the engine then performs its own cost checks and calculation. This DLL never writes player FP/HP or manually subtracts a cost, avoiding an additional mod-side deduction and preserving the native cost formula/modifiers.

However, static call-site analysis does **not** establish the full timing of the engine's cost computation, actual debit, and summon-effect invocation. A world/health/selection change or interruption between those stages could reject a heal after a debit or allow a heal using an earlier no-cost evaluation. There is currently no proven debit receipt, refund, or atomic cost/HP commit. The exact-once, insufficient-resource, no-charge-on-failure and interruption requirements therefore remain **unverified and potentially incomplete**, not passed. Next work must trace these stages and, if required, move the adapter to a verified common native transaction boundary. Do not solve this by blindly adding another FP/HP subtraction.

## Startup safety and lifecycle

Host name and exact file-version/language checks precede game address access. Known incompatible/protected DLLs cause refusal. Startup is rejected if a player is already loaded. Every original relative-call destination and function signature is checked before patching; RX stubs are prepared first. Peer thread handles are opened before suspension; other enumerated threads are suspended, instruction pointers checked, original calls rechecked, and the prepared batch written. Failed writes attempt rollback before resuming threads. No logging/heap allocation is intentional inside the suspended interval. This transaction still needs live startup testing, including thread creation and mod-loader interaction.

Hooks are enabled only after the batch is complete. Inactive/offline-ineligible callbacks pass through native routines; BuddyStone changes are compare-restored on the next task. Errors always log; optional logging records activations. Hooks, RX pages and the recurring task intentionally live until process exit. Hot unloading is unsupported. The adapter does not modify game files, saves, anti-cheat components or online behavior.

## Performance diagnostic follow-up (0.1.4)

0.1.3 exposed a severe user-reported idle performance regression once the offline gate opened. Its heartbeat corroborates the slowdown but provides no function timings. Optional-DLL checks now use constant names and raw Win32 handles, avoiding allocation/error construction for ordinary missing modules. A game-task atomic caches only the conflict boolean for item queries; the activation callback refreshes it. Online and lobby/protocol state are still checked live. A newly loaded conflicting DLL may therefore take until the next task to affect item queries; commit checks it directly.

After validating the manager against WorldChrMan, a snapshot can return the unmodified native action when there is no local group entry, no alive-summon flag and no native active state. Any retained local group takes the full original validation path. No actor pointer is cached and no health write validation is removed. Debug-only timers aggregate counts, elapsed time and maxima for each hook and major section once per ten seconds. They include nested/native time and scheduling delays; they are diagnostic evidence, not a profiler proving CPU attribution. Performance improvement is pending a user-run test.

### 0.1.5 follow-up

The 0.1.4 user-run timing data localizes the remaining idle delay to snapshot (about 19â€“20 ms per frame over three calls). Version 0.1.5 checks non-null/alignment and current WorldChrMan ownership before reading the manager and considering the idle shortcut; the native state routine already used this same manager in the synchronous callback. The VirtualQuery-based manager probe now follows the idle shortcut, and all actor/health/write probes remain on the full path. No lifetime cache is introduced. New timings distinguish identity lookup, idle/full path counts and readable-probe cost. This change is compiled but awaits user runtime validation, including future active-Spirit performance.

## Deliberately excluded upstream features

No 500-spirit patch, clearing of live-summon eligibility flags, forced activation-range flag, arbitrary gate state -1 bypass, multi-Ash group retention, follower warps, low-FP free dismissal override, additional cost scaling, damage modifiers, synthetic SpEffects, HUD editing, boss-survival override, or native clear-owner-group interception.

# 0.1.11 staged resummon timeline diagnostics

The debug GameMan observer samples scalar manager/player state no more frequently than 250 ms and logs only changed samples. Manager and player are reacquired from the current WorldChrMan; local groups are bounded and inspected without dereferencing group actor pointers. The observer stores no addresses. Player FP comes from the existing query-mode health-module validation. Timer values are rounded up to seconds to avoid frame-by-frame churn. A packed atomic retains the latest native/adapter item-query result pair, not a queue or action to execute. Callback entry and native return have separate timestamped messages. No new native calls or page probes are introduced by this observer; it cannot prove sub-sample transitions or button timing. Gameplay and resource gates are unchanged.

# 0.1.10 memory-query optimization

VirtualQuery results are now held in a fixed 16-entry stack cache for one validation pass, with uncached queries when capacity is exceeded. Each entry contains the returned region extent and read/write suitability; an object must fit entirely inside one committed, non-guard/non-noaccess region as before. Overflow and short query results reject. Snapshot reads use their own local cache, which is dropped before returning. The healing write pass creates a separate empty cache after native lookups, obtains fresh region metadata, and checks unchanged current/max HP for all targets before writing. A single query supplies both readability and writability. There are no native calls or allocations between this final validation loop and the HP writes. This still relies on the existing serialized-callback/lifetime assumption; it is not a memory pin or a solution for arbitrary concurrent deallocation. No cache survives a callback. Debug metrics record actual OS query time and cache-hit counts. Live improvement remains unverified until user testing.

# 0.1.9 registry correction

Current behavior supersedes the earlier active/local enum assumptions below. The pinned native executable uses load=4 in its character load gate and writes update=4 on the enabled branch. The adapter admits only that pair, requires the actor's chr_set_entry to point back to its matched live registry entry, and continues to validate handle resolution, module owner, health bounds, death and group ownership. The raw update byte is not used as proof of local/network ownership. Exact signatures at the native gate/writers are checked before installation without modifying those instructions. Evidence and remaining uncertainty are documented in `evidence/registry-state-0.1.9.md`. Downstream `handle_reject`, `health_reject`, `target_validated`, and `snapshot_resolved` metrics distinguish registry success from a complete valid snapshot.

# 0.1.7 diagnostic note (historical)

0.1.8 startup follow-up: after supported-executable and system-init checks, the DLL startup worker retries CSTaskImp::instance every 50 ms within a 30-second deadline. The pinned from-singleton dependency conflates null and missing instances as None, so FromStatic reports NotFound for both; the upstream wait helper aborts on that result. The local retry logs the precise error at five-second intervals and records success/timeout. No hooks or parameter writes are installed while waiting. This does not change registry validation or healing behavior, and does not recover a permanently incomplete singleton discovery map; such failures time out safely.

Registry validation keeps the 0.1.6 predicates unchanged. Debug logging now distinguishes invalid pointer shape, oversized capacity, invalid storage shape, missing pointer membership, inactive load state, non-local update state, and success. Each outcome retains only a packed scalar sample (capacity, occupied slots examined, and matched raw state bytes). No actor addresses persist. Counters and samples are emitted with the existing ten-second timing report, without adding page probes or actor reads. A sample with `matched_states=0` has no observed load/update bytes; values of 255 in that case are sentinels. Samples are observational diagnostics, not a coherent transaction across concurrent callbacks. Healing remains blocked until the observed registry rejection is understood.


## 0.1.12: native-ready groups awaiting cleanup

The 0.1.11 timeline exposed native/adapter state (0,-1) while all local slots were exiting, followed by (0,0) only after their removal. Applying live-actor health validation to those slots unnecessarily blocked native readiness. After validating manager/player ownership and the exact active goods/effect family, the new branch recognizes the tightly bounded metadata combination described in VALIDATION.md and returns Vanilla with no healing targets. Query and cost hooks preserve the original state 0; commit verifies its actual effect argument and invokes the unchanged native summon routine once. A `retiring_slots` scalar carried within the synchronous snapshot identifies this path for logging; no actor references persist across callbacks.

Registry load/update 2/2 and 4/2 are admitted only as metadata of a retiring member, never as permission to access that actor or its health. Missing membership or an unknown state cannot be interpreted as death. Both native readiness and all-member exit checks must hold; a single active, foreign, remote, warping or unrecognized member prevents this exception. Manager activation-range is not forced or used as a fresh-spawn gate: observed native-ready no-summon states have that field false, and native item/area eligibility remains responsible for fresh summons. No pending requests are replayed or scheduled by the mod.

The engine still owns removal of retired entries and acceptance/spawning of the replacement. This change removes the demonstrated adapter rejection; it cannot guarantee that the engine itself has no additional asynchronous wait. Native lifecycle interactions, including replacement survival after old entries are removed, require user-run validation. Earlier statements that every retained group necessarily takes live health validation are superseded by this narrow exception.