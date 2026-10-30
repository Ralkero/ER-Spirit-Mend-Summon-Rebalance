# Two Spirit Ash group health bars: investigation and implementation plan

## 0.3.1 user-directed scope update

The user now requests individual entity bars rather than group totals, maximum four, with the same compact layout. Implementation and manual validation guidance are recorded in IMPLEMENTATION.md and VALIDATION.md. The earlier two-group design below is historical. A compiled four-entity build is staged but has not been gameplay-tested.

## Implementation follow-up: 0.3.0 staged prototype

The extracted movie and current executable have now been statically inspected, and a two-row runtime prototype has been implemented and compiled. See IMPLEMENTATION.md and VALIDATION.md for current behavior and limits. Unlike the earlier proposal below, overflow uses the two oldest tracked groups, not selected-Ash cycling. Runtime recovery after the vanilla panel is hidden still requires restart with HUD disabled. No gameplay result is established.

Date: 2026-10-29. Investigation only; no HUD code installed or tested. User selected one bar per Ash group, not individual Spirit. User will evaluate 0.2.2 during normal gameplay; its resummon correction is not yet accepted as passed.

## Finding

A practical source-based route already exists in ArialSenki's MultipleSpiritAshes at commit `7d2e12c0fd89fedd31ce7d57ee546cf13de2e9f2`. The local checkout is clean and at that commit. It supplies both the game-facing row update and the UI layout parser; neither is included in SpiritAshResummon's current runtime. Reuse would retain the existing MIT / Apache-2.0 attribution.

This is more than a coordinate edit. Layout controls position and scale; an independent row-data path must supply a separate name and HP pair for each Ash family. Duplicating the native Buddy panel does not establish independent group binding. No installed game movie or live HUD was inspected in this investigation, and the upstream implementation is source evidence, not proof of behavior on this installation.

## Evidence and reusable pieces

| Source at pinned commit | Observation | Use in a two-row prototype |
|---|---|---|
| `src/hud_movie.rs`, constants and `apply_ash_rows` | Parses `menu/01_000_fe.gfx` as uncompressed GFX/SWF, identifies `Buddy` and `RemotePCTag*` placements by instance name, rewrites transforms | Reuse parser/checked transforms, reduce display scope to two rows |
| `src/hud_movie.rs::patch_hud_movie` | Hides Buddy; changes remote-player row template; upstream adds a seventh row | No seventh-row creation needed; require complete replacement readiness before suppressing Buddy |
| `src/hud_movie.rs::locate_scaleform_targets` / `install` | Finds Scaleform opener through RTTI; verifies MemoryFile accessor layouts; hooks OpenFile and substitutes process-lifetime bytes | Adapt to our version checks and patch transaction; reject conflicting hooks rather than automatically chaining |
| `src/hud.rs::tag_copy_detour` | Temporarily populates unused friendly tag entries immediately around the native copy into HUD values | Reuse timing concept; do not write tags from an arbitrary recurring task where native rebuild erases them |
| `src/hud.rs::collect_live_ash_groups` / `plan_rows` | Groups by summon effect, sums HP, adds member count; upstream can collapse overflow into a combined +N row | Replace actor access with our validation; two independently named groups, no combined overflow HP |
| Vendored `cs/fe_man.rs` | Seven typed `friendly_chr_tag_displays` entries and fields for names, HP, flags and handles | Binding exists already; exact layout and copy semantics still require installation-specific validation |

Pinned primary source links:
- [HUD row data and native copy hook](https://github.com/ArialSenki/MultipleSpiritAshes/blob/7d2e12c0fd89fedd31ce7d57ee546cf13de2e9f2/src/hud.rs)
- [Movie parser, transforms and loader hook](https://github.com/ArialSenki/MultipleSpiritAshes/blob/7d2e12c0fd89fedd31ce7d57ee546cf13de2e9f2/src/hud_movie.rs)
- [Version-specific native address table](https://github.com/ArialSenki/MultipleSpiritAshes/blob/7d2e12c0fd89fedd31ce7d57ee546cf13de2e9f2/src/lib.rs)

## Proposed first layout and semantics

Two vertically stacked rows under the player's status area on the left, away from quick slots and interaction prompts. Each row contains the localized Ash name, living member count and a combined HP bar. Example: `Lone Wolf Ashes x3` above `Skeletal Militiaman Ashes x2`. Actual screen placement must be adjusted from user screenshots; no exact final pixel placement is established here.

Start with upstream's 0.82 relative row scale and 45 HUD-unit vertical spacing as provisional values. Upstream stores positions in twips (20 per UI unit) and scale in 16.16 fixed point; these are not guaranteed monitor pixels. Its shifts are -91 horizontally and -350 vertically relative to loaded placements, scaled using Buddy's vertical scale. Do not assume those offsets fit every resolution, HUD scale or modified movie. Expose startup-only X/Y offsets, row spacing and scale if necessary; no GUI or live-reload system needed.

Recommended bar meaning: sum of current HP of validated surviving members divided by sum of those same members' maximum HP. Display survivor count so a death is visible even when the fraction rises after the dead member leaves the denominator. For example, two surviving full-health wolves show a full bar and x2, not the same state as three wolves. This is group status, not a promise that each member has the displayed percentage. Unknown/retiring members must never be dereferenced to preserve the bar. A temporary skeleton death is not proof of group expiry; retain its group identity through revival and handle an unavailable sample explicitly rather than manufacturing health.

Stable row order for the two-group case. For more than two active types, proposed policy is selected active Ash plus the oldest other active group, with stable slots where possible. If the selected item is not an active Ash, show the two oldest groups. This is a display limit only, not a summon limit. Do not copy upstream's combined +N health bar, which would stop the second row representing one unique Ash. Overflow selection is a design proposal, not yet implemented or user-tested.

## Integration work

1. Add an optional HUD feature independently of summoning/healing. New modules can separate pure row selection/aggregation, checked movie editing, and the native adapter. Keep feature failure from disabling the gameplay mod.
2. Use existing local-owner, Ash-family, registry-state, actor-backlink and handle validation. The upstream collector directly dereferences group actors and only filters remote entries; do not import it unchanged. Normalize upgrades using our unique-family policy. Do not infer Goods IDs from effect IDs for every Ash without validation; resolve names through the actual item mapping and cache localized strings.
3. Produce immutable scalar row snapshots on a suitable game task, at a bounded cadence (initial candidate 10 Hz). Never pass actor pointers to the render/copy hook. The copy hook should use ready row values, minimal locking, no actor traversal and no file logging. Do not add per-frame VirtualQuery probes; any lifecycle validation/probe strategy must be measured against this machine's earlier performance regression. Sampling cadence is a proposal, not a measured performance guarantee.
4. Adapt the friendly-tag copy hook. Upstream WW 2.7.2.0 uses call RVA 0x772fbd -> 0x775480 and optional message lookup 0x2671370. These are source-derived candidates, not independently checked addresses in this investigation. Verify signatures/layouts before enabling. Avoid blind raw copies of DLString ownership; use proper string assignment and restore only injected free entries.
5. Adapt the movie loader hook before the first HUD movie load. It is a second interception point beyond the tag-copy call; message lookup is a native call, not another detour. Preserve process-lifetime ownership of substituted bytes with bounded cache growth. No on-disk GFX, regulation.bin, executable or save edits are required by this approach.
6. Make movie changes transactional as a whole. Upstream treats hiding Buddy and laying out rows as independent successes, so it can hide Buddy even when row layout is unsupported. Reject incomplete layouts and leave the original bytes unchanged. Install/validate both hooks before allowing replacement. A runtime failure after the movie has already hidden Buddy still needs a deliberate visibility fallback; startup checks alone do not solve that. During the first diagnostic integration, retaining Buddy until the two-row path is established is safer than promising automatic recovery that does not exist yet.
7. Review shared templates and slot mapping. The remote tags share a sprite: hiding its multiplayer icons changes every instance using it, not just two. Moreover, native off-screen stacking and existing friendly-NPC occupants can consume positions. Merely changing ROW_COUNT to 2 is not sufficient. Preserve occupied native entries; determine packed-display ordering and either isolate a row template/slot route or temporarily suppress custom rows when the mapping cannot be kept separate. NPC ally coexistence remains an explicit investigation/test item.
8. Default to refusing unknown OpenFile detours. Upstream permits chaining an existing rel32 hook, but this project must not assume another UI mod's buffers, lifetime and ordering are compatible. Preserve vanilla UI when unsupported, with an informative diagnostic.

## Scope and alternatives

Recommended: reuse the game's own ally-row visuals through the existing source path. This avoids a new D3D overlay, new fonts/input handling and an entire replacement HUD asset. It adds a movie-loader detour and a friendly-tag copy detour to our current DLL, plus snapshot collection and configuration.

A static replacement GFX file could make a layout-only experiment simpler, but still needs native data routing for two groups, conflicts with other replacements, and packages a game asset. It is not the preferred distributable route. Directly cloning/binding another Buddy panel would preserve its exact appearance but needs additional native/UI binding research not supplied by the current source. A custom graphics overlay is feasible in principle but is a substantially broader rendering integration.

Overall assessment: feasible, with a useful existing foundation; moderate integration work rather than a simple INI edit. The unresolved work is safe data collection, failure behavior, occupied native tags and visual placement, not discovering a renderer from scratch.

## User-operated validation after a prototype exists

- Idle without summons: no extra rows, no performance change.
- One group then two: correct names/counts, independent bars, no overlap with normal HUD.
- Damage/heal only one group: only its bar changes; HP percentages agree with logged sums.
- Partial death, whole-group death, resummon and skeleton revival: counts and row lifetimes remain coherent, no stale HP reads.
- Three or more groups: deterministic two-row selection without affecting summon count.
- NPC allies, menus, HUD auto-hide, loading, resting, death and quit/continue: no displaced/overwritten native tags or stuck rows.
- Different resolutions/aspect ratios/HUD scales and long localized names: readable placement and clipping behavior.
- Missing/unsupported movie placements or hook conflicts: original UI preserved and gameplay mod remains usable.

No automated tests, game execution, HUD writes, extraction of game assets, new build or installation was performed for this investigation. The installed DLL stays at 0.2.2. No claim is made that the upstream HUD has been runtime-validated here.
