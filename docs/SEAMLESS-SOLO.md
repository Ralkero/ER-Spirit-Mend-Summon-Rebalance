# 0.6.1-solo.1 - solo play with Seamless loaded

The user requested solo play without further testing on 2026-10-30. This branch
implements an opt-in exception for the exact inspected Seamless binary. Connected
compatibility work remains separate and incomplete. This is an experimental
build: compiled and statically checked, not gameplay-validated.

`[Compatibility] AllowSeamlessSolo=true` permits that DLL at startup only after
SHA256 checks of both it and the game executable. The original executable-version,
native hook-signature, EAC/conflicting-mod and offline session gates remain in
place. Lobby and protocol must both be zero, GameMan must report offline, and
the native player registry must be empty or contain only one explicitly local
entry. Unknown state, remote entries and transitions reject modified gameplay.
No network packets or healing-replication code are added. This exception does
not authorize connected play or bypass Easy Anti-Cheat.

Startup logs identify hash or hook conflicts rather than patching unknown bytes.
No per-frame hashing is introduced. Local player-registry checks use current
SDK state inside the existing callback; no actor pointers are cached. Seamless's
solo session values and hook interaction have not been observed live yet, so
installation does not prove that every feature activates on this profile.

## Installed behavior

- Resummoning, different Ash types together, summon-anywhere mode, existing
  heal-on-Ash-reuse, four individual HUD rows, friendly lock-on and Spirit Mend.
- Duplicate same-Ash groups remain off, matching the prior installed prototype.
- Healing stays 25% maximum HP. Spirit Mend stays base 5 FP, 12 INT, with native
  staff modifiers and projectile impact healing.
- Buy Spirit Mend from **Kalé -> Study sorcery**, 300 runes, then memorize it at
  a grace. This separate shop avoids overwriting Clever's occupied shop slots.
- Launch the normal Mod Engine 2 profile, keeping Seamless and the existing
  `.me2pack` save selection. Do not open or join a co-op session with this build.
  Native constructor/HUD changes remain loaded until the process exits even
  when modified gameplay gates reject a connected session.

The installation replaces the diagnostic DLL's loader entry with this gameplay
DLL and adds a new highest-priority merged asset folder. Prior DLLs, overlays,
Seamless settings, launcher and saves remain intact. Source INIs and the earlier
standalone test installation remain unchanged. The combined regulation preserves
42,760 original rows in affected tables and 188 other binder members; English
text and unrelated NPC scripts are preserved. See `tools/SpiritMendMerge/README.md`.

## Build evidence and limits

Rust release build and C# merge-tool build succeeded. The merge decrypts its
output and compares original rows/fields, imported rows, messages and scripts
before publishing. Tests and game launch were not performed. Shop-menu behavior,
Seamless hook coexistence and solo runtime gates remain untested. The stock
assets' multiplayer spell flags remain unchanged.

Only the exact current WW 2.7.2.0 executable and pinned Seamless DLL are accepted;
the hashes are in `src/seamless_solo.rs`. Updating either requires review. The
active profile's older regulation remains 11701000; this merge does not silently
upgrade it or claim compatibility with arbitrary older files.

To undo the install, close the game and run its backup folder's `Restore.ps1`
with `-Apply`. It refuses if the profile changed after installation. It restores
the previous profile configuration and leaves the new folders inactive for
review. No save conversion or save restore is involved. Before using connected
co-op again, restore the pre-install profile and restart the game.
