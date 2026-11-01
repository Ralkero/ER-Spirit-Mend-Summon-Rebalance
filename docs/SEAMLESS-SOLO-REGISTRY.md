# 0.6.1-solo.3: preserve the Seamless Spirit registry

## Demonstrated failure

The user still saw the native dismissal prompt and could not lock on to Spirits
with 0.6.1-solo.2. Its startup log accepted both pinned Seamless redirects, then
reported `Disabled: startup patch transaction: Summon registry constructor conflict`.
No gameplay hooks were committed. The earlier redirect correction worked, but
startup rejection still prevented the requested features from running.

## Evidence and correction

Read-only inspection of the already-running process on 2026-10-30 found three
changed 32-bit immediate fields in the complete 115-byte constructor at RVA
0x49560a. Allocation and initialization lengths are both 16000 bytes; the stored
capacity is 1000 entries (16 bytes each). The original executable uses 1280 bytes
and 80 entries. All other bytes, including native allocator/initialization calls
and the epilogue, match. This is constructor evidence, not a direct observation
of the live registry's capacity or a gameplay test.

Only with the existing exact-hash Seamless solo opt-in does the adapter accept
this precise constructor profile. It leaves that constructor and allocation
untouched. Standalone mode retains the original 80-to-520 expansion. Unknown
profiles fail closed. The complete constructor is rechecked before the startup
transaction and while peer threads are frozen, using preallocated read buffers.

Runtime capacity checks require exactly 1000 entries in this pinned solo profile,
otherwise exactly 520. The slot search already reads actual capacity and retains
native allocation. Local tracking remains bounded to 500 bodies: AddGroup checks
local membership, including retiring entries, plus incoming bodies before accepting
a spawn. Free-slot checks, ownership, executable hashes, offline/session gates,
and remote-session rejection remain in force. No new hooks, assets, configuration
settings, save changes, or altered healing/cost formulas are introduced.

## Additional compatibility inspection

A read-only audit compared 58 instruction/slot spans in the current process,
including literal source signatures, explicit call destinations, full slot-search
code, the constructor profile, and Scaleform vtable targets/code. 57 matched.
The remaining mismatch is the optional HUD name lookup at RVA 0x2671370, which is
redirected by the active mod stack. Its existing signature check disables direct
name lookup and uses fallback labels; it does not block startup or gameplay.
No unknown redirect is followed by this correction. This audit covers the checked
spans, not every instruction of every engine function or future runtime changes.

## Build and validation status

- Windows release DLL compiled with `cargo build --lib --locked --release --target
  x86_64-pc-windows-msvc --target-dir ../../work/seamless-solo2-build`.
- `cargo test --lib --locked --no-run --target x86_64-pc-windows-msvc --target-dir
  ../../work/seamless-solo2-build`: test harness compiled; no tests executed.
- Constructor-profile regression coverage prepared alongside existing tests.
- `cargo clippy --lib --locked --target x86_64-pc-windows-msvc --target-dir
  ../../work/seamless-solo2-build`: completed with eight existing style warnings.
- Agent did not launch/control the game, invoke native functions, write process
  memory, or perform gameplay tests. Private memory captures remain outside git.
- Staged only while the user's game is open; installed 0.6.1-solo.2 is unchanged.
- Successful initialization and gameplay with solo.3 are **pending user testing**.
  Connected co-op remains unsupported.

## Next user acceptance

After closing the game and installing the DLL-only update:

1. Launch the normal Mod Engine 2 profile solo, load the character, wait 15 seconds.
   Check the log for `Verified pinned Seamless registry constructor`, then
   `Startup patches installed` and friendly lock-on initialization. Stop and share
   the log if startup is disabled again; no need to repeat combat tests in that case.
2. Summon Wolves on level ground, restore sufficient FP, then summon Militiamen.
   Expect both groups to remain, one normal cost per summon, and no dismissal prompt.
   Same-Ash duplicates remain disabled in the preserved INI; reuse heals that group.
3. Lock onto a wolf and cycle to another Spirit and an enemy. Report FPS and behavior.
4. If those pass, cast Spirit Mend at an injured owned Spirit. Observe HP and native
   staff-adjusted FP cost. Gameplay acceptance is separate from the build/audit.

The updater previews by default, accepts only the inspected solo.2 DLL, backs up
DLL/log and generates guarded rollback before replacement. It preserves the active
INI, launcher, merged assets, other mods, and saves. No repurchase is required.
