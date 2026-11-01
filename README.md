# ER Spirit Mend & Summon Rebalance

**Current main version: 0.6.1-solo.5.** The primary target is the reviewed
**Mod Engine 2 mod stack with Seamless Co-op loaded for solo play**, including the
Clever + Terra parameter merge. Compatibility is part of the supported setup,
not an optional future add-on. This replaces the earlier 0.5.1 source snapshot.

**Connected co-op is not supported.** Exact executable/module hashes, redirected
hook signatures and local-only session checks remain enforced. Do not join or host
with this build: session rejection does not unload its constructor/HUD patches.
See [current compatibility and setup](docs/CURRENT-COMPATIBILITY.md).

## Features and validation

| Feature | Current state |
| --- | --- |
| Multiple different Ash types | Implemented; user confirmed Wolves and Lhutel together with normal FPS in the compatible solo profile. |
| Friendly lock-on/cycling | Implemented; user confirmed cycling between those summons. |
| Four compact individual HP bars | Implemented; names corrected in solo.5 and user-confirmed for Wolves and Lhutel. More than four actors can remain active. |
| Same-Ash duplicates | Implemented, enabled in the recommended preset, three groups per Ash; current-profile gameplay acceptance pending. Ash reuse requests another full group. |
| Heal on Ash reuse | Retained when duplicate mode is disabled; 25% max HP and normal Ash cost. |
| Resummoning after death; summon-anywhere mode | Implemented; earlier standalone user evidence exists. Full lifecycle acceptance in the current compatibility profile remains pending. |
| Spirit Mend | Implemented: 25% max HP on projectile impact, 12 INT, base 5 FP, native staff cost modifiers. Earlier standalone impact-heal evidence exists; current-profile healing acceptance pending. |
| Invalid-cast cost prevention; target retention in flight | Implemented/compiled; targeted gameplay acceptance pending. |
| Academy bridge merchant migration | Asset tool compiled/readback-verified; staged locally, not installed or gameplay-accepted. |

The recommended [CompatibleSolo.ini](config/CompatibleSolo.ini) reflects the active
profile: duplicate groups, multiple Ash types, unrestricted locations, four HUD
rows, friendly lock-on, Spirit Mend and pinned Seamless solo compatibility.
`HealOnReuse=true` remains configured for the fallback mode; duplicates take
precedence while `AllowDuplicateAshGroups=true`. All successful summons retain
native FP/HP costs. Additional groups at the per-Ash cap are rejected.

## Requirements

- Windows x64; inspected worldwide executable **2.7.2.0 / App 1.17.2**, exact hash
  allowlist. This is not a claim about the latest game release.
- Reviewed Seamless 2.0.2 binary and compatible Mod Engine 2 stack. Matching hashes
  are authoritative; see [compatibility guide](docs/CURRENT-COMPATIBILITY.md).
- Offline solo setup with Easy Anti-Cheat disabled through the existing supported
  mod launcher. No protected online or anti-cheat bypass support.
- Rust pinned by rust-toolchain.toml, MSVC C++ build tools, Windows SDK and Git.
- Spell assets: .NET 9, pinned SoulsFormatsNEXT/Paramdex, the user's game/Oodle and
  English message/talk assets. Game-derived files are not distributed here.

## Build

Use PowerShell with the MSVC build environment available:

```powershell
./tools/Build.ps1 -OutputDirectory ./package-0.6.1-solo.5
```

This compiles the runtime and packages the **compatible solo preset** as
SpiritAshResummon.ini. It does not run tests unless `-RunTests` is explicitly
supplied. `-ConfigurationPreset Standalone` selects the older conservative INI
for a separate standalone setup; that is not the primary compatibility profile.
Do not overwrite an existing installed INI with defaults during updates.

Build the asset tools separately:

```powershell
./tools/Prepare-AssetDependencies.ps1
dotnet build ./tools/SpiritMendAssets -c Release
dotnet build ./tools/SpiritMendMerge -c Release
dotnet build ./tools/SpiritMendShopMove -c Release
```

[SpiritMendAssets](tools/SpiritMendAssets/README.md) generates the base spell donor;
[SpiritMendMerge](tools/SpiritMendMerge/README.md) builds the reviewed Clever + Terra
+ Spirit Mend overlay while preserving unrelated fields/text/dialogue.
[SpiritMendShopMove](tools/SpiritMendShopMove/README.md) optionally migrates its
listing to the Isolated Merchant. All tools require user-owned inputs and new
output directories. Changed/unreviewed inputs reject; this is not a universal
mod merger. The generated regulation must replace neither your game-root file
nor another mod's base regulation.

## Installation and acquisition

Follow the [compatible Mod Engine 2 setup](docs/CURRENT-COMPATIBILITY.md). Keep
Seamless, existing overlays and the isolated save extension. Add the DLL and
reviewed merged assets to that profile; do not use the standalone test launcher
as a replacement for it. Close the game before changing loaded files and keep
backups. Publication itself does not install or change any local files.

In the currently installed compatible overlay, buy Spirit Mend from **Kale ->
Study sorcery** for **300 runes**. After applying the separate shop migration,
buy it under **Purchase** from the **Isolated Merchant on the wooded bridge
outside Raya Lucaria's Main Academy Gate**, also 300 runes. Already-owned spells
remain usable without repurchase. The migration remains pending gameplay review.

## Documentation

- [CURRENT-COMPATIBILITY.md](docs/CURRENT-COMPATIBILITY.md): current setup, supported stack and limits.
- [ANALYSIS.md](ANALYSIS.md) and [IMPLEMENTATION.md](IMPLEMENTATION.md): architecture and source-reading map.
- [VALIDATION.md](VALIDATION.md): actual build/user evidence and pending checks.
- [SPIRIT-MEND.md](SPIRIT-MEND.md): spell mechanics and acceptance procedure.
- [Duplicate groups](docs/DUPLICATE-GROUPS.md): limits, identity and lifecycle.
- [CHANGELOG.md](CHANGELOG.md), [publication scope](PUBLICATION.md), [release checklist](docs/RELEASE-CHECKLIST.md).
- [Prior README through solo.5](docs/HISTORY-README-THROUGH-SOLO5.md): historical development banners, superseded by this page.

This is the current development baseline, not a claim of complete gameplay
validation. No tests or gameplay were executed for publication. Remaining
acceptance includes duplicate groups/caps/HP costs, Spirit Mend invalid targets
and flight tracking, lifecycle transitions, and the migrated merchant.

Rust runtime: MIT OR Apache-2.0. The separate SoulsFormatsNEXT-based C# asset tools
are GPL-3.0-or-later. See [UPSTREAM.md](UPSTREAM.md) and
[THIRD-PARTY-NOTICES.txt](THIRD-PARTY-NOTICES.txt). Saves, extracted game files,
mod-loader/Oodle/other-mod binaries and generated regulation/message bundles are
excluded. Personal save repairs are not part of this mod.
