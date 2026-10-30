# Item eligibility inspection

Pinned installed WW 2.7.2.0 executable; static pefile/Capstone inspection only. Native decompiler unavailable in this environment. No debugger attached, process memory modified or game inputs generated.

- 0x690830 `mov edx,r13d`: selected Goods ID.
- 0x690833 `call 0x4b73d0`: independent GetBuddyState.
- 0x690838 `cmp eax,-1`; 0x69083b `setne r13b`.
- 0x69096e `test r13b,r13b`; 0x690971 `je 0x690982`: joins native usability failure path.
- FP requirement comparison remains at 0x690964, separate from this state result.
- Existing 0.2.0 hooks did not include 0x690833; the menu path logged (-1,0), yet activation never reached the effect callback.

Correction adds one call-site redirect to the same existing per-family policy, preserving surrounding native predicates and rejecting unknown state. No runtime claim until user reproduction. Upstream item-limit clearing remains excluded.
