# Current compatibility baseline: 0.6.1-solo.5

Main targets the reviewed Mod Engine 2 stack with Seamless loaded **while playing
solo**. It supersedes the standalone-first 0.5.1 snapshot. Compatibility checks
are required parts of initialization, not a blanket permission to accept modified
native functions. All runtime/vendor files match the current local solo.5 source.

## Reviewed stack and limits

- Worldwide Elden Ring file version 2.7.2.0 / App 1.17.2.
- Seamless 2.0.2; exact SHA256 must match the accepted binary.
- Clever + Terra self-buff parameter overlay, Spirit Mend merged on top.
- Existing Quest Log, Transmogrify, PS5 UI and Ranni assets remained loaded during
  the user's observations. This is coexistence evidence for that profile, not
  blanket support for every version or every feature of those mods.
- English message assets; merged parameter version 11701000. The merger verifies
  exact reviewed input hashes and schema. Other base versions need a new review.

Accepted SHA256 values:

| File | SHA256 |
| --- | --- |
| eldenring.exe | f9118fea2cfd8fa3901e8060bfadb1552cb2471771ded8a5b662c4e298ffb328 |
| Seamless ersc.dll | 0a093d8490967bf19c62f895fdf1fda21840282d2b2b371b9b4fc24a0cfe146e |
| Existing locally installed solo.5 DLL (build provenance) | 01d03c6abcf74dcad496babbba91192b0eec391fed09536e2000af78940def40 |

The DLL hash is provenance for the observed installation, not a reproducible-build
promise across machines/paths. Game/module hashes and live instruction checks are
enforced. No per-frame file hashing was introduced.

The implementation preserves two reviewed Seamless redirects and its expanded
registry, validates remaining hook sites, accepts observed solo protocol 0 or 6
only with no lobby, offline game mode and verified local-only membership/host,
and resolves HUD names through a bounded loaded-text reader. Unknown values,
remote membership and unsupported binaries reject modified gameplay.

**Do not host or join with this build.** Connected-session support is incomplete.
The guard is not an unload mechanism: constructor/HUD changes remain resident.
Restore the pre-mod profile and restart before connected play. No replication,
EAC bypass, or protected online support is implemented.

## User evidence versus pending checks

The user confirmed Wolves and Lhutel together, normal FPS, and friendly lock-on
cycling with solo.4. With solo.5, the user confirmed both names display correctly;
reviewed logs corroborated resolved labels. These are specific observations.
Spirit Mend's current-profile healing/cost/invalid-cast/flight behavior, duplicate
same-Ash groups and cap/cost/lifecycle cases still require user acceptance.
Earlier standalone healing/resummon tests are retained as history, not substituted
for compatible-profile testing. See VALIDATION.md and the milestone documents.

## Build and integrate into an existing profile

1. Build with tools/Build.ps1. It selects config/CompatibleSolo.ini by default:
   duplicate groups (three per Ash), unrestricted zones, four individual bars,
   friendly lock-on, Spirit Mend and AllowSeamlessSolo=true. Existing INIs are not
   automatically modified. The runtime's no-INI defaults stay conservative.
2. Generate the base spell donor using tools/SpiritMendAssets, then the reviewed
   combined overlay using tools/SpiritMendMerge. Follow their input/merge guides.
   Do not use the standalone donor regulation over a modded playthrough.
3. Close the game/launchers. Back up the current Mod Engine 2 config, DLL/INI and
   asset overlay. Preserve the existing isolated save selection and other mods.
4. Place SpiritAshResummon.dll and the selected INI together in a dedicated DLL
   directory. Register that DLL once in the existing profile's external_dlls,
   preserving the other entries. Do not load another SpiritAshResummon build.
5. Put the generated merged mod directory in its own overlay and add it first in
   the existing mods list so it precedes the reviewed parameter base. Preserve
   the original lower-priority files. Do not overwrite game-root regulation.bin.
6. Launch the existing offline Mod Engine 2 profile. Review the startup log for
   solo acceptance and hook installation. Keep connected sessions disabled.

This is a source distribution; user-owned inputs and other mods are not bundled.
The narrow Install-SeamlessProbe/Install-SeamlessSolo/Update-SeamlessSoloDll scripts
retain historical transition guards. They are **not a general fresh installer**.
They deliberately reject different profiles, packages and prior DLL hashes. Do not
remove their checks to force a different setup. The standalone Launch-Test.ps1
also does not recreate this mod stack or its save selection.

## Shop location

The installed merged overlay uses Kale's Study sorcery option, 300 runes. The
compiled/readback-verified SpiritMendShopMove tool moves that listing to the
Isolated Merchant's normal Purchase inventory on the Academy bridge (also 300).
It removes Kale's added route, preserves spell ownership and balance, and produces
a new isolated output. This migration is staged but not installed/gameplay-accepted
at publication. Follow tools/SpiritMendShopMove/README.md; do not infer that a GitHub
update installs the migration into an existing profile.

## Rollback

Close the game and restore the saved profile/DLL/INI/overlay using the applicable
hash-guarded backup. Preserve saves; no conversion or automatic cleanup is needed.
Before removing all custom spell assets, unmemorize Spirit Mend and retain a save
backup. A future game/mod update needs renewed hash/signature/merge review.
