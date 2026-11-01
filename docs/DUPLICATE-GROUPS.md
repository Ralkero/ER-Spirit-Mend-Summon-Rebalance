> Active profile update, 2026-10-31: the user requested repeat-Ash spawning.
> Duplicate groups are now enabled in the installed 0.6.1-solo.5 INI with a cap
> of three groups per Ash. Normal costs apply; reuse healing is superseded.
> No DLL/assets changed. Runtime acceptance is pending. Earlier staging and
> default-off statements below describe the original development milestone.

# Duplicate same-Ash groups: 0.6.0-dev.1

Implemented and compiled; NOT automated-tested or gameplay-validated. Separate development branch and package, not installed. Stabilization commit `6ac82d2` is available independently for testing the 0.5.1 correction without duplicate functionality.

## Configuration

Add these keys to a COPY of your profile INI only after choosing to test the prototype:

```ini
[SpiritAsh]
AllowMultipleAshTypes = true
AllowDuplicateAshGroups = true
MaxGroupsPerAsh = 3
```

Default duplicates=false preserves prior heal-on-reuse. MaxGroupsPerAsh accepts integers1..8, defaults3, and counts pending and partially surviving groups. Duplicate mode requires multiple-type infrastructure; contradictory configuration disables startup with a diagnostic. It takes precedence over HealOnReuse: Ash activation only requests a new group, never heals. Spirit Mend remains the single-actor healing route, with unchanged25% max HP,5 baseFP,12INT and native modifiers. Enabling duplicates does not automatically enable the spell, targeting, HUD or summon-anywhere. Existing prototype and installed INIs are unchanged. Settings load at startup, not dynamically.

## Architecture

`groups.rs` is pure scalar accounting; `duplicate_groups.rs` adapts native ownership/registry data. Every accepted request reserves a process-monotonic G id and normalized family. Only one request may be pending across all families. Its expected actor-entry count is the number of unique BuddyParam family members. Mounts also count toward the existing free-body-slot preflight, although they are not assumed to create extra manager group entries.

The adapter snapshots owner entries at most10Hz, with a fresh pass at activation/reservation. Every entry must belong to the current main player's owner list, be local and have an unambiguous recognized family and occupied native registry slot. A full actor handle is read only after the existing active4/4, registry backlink and exact handle re-resolution gates. New handles go only into the serialized pending instance of the same family; unexpected actors block new requests rather than being guessed into groups. World/manager identity and generation-bearing player handle scope the book. No actor pointer survives a callback.

Known members in inactive slots remain counted without reading their HP or actor storage. This retains skeletal temporary-death membership. Full handle changes remove old identity; removed entries prune only their own members. Each first-observed member receives a stable ordinal. Observed deaths during loading do not erase the number already assigned. Completion requires every expected member to have been observed and the native request to finish. No timer invents a successful/failed spawn. Dismissal, absent/dead player or changed context resets accounting; normal whole-group removal frees its limit position. All actual destruction remains native or the prior narrowly guarded inactive cleanup.

`multi.rs` selects AddGroup in duplicate mode even with living same-family actors. The existing synchronous native state scope and exact-request owner-clear suppression preserve old entries. The existing native FP/HP cost path is reused; the pending token additionally checks full player handle. Native cooldown, current request, area policy, capacity, placement, requirement and offline gates remain. No new hook, native function signature, health write, slot eviction, resource subtraction or forced spawn is introduced.

The four existing HUD rows keep their layout and stable display order. Known instance labels append `G<id> #<ordinal>`; unknown/unresolved metadata falls back to the old actor label rather than fabricating membership. Lock-on and Spirit Mend continue validating each actor from native ownership independently of its display row or bookkeeping group. Group sampling is independent of HUD enablement. `duplicate_sample` is aggregate timing; debug summaries are bounded to16 groups every10seconds. No per-frame page probes.

## Deliberate limits requiring user evidence

- Native same-family acceptance, clear/cost ordering and manager-entry counts are not runtime-proven by compilation. Begin with wolves and two groups.
- An accepted request that never yields all expected identities stays reserved and blocks further summons. This is a conservative failure mode, not a timer-based retry. Rest may clear it through native dismissal; title/Continue or restart resets it. Capture logs before recovery. A member disappearing before it was ever observable can cause this case.
- Mounted/special Ashes may have different native manager-entry cardinality and need dedicated acceptance. Ambiguous layouts fail closed. There is no claim of universal modded-Ash support.
- Cleanup of a completely dead group may retain its count until native entries actually disappear. A temporarily dead skeleton is intentionally counted.
- Sampling/label lengths and performance at six/nine wolves are unmeasured. The500-body registry is a technical ceiling, not a practical target.
- Partial list transitions, parameter mods changing family mappings and other mods spawning into the same owner list are unsupported/untested. No API or registry ownership is exposed to other mods.

## User acceptance in order

First complete the stabilization spell checks in VALIDATION.md with duplicates off. Then close the game, back up the profile, use the development DLL plus the existing0.5.1 assets, enable duplicates in the profile copy, and restart offline.

1. Gatefront, enough FP: summon wolves once (three), then again after normal recovery (six). Each use costs55FP once. First three keep HP/AI; reuse does not heal them. Note FPS and wait10seconds.
2. Third use gives nine; fourth is rejected with zero cost. Repeated rapid input must not bypass the limit. Stop here for log review if counts/costs differ.
3. Cast Spirit Mend at an injured wolf in each group, including one outside the four visible HUD rows. Only selected actor heals, correct native cost, no duplicated effect.
4. Add Militiamen. Their independent family limit must not be consumed by wolves. Test insufficient FP and invalid terrain: no accepted group or unintended resource charge.
5. Let one member of a group die: remaining group still counts. After the entire group retires, exactly one limit position becomes available. Partial skeletal death/revival must retain its group; no HP-zero-only removal.
6. Rest; title/Continue; player death; permitted area transition; boss transition. Old references must not block a fresh first summon, alter unrelated survivors or crash. Check normal area restrictions with preservation=true separately.
7. Later: upgraded variants share the family cap; HP-cost Ashes charge once and reject insufficientHP; mounted/special groups; HUD disabled. These are NOT accepted by wolf tests.
8. Restart with duplicates=false: one existing family heals on reuse again, full-health reuse costs nothing. The user chooses whether to retain the feature.

Logs: `Duplicate group reserved`, `Add-group native request`, `Preserved owner groups`, `Duplicate group snapshot` (id,family,current members,pending). Reservation alone is not spawn/charge evidence. Compare native entry counts, HUD handles and user FP/HP observations; the HUD only shows four actors. Report FPS, exact input count, terrain and any rest/reload during the sequence.

First-use query/cost calls retain the native idle result when there are no owner entries and bookkeeping proves no pending/unresolved instance. This preserves pre-queue native cost handling and the cheap idle path. Actual duplicate-mode activation still reserves through AddGroup, even for the first group.
