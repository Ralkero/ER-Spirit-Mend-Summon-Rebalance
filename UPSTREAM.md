## Local binding changes in 0.5.1

`vendor/fromsoftware-rs/crates/eldenring/src/cs/bullet_manager.rs` now exposes owner/magic/bullet/target fields and a version-specific explicit target setter. `cs/bullet_ins.rs` adds a read-only native launch context accessor and an offset assertion. These are local changes to the pinned MIT/Apache-2.0 source; original notices are preserved. Earlier statements that the vendor is entirely unmodified are historical.

Since0.5.0 the local installed profile contains derived regulation/message assets generated from this user's game files. It contains no game executable, save or anti-cheat component. Earlier no-game-assets packaging statements describe the pre-sorcery DLL-only releases.

---

# Attribution and provenance

0.4.0 additionally adapts `extras.rs::update_buddy_stone_rules`, `stone_tile`, `nearest_stone`, `apply_stone_fallback` and the used-monument release concept, plus `lib.rs::player_havok_position` / `sync_spawn_origin_for_request`, from the same pinned ArialSenki commit. Changes add conservative local-entry lifetime checks, map/owner cache validation, conditional field restoration and current adapter integration. Original MIT/Apache-2.0 notices apply.

0.3.0 additionally adapts `src/hud.rs` (friendly-tag copy interception and row encoding) and `src/hud_movie.rs` (GFX/SWF parsing, placement transforms, RTTI and MemoryFile discovery, and loaded-buffer replacement), at the same pinned commit. The new collector, optional transactional hook integration, two-row policy, template isolation, asset fingerprint restriction and configuration differ from upstream. Copyright (c) 2026 ArialSenki and the retained MIT / Apache-2.0 licenses apply. No game assets are redistributed.

0.2.2 additionally adapts the per-member disappearance request and two-second delay from ArialSenki's `src/extras.rs::release_dead_groups`. This project substitutes guarded native lifecycle evidence for upstream's direct dead-actor HP predicate. The original copyright and both license files remain included.

This is a new, narrow prototype informed by [MultipleSpiritAshes](https://github.com/ArialSenki/MultipleSpiritAshes), commit `7d2e12c0fd89fedd31ce7d57ee546cf13de2e9f2` (0.2.4), inspected 2026-10-29.

Copyright (c) 2026 ArialSenki. The original MIT and Apache-2.0 licenses are retained unmodified in LICENSE-MIT and LICENSE-APACHE. Rel32 allocation/stub construction, version detection, call-site addresses, BuddyStone flag handling, and summon identity research are derived/adapted from that source. These changes are an independent development; no upstream endorsement is implied.

`vendor/fromsoftware-rs` is copied unmodified from that commit. It includes Vincent Swarte and contributors' MIT / Apache-2.0 notices, the upstream WW 2.7.2.0 address bundle, and original source credits. Its own LICENSE-MIT and LICENSE-ASL2 remain in place. THIRD-PARTY-NOTICES.txt is retained from MultipleSpiritAshes; its package inventory is a superset of this project's dependencies. Cargo.lock pins the upstream dependency versions, with the root package renamed. The copied notice's historical release/version headings describe upstream, not this release.

New project code is offered under MIT OR Apache-2.0 on the same terms. No game executable, regulation.bin, save, art asset, or anti-cheat component is included in the installable package.

0.2.0 additionally adapts the constructor-epilogue allocator/free stub and bounded shared-slot search from `src/engine_patches.rs`, plus the spawn-specific owner-clear interception concept and internal DoSummon state address. The new family policy and scoped pending-request authorization replace upstream's broader multi-summon behavior. Original notices apply to these adaptations.

Relevant upstream sources:

- `src/extras.rs`: `update_buddy_stone_rules` (sets summoned event flag to 0 independently of area rules).
- `src/native_prompt.rs`: `install_native_summon_cost_hooks` and `build_summon_cost_stub` (four GetBuddyState call sites restore native charging).
- `src/lib.rs`: version tables, `buddy_trigger_map`, `chr_alive`, rel32 allocation.
- `vendor/fromsoftware-rs/crates/eldenring/src/cs/world_chr_man.rs`: owned group map, chaining trigger map, engine handle resolution.
- `vendor/fromsoftware-rs/crates/eldenring/src/cs/chr_ins.rs`: native `apply_speffect`; `module/data.rs`: typed HP layout.

The game-specific installation was identified through REA, SHA-256 `f9118fea2cfd8fa3901e8060bfadb1552cb2471771ded8a5b662c4e298ffb328`. REA deep analysis was unavailable because no native decompiler provider was configured. The focused instruction checks in local evidence were instead performed using pefile 2024.8.26 and Capstone 5.0.9. They are static observations, not runtime traces.

