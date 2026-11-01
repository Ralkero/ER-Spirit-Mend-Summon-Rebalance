> Historical probe milestone. The later solo-only build is documented in [SEAMLESS-SOLO.md](SEAMLESS-SOLO.md); connected compatibility remains pending.

# Connected Seamless Co-op compatibility investigation

2026-10-30. Branch `dev/seamless-compatibility`, based on local development commit
`2e2feadd3abccee55fb3bc48dfaf98b5153d0ca0`. Published 0.5.1 remains
`0506238b90577b123a289a6e625c5ac1d5d0e7b2`.

## Status and scope

**Implemented / compilation-validated:** a separate, read-only diagnostic build.
**Not gameplay-validated:** this probe, connected Spirit Mend, replicated healing,
multiple Spirit groups in co-op, friendly lock-on in co-op, and co-op HUD support.
The existing gameplay DLL still rejects Seamless. No session gate was relaxed.
The user's new request explicitly includes connected Seamless sessions; earlier
multiplayer exclusions remain historical context, not the new development scope.
Protected vanilla online play and EAC bypass remain unsupported.

This is not a working compatibility release. Enabling the existing gameplay DLL
by deleting its `ersc.dll` guard would not establish network synchronization or
safe ownership. No gameplay tests or automated tests were executed by the agent.

## Evidence and unresolved mechanisms

- Installed WW executable 2.7.2.0 SHA256:
  `F9118FEA2CFD8FA3901E8060BFADB1552CB2471771DED8A5B662C4E298FFB328`.
- Installed Seamless DLL SHA256:
  `0A093D8490967BF19C62F895FDF1FDA21840282D2B2B371B9B4FC24A0CFE146E`.
  PE exports expose only `modengine_ext_init`; no exported integration API was
  found. Binary strings reference `CSSessionManager` and a Seamless buddy system.
  This does not prove that no private integration mechanism exists.
- REA recognized the PE; native decompilation was unavailable because no native
  analysis provider was configured. PE/export/string inspection used local tools.
  An incomplete linear-disassembly xref search is not negative evidence.
- The upstream capacity adaptation deliberately leaves constructor body bytes
  recognizable by Seamless. This is source evidence, not current runtime proof.
- Current healing and target validation exclude remote group entries and bind to
  the local player's full handle. These checks must not simply be removed: remote
  actors may be proxies and damage authority may differ from summon ownership.
- Heal-on-Ash-reuse directly writes validated actor HP locally. Replication is
  unproven. Spirit Mend invokes a native effect and preserves `no_sync`, but that
  alone does not prove remote clients agree or prevent duplicated healing.
- Existing Spirit Mend Magic row sets multiplayer availability bits to zero.
  Co-op eligibility requires deliberate review alongside runtime gates.
- The custom HUD uses RemotePCTag0..3. Connected play can need those rows for
  human players. Initial gameplay compatibility should leave native party HUD
  intact; don't overwrite human player rows with Spirit rows.
- The active ME2 profile contains Clever and a higher-priority Terra Magica
  overlay, based on regulation 11701000. Existing Spirit Mend assets were built
  from 11730034. A verified field-level merge is needed, including Clever's
  English item_dlc02 text. No overlay was replaced or merged in this pass.
- Seamless's [modding guide](https://ersc-docs.github.io/seamless-modding/)
  documents Mod Engine loading, matched gameplay mods across peers, and a host
  `allow_summons` setting. It is not a supported native API or proof of current
  internal layouts. The inspected profile has `allow_summons=1`.

## Probe implementation

`cargo build --lib --locked --release --features seamless-probe --target
x86_64-pc-windows-msvc --target-dir ../../work/seamless-probe-build`
(run as one line from the repository root).

Copy the resulting DLL under the distinct filename
`SpiritMendCompatibilityProbe.dll`. Do not load it as `SpiritAshResummon.dll`.
Compile-time selection excludes runtime, hook, capacity, HUD and timing modules.
The default build continues to use the existing runtime. Do not use `--all-features`
for a gameplay release: that selects the diagnostic entry point.

At startup a worker hashes the executable and loaded Seamless file before using
game structures. Exact known hashes are required. The probe refuses detected EAC
modules and conflicting gameplay DLLs, and repeats loaded-module checks while
sampling. It neither disables anti-cheat nor establishes a safe online session.
Launch only with the existing Seamless/Mod Engine profile.

The SDK scheduler registers a recurring GameMan task. Sampling occurs at most
once per second on the game thread. It records raw lobby/protocol values, online
flags, player entry count, local event identity/full handle, registry capacity,
native summon request state, and bounded group metadata. Each group summary is:

`(owner_event_id, belongs_to_local_player, local_entries, remote_entries,
retiring_entries, warping_entries, buddy_param_ids)`

These are native **actor entries**, not independent summon-instance counts. The
probe never dereferences member actor pointers, reads actor HP, writes HP/FP,
sends packets, or creates projectiles. It retains scalar snapshots only. Native
manager access uses a bounded page check; map/list walks use SDK layouts with
32-owner/128-member limits. SDK container stability on the game thread is still
an unvalidated Seamless assumption. Rust panic containment cannot catch Windows
access violations. Stop the test if instability appears.

Every five seconds 25 relevant hook/function sites are compared to 16-byte
original-executable fixtures; only changed fingerprints are logged. `original=false`
means another component changed bytes; it does not identify the writer or prove
incompatibility. Matching entry bytes do not prove an entire function untouched.
Metadata logs only on changes or a 15-second heartbeat. `metadata_us` measures
snapshot collection, excluding log I/O and hook-byte inspection. No passwords,
Steam identities, player names or save contents are logged.

## Installation and rollback

The source includes `tools/Install-SeamlessProbe.ps1`. Supply `-ProfileRoot`,
`-PackageDirectory`, `-GameExe` and `-SeamlessDll`; preview is the default. `-Apply`
adds only `dll/spirit-mend-probe/SpiritMendCompatibilityProbe.dll` to the existing
external DLL list. Existing Seamless, quest log, transmogrify, mod ordering and
launcher stay in place. It verifies package hashes, refuses a running game,
backs up configuration, and checks protected parameter/message/DLL/settings files.
It never accesses player saves. A prior probe installation is refused.

The local package `manifest.json` identifies kind `seamless-read-only-probe-1`,
source commit and hashes for the DLL and this document. Build a fresh package
directory; generated binaries, local paths and installation evidence stay out of
public source. No game assets are included.

Installation records and a guarded `Restore.ps1` are written under the profile's
`backups/spirit-mend-probe-TIMESTAMP`. Restore defaults to preview; use `-Apply`
with the game closed. It refuses if configuration has changed since installation.
Disabled probe files/logs are retained for analysis. No DLL hot-unload is supported.

## User-operated diagnostic procedure

1. Start the normal active ME2 launcher. Load the current character alone and
   wait 20 seconds. Note FPS and any unexpected behavior.
2. Host a normal Seamless session; have a player join. Wait 15 seconds with both
   players nearby. This probe adds no gameplay content; the other player does
   not need it for this first observation.
3. In a normally valid summoning location, summon Lone Wolves once. Keep them
   present for 15 seconds. Ask the other player to summon a different native Ash,
   if their current setup permits it, then wait another 15 seconds.
4. Tell us which player hosted, which player summoned each Ash, whether both
   players saw all Spirits, and FPS. Leave the game open for log inspection.
   Do not install the existing Spirit Mend gameplay DLL into either peer yet.
5. After reviewing this capture, repeat with roles reversed if necessary. Then
   capture rest, disconnect/rejoin and owner departure to establish lifecycle
   behavior. These are follow-up tests, not prerequisites for the first capture.

Log: `dll/spirit-mend-probe/SpiritMendCompatibilityProbe.log` in the ME2 profile.
`STOP:` means sampling did not start or was disabled. `SAMPLE` captures session
and ownership state; `SITE` identifies changed code at prospective hook points.
Absence of errors is not proof of multiplayer healing synchronization.

## Next implementation gates

1. Establish host/guest session and registry ownership from actual captures.
2. Review all changed hook bytes; retain full signature checks for unchanged
   sites. Avoid unsafe hook chaining or copying another DLL's trampolines.
3. Add a pinned opt-in compatibility policy with explicit local-authority
   operations, transition handling, and no custom party HUD initially.
4. Merge assets with exact collision/schema guards while preserving Terra and
   Clever. Enable the spell in supported Seamless roles only after review.
5. Validate one owned Spirit Mend heal on both peers, once-per-impact behavior,
   FP commitment and authority after reconnect. Then add multiple/duplicate
   groups, Ash-reuse healing, and HUD compatibility in separate milestones.
6. Only after that evidence should connected compatibility be advertised or the
   full gameplay build be installed into this active profile.
