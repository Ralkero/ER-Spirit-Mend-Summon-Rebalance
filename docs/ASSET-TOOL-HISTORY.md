# SpiritMendAssets (offline build tool)

SPDX-License-Identifier: GPL-3.0-or-later. This tool uses SoulsFormatsNEXT and is separate from, and never linked into, the Rust MIT/Apache runtime DLL. See LICENSE. Locally generated regulation/message assets contain game data: do not include them in a public source release; distribute the builder/patch manifest and build against the user's own files.

Required: .NET9 SDK, local Elden Ring regulation.bin and oo2core6 DLL, extracted English item message bundles, and these workspace dependencies:

- https://github.com/soulsmods/SoulsFormatsNEXT at 9e1c36f54962f80c143e3c1fe8a709204aefacf9 in workspace work/SoulsFormatsNEXT.
- https://github.com/soulsmods/Paramdex at ff7245e524329bc3eab00036723d2bd53384cedf in workspace work/Paramdex.

Apply oodle-legacy-printf.patch to the pinned SoulsFormatsNEXT checkout. It makes the absent logging callback optional while preserving version querying, compression and decompression exports. The game DLL is loaded from its original directory only. No game files are edited. The C# project references the workspace checkout; adjust ProjectReference if relocating the source tree.

From the workspace directory:

```powershell
dotnet build outputs/SpiritAshResummon/tools/SpiritMendAssets -c Release
dotnet outputs/SpiritAshResummon/tools/SpiritMendAssets/bin/Release/net9.0/SpiritMendAssets.dll `
  'C:/Program Files (x86)/Steam/steamapps/common/ELDEN RING/Game' `
  work/Paramdex/ER `
  outputs/SpiritAshResummon/development-next `
  'E:/Elden Ring Unpacked Files/msg/engus'
```

The output directory must not exist. With all four arguments, the tool inventories template rows then builds isolated mod assets and a patch manifest. Three-argument inspection uses this workstation's extracted English message path; pass the fourth argument for a portable build. On failure, preserve the partial output as diagnostic evidence and use a new output directory after correcting the problem.

Original parameter rows are cloned into collision-checked IDs. Existing row values and binder entries are preserved through library serialization. Strict schema parsing stays enabled. SpEffect byte0x353 has three unnamed high bits in the pinned schema; the tool adds an explicit3-bit field so these bits are retained. Fields are edited by name and converted by SoulsFormats cell types. This is asset generation, not proof that the game will accept the spell or that native casting/healing works. See ../../SPIRIT-MEND.md for user tests and unimplemented behavior.

The provisional shop slot100521 is within the assumed Kalé range; its actual visibility needs gameplay confirmation. Parameter text names are diagnostic only; in-game names come from Goods FMGs. No custom icons or animation files are generated.
