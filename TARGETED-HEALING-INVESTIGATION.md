# 2026-10-30 impact prototype runtime result

The user purchased and memorized Spirit Mend, then performed three successful impacts with Lusat's+10. Independent HUD snapshots corroborate25%-maxHP healing and clamping on separate wolves;8FP per cast matches the staff's1.5x multiplier on base5. User reports no FPS drops or animation oddities. No change is indicated for the successful impact path. Next development remains cast preflight/no-charge rejection and a cast-time target latch, before claiming the final targeting/resource contract or enabling duplicate same-Ash groups. See VALIDATION.md for precise evidence and coverage limits.

---

# 2026-10-30: Swift Glintstone Shard template implemented in development0.5.0

User chose healing on projectile impact. The cloned spell, isolated assets and native hit-effect adapter now compile; see SPIRIT-MEND.md for exact fields, evidence and limitations. Existing0.4.2 remains installed. This does not yet fulfill no-charge invalid casts or cast-time target latching. Custom icons remain deferred.

---

# 0.4.2 follow-up — target lifetime baseline confirmed

User reports 60 FPS with PS5 DualSense input. The grace test clears the selected old wolf and all old group entries, then successfully selects fresh wolf handles after resummoning (55 FP). See latest VALIDATION.md and evidence/0.4.2-grace-lockon-review.json. Core native mixed targeting plus this cleanup/reacquisition path need no repeat now. Proceed next with the planned Intelligence sorcery's native casting and cost path, preserving per-cast selected-owned-target validation. Combat-death, range/occlusion and other Ash coverage remain pending.

---

# 0.4.2 runtime checkpoint — native mixed targeting confirmed

The user-operated session selected each of three owned wolves, cycled among them, and switched between wolves and an enemy in both directions. The selected-handle observer corroborates the report, including sustained wolf selection and clearing on unlock. No camera correction is indicated by this session. See the newest VALIDATION.md entry and evidence/0.4.2-friendly-lockon-review.json. Target lifecycle/range/occlusion and other Ash types remain pending; this does not establish spell targeting or cost commit behavior. Next development remains the selected-Spirit Intelligence sorcery and, once healing is separated, repeated same-Ash groups.

---

# 0.4.2 implementation checkpoint

The narrow camera relation call at RVA 0x7179d6 now admits current living local Spirit Ash actors while preserving original enemy results. Native candidate geometry, visibility, scoring and directional switching remain in place. This is the first friendly acquisition/cycling prototype, not proof those native paths accept Spirits all the way through. See friendly_lockon.rs, IMPLEMENTATION.md and the first manual procedure in VALIDATION.md. Compiled; gameplay pending. The selected-handle baseline from 0.4.1 is retained to distinguish candidate admission from actual Spirit selection. No spell, custom item or duplicate same-Ash groups added yet.

---

# 0.4.1 implementation progress

Selected-target storage is now established at source/static level: PlayerIns.locked_on_enemy is a typed FieldInsHandle at+0x6b0, with native writes matching the type, and ChrIns.is_locked_on at+0xc9 is written by the native toggle routine. This resolves the earlier unknown about reading the selected actor. A compiled read-only diagnostic is now available for user correlation of lock/cycle events. Friendly candidate admission/retention, spell transactions and duplicate summon mode remain future work. No executable camera patches or game tests were performed.

---

# Design revision: Intelligence sorcery (2026-10-29)

The user now prefers a real spell suited to an Intelligence sorcery build. This supersedes the reusable quick-slot healing tool/Spiritmending Bell proposal below. The prior research is retained as history, not the current implementation specification.

Current direction:
- A staff-cast sorcery, memorized in the normal spell interface, with an Intelligence requirement and no Faith or Arcane requirement. Proposed one memory slot; numerical INT requirement and FP cost remain tunable/unselected.
- Working title: Spirit Mend. Original blue/silver spectral visual and custom sorcery icon; no bell-shaped icon commitment or new image generation yet.
- Retain requested native mixed enemy/owned-Spirit lock-on and directional cycling. Cast heals one selected owned living Spirit by25% of its maxHP, clamped to max. Normal Ash use instead summons another complete group at its normal cost.
- INT gates access and the staff is the casting tool. For the first prototype, healing remains fixed25%; do not silently multiply it by Sorcery Scaling or introduce new INT scaling. Scaling can be a separate later balance decision.
- No self-healing, ally/player healing or area healing is implied by the current scope. User's comparison to Faith healing is motivation, not confirmation that the spell should additionally heal its caster.
- Enemy/no target/full-health/invalid ownership rejects without healing. Free failed casts remain a design target, with native spell-cost commit timing to be researched. Distinguish invalid at cast start from target dying or changing state mid-cast; no unverified refund or interrupted-cast promise.

Source verification: vendored generated.rs MAGIC_PARAM_ST includes requirement_intellect, requirement_faith, requirement_luck, slot_length, mp, stamina, behavior_id, ez_state_behavior_type, icon_id, ref_category1..10 and ref_id1..10. These establish the relevant spell metadata controls, not the semantics of a chosen animation/behavior template. Select a staff sorcery template from the installed parameters before choosing exact values.

Revised asset and DLL work:
1. A new collision-checked MagicParam row, sorcery category/catalyst compatibility and casting behavior; acquisition Goods/merchant/text wiring where required by the installed spell-learning flow. A learnable spell is still an inventory acquisition and can be saved normally by the game.
2. Spell-specific FMG name/description/icon wiring rather than only Goods text and a quick-slot tool. Use the appropriate spell icon atlas mapping, retaining isolated Mod Engine assets and merge metadata.
3. Reuse the validated per-actor HP operation but research the spell cast/effect/cost path separately. The current Ash item hooks do NOT automatically provide correct spell charging, cancellation, animation timing or input handling. A marker effect dispatch remains a possibility, not a verified solution.
4. Resolve and latch the selected Spirit handle at the defined cast stage, validate again at effect time, and never retarget to another actor silently. Keep range/LOS and engine lifecycle safeguards. Friendly-target acquisition/maintenance/cycling remains the first diagnostic milestone.
5. Use existing compatible staff animation/VFX for the initial prototype, then add visual polish and generated icon. No animation, numeric FP cost, INT threshold, or spell ID is final yet.

Installed0.4.0 and source gameplay behavior remain unchanged. This revision records the design preference; no build, tests, game launch or runtime mutation performed.

---

# Duplicate Ashes and Targeted Spirit Healing — investigation

Date: 2026-10-29. Installed baseline: 0.4.0. This is a design/investigation deliverable, not a new playable build. No installed files, game memory, saves, inventory, or gameplay settings changed. No tests executed.

## Requested interaction

The user explicitly wants multiple groups of the same Ash. The healing item must use the game's lock-on system to select one individual Spirit, retaining target cycling through both owned Spirits and normally eligible enemies. This supersedes suggestions of all-Spirit or per-Ash healing.

- Normal Spirit Ash use: request another complete group, even if members of that type are alive; normal FP/HP summon cost once per successful activation. Existing members retain their health/equipment/AI. Summoning again is not resurrection of an old member.
- Reusable healing item (working name: Spiritmending Bell): heal only the selected living, local-player-owned Spirit by floor(maxHP * 0.25), capped at maxHP. It is not consumed. No automatic nearest-Spirit fallback when targeting an enemy or nothing.
- Enemy, unrelated NPC, remote summon, player, dead/unloading actor, full-health Spirit or insufficient resource: no heal and no charge. Failed casts must not silently select another Spirit.
- Continue four compact individual HUD bars, with overflow remaining active. Targetability must not depend on whether a Spirit has a visible compact HUD row.
- Existing offline/session/version gates, unrestricted-area configuration, normal rest/death/loading cleanup and 500 Spirit actor-slot ceiling remain. This ceiling is not an assertion of practical performance at 500 actors.
- Healing resource cost is not chosen yet. A fixed configurable FP cost is simplest for one universal item. Charging the original Ash's cost requires reliably retaining which Goods/upgrade variant produced each actor. No free healing or particular cost is implicitly approved here.

## Confirmed source findings

1. `src/multi.rs::snapshot_multi` intentionally returns Heal/Reject when selected-family members survive. `policy::multi_action` is the policy restriction, not a physical unique-type registry. Changing this to AddGroup in duplicate mode can reuse the scoped native DoSummon call and owner-clear guard already used for coexistence.
2. Existing pending tokens identify world/manager/owner/Goods/effect. They are not cast-instance IDs. Repeated same-effect requests require a serialized accepted-request token plus a baseline of actor handles, so an existing wolf cannot be mistaken for successful completion of the next cast. Upstream uses live counts; handle differences are preferable when another member dies during a spawn.
3. Upstream `MultipleSpiritAshes` pinned commit 7d2e12c0fd89fedd31ce7d57ee546cf13de2e9f2 implements `repeat_same_ash`, `arm_direct_summon_guard` and `repeat_groups_before`. Its source supports feasibility but is not proof on our installed build. Do not import broad dismissal resets or raw reads from dying actors.
4. The selected-family loop currently blocks on unresolved dead/reviving members. Duplicate mode must stop treating an old body's death as a reason to forbid a separately purchased new group. Still require native lifecycle safety, valid live ownership and free capacity, and do not mutate old entries.
5. Native `SummonBuddyManager.groups` is owner -> list of actor entries, not a reliable per-cast grouping abstraction. BuddyParam identifies a kind, not a distinct summon instance. For this single-target design, fresh engine handles + owner/world identity suffice for healing; cast records remain useful for diagnostics/cost attribution.
6. `src/hud.rs::MemberKey` already uses base effect plus full FieldInsHandle. Additional wolves can receive separate ordinals without grouping them into one bar. `hud_snapshot.rs` enumerates all owned registered Spirits, and the render limit is independent.
7. Vendored EquipParamGoods includes `is_consume`, `is_equip`, `max_num`, `icon_id`, `goods_use_anim`, `ref_category`, `ref_id_default`, `consume_mp`, `consume_hp`, `use_limit_summon_buddy`, offline/multiplayer flags and dialog fields. These are sufficient building blocks for a reusable, equippable item definition. Actual template values/unused IDs must be inspected before creating a patch.
8. SoloParamRepository/ParamFile expose existing-row lookup/iteration/mutation, not an established safe insertion API. Adding a genuinely new runtime row is substantially more involved than editing an existing row. Existing HUD message lookup is a reader, not a custom item registration facility.
9. The local test Mod Engine 2 profile has `mods=[]` and loads only the DLL. Asset-based custom item delivery would add an isolated mod directory to that profile; the game's original files need not be replaced.

## Lock-on investigation and main unknown

Vendored bindings expose action flags `disable_lock_on`, `disable_ability_to_lock_on`, a lock-on position, camera structures and AI/bullet targeting search slots. None establishes a complete player-camera candidate filter or verified selected-character handle. AI Friend search slots must not be confused with player lock-on cycling.

Reference inspected: https://github.com/SchuhBaum/FreeLockOnCamera at cc3317740b266c12484d22ed7f44bd137603b977 (MIT, Copyright 2023 SchuhBaum). Source inspected only; no code copied into the shipping mod. It modifies camera range, scoring and target switching, but does not implement friendly Spirit eligibility. In particular, its optional switch changes can disable/replace normal switching and are not appropriate to import wholesale.

REA `open_binary` confirmed installed x64 PE SHA256 f9118fea2cfd8fa3901e8060bfadb1552cb2471771ded8a5b662c4e298ffb328. Deep decompilation providers remain unavailable (Ghidra unconfigured; no usable native provider selected). Ordinary Python/pefile byte searches, not REA decompilation, found one executable-section match each:

| Reference-source lead | RVA |
| --- | --- |
| Camera range reset | 0x7165bb |
| Candidate score | 0x71841e |
| Target switching prologue | 0x719a60 |
| Candidate scoring after acquisition | 0x718491 |

Evidence: `evidence/targeted-healing-lockon-leads.json`; reproducible script in workspace `work/inspect_lockon_leads.py`. Semantic names are inferred from the reference source. No function boundaries were recovered by that script's PE exception-table lookup. These are investigation leads, not validated patch sites or proof of friendly-target support.

Required approach: find the player camera's ownership/hostility admission check and the corresponding maintain-lock and cycle paths. Preserve native range, line-of-sight, visibility, angle, death and action checks. Admit an additional candidate only when it resolves to a validated Spirit owned by the main player. Keep normal eligible enemies in the same native candidate/cycle order. Do not globally force the hostility predicate true, change team/NpcParam allegiance, inject fake enemies, or replace cycling with an unrelated selector.

Capture the selected actor's full handle when the heal activation is accepted, then re-resolve/revalidate it at effect time. Switching targets while the animation is playing should not redirect the already accepted heal. World transition, death, disappearance, range/LOS loss, or handle-reuse ambiguity must cancel safely. Targeted offensive aiming can naturally point at a friendly actor when locked; this must not enable friendly damage or modify AI aggression. Selected-target feedback must work for Spirits beyond the four HUD rows.

## Real custom item: recommended hybrid package

Use a DLL for validated targeting/healing plus a small Mod Engine asset package for a real new item, rather than silently replacing a vanilla item or growing live param tables before understanding their ownership/indexes.

| Component | Work |
| --- | --- |
| Item row | New collision-checked EquipParamGoods ID cloned from a suitable reusable tool; nonconsumable, quick-slot/pouch eligible, native animation, defined cost, offline-only policy |
| Activation | Dedicated effect/behavior identity, handled before the existing summon-family lookup; never reaches the normal spawn or dismissal branch |
| Text | New Goods name/summary/description entries in applicable FMG banks/locales |
| Acquisition | New shop entry (e.g. Kale) or another normal in-game acquisition method; one copy, no repeated auto-grant on load |
| Icon | New icon ID, atlas texture entry and matching layout; hi/lo assets as required by installed files |
| Distribution | Deterministic changes/merge recipe against the user's regulation/text/icon versions, isolated loader directory, originals retained |

Smithbox supports parameter/text editing and texture inspection: https://github.com/vawser/Smithbox . Its ER item icon configuration identifies 01_common and SB_Icon atlas sets with MENU_ItemIcon_ prefix: https://raw.githubusercontent.com/vawser/Smithbox/main/src/Smithbox.Data/Assets/PARAM/ER/Icon%20Configurations/Item.json . This matches relevant archives already extracted on E:\Elden Ring Unpacked Files (menu/hi and low common texture/layout bundles and engus item message bundles). We have not yet reserved new IDs or proved an unused atlas rectangle; never overwrite an occupied slot based on appearance alone.

A patched regulation.bin/texture/message bundle is a merge dependency with other mods. The deliverable should retain a small reproducible change manifest, not assume wholesale replacement is compatible. Loading a real new item also means its ID is normally saved by the game after acquisition: no external save editing is required, but uninstall/discard behavior needs testing. Do not call a persistent new inventory item save-neutral. The original request prohibits direct save modification; acquisition would use normal game mechanics, with this persistence documented.

## Healing activation and exactly-once costs

The existing item cost hooks and main-player effect interception are promising reuse points, not a proven generic custom-tool handler. The retained native call-site disassembly at RVA0x3fb268 checks effect state value0x165 before invoking DoSummon. A separate, collision-checked marker effect could potentially route through this hook without spawning, but its full parameter semantics and side effects must be verified. Current snapshot_multi rejects items lacking a unique summon family; the healing item needs its own earlier dispatch and must never borrow a real Ash identity.

First prove a diagnostic accepted/rejected activation with a vanilla placeholder icon in an isolated package. Then link native affordability and charge to a single action token with selected handle, world, item and cost. No manually deducted FP layered on top of engine charging. No per-frame HP writes or persistent raw actor pointers. Native cost commit ordering must be established for death/full-health/range changes between initial use and effect; rejecting only at the effect callback may be too late to prevent charging. Do not promise free failed casts until that transaction is observed.

For HP modification, reuse the validated/clamped per-actor operation already exercised by the user. A native percentage-heal SpEffect remains an alternative, but it must not introduce regeneration, revive members or heal the player. No additional SpEffect investigation is needed merely to rebrand the existing working healing calculation.

## Icon direction (concept only)

Working name: Spiritmending Bell. A worn silver handbell with a small ivory handle, pale blue spectral wisps and a subtle warm light within; isolated transparent background, strong silhouette, no lettering/frame, readable at small inventory scale. This visually distinguishes healing from the existing Ash icons while using the bell animation concept.

Image generation can provide the transparent artwork. It does not register an icon in the game: export at the actual atlas dimensions, encode the appropriate DDS format/mips, allocate a collision-free icon mapping, and repack the texture/layout files. Verify small-size readability before packing. Final icon generation is deferred until the item identity and loading path are chosen; no asset has been generated or installed.

## Smallest useful development sequence

1. Read-only targeting diagnostic: identify player lock-on selected handle and candidate/maintain/cycle filters on this exact executable. Prove mixed enemy/owned-Spirit cycling through user testing, with no allegiance changes. This is the highest-risk requirement and should precede polished assets.
2. Add opt-in duplicate-mode spawn policy, request-instance accounting and handle-delta diagnostics; preserve all groups and normal costs. Wolves twice should produce six, no reset of the first three,55FP per cast. Keep legacy mode available until the healing item works.
3. Build the actual reusable tool with temporary icon/text and verified preflight/commit/charge behavior; heal only the latched eligible Spirit, never enemies or all copies of its Ash.
4. Add final generated icon, localization and normal acquisition; retain merge/rollback metadata.
5. User tests keyboard/mouse and controller cycling, initial acquisition, lock retention, partial group death, skeleton revival, selected target dying mid-cast, enemies/no target/full HP/insufficient FP, more than four bodies, duplicate upgraded Ashes, scene transitions, HP-cost summons, performance and uninstall behavior.

No new release was built or installed during this investigation. There is a feasible duplicate/custom-item route; mixed friendly/enemy lock-on is not yet proven. The next implementation should start with that diagnostic rather than treating four byte matches as authorization to patch camera behavior.
