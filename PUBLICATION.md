# Main publication: 0.6.1-solo.5 (2026-11-01)

The user authorized replacing the main repository baseline with the compatible
Mod Engine 2 / pinned Seamless solo version. A clean publication commit is based
on the existing public main (0506238), importing the current tracked development
tree without publishing intermediate local history or untracked artifacts.

Runtime and vendored binding files are unchanged from the installed solo.5 source.
Publication changes consolidate the README, document compatibility as the primary
setup, add CompatibleSolo.ini matching the active gameplay toggles, and make the
build package select that preset by default. Standalone defaults remain available.
No new native hooks or game-balance changes are introduced for publication.

SOURCE-MANIFEST.json hashes all current tracked files except itself. The old
manifest is preserved at docs/SOURCE-MANIFEST-0.5.1.json. publication-copy-map.json
and docs/HISTORY-PUBLICATION-0.5.1.md describe the original import only. Archived
build/install reports retain their historical versions and acceptance boundaries.

Included: Rust source and vendored bindings; pinned toolchain/lockfile; current and
historical INIs; three C# asset tools and licenses; guarded profile migration tools;
architecture, validation and provenance documentation; sanitized existing evidence.
The merchant migration source is included, but its installation/gameplay remains
pending. This publication does not apply the staged migration locally.

Excluded: generated regulation/message/dialogue bundles, extracted game assets,
game executable/archives, Oodle/mod-loader/other-mod binaries, saves and save-repair
artifacts, credentials, personal paths, live logs, local packages/backups, DLL/PDB
build outputs, and unrelated repositories. .gitignore adds modded-save exclusions.
No new binaries, tags or GitHub Release are published. Main is the current source
baseline; it is not an assertion of complete gameplay acceptance.

Build/inspection evidence for this publication is recorded at
[evidence/publication-0.6.1-solo.5.json](evidence/publication-0.6.1-solo.5.json).
Tests and gameplay remain user-operated. Publication changes no installed game,
mod profile or save files. Connected co-op remains unsupported.
