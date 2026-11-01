# Spirit Mend: Academy bridge shop migration

Asset-only update for the reviewed Clever + Terra + Spirit Mend solo profile.
Spirit Mend moves from Kale's custom Study sorcery option to **Purchase** at the
Isolated Merchant on the wooded bridge outside Raya Lucaria's Main Academy Gate.
Price remains **300 runes**. Existing ownership and memorization are unaffected.
DLL 0.6.1-solo.5, configuration, healing and summon behavior remain unchanged.

## Scope and guards

- Relocate ShopLineupParam row 100599 to vacant row 100699; preserve every field.
- The destination's native shop range is 100675..100699. Its script is unchanged.
- Remove exactly the three custom Kale menu edits; his script must then serialize
  identically to the reviewed vanilla script.
- Spell/Goods ID 990001, unlimited stock (-1) and stock flag 0 remain unchanged.
- Require exact input hashes in Program.cs, parameter version 11701000, and a
  vacant destination row. This is a narrow migration, not a general mod merger.
- Reject changed inputs; use a new output directory and publish only after readback.
- Compare every shop row field, unrelated regulation member and NPC script.

Supports the reviewed App 1.17.2 profile (executable 2.7.2.0) and its merged
11701000 regulation. Other profiles require separate inspection. Historical
SpiritMendAssets/SpiritMendMerge builders retain their original acquisition paths;
apply this migration after generating the reviewed solo overlay.

## Build and generate

Requires .NET SDK 9, the pinned SoulsFormatsNEXT and Paramdex dependencies from
../SpiritMendAssets, and the user's game Oodle library. GPL-3.0-or-later; see LICENSE.
No game assets or proprietary dependencies are included in source control.

From the repository root (replace example paths):

```powershell
dotnet build tools/SpiritMendShopMove -c Release `
  -p:SoulsFormatsProject=C:/deps/SoulsFormatsNEXT/SoulsFormats/SoulsFormats.csproj
dotnet tools/SpiritMendShopMove/bin/Release/net9.0/SpiritMendShopMove.dll `
  'C:/path/to/Game' 'C:/deps/Paramdex/ER' `
  'C:/profile/mod/spirit-mend-solo-0.6.1/regulation.bin' `
  'C:/profile/mod/spirit-mend-solo-0.6.1/script/talk/m60_00_00_00.talkesdbnd.dcx' `
  'E:/extracted/script/talk/m60_00_00_00.talkesdbnd.dcx' `
  './generated-assets-shop-raya-lucaria-2'
```

The output must not exist. Failed builds retain .building-* evidence. Successful
output contains two assets under mod/ and a local manifest with paths and hashes;
keep the entire generated folder private and out of source control.

## Install and rollback

```powershell
# Preview is safe while the game is running.
./tools/Update-SpiritMendShop.ps1 -ProfileRoot 'C:/profile' `
  -PackageDirectory './generated-assets-shop-raya-lucaria-2'
# Close the game and launchers, then repeat with -Apply.
```

The installer only replaces the active solo overlay's regulation and m60 talk
bundle. It verifies source and installed hashes, backs up both files, and checks
the DLL, INI, launcher, config and other regulation/message/dialogue assets remain
unchanged. Saves and game-root files are never touched. Partial installation
failures restore replaced files. The timestamped profile backups directory holds
installation.json and Restore.ps1; run Restore.ps1 to preview, then -Apply while
the game is closed. Rollback rejects assets changed since this installation.

## Evidence and user acceptance

2026-10-31: Release compilation succeeded with zero warnings/errors. Generation
and decrypted readback preserved 164 unrelated NPC scripts, 193 regulation members
byte-for-byte, and every field/name across 1,324 shop rows (accounting for the moved
ID). Installer preview identified 25 other protected files. No automated test
suite or gameplay tests were run. Installation is pending game closure.

After installation, the user should:

1. Launch the usual Mod Engine 2 solo profile and visit the bridge merchant.
2. Open Purchase; verify Spirit Mend appears for 300 runes and other stock remains.
3. Purchase if needed, then verify memorization and casting. An already-owned spell
   should still work without repurchasing.
4. Verify Sell and Leave work normally, and Kale no longer has Study sorcery while
   his ordinary shop remains usable.

Static readback is not in-game acceptance. Merchant-death/Bell Bearing inventory
behavior has not been investigated or validated in this migration.
