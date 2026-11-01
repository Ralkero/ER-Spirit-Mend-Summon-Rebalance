# Development audit: published 0.5.1

Reference: `0506238b90577b123a289a6e625c5ac1d5d0e7b2`, verified against fetched origin/main on 2026-10-30. Development is isolated on `dev/0.5.1-stabilization-duplicate-groups`. No installed files, settings, saves or game-root files are changed. SOURCE-MANIFEST.json remains the historical publication manifest for that commit, not an integrity manifest for subsequent development.

## Evidence categories

| Area | State and evidence |
| --- | --- |
| Different Ash groups, per-family item healing | Implemented; prior user observations support wolves/Militiamen coexistence, isolated healing and native costs at 60 FPS |
| Resummon and transitions | Implemented; prior user evidence for whole-group resummon, rest, death, reload; mixed-group 0.2.2 correction lacks explicit acceptance; boss resummon was not actually attempted |
| Four individual HUD rows / unrestricted zones | Implemented; user accepted UI and Elleh/Gatefront travel; not universal location coverage |
| Friendly lock-on | Implemented; user validated DualSense cycling and rest/reacquisition |
| Spirit Mend impact healing | 0.5.0 gameplay-validated for three wolf impacts, clamping and Lusat 8 FP cost; not every catalyst or Ash |
| 0.5.1 invalid release and target persistence | Implemented, prior compilation/static-validated; gameplay pending |
| Current regression additions | Prepared; test execution reserved for user; compilation is reported separately |

## Phase 1 audit and correction

Read the current guides and historical architecture/validation, hook source, scalar policy, ownership/lifetime paths, configuration, asset manifest and builder. No new hook, address, native ABI, resource operation or asset change is needed for the first correction.

Confirmed source-level defect: the old impact ledger expired every key after five seconds regardless of whether its projectile still existed. That does not establish an observed double heal, but contradicts lifetime-long duplicate suppression. The replacement retains consumed full generation-bearing projectile handles until a complete bounded native registry walk proves absence. Truncated/ambiguous walks never prune. A full 256-entry ledger rejects additional deliveries instead of evicting live keys. Claim remains before effect invocation, conservatively preventing retries of uncertain native results. Only scalar handles persist; scans occur on custom impacts, not per frame.

Pure mend_policy eligibility and ledger rules now have prepared regression cases for missing/dead/full/foreign targets, independent targets, duplicate impacts, live retention, new generations and capacity. Existing health arithmetic, ownership/session, version/layout and configuration tests remain. Compact call-byte fixtures derived from the supplied hook-review.json cover six native call encodings and changed-target rejection; these are historical fixture tests, not a fresh executable inspection or ABI proof.

## Remaining native assumptions and risks

- Both cast callers statically branch around native resource commit after false spawn. Actual zero-FP behavior, interrupted/repeated casting and both native paths need user observation.
- The constructor writes the captured full handle after native initialization. The engine launch context, not mutable camera selection, is inspected at impact. Packet +1d8 and bullet launch offsets are signature/layout guarded but native lifetime at collision still needs runtime evidence.
- Registry traversal and current actor resolution rely on serialized native callback ownership. Rust catch_unwind does not catch access violations or make concurrent engine destruction safe.
- A validated launch can spend FP and later miss, lose its actor, or be suppressed when the target is full. There is no refund.
- Traversal over 256 bullets can fail closed; a valid paid cast may then not heal. This bound is deliberate and not a performance promise for large battles.
- Existing hook-install rollback/relay lifetime and unsupported assets without active hooks are limitations: an inert custom spell can still spend native FP when hooks are disabled. No new online or version allowance was introduced.

## Asset compatibility audit (implementation deferred)

The manifest preserves seven cloned rows: Magic/Goods990001, Bullet/Atk99000100, SpEffect99000100/99000101, shop100521; English FMG categories10/20/24 use990001. The builder rejects row and text ID collisions. It creates output early and can leave a partial directory on failure. It does not strictly allowlist regulation version or compare template fields against an original baseline, and its attack damage safety currently relies on the chosen friendly template. Do not treat it as a verified merger for overhauls. A future transactional merge should validate the exact input version, compare template field expectations, stage all parameter/message outputs privately, then promote only a complete output; keep source files untouched. No spell balance or acquisition edits in this pass.

## Phase 1 build record

Windows x64 release compilation succeeded: `cargo build --lib --locked --release --target x86_64-pc-windows-msvc --target-dir ../../work/stabilization-build` (17.62s). `cargo test --lib --locked --no-run --target x86_64-pc-windows-msvc --target-dir ../../work/stabilization-build` compiled the test executable (19.72s). No tests executed, no gameplay launched.

Development follow-up: the ledger also resets on absent/dead player or world/full-player-handle changes, observed by the existing game task without projectile scans. Impact establishes the same context before claiming, so the first subsequent task does not clear a new claim. A prepared regression covers reload and player generation changes.
