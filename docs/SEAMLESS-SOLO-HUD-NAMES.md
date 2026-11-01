> Update 2026-10-30: installed solo.5 naming is now user-validated for Wolves and
> Lhutel. Log PID 32412 confirms both resolved names and all four correct HUD rows.
> Earlier pending/staging statements below record the build-time state. Additional
> performance, healing and lifecycle checks remain separate from this naming test.

# 0.6.1-solo.5: restore loaded Spirit Ash names

## User-operated evidence

On 2026-10-30 the user reported normal FPS, successful simultaneous Wolves and
Lhutel the Headless, and working friendly lock-on/cycling with solo.4. Militiamen
are unavailable on this save. This validates those observed behaviors only;
Spirit Mend healing and connected play have not been accepted in this profile.

The user reported wolf HUD labels `Spirit Ash 232000 #1`, `#2`, `#3`. The log
confirms these fallback labels, active solo gates, actor registry validation,
and admitted friendly lock-on candidates. Do not infer a fresh healing/cost test
from this report.

## Cause and evidence

The name wrapper at game RVA 0x2671370 begins with an external rel32 redirect.
The existing optional signature check deliberately leaves LOOKUP unset. That
makes goods_name return None and the HUD use `Spirit Ash {base}`. This was the
optional name-lookup mismatch identified during solo.3's startup audit.

Read-only inspection of PID 24988 found the remaining 44 bytes of the 49-byte
wrapper intact. They select language/category tables from MsgRepository (+8,
counts +0x10/+0x14), select an FMG, and tail-call RVA 0x2671c40 with the item ID.
The complete 120-byte FMG lookup leaf matches the pinned executable. It binary
searches 16-byte groups at +0x28, reads the relocated offsets pointer at +0x18,
and adds a relative string offset to the FMG base. The file layout was also
corroborated against the available SoulsFormatsNEXT FMG reader source.

This is ordinary local PE/Capstone/ReadProcessMemory evidence, not REA Evidence.
The initial attempt to inspect the redirect destination crossed an unreadable
range and was abandoned; no unknown redirect is invoked or trusted. No native
functions were executed, process memory modified, or gameplay operated by the
agent. Private captures remain outside git.

## Correction

The ordinary unmodified name-wrapper path remains available. Only the exact-hash
Seamless solo profile may enable the loaded-table reader, and only when the wrapper
has the observed redirect plus the exact pinned tail and complete lookup-leaf
fingerprints. This does not accept arbitrary code changes or replace another mod's
hook. No new native call or hook is introduced.

`hud_names.rs` reads bounded copies using ReadProcessMemory and searches them in
Rust. Every read can fail without dereferencing arbitrary pointers. It checks
FMG version/endian/Unicode, file size, group/string counts, table ranges, string
index and relative offset, termination and UTF-16 validity. Unknown layouts,
unreadable data, missing names, and invalid bounds retain the safe generic label.
Strings are limited to the existing 128 UTF-16-unit window. Language slot and
Goods-name category ordering match the original caller.

Names come from the currently loaded modded/localized FMG tables, with no hardcoded
Ash-name dictionary, game-asset redistribution, or sidecar file. Purely dynamic
text transformations performed by another mod's redirected name wrapper are not
reproduced. Loaded FMG replacements are respected. This path still relies on the
existing game-task lifecycle; no raw native pointer is retained across callbacks.

The existing per-family name cache means these reads occur only when a new label
is resolved, not each frame. Only owned strings and scalar HP snapshots reach the
HUD renderer. A debug result line is emitted per cache fill. No gameplay policies,
HP/FP formulas, summon hooks, targeting, layout, row capacity or assets change.

## Validation and installation status

- Windows release compilation and formatting: completed.
- Regression harness compiled with --no-run; no automated tests executed.
- Prepared synthetic FMG coverage: correct/missing ID, malformed headers/tables,
  unreadable memory, missing terminator and invalid UTF-16.
- Clippy completed with eight existing warnings; no new name-reader warnings.
- Live naming on solo.5 and first-label overhead: pending user testing.
- The DLL-only package is staged while the user's game is open. The updater requires
  game closure, verifies solo.4, backs up DLL/log, and preserves the 26 config/asset
  files. No save or game-root changes. Source is committed locally, not published.

## Next acceptance

After closing the game and installing solo.5, launch the usual solo profile.
Summon Wolves and Lhutel, inspect the four compact rows, and check that normal
names replace the numbered fallback while member ordinals remain. The log should
show `HUD names: verified loaded-table reader enabled`, then
`HUD loaded-table name: goods=... resolved=true` and normal names in `shown`.
Report whether FPS remains normal when those labels first appear. Repeat one
lock-on cycle to confirm the accepted behavior remains intact. If a fallback
remains, preserve the log; no need to repeat combat tests.
