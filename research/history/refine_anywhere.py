from pathlib import Path
p=Path('outputs/SpiritAshResummon/src/runtime.rs');s=p.read_text(encoding='utf-8-sig')
s=s.replace('''    let Ok(repo) = (unsafe { SoloParamRepository::instance_mut() }) else {
        return;
    };''','''    let Ok(repo) = (unsafe { SoloParamRepository::instance_mut() }) else {
        ANYWHERE_READY.store(false, Ordering::Release);
        return;
    };''',1)
s=s.replace('''    if !param_ready::<BuddyStoneParam>(repo) {
        return;
    }''','''    if !param_ready::<BuddyStoneParam>(repo) {
        ANYWHERE_READY.store(false, Ordering::Release);
        return;
    }''',1)
a=s.index('fn update_stones(');b=s.index('include!("anywhere.rs")',a)
part=s[a:b].replace('''        }) {
            return;
        }
        *state = None;''','''        }) {
            ANYWHERE_READY.store(config().anywhere, Ordering::Release);
            return;
        }
        *state = None;''')
s=s[:a]+part+s[b:]
s=s.replace('''    // Re-establish the fallback after the engine's own monument search, including
    // item queries between game tasks. This never changes native state results.
    prepare_anywhere(m);
''','')
s=s.replace('''    guarded("item query", -1, || unsafe { query_inner(m, goods) })''','''    guarded("item query", -1, || {
        prepare_anywhere(m);
        unsafe { query_inner(m, goods) }
    })''')
s=s.replace('''    guarded("item eligibility", -1, || {
        let native''','''    guarded("item eligibility", -1, || {
        prepare_anywhere(m);
        let native''')
s=s.replace('''    guarded("cost query", 2, || unsafe { cost_inner(m, goods) })''','''    guarded("cost query", 2, || {
        prepare_anywhere(m);
        unsafe { cost_inner(m, goods) }
    })''')
s=s.replace('''    guarded("summon effect", false, || unsafe {
        summon_inner(m, effect)
    })''','''    guarded("summon effect", false, || {
        prepare_anywhere(m);
        unsafe { summon_inner(m, effect) }
    })''')
p.write_text(s,encoding='utf-8',newline='\n')
