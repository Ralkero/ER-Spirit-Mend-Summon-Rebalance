# ER Spirit Mend & Summon Rebalance

Current source snapshot: **0.5.1**, a Windows x64 runtime DLL plus an offline asset builder for Elden Ring. Start with [ANALYSIS.md](ANALYSIS.md) for a code-reading map, or [SPIRIT-MEND.md](SPIRIT-MEND.md) for the manual gameplay procedure.

The current project has evolved beyond the original restricted-zone, one-Ash design. It supports reusable summons, multiple different Ash groups, optional unrestricted summon zones, four individual Spirit health bars, friendly Spirit lock-on/cycling, and the **Spirit Mend** Intelligence sorcery. Duplicate groups from the same Ash are still deferred: reusing that Ash heals its existing group.

## Current behavior and evidence

| Feature | Implementation / validation status |
| --- | --- |
| Resummon after the group dies | Implemented; earlier single-group resummon tests succeeded. Mixed-group correction 0.2.2 was left for gameplay follow-up. |
| Reuse an Ash to heal its survivors | 25% of each survivor's maximum HP, capped; normal Ash FP/HP cost once. Earlier user tests covered costs, partial groups, full health and insufficient FP. |
| Multiple Ash types | User observed Wolves and Militiamen together, independent group healing and stable 60 FPS. One group per unique Ash remains. |
| Summon outside vanilla zones | Optional; user observed wolves at Church of Elleh and persistence across the Gatefront boundary at 60 FPS. |
| Compact Spirit HUD | Up to four individual entity rows; user accepted combat/healing behavior. Overflow summons remain active. |
| Friendly Spirit lock-on | Owned living Spirits join native target selection and enemy cycling; user accepted DualSense cycling and rest/resummon cleanup. |
| Spirit Mend impact heal | 0.5.0 user observed individual 25%-max-HP heals, capping, normal animation, and stable FPS. Lusat's staff cost 8 FP for the base-5-FP spell. |
| Invalid-cast cost prevention and flight target retention | Implemented and compiled in 0.5.1; **gameplay acceptance pending**. |

Spirit Mend uses Swift Glintstone Shard's casting animation, visuals, projectile travel and temporary icon. It requires 12 INT, one slot and base 5 FP; native staff/equipment cost modifiers apply. It is sold by Kalé for 300 runes. A successful impact heals only the selected owned Spirit by 25% of its own maximum HP.

In 0.5.1, selection is captured at **projectile release**, not button press. A missing/enemy/full-health target rejects projectile creation and the following native resource commitment. The animation may still play. After a valid release, changing lock-on must not transfer the heal to another target. Valid misses, obstructions or target loss after release still spend normal FP.

## Requirements

- Windows x64; runtime allowlist currently targets the inspected worldwide executable file version **2.7.2.0 / App 1.17.2**. This is the inspected version, not a claim that it is the latest game version.
- Offline single-player with Easy Anti-Cheat disabled through the user's existing supported offline mod setup. No anti-cheat bypass or protected online support is provided.
- Rust version pinned in [rust-toolchain.toml](rust-toolchain.toml), Microsoft C++ build tools, Windows SDK, and Git.
- For spell assets: .NET 9 SDK, pinned SoulsFormatsNEXT/Paramdex dependencies, the user's own `regulation.bin`/Oodle DLL, and extracted English message bundles.
- A compatible Mod Engine 2 installation. Its executable and game files are not included.

## Build and local setup

From a PowerShell terminal in the repository root:

```powershell
./tools/Build.ps1 -OutputDirectory ./package-0.5.1
```

This compiles/packages the DLL. It does **not** run tests unless `-RunTests` is supplied. Automated and gameplay tests remain user-operated. The published runtime and vendored Rust files match the installed 0.5.1 source; [PUBLICATION.md](PUBLICATION.md) explains the portable tooling changes and excluded generated files.

To prepare the separately licensed asset tool:

```powershell
./tools/Prepare-AssetDependencies.ps1
dotnet build ./tools/SpiritMendAssets -c Release
dotnet ./tools/SpiritMendAssets/bin/Release/net9.0/SpiritMendAssets.dll `
  'C:/path/to/ELDEN RING/Game' `
  ./external/Paramdex/ER `
  ./generated-assets-0.5.1 `
  'E:/path/to/extracted/msg/engus'
```

The output directory must be new. See [the asset tool guide](tools/SpiritMendAssets/README.md) for exact dependencies, schema handling and extraction requirements. The builder creates an isolated `mod` directory; it does not overwrite the game's regulation or edit saves. The corrected changes are documented in [assets/patch-manifest-0.5.1.json](assets/patch-manifest-0.5.1.json).

For the launcher layout, create a new local `dist` folder, copy the package contents into it, and copy the generated `mod` directory into `dist/mod`. Choose either the conservative [default INI](config/SpiritAshResummon.ini) or the [current prototype INI](config/SpiritMend-Prototype.ini); save the selected configuration as `dist/SpiritAshResummon.ini`. The prototype enables Spirit Mend, friendly lock-on, HUD, unrestricted zones and diagnostics. Settings are read at startup.

```powershell
./tools/Launch-Test.ps1 `
  -Loader 'C:/path/to/modengine2_launcher.exe' `
  -GameExe 'C:/path/to/ELDEN RING/Game/eldenring.exe'
```

The launcher uses only this repository's local `dist` DLL/assets and writes a local Mod Engine 2 profile. It does not redirect saves. Use a test character and your existing offline setup. Buying/memorizing a custom spell is saved by the game normally; the mod does not directly modify save files.

## Compatibility and removal

Exact native call/signature checks reject incompatible hook sites. Other DLLs hooking those sites may conflict; this project does not assume arbitrary hook chaining is safe. Spell assets contain a full derived regulation, so other parameter mods need a deliberate merge. English message bundles are the only prepared language. A disabled Spirit Mend hook leaves its custom projectile marker inert and does not guarantee prevention of native cast costs.

Close the game before changing DLLs. To revert a local update, restore the saved previous package/profile. To remove the spell asset profile entirely, unequip/unmemorize the custom spell first, then stop loading this DLL and its `mod` directory. Keep save backups; there is no automatic save cleanup. No original game files need replacement.

## Documentation and licensing

- [ANALYSIS.md](ANALYSIS.md): architecture, reading order, native hook map and open questions.
- [IMPLEMENTATION.md](IMPLEMENTATION.md): detailed architecture and development history.
- [VALIDATION.md](VALIDATION.md): observed results, compilation records and unexecuted scenarios.
- [SPIRIT-MEND.md](SPIRIT-MEND.md): current spell behavior and user tests.
- [CHANGELOG.md](CHANGELOG.md): incremental changes through 0.5.1.
- [HUD investigation](HUD-INVESTIGATION.md) and [targeted healing investigation](TARGETED-HEALING-INVESTIGATION.md).
- [UPSTREAM.md](UPSTREAM.md) and [THIRD-PARTY-NOTICES.txt](THIRD-PARTY-NOTICES.txt): provenance and retained notices.
- [PUBLICATION.md](PUBLICATION.md): snapshot scope, sanitization and reproducibility limits.

The Rust runtime is MIT OR Apache-2.0, with ArialSenki's MultipleSpiritAshes and Vincent Swarte/contributors' bindings credited. The separate SoulsFormatsNEXT-based C# asset builder is **GPL-3.0-or-later**; see its [LICENSE](tools/SpiritMendAssets/LICENSE). Game executables, generated regulation/message bundles, extracted HUD art, Oodle, saves, private paths and debug binaries are not distributed here.
