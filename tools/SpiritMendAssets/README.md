# SpiritMendAssets - offline asset builder

SPDX-License-Identifier: GPL-3.0-or-later. This separate .NET 9 program uses SoulsFormatsNEXT. It is not linked into the MIT/Apache Rust DLL. See LICENSE and the retained upstream notices.

Run from the repository root:

```powershell
./tools/Prepare-AssetDependencies.ps1
dotnet build ./tools/SpiritMendAssets -c Release
dotnet ./tools/SpiritMendAssets/bin/Release/net9.0/SpiritMendAssets.dll `
  'C:/path/to/ELDEN RING/Game' `
  ./external/Paramdex/ER `
  ./generated-assets-0.5.1 `
  'E:/path/to/extracted/msg/engus'
```

Provide all four arguments to generate assets. The output directory must not exist. Three arguments perform parameter inspection only; no workstation-specific message path is assumed. The fourth directory must contain the user's extracted English item.msgbnd.dcx, item_dlc01.msgbnd.dcx and item_dlc02.msgbnd.dcx. UXM Selective Unpacker can extract those files from the user's own installation; do not use its game-patching feature for this workflow. HUD files are not needed to generate Spirit Mend assets.

Pinned dependencies:

- SoulsFormatsNEXT: https://github.com/soulsmods/SoulsFormatsNEXT at 9e1c36f54962f80c143e3c1fe8a709204aefacf9.
- Paramdex: https://github.com/soulsmods/Paramdex at ff7245e524329bc3eab00036723d2bd53384cedf.

Prepare-AssetDependencies.ps1 clones these into external and applies oodle-legacy-printf.patch. It refuses to reset existing checkouts. To use an existing compatible checkout, build with `-p:SoulsFormatsProject=C:/path/to/SoulsFormats/SoulsFormats.csproj` instead. Apply the included compatibility patch to that pinned checkout first. This makes the optional Oodle printf export optional while retaining version querying, compression and decompression. The user's Oodle DLL is loaded from the game directory; no DLL is copied into the repository.

The builder clones Swift Glintstone Shard and friendly Heal from Afar template rows into collision-checked IDs, zeros attack damage/poise/knockback, creates inert marker and native instant percentage-heal effects, and adds Kalé shop row100521 plus English text. This shop slot and 0.5.0 healing were user-validated. The same assets are used by 0.5.1; its new DLL behavior awaits testing.

Strict schema parsing stays enabled. The pinned SpEffect schema omits three high bits at byte0x353; the tool explicitly preserves them as a three-bit field. Existing row values and unrelated binder entries are retained through serialization. On failure, retain the partial output for inspection and choose a new directory after correcting the issue.

Outputs include `mod/regulation.bin`, the English message bundles, a patch manifest and local inspection JSON. Copy the generated `mod` directory to your separate mod profile. Do not overwrite the game-root regulation. These outputs contain game data and are intentionally not included in this public source repository. Rebuild against your own files. This is a full regulation overlay, not an automatic merge for other mods.

See ../../assets/patch-manifest-0.5.1.json for the exact edits, ../../SPIRIT-MEND.md for behavior/tests and ../../PUBLICATION.md for publication scope. The historical workstation instructions are retained in ../../docs/ASSET-TOOL-HISTORY.md.
