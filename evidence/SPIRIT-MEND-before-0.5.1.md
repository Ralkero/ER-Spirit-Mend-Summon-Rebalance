## Latest user validation — 2026-10-30

Shop acquisition, memorization and three selected-wolf impact heals succeeded. HP observations:392->500,365->490 and381->500 at500maxHP. User reports no FPS drops or animation problems. Lusat's Glintstone Staff+10 produced8FP per cast, consistent with its parameter-defined1.5x FP multiplier applied to base5. No indication of double charging. See the newest VALIDATION.md entry for evidence and remaining limitations. Earlier first-test instructions below are retained as the reproducible procedure; they are no longer wholly unexecuted.

# Spirit Mend — 0.5.0 development prototype

Status: Windows DLL and offline asset tool compiled; isolated parameter/message package generated. Installation status is recorded in evidence/install-0.5.0.json. No game launch, automated tests or gameplay tests were performed by the agent. The first gameplay result must establish whether the proposed native impact path is actually traversed and whether the native heal delivers the expected capped amount.

## Behavior implemented

A separate sorcery copied from Swift Glintstone Shard, with the original unchanged. Temporary tuning retains 12 Intelligence, zero Faith/Arcane, one memory slot, 5 FP, 9 stamina and the original casting behavior/VFX/icon. Balance is provisional. The projectile retains the template's speed, range, homing, hit radius and collision lifetime; it does not spawn a healing area or additional projectile. On impact, the intended effect is one instantaneous 25% maximum-HP native heal, capped by the engine. No Sorcery Scaling multiplier is intentionally introduced.

A cloned nondamaging Heal from Afar attack row supplies zero damage, poise damage and knockback with only friendly targets enabled; self/enemy flags are off. The bullet uses this row and carries one inert marker effect. Two narrow impact-effect call sites route that marker through the DLL. The DLL validates the local player source, owned living Spirit registry/group membership, full handle and data ownership, current lock-on selection and injured health. It also checks custom effect rows. Only then does it substitute the native percentage-heal effect. Without the DLL, enabled setting or supported hooks the marker has no healing or damage. All unrelated effects pass through unchanged.

The native heal uses changeHpRate=-25, changeHpPoint=0, effectEndurance=0, motionInterval=0, stateInfo=0, based on the allied Heal from Afar effect. State284 (used by the original heal) is removed to avoid its spell-healing scaling path. Native parameter semantics and static comparisons support this choice, but exact healing/clamping and rounding on Spirits are NOT yet gameplay-verified. The DLL logs expected HP and native acceptance; those are not proof of actual post-effect HP.

## Explicit prototype limitations

- Casting retains native costs, including casts without an injured eligible target, full-health hits, rejected hits and misses. No refund or custom FP deduction is added. The requested no-charge rejection behavior is NOT implemented yet; it needs a verified native cast preflight gate.
- Keep the intended Spirit selected until impact. The prototype checks current lock-on at impact; it does not yet latch the actor at cast start. Changing targets can cancel the heal, and a different owned Spirit intercepted after a target switch is not protected by a cast-time latch. This must be corrected before claiming the final selected-target contract.
- Same-Ash duplicate groups remain deferred; normal Ash reuse still heals its existing group. No new UI or icon artwork.
- Only English message bundles are prepared. Native shop visibility, acquisition/memorization of the custom row ID, projectile friendly collision, actual hook path, healing/cost behavior, quick consecutive casts and staff variants await user testing.
- This package includes a full derived regulation.bin. It does not automatically merge concurrent param mods. It is built from this machine's source regulation version11730034; source/output hashes are in patch-manifest.json. Game installation files and save files are never edited by the builder. Buying a custom spell will of course be recorded by the game's normal save behavior.

## Prepared files and first user-operated check

The installed profile uses dist/SpiritAshResummon.dll and dist/mod (regulation and English message bundles). Use the usual tools/Launch-Test.ps1; tools/Launch-SpiritMend.ps1 is an alias for the same installed profile. The launcher now includes the spell assets and writes config_spiritash_test.toml accordingly. The staged development-0.5.0 directory remains as build evidence. No game-root overwrite is needed.

1. Use the test character with the separate development profile. At Church of Elleh, look for Spirit Mend in Kale's shop (prototype price300runes). If absent or unnamed, stop and report; do not edit inventory/save files. The corrected entry uses unused row100521 within the documented100500-100524 range; visibility still awaits user confirmation.
2. Acquire one, memorize it at grace, and equip a staff with at least12INT and sufficientFP. It should show one slot and5FP. Swift Glintstone Shard should remain separately available and unchanged.
3. Summon wolves and let one take damage. Select that wolf, record HP if visible and player FP, and cast Spirit Mend once while keeping it selected. Look for the Swift shard animation and travelling projectile; healing should occur on impact, not at cast start.
4. Expected for a500maxHP wolf: +125HP capped at500; exactly5FP for one cast; only the struck selected wolf heals, with no stagger or duplication. Wait10seconds and leave the game open for log review. Report visible projectile, heal timing, FP change andFPS. The installed log is dist/SpiritAshResummon.log.
5. If the projectile does not heal, stop after one or two attempts. An absent impact message vs an explicit rejected marker vs native accepted result distinguishes dispatch, ownership/selection and effect problems. Do not infer success solely from native_accepted=1.

After the first hit is proven, check rapid casts, full health, other targets/enemies, no target, target switches/death in flight, obstruction, insufficientFP and rest/load cleanup. Invalid casts currently spend their native cost when the engine permits casting; this is a known missing preflight feature, not an expected final design.

Rollback: restore the previous dist files, tools/Launch-Test.ps1, tools/Launch-SpiritMend.ps1 and config_spiritash_test.toml from the backup identified in evidence/install-0.5.0.json, with the game fully closed. The restored profile does not load dist/mod. Unequip the custom spell before removing its asset profile. No agent-driven save repair or cleanup is provided.

## Implementation and reproducibility

- src/spirit_mend.rs is included in runtime and enabled by [Sorcery] EnableSpiritMend=true (defaultfalse). Startup checks supported executable gates, original calls at RVAs4493d3/4494b8 and the3e8e70 callee signature, then uses existing transactional near relays. No global effect-entry detour or actor HP write. Original nine-argument Windows ABI is forwarded unchanged for unrelated effects. The callback contains no per-frame loop.
- Static evidence: evidence/spell-static/apply_context.asm and callers-3e8e70.asm. The receiver/source interpretation is based on typed binding and static call flow; live impact attribution remains unverified. REA deep native providers are unavailable; these files come from ordinary pefile/Capstone disassembly, not REA decompilation.
- IDs: Magic/Goods990001; Bullet/Atk99000100; inert SpEffect99000100; healing SpEffect99000101; shop100521. Builder refuses ID collisions rather than overwriting existing rows. Text adds only GoodsName/Info/Caption IDs10/20/24 in the three English bundles; unrelated ID4010 accessory texts are untouched.
- tools/SpiritMendAssets is separate GPL-3.0-or-later offline C# tooling using SoulsFormatsNEXT (commit9e1c36f54962f80c143e3c1fe8a709204aefacf9). It is not linked into the Rust DLL. Paramdex commitff7245e524329bc3eab00036723d2bd53384cedf supplies schemas.
- The SpEffect schema leaves three nonzero high bits unnamed at byte0x353. The builder explicitly reads/writes them as preserved353High:3 instead of disabling strict parsing or losing data. SoulsFormatsNEXT's optional Oodle printf hook also needs a compatibility patch for the installed oo2core6 DLL; see the saved patch. No Oodle DLL is copied or redistributed.
- Build DLL with the existing locked release command. Build assets with `dotnet build tools/SpiritMendAssets -c Release`, then run the generated tool with game directory, Paramdex/ER directory, a NEW output directory, and extracted msg/engus directory. See tools/SpiritMendAssets/README.md. Asset generation reads and repacks local files; no automated test suite was run. Initial dependency compilation produced78 upstream warnings; final incremental tool compilation had zero errors/warnings. DLL compilation had zero warnings.

Next work: user impact-path evidence, then native cast preflight and target latching before final cost/target guarantees; tune FP/INT after behavior is sound. Custom icons remain deferred as requested.
