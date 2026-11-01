# 0.6.1-solo.2: preserve the inspected Seamless function redirects

## Confirmed failure in 0.6.1-solo.1

The user observed the native dismissal prompt when attempting another summon and
could not lock on to Spirits. The captured log identifies the cause:

```
Disabled: startup patch transaction: Function signature mismatch at 4b73d0; no hooks installed
```

This occurred before friendly lock-on, Spirit Mend, HUD or the GameMan task were
initialized. The DLL loaded but its gameplay hooks were inactive. Spirit Mend's
custom healing handler was therefore also inactive; the user has not tested it
in this profile. Successful file installation was not successful compatibility.

## Evidence (2026-10-30)

Used ordinary Windows `OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ)`
and bounded `ReadProcessMemory` reads of the already-running game, plus PE and
Capstone analysis. No debugger attachment, process suspension, function calls,
memory writes, DLL injection, game controls or gameplay tests were performed.
REA offers no Windows live-memory reader in this session; this is local-tool
evidence, not a claimed REA analysis record. Private captures stay outside git.

Of 31 sampled hook/entry sites, two native prologues had changed:

| Native game RVA | Observed chain after rel32 jump | Seamless handler RVA | Callback RVA |
| --- | --- | --- | --- |
| 4b73d0, summon-state query | movabs RAX/jmp RAX relay, closure | 96ed0 | 8b4f0 |
| 4b8690, summon request | indirect RIP+0 relay, closure | 96480 | 8b4c0 |

Both closures load a context pointer into XMM5, then tail-jump through R11 into
the pinned Seamless image. The handlers receive the original RCX and EDX inputs,
preserve Win64 nonvolatile registers/stack conventions and return EAX (with the
summon caller consuming AL). The original remainder of each native prologue
was unchanged. Other sampled sites, including friendly lock-on and Spirit Mend,
matched the original executable at their inspected spans. Sampling does not
establish that every byte of every function was unchanged.

Handler and callback ranges come from Seamless's PE exception table. Their full
captured code bytes match the exact pinned file. The context contains the
observed vtable, callback address and self-reference. No absolute runtime heap
address is embedded in source; randomized pointers are resolved afresh.

## Correction

`seamless_hooks.rs` accepts only these two RVAs when the existing exact-hash solo
exception is active. It requires the intact native prologue tail, the two
observed relay encodings, exact XMM5/R11 closure encoding, handler destinations,
context identity fields and full handler/callback SHA256 fingerprints. Unknown
redirects, code changes, unreadable/non-executable code mappings, malformed
pointers and arithmetic overflow reject startup. All other signature checks
remain unchanged.

The adapter still hooks only its original native call sites. It does not
overwrite Seamless's prologues, build a replacement original-function trampoline,
call private callbacks directly, or bypass Seamless's callback behavior. Its
original-call pointers remain the game function entries, so their existing
Seamless chains receive calls through the same ABI as native callers.

Captured redirect bytes and context identity are rechecked before freezing
threads and immediately before committing the patch transaction while peers
are frozen. The frozen-phase check uses preallocated buffers and the raw Win32
read API; it allocates no memory and invokes no game code. These guards run only
during startup, not per-frame. Session, ownership, resource, EAC and executable
restrictions remain in force. No gameplay formulas or assets changed.

## Validation and remaining limits

- Rust Windows release build: succeeded.
- Regression cases for relay/closure instruction matching and rel32 overflow:
  prepared and compiled with `cargo test --no-run`; **not executed**.
- Code fingerprints and redirect/context identities: read from the user's
  running process; no native functions invoked by the agent.
- Installation into the running process: **not attempted**. The update is staged
  separately until the game is closed.
- Successful startup with the correction, session gates, native cost behavior,
  coexistence and healing: **not gameplay-validated**. This fixes the demonstrated
  startup rejection; it is not proof of complete Seamless integration.

Expected new startup lines are `Verified pinned Seamless redirect at 4b73d0`,
the equivalent line for `4b8690`, and `Startup patches installed`. Later feature
initialization and activation gates still need to succeed. No co-op partner is
required for solo use; connected co-op remains unsupported.

`tools/Update-SeamlessSoloDll.ps1` previews by default. With `-Apply` it requires
the game and launchers closed, verifies the package and current DLL hashes,
backs up the installed DLL/log, replaces only the DLL and checks preservation of
configuration/assets. It provides guarded rollback. INI and merged assets stay
unchanged; no repurchase or save-file editing is required.
