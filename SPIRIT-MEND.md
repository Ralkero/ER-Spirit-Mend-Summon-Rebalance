# Spirit Mend 0.5.1

A Swift Glintstone Shard-based Intelligence sorcery that heals one owned living Spirit on projectile impact by 25% of its maximum HP, capped at maximum. Original visuals, staff casting animation and icon remain. Base requirements: 12 INT, one slot, 5 FP, zero Faith/Arcane. Native equipment cost modifiers apply: the user's Lusat's Glintstone Staff+10 produced 8 FP per cast in 0.5.0, consistent with its 1.5x multiplier and native rounding.

Use the usual offline `tools/Launch-Test.ps1`, or its `Launch-SpiritMend.ps1` alias. Existing purchase/memorization carries over; no need to buy again. For a new character, Kale at Church of Elleh sells Spirit Mend for 300 runes (shop row100521). Installed local assets are under `dist/mod`; do not replace the game-root regulation. The full derived regulation is not automatically mergeable with other parameter mods. Only English message assets are prepared.

## Changes in this prototype

At projectile release, Spirit Mend requires a locked-on, living, injured Spirit belonging to the local player. No target, an enemy/unrelated NPC, a full-health Spirit, or a failed ownership/asset gate rejects the release before native projectile creation and FP/resource commitment. The casting animation may still play. Selection is captured when the shard is released, not when the button is first pressed.

Once released, each projectile retains its own target in engine launch data. Unlocking or selecting another target afterward must not change who can receive the heal. At impact, both actual receiver and captured handle must match, and the Spirit must still be owned, living and injured. No healing is transferred to another ally that intersects the shard. Native flight/collision/range still apply: this is not a guaranteed-hit spell.

A valid release spends its normal native FP cost even if it later misses, hits terrain, the target dies, or another heal fills the target first. There is no FP refund, direct FP write or custom cost calculation. Normal insufficient-FP/requirements handling remains native. Same-Ash reuse still heals the existing Ash group; duplicate groups remain deferred.

## User-operated tests

The DLL compiles, but 0.5.1 has not been gameplay-tested. All automated and gameplay test execution is left to you.

1. Launch the usual offline profile and load the test character. With ample FP, select Spirit Mend and attempt one cast with NO lock-on. Record FP before/after: it should remain unchanged and no shard should launch; animation may play.
2. Repeat once while locked on to an enemy, and once on a full-health wolf. Both should cost zero and produce no healing shard. Keep out of the enemy's attack range where practical.
3. Let one wolf take damage. Lock on and cast once, keeping lock-on through impact for this baseline. Expected: only that wolf heals by 25% max HP, capped; one normal charge (8 FP with Lusat's staff under the prior setup), no duplication/stagger and stable FPS.
4. With an injured wolf several metres away but still in normal shard range, cast and UNLOCK as soon as the shard visibly leaves the staff. If it reaches the wolf, that original wolf should heal. Record FP and what happened. Changes before release are evaluated as a new release selection and do not test flight tracking.
5. Repeat, switching to a different wolf or an enemy after release. Only the original wolf may heal. Use some distance to make the short flight window easier to observe; Swift's flight is brief and range is limited.
6. After the above succeeds, try successive casts to different injured wolves, a blocked/missed valid shot, insufficient FP, resting while summons are active then resummoning, and one ordinary damaging sorcery. Normal spells must retain their normal cost and damage; no crashes or FPS drop should occur. Target death in flight can be checked opportunistically; no need to force a difficult setup.

Wait 10 seconds after a short test batch, leave the game open, and report the results plus FP changes and FPS for log review. Stop after one or two unexpected valid casts instead of spending many resources. Log: `dist/SpiritAshResummon.log`. Look for the 0.5.1 hook banner, `release rejected`, `native_spawn_accepted`, `projectile evidence` (original versus current target), and `impact`/rejection records. Logged expected HP and native acceptance require visible or independent HP confirmation.

## Validation and rollback

0.5.0 was user-validated: acquisition/memorization, three wolf heals with observed392->500,365->490,381->500 at500maxHP, 8 FP each using Lusat's staff, and no reported animation/FPS problems. Those observations do not establish 0.5.1 acceptance. No tests were run by the agent.

The next install replaces the DLL/PDB and documentation with a verified backup, preserving the corrected spell assets, INI, existing log and profile paths. See `evidence/install-0.5.1.json` for the exact backup directory and hashes. To roll back with the game closed, restore its `dist` and backed-up launcher files over the current matching paths. This returns to the working0.5.0 custom-spell assets; do not remove or replace saves.

Runtime is limited to the previously inspected WW2.7.2.0/App1.17.2 offline executable and exact hook signatures. Hook conflicts disable the feature. Unsupported/disabled hooks do not prevent native costs from inert custom-spell assets. No online/EAC bypass is supplied. Read IMPLEMENTATION.md for native addresses, lifetime/ownership safeguards and source provenance; `tools/SpiritMendAssets/README.md` documents the separate offline asset builder. Icon work and balance changes are deferred.
