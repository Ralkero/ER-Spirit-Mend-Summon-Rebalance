from pathlib import Path
p=Path('outputs/SpiritAshResummon/TARGETED-HEALING-INVESTIGATION.md')
s=p.read_text(encoding='utf-8-sig')
update='''# Design revision: Intelligence sorcery (2026-10-29)

The user now prefers a real spell suited to an Intelligence sorcery build. This supersedes the reusable quick-slot healing tool/Spiritmending Bell proposal below. The prior research is retained as history, not the current implementation specification.

Current direction:
- A staff-cast sorcery, memorized in the normal spell interface, with an Intelligence requirement and no Faith or Arcane requirement. Proposed one memory slot; numerical INT requirement and FP cost remain tunable/unselected.
- Working title: Spirit Mend. Original blue/silver spectral visual and custom sorcery icon; no bell-shaped icon commitment or new image generation yet.
- Retain requested native mixed enemy/owned-Spirit lock-on and directional cycling. Cast heals one selected owned living Spirit by25% of its maxHP, clamped to max. Normal Ash use instead summons another complete group at its normal cost.
- INT gates access and the staff is the casting tool. For the first prototype, healing remains fixed25%; do not silently multiply it by Sorcery Scaling or introduce new INT scaling. Scaling can be a separate later balance decision.
- No self-healing, ally/player healing or area healing is implied by the current scope. User's comparison to Faith healing is motivation, not confirmation that the spell should additionally heal its caster.
- Enemy/no target/full-health/invalid ownership rejects without healing. Free failed casts remain a design target, with native spell-cost commit timing to be researched. Distinguish invalid at cast start from target dying or changing state mid-cast; no unverified refund or interrupted-cast promise.

Source verification: vendored generated.rs MAGIC_PARAM_ST includes requirement_intellect, requirement_faith, requirement_luck, slot_length, mp, stamina, behavior_id, ez_state_behavior_type, icon_id, ref_category1..10 and ref_id1..10. These establish the relevant spell metadata controls, not the semantics of a chosen animation/behavior template. Select a staff sorcery template from the installed parameters before choosing exact values.

Revised asset and DLL work:
1. A new collision-checked MagicParam row, sorcery category/catalyst compatibility and casting behavior; acquisition Goods/merchant/text wiring where required by the installed spell-learning flow. A learnable spell is still an inventory acquisition and can be saved normally by the game.
2. Spell-specific FMG name/description/icon wiring rather than only Goods text and a quick-slot tool. Use the appropriate spell icon atlas mapping, retaining isolated Mod Engine assets and merge metadata.
3. Reuse the validated per-actor HP operation but research the spell cast/effect/cost path separately. The current Ash item hooks do NOT automatically provide correct spell charging, cancellation, animation timing or input handling. A marker effect dispatch remains a possibility, not a verified solution.
4. Resolve and latch the selected Spirit handle at the defined cast stage, validate again at effect time, and never retarget to another actor silently. Keep range/LOS and engine lifecycle safeguards. Friendly-target acquisition/maintenance/cycling remains the first diagnostic milestone.
5. Use existing compatible staff animation/VFX for the initial prototype, then add visual polish and generated icon. No animation, numeric FP cost, INT threshold, or spell ID is final yet.

Installed0.4.0 and source gameplay behavior remain unchanged. This revision records the design preference; no build, tests, game launch or runtime mutation performed.

---

'''
p.write_text(update+s,encoding='utf-8',newline='\n')
