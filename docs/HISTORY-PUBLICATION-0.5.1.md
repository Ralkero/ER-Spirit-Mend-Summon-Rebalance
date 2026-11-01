# Development boundary

SOURCE-MANIFEST.json and publication-copy-map.json describe commit0506238, not this development tree. Subsequent local commits are not pushed or released. No game assets or generated proprietary files were added.

# Public 0.5.1 snapshot

This repository contains the complete current runtime source, vendored Rust bindings, Cargo/toolchain configuration, offline C# asset-builder source and license, default and prototype INIs, portable launch/build tools, full development documentation and sanitized technical evidence. `publication-copy-map.json` maps imported material to workspace-relative provenance. `SOURCE-MANIFEST.json` records hashes of the published snapshot.

The runtime `src` and `vendor` trees are byte-for-byte copies of the 0.5.1 local source that produced the installed DLL. The locally installed DLL's SHA256 is `5007A881D42ECCBCD105E48D29E74CCFEE64F8C6A299C1C312A51120ED755D0C`. This hash identifies that local build, not a promise that a build at another absolute path/tool environment will be bit-identical.

Publication-only changes:

- A current README and analysis map supersede older status banners. The original cumulative README is retained in `docs/HISTORY-README.md`.
- The launcher takes the Mod Engine 2 location explicitly instead of embedding a personal folder.
- The asset project's SoulsFormats reference defaults to repository-local `external/SoulsFormatsNEXT` and can be overridden with the `SoulsFormatsProject` MSBuild property.
- Three-argument asset inspection no longer falls back to this workstation's extracted-message path. Four explicit arguments still build the same spell assets.
- A pinned dependency-preparation script and portable instructions are included.
- Personal workspace/user/Steam-account paths in documentation, logs and historical scripts are replaced with placeholders. Source and original evidence hashes recorded in older manifests may therefore describe the unredacted local originals, not the published text. They are historical provenance, not current-tree integrity checks.

Not published: game executable/archives, generated regulation or message bundles, extracted HUD/movie/art files, screenshots, Oodle or mod-loader binaries, saves, raw full disassembly, build caches, PDBs, DLL outputs, repeated staged packages/backups, credentials or chat transcripts. These are generated, proprietary, redundant or installation-specific rather than necessary editable project source. The builder, patch manifest, pinned dependency references, compact native signature evidence and analysis scripts provide the reproducible project-side inputs.

`evidence/` includes sanitized archived gameplay logs, versioned review summaries, build transcripts and local install records. These are technical observations, not newly executed tests. `research/history/` retains original development/analysis/installation scripts for provenance; these can contain obsolete paths, IDs or migration assumptions and are **not the supported setup workflow**. Read them as historical material. Use `tools/` and README.md for current setup. Do not run historical installers against a current installation.

Some historical documents reference raw assembly, old staged directories or screenshots that intentionally stay local. Those references do not imply missing runtime/build dependencies. External third-party source repositories are pinned and fetched by `tools/Prepare-AssetDependencies.ps1`; their complete unrelated history is not mirrored here. The entire vendored Rust binding source needed by Cargo is included.

The existing local installed mod, configuration, game files and saves are not changed by publication. No game was launched and no automated/gameplay test suite was run as part of this upload. 0.5.1 gameplay acceptance remains pending.
