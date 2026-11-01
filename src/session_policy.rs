//! Scalar solo-session policy. No native calls, pointers, or logging.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SoloMembers {
    pub count: usize,
    pub all_local: bool,
    pub host_is_local_or_empty: bool,
}

/// Keep the standalone offline policy unchanged. Only the hash-pinned Seamless
/// profile may treat protocol 6 (native Ingame) as solo, with no lobby/remote peer.
pub fn allowed(
    online: Option<bool>,
    session: Option<(u32, u32)>,
    conflict: bool,
    pinned_seamless: bool,
    members: Option<SoloMembers>,
) -> bool {
    if !pinned_seamless {
        return crate::policy::offline_allowed(online, session, conflict);
    }
    if conflict || online != Some(false) || !matches!(session, Some((0, 0 | 6))) {
        return false;
    }
    members.is_some_and(|m| m.count <= 1 && m.all_local && m.host_is_local_or_empty)
}

/// Read the pinned MSVC vector header as scalar addresses before constructing
/// a slice. Reject partial entries, inverted ranges, excessive capacity and
/// noncanonical/misaligned addresses. A remote/multiple-player list needs no walk.
pub fn solo_vector_count(first: usize, last: usize, end: usize) -> Option<usize> {
    if first == 0 {
        return (last == 0 && end == 0).then_some(0);
    }
    let valid = |p: usize| (0x10000..0x0000_8000_0000_0000).contains(&p) && p.is_multiple_of(8);
    if !valid(first) || !valid(last) || !valid(end) || last < first || end < last {
        return None;
    }
    let used = last - first;
    let allocated = end - first;
    if !used.is_multiple_of(0x100) || !allocated.is_multiple_of(0x100) || allocated > 0x100 * 64 {
        return None;
    }
    Some(used / 0x100)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SOLO: SoloMembers = SoloMembers {
        count: 0,
        all_local: true,
        host_is_local_or_empty: true,
    };
    fn seam(session: Option<(u32, u32)>, members: Option<SoloMembers>) -> bool {
        allowed(Some(false), session, false, true, members)
    }
    #[test]
    fn observed_solo_ingame_requires_pinned_profile() {
        assert!(seam(Some((0, 6)), Some(SOLO)));
        assert!(seam(Some((0, 0)), Some(SOLO)));
        assert!(seam(Some((0, 6)), Some(SoloMembers { count: 1, ..SOLO })));
        assert!(!allowed(
            Some(false),
            Some((0, 6)),
            false,
            false,
            Some(SOLO)
        ));
        assert!(allowed(Some(false), Some((0, 0)), false, false, None));
    }
    #[test]
    fn remote_peer_host_and_unavailable_membership_reject() {
        for state in [0, 6] {
            for member in [
                None,
                Some(SoloMembers { count: 2, ..SOLO }),
                Some(SoloMembers {
                    count: 1,
                    all_local: false,
                    ..SOLO
                }),
                Some(SoloMembers {
                    host_is_local_or_empty: false,
                    ..SOLO
                }),
            ] {
                assert!(!seam(Some((0, state)), member));
            }
        }
    }
    #[test]
    fn lobby_transitions_online_and_unknown_state_stay_rejected() {
        for lobby in 1..=8 {
            assert!(!seam(Some((lobby, 6)), Some(SOLO)));
        }
        for protocol in [1, 2, 3, 4, 5, 7, 8, u32::MAX] {
            assert!(!seam(Some((0, protocol)), Some(SOLO)));
        }
        assert!(!seam(None, Some(SOLO)));
        for online in [None, Some(true)] {
            assert!(!allowed(online, Some((0, 6)), false, true, Some(SOLO)));
        }
        assert!(!allowed(Some(false), Some((0, 6)), true, true, Some(SOLO)));
    }
    #[test]
    fn connection_and_disconnection_are_reassessed_without_cached_permission() {
        let states = [
            Some(SOLO),
            Some(SoloMembers {
                count: 1,
                all_local: false,
                ..SOLO
            }),
            None,
            Some(SOLO),
        ];
        assert_eq!(
            states.map(|m| seam(Some((0, 6)), m)),
            [true, false, false, true]
        );
    }
    #[test]
    fn vector_bounds_reject_malformed_or_partial_entries() {
        assert_eq!(solo_vector_count(0, 0, 0), Some(0));
        assert_eq!(solo_vector_count(0x10000, 0x10000, 0x10200), Some(0));
        assert_eq!(solo_vector_count(0x10000, 0x10100, 0x10200), Some(1));
        assert_eq!(solo_vector_count(0x10000, 0x10200, 0x10200), Some(2));
        for (first, last, end) in [
            (0, 1, 0),
            (0x10001, 0x10101, 0x10101),
            (0x10100, 0x10000, 0x10200),
            (0x10000, 0x10100, 0x10000),
            (0x10000, 0x10080, 0x10100),
            (0x10000, 0x10000, 0x14100),
            (usize::MAX, usize::MAX, usize::MAX),
        ] {
            assert_eq!(solo_vector_count(first, last, end), None);
        }
    }
}
