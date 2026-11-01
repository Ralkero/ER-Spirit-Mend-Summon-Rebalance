> Current main publication, 2026-11-01: solo.5 source publication is authorized. Use CURRENT-COMPATIBILITY.md for the supported pinned Seamless SOLO setup. The narrow Clever + Terra merge exists; universal merging and connected co-op remain unsupported. The older checklist below records the earlier development boundary. Test execution remains reserved for the user; no binary release is implied.

# Experimental build and release checklist

No automatic publication. A compiling DLL is not an accepted release. Current development packages are local only; no install, push, tag, merge or release performed.

- [ ] Record source commit, clean/dirty state, toolchain/lockfile and artifact hashes.
- [ ] Windows x64 release compilation; compile C# builder against pinned open-source dependencies.
- [ ] User runs Rust tests; record actual counts/results, never substitute --no-run.
- [ ] Formatting and Clippy checks; distinguish existing lint debt from new diagnostics.
- [ ] Compare manifest IDs and spell balance to0.5.1; no proprietary assets/dependencies committed.
- [ ] Complete invalid-cast/flight acceptance in VALIDATION.md.
- [ ] Complete duplicate off/on, cap, costs, group lifecycle and targeting tests.
- [ ] Record performance with increasing small group counts and HUD off/on; retain callback metrics.
- [ ] Validate supported WW2.7.2.0/App1.17.2 and exact hook signatures. No broader version claim.
- [ ] State pending HP-cost, boss, mounted-Ash and compatibility cases explicitly.
- [ ] Package DLL, optional INI, licenses/notices and docs; generate matching assets from user's own files.
- [ ] Back up isolated mod profile, DLL/INI/assets with hashes while game is closed; preserve saves/game root.
- [ ] Install into separate offline profile, preserving settings. Do not run alongside other summon hooks or Seamless Co-op.
- [ ] Rollback restores profile backup while game is closed; no save editing or automatic spell removal.
- [ ] Obtain explicit authorization for push/merge/tag/release.

## Later milestones

Phase3: priority selection (locked-on, wounded, healthy) with hysteresis is deferred; current small HUD order stays accepted/stable, instance labels added only.

Phase4: field-level merger is deferred. Audit identified missing strict version/template expectations and nontransactional partial-output behavior. Preserve current full-overlay workflow; no universal overhaul compatibility or Terra Magica integration is claimed.

Phase5: Windows CI is deferred until source checks are settled. Prefer manually dispatched CI while the user reserves test execution; future jobs may build Rust, run tests/Clippy/fmt and validate fixtures, plus compile the pinned C# builder. Asset generation requires user-owned regulation/messages/Oodle and must stay outside CI. No automatic binary release.

Phase6: current entry points link to audit and duplicate guide; historical documents and evidence remain intact.
