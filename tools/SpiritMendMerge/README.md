# Reviewed Clever + Terra + Spirit Mend merge

GPL-3.0-or-later offline tool, separate from the Rust DLL. Uses the same pinned
SoulsFormatsNEXT and Paramdex dependencies as `../SpiritMendAssets` (including
its optional Oodle printf patch). The user's Oodle DLL is loaded from the game
directory. No proprietary files are distributed with this source.

This narrow merge accepts only the exact audited target and donor hashes in
`Program.cs`. It is not a general old-version bypass or a universal mod merger.
Target regulation 11701000 is the existing Clever + Terra overlay; donor
11730034 contains the accepted Spirit Mend spell assets. Strict version-filtered
schemas, equal field layouts and equal row sizes are required. Original rows
are compared field-by-field after decrypting the output. Six tables receive seven
new rows; all other binder members remain byte-identical. Message-ID collisions
also reject, and existing Clever item text is preserved.

The target already occupies shop row 100521 and every slot in Kalé's regular
100500..100524 shop. Spirit Mend is instead cloned to free row 100599. The tool
adds a native **Study sorcery** menu entry at Kalé, using existing localized
EventTextForTalk 23300000, and opens only 100599..100599. It does not expand his
regular shop or replace existing items. Cost remains 300 runes. Spell/Goods ID
990001 and all combat-related IDs and values are unchanged.

The exact extracted `m60_00_00_00.talkesdbnd.dcx` is required. Only
`t800006000.esd`, group 2147483596 is edited: insert talk choice 6 before Leave
in state 3, insert its condition in state 5, and add state 32 with a cloned
native shop-open/wait path. After readback, undoing these three additive changes
must serialize identically to the original script; 164 other NPC scripts remain
byte-identical. A mod already overriding this talk bundle needs a new review.

Build (from repository root):

```powershell
dotnet build tools/SpiritMendMerge -c Release
# Supply -p:SoulsFormatsProject=<absolute csproj> when using an existing checkout.
dotnet tools/SpiritMendMerge/bin/Release/net9.0/SpiritMendMerge.dll `
  'C:/path/to/Game' './external/Paramdex/ER' `
  'C:/profile/mod/terra-self-buff/regulation.bin' 'C:/prior-Spirit-Mend/mod' `
  'E:/extracted/msg/engus' 'C:/profile/mod/clever/msg/engus' `
  './generated-assets-solo' 'E:/extracted/script/talk/m60_00_00_00.talkesdbnd.dcx'
```

Output must be new. Inputs are never overwritten. Work happens in a sibling
`.building-<GUID>` folder and becomes the output only after readback checks pass.
Failed builds leave their staging evidence. `merge-report.json` records local
input paths/hashes and preservation results; keep it outside public source.
Gameplay and menu usability remain untested; serialization checks are not live
acceptance. Existing full-overlay generation remains supported separately.
