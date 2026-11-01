//! Scalar-only policy. Native adapters must establish ownership and lifetime first.
use crate::policy::Health;

pub fn eligible(owned_living: bool, health: Option<Health>) -> bool {
    owned_living && health.is_some_and(|h| h.hp > 0 && h.hp < h.max)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlightKey {
    pub bullet: u64,
    pub owner: u64,
    pub target: u64,
}

/// Never expire a live projectile by wall-clock time. Capacity exhaustion rejects
/// new deliveries instead of evicting a still-live duplicate suppression key.
#[derive(Default)]
pub struct ImpactLedger {
    consumed: Vec<FlightKey>,
    context: Option<(usize, u64)>,
}
impl ImpactLedger {
    pub const fn new() -> Self {
        Self {
            consumed: Vec::new(),
            context: None,
        }
    }
    /// Context contains scalar world identity and full player handle, never a
    /// dereferenceable actor. Missing/dead-player observations end the session.
    pub fn enter_context(&mut self, context: Option<(usize, u64)>) {
        if self.context != context {
            self.consumed.clear();
            self.context = context;
        }
    }
    /// Only call with a COMPLETE native registry snapshot, not a truncated walk.
    pub fn retain_live(&mut self, live: &[u64]) {
        self.consumed.retain(|key| live.contains(&key.bullet));
    }
    pub fn claim(&mut self, key: FlightKey, owner: u64, receiver: u64) -> bool {
        if key.owner != owner
            || key.target != receiver
            || self.consumed.len() >= 256
            || self.consumed.iter().any(|k| k.bullet == key.bullet)
        {
            return false;
        }
        // Claim before dispatch: a failed/uncertain native effect is not retried.
        self.consumed.push(key);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn shot(bullet: u64, target: u64) -> FlightKey {
        FlightKey {
            bullet,
            owner: 7,
            target,
        }
    }
    #[test]
    fn release_requires_owned_living_injured_target() {
        assert!(eligible(true, Some(Health { hp: 1, max: 100 })));
        assert!(!eligible(false, Some(Health { hp: 1, max: 100 })));
        for h in [
            None,
            Some(Health { hp: 0, max: 100 }),
            Some(Health { hp: 100, max: 100 }),
            Some(Health { hp: 101, max: 100 }),
            Some(Health { hp: -1, max: 0 }),
        ] {
            assert!(!eligible(true, h));
        }
    }
    #[test]
    fn flights_keep_independent_release_targets() {
        let mut ledger = ImpactLedger::new();
        assert!(!ledger.claim(shot(1, 10), 7, 11));
        assert!(!ledger.claim(shot(1, 10), 8, 10));
        assert!(ledger.claim(shot(2, 11), 7, 11));
        assert!(ledger.claim(shot(1, 10), 7, 10));
        assert!(!ledger.claim(shot(1, 10), 7, 10));
    }
    #[test]
    fn live_projectiles_never_expire_and_new_generations_can_deliver() {
        let mut ledger = ImpactLedger::new();
        assert!(ledger.claim(shot(1, 10), 7, 10));
        for _ in 0..1000 {
            ledger.retain_live(&[1]);
        }
        assert!(!ledger.claim(shot(1, 10), 7, 10));
        assert!(ledger.claim(shot(0x100000001, 10), 7, 10));
        ledger.retain_live(&[]);
        assert!(ledger.consumed.is_empty());
    }
    #[test]
    fn full_ledger_never_evicts_a_live_key() {
        let mut ledger = ImpactLedger::new();
        for b in 0..256 {
            assert!(ledger.claim(shot(b, 10), 7, 10));
        }
        assert!(!ledger.claim(shot(256, 10), 7, 10));
        assert!(!ledger.claim(shot(0, 10), 7, 10));
        ledger.retain_live(&[0]);
        assert!(ledger.claim(shot(256, 10), 7, 10));
    }
    #[test]
    fn reload_and_player_generation_end_old_delivery_context() {
        let mut ledger = ImpactLedger::new();
        ledger.enter_context(Some((1, 7)));
        assert!(ledger.claim(shot(1, 10), 7, 10));
        ledger.enter_context(Some((1, 7)));
        assert!(!ledger.claim(shot(1, 10), 7, 10));
        ledger.enter_context(None);
        ledger.enter_context(Some((1, 7)));
        assert!(ledger.claim(shot(1, 10), 7, 10));
        ledger.enter_context(Some((1, 8)));
        assert!(ledger.consumed.is_empty());
    }
}
