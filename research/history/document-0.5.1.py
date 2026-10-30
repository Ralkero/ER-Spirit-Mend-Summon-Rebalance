from pathlib import Path
root=Path('outputs/SpiritAshResummon')
def prepend(name,text):
 p=root/name;p.write_text(text.strip()+'\n\n---\n\n'+p.read_text(encoding='utf-8-sig'),encoding='utf-8')
prepend('README.md','''# SpiritAshResummon 0.5.1 - Spirit Mend release validation

Current local release: native preflight rejects Spirit Mend when no injured, living, owned Spirit is locked on at projectile release. Rejected releases create no shard and skip native FP/resource commitment. The casting animation may still play. Accepted shards retain the original target in native projectile launch data; impact validation no longer depends on current lock-on.

Build succeeded for Windows x64. This release still needs user gameplay acceptance; no automated or gameplay tests were run by the agent. The previous 0.5.0 impact healing was user-validated, including 8 FP per cast with Lusat's staff. See [SPIRIT-MEND.md](SPIRIT-MEND.md) for the current procedure and limitations.

Use the usual offline `tools/Launch-Test.ps1` (or `Launch-SpiritMend.ps1`). Existing purchased/memorized Spirit Mend is reused; no repurchase or save edit. Corrected Kale shop assets, configuration and other summon features are retained. Installation and backup are recorded in `evidence/install-0.5.1.json`. Earlier release sections below are historical.''')
prepend('IMPLEMENTATION.md','''# 0.5.1 - native release gate and per-projectile target

`src/mend_cast.rs` is included by `spirit_mend.rs`. All addresses below are RVAs in the inspected WW 2.7.2.0 executable (App 1.17.2), SHA256 F9118FEA2CFD8FA3901E8060BFADB1552CB2471771DED8A5B662C4E298FFB328. Static evidence is in `evidence/cast-static`; these are pefile/Capstone observations, not REA decompilation or runtime traces.

- Calls 3c05d6/3c0611 invoke projectile action 3c0f80. In both callers, a false result returns before resource application 3c0e70. The action prefix contains owner ChrIns at +0, magic ID at +c. The wrapper affects only Magic 990001. It validates offline state, current player, locked-on owned living injured Spirit and healing assets, then calls the native function. Rejection returns false, with no manual FP subtraction or refund. Native intelligence/equipment/FP eligibility and staff modifiers remain in the game. The release check occurs after animation start, not at button press.
- Scoped thread-local release handles are available only during that native call. At 3c0ff0 -> 3c10c0, the normal builder finishes first. Its fresh BulletSpawnData output is checked for magic/owner/bullet IDs and an empty extended target. The explicit target at +20 is set, the borrowed alternate auto-target pointer is cleared, and the forced free-aim bit is cleared for this locked-on custom spell. No camera/actor state is rewritten. Unexpected context makes the bullet ID invalid, preventing spawn/cost.
- Native 38e750 selects the explicit handle; 3950c0 constructs a type-2 target with the full handle at +8, then stores it in launch context +140. 39e90f copies the 660-byte launch context to CSBulletIns+4b0. Each native projectile therefore carries its own original target, separate from mutable homing state at +d30. There is no global last-target variable or between-frame actor-pointer cache.
- Call 449ffa -> 4489f0 supplies the attack packet. For custom attack ID +40 = 99000100, its original FieldIns pointer at +1d8 is compared against the bounded current CSBulletManager list before dereference. Checks include bullet type/full handle, owner backlink, fly parameter, launch bullet/magic IDs, original target kind/handle and current player ownership. A scoped value carries those scalar identities through the existing effect calls at 4493d3/4494b8.
- Impact receiver must match the original launch target and still pass owned/living/injured/asset checks. Current camera selection is diagnostic only. A bounded 64-entry, five-second scalar handle ledger prevents duplicate delivery from the same projectile. No borrowed pointers survive a callback. Native registry lifetime and generation-bearing handles are checked anew; engine destruction/load removes the projectile. Expired delivery keys are pruned on the next impact.
- The existing native 3e8e70 operation applies SpEffect 99000101 (instant -25 percent HP-rate, zero flat HP, duration/interval/stateInfo zero). A valid release that later misses, meets an obstruction, loses its target, or arrives after another heal still spends its cost. It cannot transfer its heal to a new selection or a different intersecting ally.
- All six call-site adapters install as one existing transaction, with original-call checks, callee signatures, native cost-branch/context signatures and compile-time layout checks. A mismatch disables Spirit Mend hooks. Without hooks, assets remain inert and native casts may still cost FP; no invalid-cast guarantee applies to a disabled/unsupported installation.

Vendored bindings add four public BulletSpawnData fields, a narrowly scoped explicit-target method, and a read-only CSBulletIns launch-byte accessor. Licenses are retained. Ordinary casts/effects are forwarded. No per-frame scans, VirtualQuery loops, native target/team writes, custom costs, passive healing or new gameplay tests are introduced.''')
prepend('VALIDATION.md','''# 0.5.1 - compilation/static review; gameplay pending

2026-10-30: Windows x64 release DLL compiled using the locked dependencies. Final build evidence: `evidence/build-0.5.1.txt`. One initial compilation caught a signed BlockId mismatch; fixed with an i32 byte decode before the successful build. Static PE inspection confirmed all six original call targets, cost short-circuit branches, bullet launch-copy path and packet offsets against the inspected executable. `evidence/cast-static/hook-review.json` records the executable hash and original call bytes. Compile-time assertions cover relevant local layouts. These observations do not prove in-game behavior.

Automated tests: NOT RUN, per user instruction. Agent gameplay tests: NOT RUN. Game not launched by agent. New 0.5.1 behavior: NOT YET USER-VALIDATED. The prior 0.5.0 user results below remain evidence only for that release. Installation evidence separately records process checks, backup and file hashes; file verification is not gameplay validation.

User procedure: follow SPIRIT-MEND.md. First establish invalid casts leave FP unchanged (no target, enemy, full-health wolf), then check one paid injured-wolf heal, then unlock/switch after projectile release. Continue with quick successive casts to different wolves, a wall/missed shard, target death during flight if practical, rest/resummon and an ordinary sorcery. Leave the game open and report FP before/after, chosen target, visible projectile/HP behavior and FPS. Do not infer actual HP from `native_accepted` alone.

Known limits: preflight is at release, so a rejected cast may animate; staff modifiers affect costs; valid misses are not refunded. Native hit-packet registry lifetime and the two casting paths still require live acceptance. If a new hook/layout guard rejects, Spirit Mend is disabled with a diagnostic. No multi-target or duplicate same-Ash behavior is added.''')
prepend('CHANGELOG.md','''# 0.5.1 - 2026-10-30

- Added Spirit Mend target/health/ownership preflight before native projectile creation and resource commitment; no refunds or manual deductions.
- Capture the selected target at release into native projectile launch data; validate that original target at impact independently of current lock-on.
- Add live-bullet identity/lifetime checks, bounded duplicate-impact suppression, scoped callback state and projectile diagnostics.
- Preserve the corrected Kale shop assets, existing spell ID, native staff cost modifiers, animation, impact heal and prior summon features.
- Compile Windows x64 DLL and prepare a reversible installation. Gameplay and automated tests remain user-operated.''')
p=root/'SPIRIT-MEND.md'
(root/'evidence'/'SPIRIT-MEND-before-0.5.1.md').write_bytes(p.read_bytes())
p.write_text('''# Spirit Mend 0.5.1

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
''',encoding='utf-8')
prepend('UPSTREAM.md','''## Local binding changes in 0.5.1

`vendor/fromsoftware-rs/crates/eldenring/src/cs/bullet_manager.rs` now exposes owner/magic/bullet/target fields and a version-specific explicit target setter. `cs/bullet_ins.rs` adds a read-only native launch context accessor and an offset assertion. These are local changes to the pinned MIT/Apache-2.0 source; original notices are preserved. Earlier statements that the vendor is entirely unmodified are historical.

Since0.5.0 the local installed profile contains derived regulation/message assets generated from this user's game files. It contains no game executable, save or anti-cheat component. Earlier no-game-assets packaging statements describe the pre-sorcery DLL-only releases.''')
for name in ['tools/Launch-Test.ps1','tools/Launch-SpiritMend.ps1']:
 p=root/name;p.write_text(p.read_text(encoding='utf-8-sig').replace('0.5.0','0.5.1'),encoding='utf-8')
prepend('START-HERE.txt','''Current release: 0.5.1. Use the usual tools/Launch-Test.ps1 offline launcher.
Your existing Spirit Mend purchase and memorization remain usable.
Start with SPIRIT-MEND.md steps 1-3: invalid casts should spend zero FP;
one injured-wolf cast should heal on impact for the normal modified cost.
Then test unlocking/switching AFTER projectile release.
No automated/gameplay tests were run by the agent. User acceptance pending.''')
print('Updated 0.5.1 documentation and launcher version labels.')
