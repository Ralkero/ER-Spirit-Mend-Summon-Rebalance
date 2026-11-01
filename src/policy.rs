//! Pure rules shared by the native adapter and automated tests.
/// Connection capability is not session state. Unknown state always rejects.
pub fn offline_allowed(online: Option<bool>, session: Option<(u32, u32)>, conflict: bool) -> bool {
    !conflict && online == Some(false) && session == Some((0, 0))
}

#[cfg(test)]
mod offline_tests {
    use super::offline_allowed;
    #[test]
    fn offline_with_no_session_is_allowed() {
        assert!(offline_allowed(Some(false), Some((0, 0)), false));
    }
    #[test]
    fn online_or_conflicting_environment_is_rejected() {
        assert!(!offline_allowed(Some(true), Some((0, 0)), false));
        assert!(!offline_allowed(Some(false), Some((0, 0)), true));
    }
    #[test]
    fn active_or_transitioning_sessions_are_rejected() {
        for state in 1..=8 {
            assert!(!offline_allowed(Some(false), Some((state, 0)), false));
            assert!(!offline_allowed(Some(false), Some((0, state)), false));
        }
    }
    #[test]
    fn unavailable_session_or_game_state_is_rejected() {
        assert!(!offline_allowed(None, Some((0, 0)), false));
        assert!(!offline_allowed(Some(false), None, false));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Health {
    pub hp: i32,
    pub max: i32,
}

pub fn healed(h: Health, percent: f64) -> Option<i32> {
    if h.hp <= 0
        || h.max <= 0
        || h.hp > h.max
        || !percent.is_finite()
        || !(0.0..=100.0).contains(&percent)
    {
        return None;
    }
    // HP is an integer in the engine. Round the gain down, never the resulting cap.
    let gain = (f64::from(h.max) * percent / 100.0).floor() as i64;
    Some((i64::from(h.hp) + gain).min(i64::from(h.max)) as i32)
}

/// Chaining map entries can associate several different BuddyParams with one trigger.
pub fn family(map: &[(i32, i32)], effect: i32) -> Vec<i32> {
    let Some(base) = (0..=10)
        .filter_map(|n| effect.checked_sub(n))
        .find(|base| *base > 0 && map.iter().any(|(e, _)| e == base))
    else {
        return vec![];
    };
    let mut ids: Vec<_> = map
        .iter()
        .filter(|(e, b)| *e == base && *b > 0)
        .map(|(_, b)| *b)
        .collect();
    ids.sort_unstable();
    ids.dedup();
    ids
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Vanilla,
    Reject,
    Heal,
    AddGroup,
}

/// Called only after inactive registry, ownership and page checks succeed.
pub fn release_inactive_member(native_excluded: bool, temporary_death: bool) -> bool {
    native_excluded && !temporary_death
}

/// Canonical type identity ignores upgrade offsets, but rejects overlapping maps.
pub fn unique_family(map: &[(i32, i32)], effect: i32) -> Option<(i32, Vec<i32>)> {
    let bases: Vec<_> = (0..=10)
        .filter_map(|n| effect.checked_sub(n))
        .filter(|base| *base > 0 && map.iter().any(|(e, _)| e == base))
        .collect();
    if bases.len() != 1 {
        return None;
    }
    let base = bases[0];
    let ids = family(map, effect);
    if ids.is_empty() || map.iter().any(|(e, b)| *e != base && ids.contains(b)) {
        return None;
    }
    Some((base, ids))
}

pub fn multi_action(
    native_active: i32,
    selected_live: bool,
    eligible: bool,
    wounded: bool,
    heal: bool,
) -> Action {
    if !eligible || !matches!(native_active, 0..=2) {
        return Action::Reject;
    }
    if selected_live {
        if heal && wounded {
            Action::Heal
        } else {
            Action::Reject
        }
    } else {
        Action::AddGroup
    }
}

pub fn action(
    native: i32,
    any_live: bool,
    same_item: bool,
    eligible: bool,
    wounded: bool,
    heal_enabled: bool,
) -> Action {
    if !any_live {
        return Action::Vanilla;
    }
    if !same_item || !eligible {
        return Action::Reject;
    }
    if !heal_enabled {
        return Action::Vanilla;
    }
    if native == 2 && wounded {
        Action::Heal
    } else {
        Action::Reject
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn requested_examples() {
        for (hp, expected) in [(900, 1400), (1800, 2000), (200, 700), (2000, 2000)] {
            assert_eq!(healed(Health { hp, max: 2000 }, 25.0), Some(expected));
        }
    }
    #[test]
    fn dead_and_invalid_never_heal() {
        for (hp, max) in [(0, 2000), (-1, 2000), (2, 1), (1, 0)] {
            assert_eq!(healed(Health { hp, max }, 25.0), None);
        }
    }
    #[test]
    fn huge_hp_cannot_overflow() {
        assert_eq!(
            healed(
                Health {
                    hp: i32::MAX - 1,
                    max: i32::MAX
                },
                100.0
            ),
            Some(i32::MAX)
        );
    }
    #[test]
    fn integer_rounding_is_explicit() {
        assert_eq!(healed(Health { hp: 1, max: 7 }, 25.0), Some(2));
    }
    #[test]
    fn each_group_member_uses_own_max() {
        assert_eq!(
            [
                Health { hp: 100, max: 1000 },
                Health { hp: 900, max: 2000 },
                Health { hp: 0, max: 3000 }
            ]
            .map(|h| healed(h, 25.0)),
            [Some(350), Some(1400), None]
        );
    }
    #[test]
    fn all_buddies_at_upgrade_ten() {
        assert_eq!(
            family(&[(2000, 3), (2000, 1), (2000, 3), (3000, 2)], 2010),
            vec![1, 3]
        );
    }
    #[test]
    fn unknown_effect_has_no_family() {
        assert!(family(&[(2000, 3)], 2011).is_empty());
    }
    #[test]
    fn first_and_dead_group_use_native_summon() {
        assert_eq!(action(0, false, true, true, false, true), Action::Vanilla);
    }
    #[test]
    fn living_matching_wounded_heals() {
        assert_eq!(action(2, true, true, true, true, true), Action::Heal);
    }
    #[test]
    fn different_ash_never_replaces() {
        for n in [-1, 0, 1, 2] {
            assert_eq!(action(n, true, false, true, true, true), Action::Reject);
        }
    }
    #[test]
    fn full_hp_does_not_charge() {
        assert_eq!(action(2, true, true, true, false, true), Action::Reject);
    }
    #[test]
    fn restricted_area_cannot_heal() {
        assert_eq!(action(2, true, true, false, true, true), Action::Reject);
    }
    #[test]
    fn native_cooldown_is_not_bypassed() {
        assert_eq!(action(-1, true, true, true, true, true), Action::Reject);
    }
    #[test]
    fn disabled_heal_keeps_native_same_ash_behavior() {
        assert_eq!(action(2, true, true, true, true, false), Action::Vanilla);
    }
    #[test]
    fn repeated_healing_clamps_without_changing_max() {
        let mut h = Health { hp: 1, max: 2000 };
        for _ in 0..100 {
            h.hp = healed(h, 25.0).unwrap();
        }
        assert_eq!(
            h,
            Health {
                hp: 2000,
                max: 2000
            }
        );
    }
}

#[cfg(test)]
mod multi_tests {
    use super::*;
    #[test]
    fn inactive_cleanup_requires_native_exclusion_without_temporary_death() {
        assert!(!release_inactive_member(false, false));
        assert!(!release_inactive_member(false, true));
        assert!(!release_inactive_member(true, true));
        assert!(release_inactive_member(true, false));
    }
    #[test]
    fn upgrades_have_one_type_and_reject_ambiguous_maps() {
        let map = [(2000, 1), (2000, 2), (3000, 3)];
        assert_eq!(unique_family(&map, 2000), unique_family(&map, 2010));
        assert_eq!(unique_family(&map, 3010), Some((3000, vec![3])));
        assert_eq!(unique_family(&[(2000, 1), (2005, 2)], 2007), None);
        assert_eq!(unique_family(&[(2000, 1), (3000, 1)], 2000), None);
        assert_eq!(unique_family(&map, 2011), None);
    }
    #[test]
    fn each_selected_type_controls_its_action() {
        assert_eq!(multi_action(2, false, true, false, true), Action::AddGroup);
        assert_eq!(multi_action(2, true, true, true, true), Action::Heal);
        assert_eq!(multi_action(2, true, true, false, true), Action::Reject);
        assert_eq!(multi_action(2, true, true, true, false), Action::Reject);
        for state in [-1, 3, i32::MAX] {
            assert_eq!(multi_action(state, false, true, true, true), Action::Reject);
        }
        for live in [false, true] {
            assert_eq!(multi_action(2, live, false, true, true), Action::Reject);
        }
    }
}

/// Duplicate mode bypasses only same-family heal selection, never native gates.
pub fn duplicate_action(native: i32, eligible: bool, room: bool) -> Action {
    if matches!(native, 0..=2) && eligible && room {
        Action::AddGroup
    } else {
        Action::Reject
    }
}
pub fn group_capacity(required: usize, free: usize) -> bool {
    required > 0 && free >= required
}
#[cfg(test)]
mod duplicate_policy_tests {
    use super::*;
    #[test]
    fn native_rejection_and_pending_or_limit_rejection_cannot_commit() {
        for n in [-1, 3, i32::MAX] {
            assert_eq!(duplicate_action(n, true, true), Action::Reject);
        }
        for n in 0..=2 {
            assert_eq!(duplicate_action(n, false, true), Action::Reject);
            assert_eq!(duplicate_action(n, true, false), Action::Reject);
            assert_eq!(duplicate_action(n, true, true), Action::AddGroup);
        }
    }
    #[test]
    fn entire_group_including_mounts_must_fit() {
        assert!(!group_capacity(0, 500));
        assert!(!group_capacity(3, 2));
        assert!(group_capacity(3, 3));
        assert!(!group_capacity(2, 1));
    }
}
