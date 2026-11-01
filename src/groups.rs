//! Summon-instance accounting, independent of native memory and HP.
//! Unknown/loading observations can retain known membership but never invent it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Member {
    pub slot: u16,
    pub handle: u64,
    pub ordinal: usize,
}
#[derive(Clone, Copy, Debug)]
pub struct Observation {
    pub slot: u16,
    pub handle: Option<u64>,
    pub family: i32,
}
#[derive(Clone, Debug)]
pub struct Group {
    pub id: u64,
    pub family: i32,
    pub members: Vec<Member>,
    expected: usize,
    assigned: usize,
    pending: bool,
}
impl Group {
    pub fn is_pending(&self) -> bool {
        self.pending
    }
}
#[derive(Default, Debug)]
pub struct Groups {
    pub groups: Vec<Group>,
    next: u64,
    ready: bool,
}
impl Groups {
    pub const fn new() -> Self {
        Self {
            groups: Vec::new(),
            next: 0,
            ready: false,
        }
    }
    pub fn is_idle(&self) -> bool {
        self.ready && self.groups.is_empty()
    }
    pub fn invalidate(&mut self) {
        self.ready = false;
    }
    pub fn reset(&mut self) {
        self.groups.clear();
        self.ready = false;
        // Monotonic IDs across session resets; never recycle a displayed G id.
    }
    pub fn can_reserve(&self, family: i32, limit: usize) -> bool {
        self.ready
            && (1..=8).contains(&limit)
            && !self.groups.iter().any(|g| g.pending)
            && self.groups.iter().filter(|g| g.family == family).count() < limit
    }
    pub fn reserve(&mut self, family: i32, expected: usize, limit: usize) -> Option<u64> {
        if !self.can_reserve(family, limit) || !(1..=500).contains(&expected) {
            return None;
        }
        self.next = self.next.checked_add(1)?;
        let id = self.next;
        self.groups.push(Group {
            id,
            family,
            members: Vec::new(),
            expected,
            assigned: 0,
            pending: true,
        });
        Some(id)
    }
    /// Only explicit native rejection may cancel an empty reservation. No timeout.
    pub fn cancel(&mut self, id: u64) {
        self.groups
            .retain(|g| !(g.id == id && g.pending && g.members.is_empty()));
    }
    pub fn label(&self, handle: u64) -> Option<u64> {
        self.groups
            .iter()
            .find(|g| g.members.iter().any(|m| m.handle == handle))
            .map(|g| g.id)
    }
    pub fn member_label(&self, handle: u64) -> Option<(u64, usize)> {
        self.groups.iter().find_map(|g| {
            g.members
                .iter()
                .find(|m| m.handle == handle)
                .map(|m| (g.id, m.ordinal))
        })
    }
    pub fn reconcile(&mut self, observations: &[Observation], native_pending: bool) -> bool {
        self.ready = false;
        if observations.len() > 500 {
            return false;
        }
        // Reject conflicting registry observations before changing attribution.
        for (i, a) in observations.iter().enumerate() {
            if observations[..i]
                .iter()
                .any(|b| a.slot == b.slot || a.handle.is_some() && a.handle == b.handle)
            {
                return false;
            }
        }
        for group in &mut self.groups {
            group.members.retain(|m| {
                observations.iter().any(|o| {
                    o.slot == m.slot
                        && o.family == group.family
                        && o.handle.is_none_or(|h| h == m.handle)
                })
            });
        }
        self.groups.retain(|g| g.pending || !g.members.is_empty());
        let mut resolved = true;
        for o in observations {
            if self.groups.iter().any(|g| {
                g.family == o.family
                    && g.members
                        .iter()
                        .any(|m| m.slot == o.slot && o.handle.is_none_or(|h| h == m.handle))
            }) {
                continue;
            }
            let Some(handle) = o.handle else {
                resolved = false;
                continue;
            };
            let Some(group) = self
                .groups
                .iter_mut()
                .find(|g| g.pending && g.family == o.family)
            else {
                resolved = false;
                continue;
            };
            if group.assigned >= group.expected {
                resolved = false;
                continue;
            }
            group.assigned += 1;
            group.members.push(Member {
                slot: o.slot,
                handle,
                ordinal: group.assigned,
            });
        }
        if resolved && !native_pending {
            for group in &mut self.groups {
                if group.pending && group.assigned == group.expected {
                    group.pending = false;
                }
            }
        }
        self.ready = resolved && !native_pending;
        resolved
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn o(slot: u16, handle: Option<u64>, family: i32) -> Observation {
        Observation {
            slot,
            handle,
            family,
        }
    }
    fn ready() -> Groups {
        let mut g = Groups::new();
        assert!(g.reconcile(&[], false));
        g
    }
    #[test]
    fn three_groups_limit_includes_pending_and_partial_survivors() {
        let mut g = ready();
        let mut actors = Vec::new();
        for slot in 0..3 {
            assert!(g.reserve(10, 1, 3).is_some());
            assert!(g.reserve(10, 1, 3).is_none());
            actors.push(o(slot, Some(slot as u64 + 100), 10));
            assert!(g.reconcile(&actors, false));
        }
        assert!(!g.can_reserve(10, 3));
        actors.remove(0);
        assert!(g.reconcile(&actors, false));
        assert!(g.can_reserve(10, 3));
    }
    #[test]
    fn partial_loading_and_skeletal_inactivity_keep_reservations() {
        let mut g = ready();
        let id = g.reserve(10, 2, 3).unwrap();
        assert!(!g.reconcile(&[o(20, Some(100), 10), o(21, None, 10)], true));
        assert!(!g.can_reserve(10, 3));
        g.reconcile(&[o(20, Some(100), 10), o(21, Some(101), 10)], false);
        g.reconcile(&[o(20, None, 10), o(21, Some(101), 10)], false);
        assert_eq!(g.label(100), Some(id));
        g.reconcile(&[o(20, Some(100), 10)], false);
        assert_eq!(g.groups.len(), 1);
        assert_eq!(g.label(100), Some(id));
    }
    #[test]
    fn generation_reuse_is_not_old_membership_and_unknowns_block() {
        let mut g = ready();
        let id = g.reserve(10, 1, 3).unwrap();
        g.reconcile(&[o(20, Some(100), 10)], false);
        assert!(!g.reconcile(&[o(20, Some(200), 10)], false));
        assert_eq!(g.label(100), None);
        assert_eq!(g.label(200), None);
        assert!(!g.can_reserve(10, 3));
        g.reset();
        g.reconcile(&[], false);
        assert!(g.reserve(10, 1, 3).unwrap() > id);
    }
    #[test]
    fn canceled_request_and_independent_families() {
        let mut g = ready();
        let id = g.reserve(10, 1, 1).unwrap();
        g.cancel(id);
        g.reserve(20, 1, 1).unwrap();
        g.reconcile(&[o(20, Some(100), 20)], false);
        assert!(g.can_reserve(10, 1));
        assert!(!g.can_reserve(20, 1));
        assert!(!g.can_reserve(10, 0));
        assert!(!g.can_reserve(10, 9));
    }
    #[test]
    fn incomplete_request_never_expires_into_an_extra_group() {
        let mut g = ready();
        g.reserve(10, 3, 3).unwrap();
        for _ in 0..1000 {
            g.reconcile(&[], false);
        }
        assert!(!g.can_reserve(10, 3));
        g.reset();
        g.reconcile(&[], false);
        assert!(g.can_reserve(10, 3));
    }
    #[test]
    fn death_during_loading_does_not_erase_the_instance_or_reuse_ordinals() {
        let mut g = ready();
        let id = g.reserve(10, 2, 3).unwrap();
        g.reconcile(&[o(20, Some(100), 10)], true);
        g.reconcile(&[o(21, Some(101), 10)], false);
        assert_eq!(g.member_label(101), Some((id, 2)));
        assert!(g.can_reserve(10, 3));
        assert_eq!(g.member_label(100), None);
    }
    #[test]
    fn foreign_family_and_conflicting_registry_cannot_be_attributed() {
        let mut g = ready();
        g.reserve(10, 1, 3).unwrap();
        assert!(!g.reconcile(&[o(20, Some(100), 20)], false));
        assert_eq!(g.label(100), None);
        assert!(!g.reconcile(&[o(20, Some(100), 10), o(20, Some(101), 10)], false));
        assert!(!g.can_reserve(10, 3));
    }
}
