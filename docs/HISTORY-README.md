# SpiritAshResummon 0.5.1 - Spirit Mend release validation

Current local release: native preflight rejects Spirit Mend when no injured, living, owned Spirit is locked on at projectile release. Rejected releases create no shard and skip native FP/resource commitment. The casting animation may still play. Accepted shards retain the original target in native projectile launch data; impact validation no longer depends on current lock-on.

Build succeeded for Windows x64. This release still needs user gameplay acceptance; no automated or gameplay tests were run by the agent. The previous 0.5.0 impact healing was user-validated, including 8 FP per cast with Lusat's staff. See [SPIRIT-MEND.md](SPIRIT-MEND.md) for the current procedure and limitations.

Use the usual offline `tools/Launch-Test.ps1` (or `Launch-SpiritMend.ps1`). Existing purchased/memorized Spirit Mend is reused; no repurchase or save edit. Corrected Kale shop assets, configuration and other summon features are retained. Installation and backup are recorded in `evidence/install-0.5.1.json`. Earlier release sections below are historical.

---

# 0.5.0 local installed-profile instructions

Use the usual `tools/Launch-Test.ps1` offline launcher. The 0.5.0 installer deploys the DLL plus isolated spell assets under `dist/mod` and enables Spirit Mend. `tools/Launch-SpiritMend.ps1` uses the same installed profile. Get Spirit Mend from Kale at Church of Elleh for300runes, then memorize it at grace (12INT, one slot,5FP). See SPIRIT-MEND.md for the first impact test and known limitations. The installation manifest is evidence/install-0.5.0.json; earlier staged-build descriptions below are historical.

---

# SpiritAshResummon 0.5.0 — Spirit Mend development build

A separate Swift Glintstone Shard-based healing sorcery is compiled and staged in development-0.5.0. Existing dist0.4.2 remains installed. Read [SPIRIT-MEND.md](SPIRIT-MEND.md) for behavior, known cost/target limitations, separate launcher and first user-run test. This build is not gameplay-validated.

---

# SpiritAshResummon 0.4.2 — friendly Spirit lock-on prototype

Adds optional native lock-on admission for the local player's living Spirit Ash entities. The camera's existing range, visibility, selection and directional cycling code then processes enemies and eligible owned Spirits together. This behavior is implemented and compiled; acquisition, retention and mixed cycling still require the user-operated gameplay test in VALIDATION.md.

The installed prototype enables `[Targeting] EnableFriendlySpiritLockOn = true`. Set it to `false` and restart to restore native targeting. The example/default configuration leaves this experimental feature off. Selected-handle diagnostics remain enabled in the local package. Native enemy relation results pass through; NPC allegiance, damage rules and AI are not changed. Existing group healing on Ash reuse, normal costs, unrestricted zones and four individual HUD rows remain as configured. Duplicate same-Ash groups and the Intelligence healing sorcery are not implemented in this release.

Installation uses the existing offline Mod Engine 2 test profile and dist DLL. Do not combine this prototype with another camera/lock-on hook at the same site. Startup verifies the supported executable version and exact camera call/context and function signature; a mismatch disables this feature. Fully exit before swapping the DLL. Restore the previous dist backup to roll back, or disable the setting and restart. No game installation files or saves are edited. Earlier version sections below are historical; current results are at the top of VALIDATION.md.

---

# SpiritAshResummon 0.4.1 — read-only lock-on diagnostic

This build retains0.4.0 gameplay and adds an optional selected-target observer for the planned Intelligence healing sorcery. It does not enable duplicate same-Ash summons, friendly lock-on or spell casting. `[Debug] EnableTargetDiagnostics=true` together with EnableLogging=true enables5Hz sampling and change/10-second heartbeat logs. Defaults are off; the local diagnostic package enables both.

The observer reads typed PlayerIns.locked_on_enemy and ChrIns.is_locked_on after exact native instruction/layout checks, then compares the selected scalar handle against current validated local Spirits. No arbitrary enemy pointer is dereferenced, no camera hook or game-state write is added, no health read is added and no per-sample VirtualQuery is used. A diagnostic signature mismatch disables only the observer. See the current procedure at the top of VALIDATION.md. Existing gameplay details below remain applicable unless explicitly superseded.

---

# SpiritAshResummon 0.4.0 — unrestricted summon zones

0.4.0 adds optional summoning outside vanilla Spirit Ash zones, including cleared boss areas. The local 0.4.0 package enables it with `[SpiritAsh] PreserveSummoningRestrictions = false`. Set it back to `true` and restart to restore vanilla zone rules. Missing INI/default configuration still preserves zones. Normal costs, animations, offline-only operation, one group per unique Ash, healing on reuse and the compact four-entity HUD remain in place.

This uses runtime monument parameter rules plus a nearest-monument fallback and player-relative spawn origin. It is compiled but not gameplay-validated. Native collision/navigation placement, scripted item-use locks (including potentially hub/cutscene restrictions), rest/death/loading cleanup and the 500-actor technical ceiling remain. This is not a guarantee that a summon can physically spawn on every ledge or in every scripted scene. See VALIDATION.md for the first Church of Elleh test.

**0.3.1 replaces group-total bars with up to four individual Spirit bars.** Each row uses that entity's own current/max HP and a localized Ash name plus member number (for example, Lone Wolf Ashes #1). Valid living members keep their order/number while continuously tracked. The first four tracked entities are shown; overflow stays summoned and fills available rows as earlier members disappear. Wolves plus Militiamen therefore show four of five entities, not five bars. Death/unknown lifecycle states remove only the affected row, and a reviving skeleton can reappear with a new row position/number.

The approved compact appearance is retained: scale 0.82, spacing 45, offsets -91/-350. Existing summoning, group-wide healing and resource-cost behavior is unchanged. The user reports 0.3.1 working well with no issues in the new UI; detailed 0.4.0 gameplay validation remains pending. 0.3.0's two-group display and each Ash's combat/healing display were observed working by the user.

The runtime edits only the recognized original HUD movie in memory. No game assets are shipped. Unknown movie fingerprints or hook conflicts preserve the original movie. Other friendly-NPC tags suppress custom row injection; the first four native tag positions can still reflect this layout. After a runtime HUD fault, automatic restoration of the loaded vanilla panel is not implemented: exit and set [HUD] EnableGroupHud=false, then restart. Avoid combining this prototype with another HUD replacement.

`EnableGroupHud` retains its existing name for configuration compatibility; in 0.3.1 it enables the individual-entity HUD, not the old aggregate display. The display limit is fixed at four; summon capacity is independent.

HUD settings (read at startup):

```ini
[HUD]
EnableGroupHud = true
OffsetX = -91.0
OffsetY = -350.0
RowSpacing = 45.0
Scale = 0.82
```

Offsets are relative to original UI placements, in HUD units, not physical display pixels. X offsets each native row; Y offsets from Buddy's original vertical position. At the extracted default scale, row origins are about (105.7, 211), (105.95, 256), (105.95, 301), and (105.95, 346). Child text/bar offsets and runtime HUD scaling affect the final screen appearance. Scale accepts 0.5–1.5, spacing 24–150, offsets -1000–1000. Setting changes require restart. See VALIDATION.md for the first manual test.

A standalone Rust DLL for offline, single-player Elden Ring. Different Spirit Ash types can coexist, with one group per type. Reusing an Ash heals only its own surviving members by 25% of each member's maximum HP. If every member of that type is gone, reuse requests a fresh group.

**0.2.2 is a compiled, staged correction awaiting user gameplay testing.** A later 0.2.1 test failed to resummon wolves while Militiamen remained active: inactive old members blocked the adapter. The correction requests native cleanup only after guarded lifecycle checks. Its effectiveness and skeleton behavior remain unverified.

**0.2.1 has successful user-operated coexistence and healing observations in both directions:** three wolves and two Militiamen remained present at stable 60 FPS; wolf reuse healed only wolves for 55 FP, and three Militiaman reuses healed only Militiamen for 44 FP each. Healing amounts and wolf HP capping match the intended formula. Resummoning one type while another survives and other regressions still require testing. All gameplay and automated test execution is left to the user; boss-fight resummoning remains deferred.


## Intended behavior

- Summon different Ash types together. Upgrade variants count as the same type, preventing duplicates through different upgrade levels.
- Reuse one living type to heal that type's survivors only, with normal FP/HP cost once. Other types remain untouched; dead individual members are not revived.
- Full-health reuse is a no-op. Entirely dead types can be resummoned without dismissing other types.
- With PreserveSummoningRestrictions=false, ordinary monument location/region and boss-clear restrictions are removed. Animation, scaling, resource requirements and native rest/death/loading cleanup remain. No resource subtraction is implemented in the DLL.
- No configurable Ash-type count limit. A bounded native pool provides **500 Spirit actor slots** (mounted actors may consume more than one slot). Requests that cannot fit the whole group are rejected. This is a technical ceiling, not literal infinite allocation; high-count performance is untested.
- No damage/defense/base-HP changes, passive regeneration, custom follower warps, save editing or regulation.bin editing.

## Requirements and current compatibility

- Windows x64; **WW executable file version 2.7.2.0 (game 1.17.2) only**. Other versions, including 2.7.1.0 and JP 2.7.2.1, are intentionally rejected by this prototype despite upstream supporting them.
- Offline launch through an existing supported mod loader with Easy Anti-Cheat disabled. This package neither disables nor bypasses anti-cheat.
- At runtime GameMan must report offline mode and CSSessionManager must report no lobby and no multiplayer protocol session. Missing state is rejected. The general server-connection capability flag can remain true in the observed offline run and is diagnostic only.
- Do not combine with MultipleSpiritAshes, NativeTweaks, Seamless Co-op, summon-system mods or other hooks at these functions. Recognized conflicting DLLs cause refusal. Unknown hook conflicts are also checked by instruction signatures at startup; this cannot guarantee compatibility with every mod or a DLL loaded later.
- No hot injection or hot unloading. Load at process startup and completely exit the game to unload.

## Installation for controlled testing

1. Read VALIDATION.md. Start with a backed-up, disposable test character/profile; preserve your existing saves. This project has not copied, renamed or edited saves and supplies no save-redirection mechanism.
2. Extract the installable ZIP to a **new folder outside the game directory**. Keep `SpiritAshResummon.dll` and its optional INI together.
3. In a separate clean Mod Engine 2 test profile, set `[modengine] external_dlls` to the DLL's absolute path. Example:

   ```toml
   [modengine]
   external_dlls = ['C:\Mods\SpiritAshResummon\SpiritAshResummon.dll']
   ```

   Keep the remaining standard Mod Engine 2 configuration from its supplied template. Use no other DLL or parameter mods for the initial test. The existing modded co-op profile on this machine is unsuitable and has not been changed.
4. Set the game's launch setting to offline and use the loader's established offline launch procedure. Do not use a protected online launch. Loader integration is documented from upstream and local configuration, but not executed for this build.
5. Inspect `SpiritAshResummon.log` beside the DLL. Startup and refusal diagnostics are always written; `EnableLogging=true` enables activation details. Expect `Startup patches installed: call_sites=11, multi_type=true, actor_pool=520`, then `Game task ready` and a BuddyStone row count. The 520-entry registry reserves its first 20 slots; 500 are available to Spirits. A refusal means the feature is inactive.
6. Load the test area after startup. The parameter change must be present when the game builds the area's monument state. Late loading while playing is deliberately rejected.

Uninstall: exit the game, remove this DLL's entry from the test loader profile, then remove the extracted folder. No game-file restoration is necessary. Do not unload the DLL while the game is running; the game holds its task and hook addresses until exit.

## Configuration

The optional `SpiritAshResummon.ini` is read once next to the DLL. Defaults are multiple types/resummoning/healing enabled, 25%, logging off. `AllowMultipleAshTypes=false` selects the legacy single-type behavior and omits the capacity/spawn-retention patches. `HealPercentage` accepts finite values in `(0, 100]`. Integer HP gains round down. With multiple types enabled, `HealOnReuse=false` rejects living-Ash reuse to avoid dismissing unrelated groups. In legacy single-type mode it keeps native same-Ash dismissal. Setting `AllowResummoning=false` leaves the rest flag untouched.

`PreserveSummoningRestrictions` and `RequireNormalResourceCost` must remain true; false or malformed/unknown settings disable initialization with a diagnostic. They are invariants, not cheat toggles. Full-health reuse is a no-op in the policy, but the actual item/cost ordering still needs gameplay testing.

## Building

Install Rust 1.99.1 with the `x86_64-pc-windows-msvc` target and MSVC C++ build tools / Windows SDK. Run in PowerShell:

```powershell
.\tools\Build.ps1
.\tools\Preflight.ps1 -GameExe 'C:\path\to\eldenring.exe'
```

Cargo.lock and the vendored engine bindings are committed to the source package. `Build.ps1` builds and stages a separate package without running tests or changing the installed `dist/`. Use `-RunTests` only when you explicitly want to execute automated tests. It does not install or launch the mod. `Preflight.ps1` is read-only and verifies file version and on-disk hook signatures. A successful preflight is not a gameplay test.

See IMPLEMENTATION.md for the architecture, VALIDATION.md for completed checks and remaining risks, and UPSTREAM.md for source/license attribution.

