from pathlib import Path
p=Path('outputs/SpiritAshResummon/src/runtime.rs')
s=p.read_text(encoding='utf-8-sig')
s=s.replace('type StoneOriginals = (usize, HashMap<u32, (usize, u32)>);','''#[derive(Clone, Copy)]
struct StoneOriginal {
    address: usize,
    flag: u32,
    range: u16,
    return_range: i16,
    region: u32,
    eliminate: u32,
}
type StoneOriginals = (usize, HashMap<u32, StoneOriginal>);
static ANYWHERE_READY: AtomicBool = AtomicBool::new(false);''')
s=s.replace('update_stones(offline() && config().resummon);','''update_stones(offline() && (config().resummon || config().anywhere));
    prepare_anywhere(std::ptr::null_mut());''')
a=s.index('    if !enable {',s.index('fn update_stones('))
b=s.index('\nfn param_ready',a)
s=s[:a]+'''    if !enable {
        ANYWHERE_READY.store(false, Ordering::Release);
        if let Ok(mut cache) = ANYWHERE_CACHE.lock() { *cache = None; }
        if let Some((_, originals)) = state.take() {
            for (id, old) in originals {
                if let Some(row) = repo.get_mut::<BuddyStoneParam>(id)
                    && row as *mut _ as usize == old.address
                {
                    // Restore only values still equal to this mod's writes.
                    if config().resummon && row.summoned_event_flag_id() == 0 {
                        row.set_summoned_event_flag_id(old.flag);
                    }
                    if config().anywhere {
                        if row.activate_range() == u16::MAX { row.set_activate_range(old.range); }
                        if row.overwrite_return_range() == -1 { row.set_overwrite_return_range(old.return_range); }
                        if row.overwrite_activate_region_entity_id() == 0 { row.set_overwrite_activate_region_entity_id(old.region); }
                        if row.eliminate_target_entity_id() == 0 { row.set_eliminate_target_entity_id(old.eliminate); }
                    }
                }
            }
        }
        return;
    }
    if let Some((_, originals)) = state.as_ref() {
        // A singleton can survive a parameter-table replacement. Check an anchor row.
        if originals.iter().next().is_some_and(|(id, old)| {
            repo.get::<BuddyStoneParam>(*id)
                .is_some_and(|row| row as *const _ as usize == old.address)
        }) {
            return;
        }
        *state = None;
    }
    ANYWHERE_READY.store(false, Ordering::Release);
    if let Ok(mut cache) = ANYWHERE_CACHE.lock() { *cache = None; }
    let mut originals = HashMap::new();
    for (id, row) in repo.rows_mut::<BuddyStoneParam>() {
        originals.insert(id, StoneOriginal {
            address: row as *mut _ as usize,
            flag: row.summoned_event_flag_id(),
            range: row.activate_range(),
            return_range: row.overwrite_return_range(),
            region: row.overwrite_activate_region_entity_id(),
            eliminate: row.eliminate_target_entity_id(),
        });
        if config().resummon { row.set_summoned_event_flag_id(0); }
        if config().anywhere {
            row.set_activate_range(u16::MAX);
            row.set_overwrite_return_range(-1);
            row.set_overwrite_activate_region_entity_id(0);
            row.set_eliminate_target_entity_id(0);
        }
    }
    if !originals.is_empty() {
        log(format!(
            "BuddyStone rules applied: rows={} resummon={} summon_anywhere={} keep_after_boss={}",
            originals.len(), config().resummon, config().anywhere, config().anywhere
        ));
        *state = Some((identity, originals));
        ANYWHERE_READY.store(config().anywhere, Ordering::Release);
    }
}

include!("anywhere.rs");
''' + s[b:]
s=s.replace('unsafe fn native_state(m: *mut SummonBuddyManager, goods: i32) -> i32 {','''unsafe fn native_state(m: *mut SummonBuddyManager, goods: i32) -> i32 {
    // Re-establish the fallback after the engine's own monument search, including
    // item queries between game tasks. This never changes native state results.
    prepare_anywhere(m);''')
s=s.replace('m.is_within_activation_range && !m.disappear_requested','area_allowed(m) && !m.disappear_requested')
s=s.replace('            let started = Instant::now();\n            let result = unsafe { original(m, effect) };','            sync_anywhere_origin(m);\n            let started = Instant::now();\n            let result = unsafe { original(m, effect) };')
p.write_text(s,encoding='utf-8',newline='\n')
p=Path('outputs/SpiritAshResummon/src/multi.rs');s=p.read_text(encoding='utf-8-sig').replace('(m.is_within_activation_range\n','(area_allowed(m)\n').replace('!m.is_within_activation_range','!area_allowed(m)').replace('    let result = unsafe { original(manager, effect) };','    sync_anywhere_origin(manager);\n    let result = unsafe { original(manager, effect) };');p.write_text(s,encoding='utf-8',newline='\n')
p=Path('outputs/SpiritAshResummon/src/cleanup.rs');s=p.read_text(encoding='utf-8-sig').replace('!m.is_within_activation_range','!area_allowed(m)');p.write_text(s,encoding='utf-8',newline='\n')
for name in ['Cargo.toml','Cargo.lock']:
 p=Path('outputs/SpiritAshResummon')/name;s=p.read_text(encoding='utf-8-sig').replace('name = "spirit-ash-resummon"\nversion = "0.3.1"','name = "spirit-ash-resummon"\nversion = "0.4.0"');p.write_text(s,encoding='utf-8',newline='\n')
