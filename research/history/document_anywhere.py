from pathlib import Path
root=Path('outputs/SpiritAshResummon')
def read(name):
 p=root/name
 try:return p.read_text(encoding='utf-8-sig')
 except UnicodeDecodeError:return p.read_text(encoding='cp1252')
def write(name,s):(root/name).write_text(s,encoding='utf-8',newline='\n')
s=read('README.md').replace('# SpiritAshResummon 0.3.1 — four individual Spirit bars','# SpiritAshResummon 0.4.0 — unrestricted summon zones',1)
intro='''
0.4.0 adds optional summoning outside vanilla Spirit Ash zones, including cleared boss areas. The local 0.4.0 package enables it with `[SpiritAsh] PreserveSummoningRestrictions = false`. Set it back to `true` and restart to restore vanilla zone rules. Missing INI/default configuration still preserves zones. Normal costs, animations, offline-only operation, one group per unique Ash, healing on reuse and the compact four-entity HUD remain in place.

This uses runtime monument parameter rules plus a nearest-monument fallback and player-relative spawn origin. It is compiled but not gameplay-validated. Native collision/navigation placement, scripted item-use locks (including potentially hub/cutscene restrictions), rest/death/loading cleanup and the 500-actor technical ceiling remain. This is not a guarantee that a summon can physically spawn on every ledge or in every scripted scene. See VALIDATION.md for the first Church of Elleh test.
'''
s=s.replace('\n\n**0.3.1', '\n'+intro+'\n**0.3.1',1)
s=s.replace('The four-row build is compiled and staged, not yet gameplay-tested.','The user reports 0.3.1 working well with no issues in the new UI; detailed 0.4.0 gameplay validation remains pending.')
s=s.replace('- Normal locations, animation, scaling, resource requirements and native area/death/boss cleanup remain required. No resource subtraction is implemented in the DLL.','- With PreserveSummoningRestrictions=false, ordinary monument location/region and boss-clear restrictions are removed. Animation, scaling, resource requirements and native rest/death/loading cleanup remain. No resource subtraction is implemented in the DLL.')
s=s.replace('- No summon-anywhere, damage/defense/base-HP changes, passive regeneration, HUD changes, follower warps, save editing or regulation.bin editing.','- No damage/defense/base-HP changes, passive regeneration, custom follower warps, save editing or regulation.bin editing.')
write('README.md',s)
s=read('IMPLEMENTATION.md');s=s.replace('# Implementation — 0.3.1','# Implementation — 0.4.0',1)
intro='''
## 0.4.0: user-authorized unrestricted zones

The user explicitly superseded the original vanilla-area restriction after accepting the four-row HUD. `Config.anywhere` is the inverse of the existing `PreserveSummoningRestrictions` setting. Costs remain mandatory. Default/no-INI behavior preserves zones; the prepared local package sets preservation false.

`runtime.rs::update_stones` now captures row address plus five originals per BuddyStoneParam. Anywhere mode applies activate_range=65535, overwrite_return_range=-1, overwrite_activate_region_entity_id=0 and eliminate_target_entity_id=0; the independent resummon switch still controls summoned_event_flag_id=0. These are the rules from upstream extras.rs::update_buddy_stone_rules. No disk parameter files change. Disable/fault restoration compares each current field to our written value and validates the row address. Table readiness/repository/anchor checks gate the readiness flag and reset the tile cache. Parameters loaded before a transition can retain native cached data until restart/reload; live setting toggles are not supported.

`anywhere.rs` adapts upstream stone_tile/nearest_stone/apply_stone_fallback. It prefers native nonzero monuments; when none is loaded it supplies an actual captured parameter row ID. Selection is cached by current world/player/manager identity and map tile, including 2x2/4x4 overworld blocks. Same-area nearest IDs are preferred; if an area has none, a deterministic real row is used. No entity pointer is manufactured. This cross-area fallback extends the upstream last-stone fallback and especially needs gameplay validation in dungeons/hubs. Cache entries contain scalar identities, not dereferenceable retained actors.

The helper runs in the existing game task and at the outer item query/cost/eligibility/activation callbacks, before actor snapshots. This covers native monument searches between tasks without adding hooks or rewriting native state return codes. It requires the current world manager, live player, valid finite block coordinates, applied params and existing offline/version/conflict gates. Adapter range checks admit the configured anywhere mode; native collision, action/cooldown and lifecycle checks still apply. It does not blindly clear disappearance flags, revive entities or erase groups. The used-monument lock is released only when no local native entries remain, no live-summon flag or pending request exists and cooldown is over, avoiding the upstream raw-HP scan of reviving skeletons.

At accepted fresh summon calls and pending-request task updates, the spawn origin is synchronized to the current player's finite Havok position using typed module pointers. This prevents a remote fallback monument supplying the location. Native spawn placement still decides whether terrain is suitable. There are no new per-frame VirtualQuery probes, no DLL hook addresses added, and no changes to healing/cost calculation or HUD files. Existing guards/version signatures remain pinned to WW 2.7.2.0. Engine-owned pointer access retains the project's serialized game-callback lifetime assumption; arbitrary concurrent game-state mutation is not supported.

All older claims below about excluding area-rule changes describe versions through 0.3.1. The 0.4.0 section takes precedence. Native scene scripts may still prohibit item use or remove entities; those are untested, not globally bypassed. Rest, death, map unload and scripted cleanup are intentionally preserved.
'''
s=s.replace('\n\n## 0.3.1','\n'+intro+'\n## 0.3.1',1);write('IMPLEMENTATION.md',s)
s=read('VALIDATION.md');intro='''# 0.4.0 validation — unrestricted zones (2026-10-29)

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

''';write('VALIDATION.md',intro+s)
s=read('CHANGELOG.md');write('CHANGELOG.md','''# 0.4.0 — unrestricted summon-zone prototype

- User authorized removal of vanilla Spirit Ash zone restrictions after reporting the 0.3.1 UI working well.
- Added optional expanded monument range, no activation-region requirement, no return-distance dismissal and no monument boss-elimination target.
- Added cached nearest-monument fallback and current-player spawn-origin synchronization. Preserved rest/death/loading cleanup, costs, one group per Ash and four individual HUD bars.
- Existing PreserveSummoningRestrictions=false now selects unrestricted mode; true/default restores prior zone policy. RequireNormalResourceCost=false remains rejected.
- Preserved upstream notices and source backup. Release compiled; no automated/gameplay tests run.

---

'''+s)
s=read('UPSTREAM.md');write('UPSTREAM.md',s.replace('# Attribution and provenance','# Attribution and provenance\n\n0.4.0 additionally adapts `extras.rs::update_buddy_stone_rules`, `stone_tile`, `nearest_stone`, `apply_stone_fallback` and the used-monument release concept, plus `lib.rs::player_havok_position` / `sync_spawn_origin_for_request`, from the same pinned ArialSenki commit. Changes add conservative local-entry lifetime checks, map/owner cache validation, conditional field restoration and current adapter integration. Original MIT/Apache-2.0 notices apply.',1))
s=read('START-HERE.txt');write('START-HERE.txt','''0.4.0: UNRESTRICTED ZONE PROTOTYPE
The prepared local INI sets PreserveSummoningRestrictions=false.
Start with the Church of Elleh check in VALIDATION.md. User performs all gameplay tests.
Normal costs, one group per Ash and four individual HUD rows remain.
Set PreserveSummoningRestrictions=true and restart to restore normal zones.
See the install manifest for installed version/hash; older instructions below are historical.

'''+s)
