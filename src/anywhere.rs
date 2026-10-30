// Monument rules and tile selection adapted from ArialSenki/MultipleSpiritAshes
// extras.rs (MIT OR Apache-2.0; see LICENSE-* and UPSTREAM.md).
// No new hooks, persistent actor references, or per-frame OS page queries.
#[derive(Clone, Copy, PartialEq, Eq)]
struct AnywhereKey {
    world: usize,
    player: usize,
    manager: usize,
    area: u8,
    x: i32,
    y: i32,
}
static ANYWHERE_CACHE: Mutex<Option<(AnywhereKey, u32)>> = Mutex::new(None);

fn area_allowed(manager: &SummonBuddyManager) -> bool {
    manager.is_within_activation_range || ANYWHERE_READY.load(Ordering::Acquire)
}

// Overworld monument IDs encode area/tile; dungeon IDs encode area/block.
fn monument_tile(id: u32) -> (u8, i32, i32) {
    if id >= 1_000_000_000 {
        let area = if id / 100_000_000 == 20 { 61 } else { 60 };
        (
            area,
            ((id / 1_000_000) % 100) as i32,
            ((id / 10_000) % 100) as i32,
        )
    } else {
        ((id / 1_000_000) as u8, ((id / 10_000) % 100) as i32, 0)
    }
}

/// Only the current world's manager is accepted. A null expected pointer is the
/// game-task entry point; hooks pass their manager for an ownership check.
fn prepare_anywhere(expected: *mut SummonBuddyManager) {
    if !ANYWHERE_READY.load(Ordering::Acquire) || !enabled() {
        return;
    }
    let Ok(world) = (unsafe { WorldChrMan::instance() }) else {
        return;
    };
    let Some(player) = world.main_player.as_ref() else {
        if let Ok(mut cache) = ANYWHERE_CACHE.lock() {
            *cache = None;
        }
        return;
    };
    if player.chr_ins.chr_flags1c5.death_flag() {
        return;
    }
    let manager = unsafe {
        std::ptr::addr_of!(world.summon_buddy_manager)
            .cast::<*mut SummonBuddyManager>()
            .read()
    };
    if !pointer_shape(manager) || (!expected.is_null() && expected != manager) {
        return;
    }
    let block = player.current_block_id;
    // Invalid/unloaded block and position must never authorize a fallback.
    if block.area() == 0 || block.area() == u8::MAX {
        return;
    }
    let pos = &player.block_position;
    if !pos.x.is_finite() || !pos.y.is_finite() || !pos.z.is_finite() {
        return;
    }
    let (x, y) = if matches!(block.area(), 60 | 61) {
        let scale = 1i32 << block.index().min(2);
        let half = scale as f32 * 128.0;
        let sub = |v: f32| (((v + half) / 256.0).floor() as i32).clamp(0, scale - 1);
        (
            block.block() as i32 * scale + sub(pos.x),
            block.region() as i32 * scale + sub(pos.z),
        )
    } else {
        (block.block() as i32, 0)
    };
    let key = AnywhereKey {
        world: world as *const _ as usize,
        player: player as *const _ as usize,
        manager: manager as usize,
        area: block.area(),
        x,
        y,
    };
    let cached = ANYWHERE_CACHE
        .lock()
        .ok()
        .and_then(|c| *c)
        .filter(|(k, _)| *k == key)
        .map(|(_, stone)| stone);
    let stone = if let Some(stone) = cached {
        stone
    } else {
        // Parameter rows were validated and captured by update_stones. Only
        // scalar IDs are used here; no retained row/actor pointer is dereferenced.
        let selected = STONES.lock().ok().and_then(|state| {
            let (_, rows) = state.as_ref()?;
            rows.keys().copied().filter(|id| *id > 0).min_by_key(|id| {
                let (area, sx, sy) = monument_tile(*id);
                (area != key.area, (sx - x).abs().max((sy - y).abs()), *id)
            })
        });
        let Some(selected) = selected else {
            return;
        };
        if let Ok(mut cache) = ANYWHERE_CACHE.lock() {
            *cache = Some((key, selected));
        }
        debug(format!(
            "Anywhere tile m{:02}_{x}_{y}: fallback monument={selected}",
            key.area
        ));
        selected
    };
    let m = unsafe { &mut *manager };
    if m.disappear_requested {
        return;
    }
    // Prefer the native monument when present; supply a real, patched row when
    // this map has no loaded monument. Never manufacture an entity pointer.
    if m.buddy_stone_entity_id == 0 {
        m.buddy_stone_entity_id = stone;
    }
    // Release the used-monument lock only after ALL local native entries are
    // gone. Avoid the upstream raw HP scan, which mistakes reviving skeletons
    // for permanently dead members. Existing retirement cleanup handles them.
    if config().resummon
        && !m.player_has_alive_summon
        && m.request_summon_speffect_id == -1
        && m.groups.len() <= 32
        && m.item_use_cooldown_timer.is_finite()
        && m.item_use_cooldown_timer <= 0.0
    {
        let owner = player.chr_ins.event_entity_id as i32;
        let has_local_entries = m
            .groups
            .iter()
            .any(|e| e.first == owner && e.second.len() > 0);
        if !has_local_entries {
            m.active_summmon_buddy_stone_entity_id = 0;
        }
    }
    if m.request_summon_speffect_id > 0 {
        sync_anywhere_origin(manager);
    }
}

/// Supply the same current Havok coordinates used by upstream; an arbitrary
/// distant fallback monument must not become the spawn location. Native
/// collision/navigation placement, animation, and cost handling remain intact.
fn sync_anywhere_origin(manager: *mut SummonBuddyManager) {
    if !ANYWHERE_READY.load(Ordering::Acquire) || !enabled() {
        return;
    }
    let Ok(world) = (unsafe { WorldChrMan::instance() }) else {
        return;
    };
    let expected = unsafe {
        std::ptr::addr_of!(world.summon_buddy_manager)
            .cast::<*mut SummonBuddyManager>()
            .read()
    };
    if !pointer_shape(manager) || manager != expected {
        return;
    }
    let Some(player) = world.main_player.as_ref() else {
        return;
    };
    let chr = &player.chr_ins;
    if chr.chr_flags1c5.death_flag() {
        return;
    }
    let modules = chr.modules.as_ptr();
    if !pointer_shape(modules) {
        return;
    }
    let physics = unsafe { (*modules).physics.as_ptr() };
    if !pointer_shape(physics) {
        return;
    }
    let position = unsafe { (*physics).position };
    if !position.0.is_finite() || !position.1.is_finite() || !position.2.is_finite() {
        return;
    }
    let m = unsafe { &mut *manager };
    if !m.disappear_requested {
        m.spawn_origin = position;
    }
}
