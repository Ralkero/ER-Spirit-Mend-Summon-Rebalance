# Registry state verification for 0.1.9

Target: installed eldenring.exe, WW file version 2.7.2.0, SHA256 `f9118fea2cfd8fa3901e8060bfadb1552cb2471771ded8a5b662c4e298ffb328`.

REA opened and identified that target, then closed it without executing or modifying it. The native decompiler provider was unavailable (Ghidra not configured), so focused PE/Capstone static disassembly supplied the instruction evidence in `registry-state-0.1.9.asm`. No new gameplay test was performed by the agent.

## Observations

- The prior user-operated 0.1.8 session reported living Skeletal Militiamen at 60 FPS. Repeated matching registry entries had load byte 4 and update byte 4. The first member failed the old predicate, so that log does not establish all members' state values.
- MSVC RTTI `.?AVChrSet@CS@@` at RVA 0x3c87148 leads to complete-object locator 0x32d7e68 and vtable 0x2a43348. Vtable accessors corroborate capacity at +0x10, entries at +0x18, 16-byte entry stride and actor pointer at entry +0. Relevant accessor RVAs: 0x493d90 and 0x493e20.
- Native character predicate RVA 0x3f2d30 reads actor +0x10 (the binding's chr_set_entry back-pointer), then requires entry byte +8 == 4 to continue past the load-state gate. Other flag checks can still reject the actor: this is not a complete readiness oracle.
- Native lifecycle code reads entry +8, dispatches over states 0 through 5, and writes 4 at RVA 0x3f8ce6 after preceding successful work. This corroborates 4 as the fully loaded branch for this profile, rather than treating the Rust enum name Active=2 as authoritative.
- Update logic writes entry +9 = 4 at RVA 0x40143d on its enabled branch and writes 2 at RVA 0x40144f on the alternative branch. The preceding logic includes character update flags and conditions; a raw value of 4 is not sufficient evidence of network ownership. The upstream label Remote=4 is not used as an ownership rule.
- Entry cleanup RVA 0x3c3830 clears its actor pointer and both state bytes. Native handle lookup at 0x493de0 resolves the indexed actor through ChrSet accessors; it does not itself compare these state bytes or prove lifetime safety across threads.

## Narrow implementation decision

Admit only the observed and statically corroborated load=4/update=4 pair for this executable profile. Keep other states rejected; do not expand the allowed set to include 2/0. Read raw bytes without changing upstream enum definitions. This is a conservative supported state pair, not a complete naming or behavioral reconstruction of all states.

Before any hooks install, require exact instruction bytes at the load gate, load-state write and update-state write. These locations are checked only, never patched. Existing executable and hook signatures remain required.

Add actor-to-registry back-pointer equality after live registry membership and state validation. Retain engine-handle resolution, module owner, HP/max bounds, death flag, local-player group, matching Ash family, group is_remote and transition flags, and offline/session checks. No actor address is cached across callbacks. Commit still uses fresh OS page/write checks. These checks still rely on the existing serialized-callback/lifetime assumption; this inspection does not prove teardown race freedom.

## Remaining validation

User must verify both members now pass registry and downstream health checks, and that full validation remains fast. Healing, charging on healing, skeleton recovery/death, group teardown, transitions and resummoning remain unverified. Update=2 stays rejected even if it may be a legitimate temporarily non-updating character state. Do not infer that every 4/4 actor is safe or belongs to the player without the remaining checks.
