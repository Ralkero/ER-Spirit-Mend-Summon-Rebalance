# 0.6.1-solo.4: recognize the observed Seamless solo session

## Confirmed failure and evidence

The solo.3 log for PID 24104 accepted both native redirects, the 1000-entry
registry constructor, all 11 summon call sites, friendly lock-on, Spirit Mend,
and HUD hooks. Task readiness also succeeded. Loading the user's solo character
then moved the protocol from 0 through 5 to 6, while lobby stayed 0 and the game
online flag remained false. The old policy accepted only (lobby=0, protocol=0),
so it reverted BuddyStone changes and passed gameplay callbacks through.
Hook installation was successful; gameplay activation was not.

SDK ProtocolState::Ingame is 6. That enum alone does not prove connected peers.
The user-operated solo session is evidence that rejecting every protocol-6 state
is incompatible with this pinned Seamless profile. The process exited during
this investigation, before membership could be captured. Its exit cause is not
established. We did not launch, control, suspend, or write into the game process.

Ordinary local PE/Capstone inspection of the hash-pinned ersc.dll corroborates
that Seamless consumes CSSessionManager's native player list:

- RVAs 26ed9/26edd read vector first/last at manager+0x78/+0x80.
- RVAs 26f00/26f0c walk 0x100-byte entries and skip local entries at +0xe9.
- RVAs 9077b..907a5 use the same vector/stride/local flag when updating remote entries.
- The associated null-manager diagnostic references CSSessionManager.

SHA256 remains 0a093d8490967bf19c62f895fdf1fda21840282d2b2b371b9b4fc24a0cfe146e.
These are static observations from local tools, not REA Evidence or observations
of remote-peer behavior. Private disassembly captures stay outside the repository.
REA offers no Windows live-process reader in this session. No provider setup or
new dependencies were required.

## Narrow correction

Standalone offline rules are unchanged. Only the existing explicit AllowSeamlessSolo
exception, after exact executable and Seamless hashes pass, may accept (0,6) in
addition to (0,0). It still requires:

- The native game online flag is false and no conflicting/protected module exists.
- Lobby is exactly 0; transition and unknown protocols reject.
- A readable native session object with a structurally valid bounded player vector.
- At most one member, with any present member explicitly marked local.
- An empty host record, or a host identity equal to that local member's nonzero identity.

Multiple members, remote members, a nonlocal host, missing state, malformed vector
bounds, or an unknown bool encoding reject. The vector header is validated as
scalar addresses before constructing a native slice or dereferencing one entry;
compile-time SDK layout assertions corroborate the pinned native offsets. Membership
is evaluated afresh in the callback, never cached as permission. There are no
retained actor pointers, added native hooks, per-frame page probes, new FP/HP rules,
asset changes, or config changes. Remote-session support remains unvalidated and
unsupported; this is not a networking/replication implementation.

The native player-list lifecycle still relies on the existing game-thread callback
assumption. Static code confirms usage of these fields by Seamless, not complete
network lifecycle or race coverage. Those limitations are not treated as passed tests.

## Diagnostics

Activation gates now include `seamless_solo`, `solo_members`, and `offline_allowed`.
For this profile, look for online_flags=(false,...), session_states=(0,6),
seamless_solo=true, and offline_allowed=true. solo_members should show count 0 or 1,
all_local=true, host_is_local_or_empty=true. Native identifiers are compared but
never logged. Existing change/10-second logging cadence is retained.

After acceptance, BuddyStone captured_rows should be 304 on this installation;
lock-on and summon snapshots should run when the user performs those actions.
A successful startup line alone still does not establish activation or gameplay.

## Validation

- Rust Windows release DLL: compiled (normal gameplay feature set).
- Rust test harness: compiled with --no-run; no automated tests executed.
- Five new pure regression tests prepared: pinned/standalone separation, peer and
  host rejection, transition/online/unknown rejection, fresh membership decisions,
  and malformed vector bounds. Existing offline rejection tests are retained.
- Clippy completed with the same eight pre-existing style warnings.
- Formatting and diff whitespace checked.
- Live membership with solo.4, summons, lock-on, healing and performance: pending
  user-operated testing. Connected sessions: unsupported and untested.

Build commands from repository root (test command compiles only):

```powershell
cargo fmt --all --check
cargo build --lib --locked --release --target x86_64-pc-windows-msvc --target-dir ../../work/seamless-solo2-build
cargo test --lib --locked --no-run --target x86_64-pc-windows-msvc --target-dir ../../work/seamless-solo2-build
cargo clippy --lib --locked --target x86_64-pc-windows-msvc --target-dir ../../work/seamless-solo2-build
```

The DLL-only updater requires a closed game, verifies the installed solo.3 hash,
backs up DLL/log, generates rollback, and preserves the INI/launcher/26 protected
config and asset files. It never changes saves or game-root files.

## User acceptance

1. Launch the usual Mod Engine 2 profile solo, load the character, wait 15 seconds,
   and request a log check. Confirm offline_allowed=true before combat testing.
2. Summon Wolves on level ground, replenish FP, then summon Militiamen. Expect both
   groups to remain without the dismissal prompt and normal costs once per summon.
3. Lock onto a Spirit and cycle to other Spirits and enemies; report FPS.
4. Cast Spirit Mend at an injured owned Spirit and observe healing/native FP cost.
   Existing same-Ash heal-on-reuse remains enabled; duplicate mode stays default-off.
5. Rest/reload and repeat a summon/lock-on once. Connected testing is not requested.

If the membership guard rejects, capture its new scalar diagnostics; do not remove
membership/host checks to force activation.
