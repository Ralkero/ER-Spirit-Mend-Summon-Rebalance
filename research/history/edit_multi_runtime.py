from pathlib import Path
p=Path('outputs/SpiritAshResummon/src/runtime.rs');s=p.read_text()
s=s.replace('BuddyStoneParam, CSChrDataModule,', 'BuddyParam, BuddyStoneParam, CSChrDataModule,')
s=s.replace('    collections::{HashMap, HashSet},','    cell::Cell,\n    collections::{HashMap, HashSet},')
anchor='    if let Err(error) = wait_for_system_init(&p, Duration::from_secs(30)) {'
s=s.replace(anchor,'''    // Capacity must be patched before the engine constructs its summon registry.
    // Gameplay callbacks pass through until the game task publishes READY.
    if let Err(error) = unsafe { hooks::install(config().multiple) } {
        log(format!("Disabled: startup patch transaction: {error}"));
        return;
    }
    log(format!("Startup patches installed: call_sites={}, multi_type={}, actor_pool={}; awaiting game task readiness", if config().multiple { 10 } else { 8 }, config().multiple, if config().multiple { crate::capacity::ENTRIES } else { 80 }));
'''+anchor)
a=s.index('        match unsafe { hooks::install() }');b=s.index('\n    }\n    if !hooks::READY',a)
s=s[:a]+'''        if !hooks::INSTALLED.load(Ordering::Acquire) {
            return;
        }
        hooks::READY.store(true, Ordering::Release);
        log("Game task ready; gameplay adapter enabled after startup validation");'''+s[b:]
s=s.replace('    report_gate_status();','    refresh_pending_spawn();\n    if config().multiple {\n        if let Ok(world) = unsafe { WorldChrMan::instance() } {\n            if world.main_player.is_some() && world.summon_buddy_chr_set.capacity != crate::capacity::ENTRIES {\n                FAULT.store(true, Ordering::Release);\n                log(format!("Disabled: summon registry has {} entries, expected {}; constructor was too early or another mod changed it. Restart required", world.summon_buddy_chr_set.capacity, crate::capacity::ENTRIES));\n                return;\n            }\n        }\n    }\n    report_gate_status();')
s=s.replace('if entry.second.len() > 64 {','if entry.second.len() > crate::capacity::MAX_GROUP_MEMBERS {')
s=s.replace('unsafe fn snapshot(\n','unsafe fn snapshot_single(\n',1)
s=s.replace('        Some(Action::Heal) => 0,','        Some(Action::Heal | Action::AddGroup) => 0,')
# Supply explicit AddGroup commit branch before the original Vanilla branch.
a=s.index('    match s.action {',s.index('unsafe fn summon_inner'))
s=s[:a]+s[a:].replace('    match s.action {','''    match s.action {
        Action::AddGroup => unsafe { add_group(m, effect, s.goods, original) },''',1)
p.write_text(s)
