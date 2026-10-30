# Project reading guide

Read README.md and the top of VALIDATION.md first. The numbered development entries in the longer documents are historical; an old “not implemented” statement is not the current status. Source is authoritative for implemented behavior, while user observations are authoritative only for the versions and scenarios actually exercised.

## Reading order

1. `Cargo.toml`, `rust-toolchain.toml`, `src/lib.rs`, `src/config.rs`: build and DLL initialization/configuration.
2. `src/runtime.rs`, `src/hooks.rs`: version/session gates, ownership and registry validation, native calls and transactional patches.
3. `src/policy.rs`, `src/multi.rs`, `src/cleanup.rs`, `src/capacity.rs`: Ash-family behavior, surviving members, resummon cleanup and technical capacities.
4. `src/anywhere.rs`: optional monument/zone restrictions and spawn origin handling.
5. `src/hud.rs`, `src/hud_movie.rs`, `src/hud_snapshot.rs`: native HUD interception, runtime layout and four individual rows.
6. `src/friendly_lockon.rs`, `src/targeting_diagnostic.rs`: native camera eligibility and selected-handle telemetry.
7. `src/spirit_mend.rs`, `src/mend_cast.rs`: native spell release gate, per-projectile target and impact effect.
8. `tools/SpiritMendAssets/Program.cs`, `assets/patch-manifest-0.5.1.json`: cloned spell/goods/bullet/attack/effects/shop/messages. This is a separate offline program, not a DLL dependency.
9. `vendor/fromsoftware-rs`: complete pinned Rust bindings, with the documented local bullet extensions.

`runtime.rs` includes multiple source files into a shared module. Those files intentionally share imports, globals and validation helpers; they are not all standalone Rust modules.

## Spirit Mend data flow

```text
Native casting animation and requirement checks
  -> custom Magic 990001 reaches projectile action
  -> selected actor must be owned, living, injured and locked on at RELEASE
  -> failure: return false before native spawn/cost
  -> native BulletSpawnData receives explicit target handle
  -> native projectile stores its own launch context
  -> successful spawn permits native resource application, including staff modifiers
  -> collision packet identifies original bullet
  -> live registry + owner + full handle + bullet/magic IDs validated
  -> actual receiver must equal original launch target
  -> native instant percentage-healing SpEffect; no direct HP/FP write
```

Changing the camera target after release is irrelevant to impact admission. Valid shots can miss and are not refunded. No actor pointer is retained between callbacks. A small expiring ledger stores scalar projectile/owner handles to suppress duplicate delivery. There is no global “last selected target” shared across shots.

## Native addresses

These are RVAs for the inspected WW2.7.2.0 executable only, not transferable addresses for other versions. Inspected SHA256: `F9118FEA2CFD8FA3901E8060BFADB1552CB2471771DED8A5B662C4E298FFB328`.

| Site/function | Role |
| --- | --- |
| 3c05d6 / 3c0611 -> 3c0f80 | Projectile action; returning false short-circuits resource application |
| 3c0e70 | Following native resource commitment |
| 3c0ff0 -> 3c10c0 | Native spawn-data construction |
| 38e750 / 3950c0 | Explicit target -> native type-2 target record |
| 39e90f -> 38d140 | Launch context copied into CSBulletIns+4b0 |
| 449ffa -> 4489f0 | Impact packet carrying original FieldIns |
| 4493d3 / 4494b8 -> 3e8e70 | Native effect dispatch; custom marker substituted only after validation |
| 7179d6 -> 51b8b0 | Camera relation predicate used by friendly Spirit lock-on |

The complete summon/zone/capacity/HUD hooks are documented in source and IMPLEMENTATION.md. `evidence/cast-static/hook-review.json` retains compact original-byte evidence. Full executable disassembly is deliberately not distributed; `research/history/cast_inspect.py` records the local analysis method and must be adapted to the reader's own executable/path.

## Evidence boundaries and useful review questions

- 0.5.0 paid impact healing succeeded in user-operated gameplay, including independent HUD confirmation. 0.5.1 compiled and was installed, but its new preflight/flight paths have no reported gameplay result yet.
- Review the assumed native ABI, cost ordering, constructor-target priority, registry lifetime at impact and version/signature guards. Static inspection supports them; it does not establish live correctness.
- Do not infer healing success just from `native_accepted=1`; check independently observed HP. Do not infer zero cost solely from a rejection log; the user should observe FP before/after.
- Earlier per-frame memory probing caused severe FPS regressions. Current validation avoids per-frame VirtualQuery and bounds traversals. Avoid reintroducing expensive work in hot callbacks.
- Duplicate same-Ash groups, custom spell icons and final balance tuning remain deferred. Boss-resummon acceptance was not completed before the boss died in earlier tests.
- The initial project scope preserved vanilla zones and allowed one type. Later user requests explicitly changed those policies; current prototype settings reflect the later requests.

No automated or gameplay tests were executed for this publication. Existing test source is included so the user can run it explicitly. Historical compilation/test transcripts retain their original version context.
