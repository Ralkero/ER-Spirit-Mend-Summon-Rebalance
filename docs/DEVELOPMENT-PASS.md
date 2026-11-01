# Development pass: 2026-10-30

Baseline0506238 matched fetched origin/main. Work is isolated on dev/0.5.1-stabilization-duplicate-groups. Local milestone6ac82d2 contains the initial stabilization correction; the subsequent duplicate prototype is version0.6.0-dev.1. No push, merge, tag, release, installed-mod or save change.

## Delivered source

New mend_policy.rs: pure target eligibility and lifetime-scoped impact ledger. New groups.rs: serialized pending/active group accounting, limit and member identity policy. New duplicate_groups.rs:10Hz native registry adapter, exact-handle membership, reservations and diagnostics. multi.rs routes optional same-Ash requests through existing native paths; runtime.rs schedules bounded observation/session cleanup. config.rs adds default-off duplicate mode and1..8 cap. hud.rs/hud_snapshot.rs add G-id labels while preserving four-row size/order. policy.rs supplies testable eligibility/capacity rules. hooks.rs adds supplied native-call fixture checks. timing.rs adds duplicate_sample. Cargo version is explicitly experimental. Build.ps1 includes current guides and optional example configuration in new packages.

Current guides: STABILIZATION.md, DUPLICATE-GROUPS.md, RELEASE-CHECKLIST.md; README/ANALYSIS/IMPLEMENTATION/VALIDATION/CHANGELOG/SPIRIT-MEND/PUBLICATION point to the new state. Historical text and evidence retained. Default config remains duplicates=false; a separate DuplicateGroups-Experimental.ini is provided, with no automatic active-INI edits.

## Defects and uncertainties

Confirmed source defect: five-second delivery-key expiry could forget a still-live projectile; corrected with complete-registry lifetime pruning and world/player session reset. No observed duplicate-heal incident is asserted. Compilation caught two intermediate implementation errors, fixed before successful builds.

Native0.5.1 release/impact callback ordering and same-Ash duplication are not runtime-accepted. Incomplete accepted spawn observations deliberately block further requests until native dismissal/load/death reset. Mounted/special group cardinality, cost timing and performance need gameplay evidence. Asset builder audit found incomplete strict version/template conflict protection and partial-output behavior; merger work is deferred, not claimed fixed.

## Commands and actual outcomes

See evidence/development-0.6.0-dev.1-build.json. Windows release compilation succeeded; all lib test code compiled with --no-run. No tests executed, passed or failed. Strict Clippy initially failed on style lints; two new lints corrected, normal Clippy succeeded with eight baseline warnings. No full-tree formatting acceptance is claimed. No C# rebuild or asset regeneration was needed because the builder/assets were not modified; prior publication build evidence is retained separately. Initial diff whitespace warnings were CRLF conventions; use git -c core.whitespace=cr-at-eol diff --check for this byte-preserving repository.

## Milestone status

| Phase | Status |
| --- | --- |
| 1 audit/stabilization | Source work complete for this pass; regression code compiled; automated/gameplay acceptance pending |
| 2 duplicate prototype | Implemented/compiled, defaultoff; partial milestone until user tests and special-Ash gaps resolved |
| 3 HUD | Instance labels implemented; smarter priority/hysteresis deferred |
| 4 parameter compatibility | Audited; transactional field merger deferred |
| 5 release infrastructure | Separate builds/checklist and build packaging updated; CI/strict lint cleanup deferred |
| 6 docs | Current entry points updated, historical record retained |

## Next user action and implementation

Do not replace the installed mod while it is running. Use a backed-up separate offline profile with the development DLL, existing matching0.5.1 assets and a copy of the current INI. Begin with duplicates=false and VALIDATION.md steps1..10, then enable the two duplicate keys and follow DUPLICATE-GROUPS.md. First batch: three wolves, then six,55FP per use, first group's HP unchanged, stableFPS. Next: third group then cap rejection, targeted Mend, partial/permanent deaths and transitions. Stop/report unexpected counts, costs or stalls before broad testing.

Next engineering step: reconcile user release/flight and first two-group logs; refine any demonstrated native request/cardinality issue before HUD priorities or parameter merging. The pending-request conservative failure mode must be observed before considering any automatic recovery rule. No timeout-based cancellation should be invented without native evidence.
