//! Process-wide resource admission budget shared by subprocess and SQLite adapters.
//!
//! The global operation cap bounds the number of simultaneously active resource-heavy
//! operations across all interpreter instances. Database queries have a stricter nested cap
//! because each query can materialize up to 16 MiB of output.

use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) const MAX_ACTIVE_RESOURCE_OPERATIONS: usize = 8;
pub(crate) const MAX_ACTIVE_DATABASE_QUERIES: usize = 4;

static ACTIVE_RESOURCE_OPERATIONS: AtomicUsize = AtomicUsize::new(0);
static ACTIVE_DATABASE_QUERIES: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy)]
enum ResourceKind {
    Process,
    Database,
}

pub(crate) struct ResourcePermit {
    kind: ResourceKind,
}

impl ResourcePermit {
    pub(crate) fn process() -> Option<Self> {
        reserve(&ACTIVE_RESOURCE_OPERATIONS, MAX_ACTIVE_RESOURCE_OPERATIONS)
            .then_some(Self { kind: ResourceKind::Process })
    }

    pub(crate) fn database() -> Option<Self> {
        if !reserve(&ACTIVE_RESOURCE_OPERATIONS, MAX_ACTIVE_RESOURCE_OPERATIONS) {
            return None;
        }
        if !reserve(&ACTIVE_DATABASE_QUERIES, MAX_ACTIVE_DATABASE_QUERIES) {
            ACTIVE_RESOURCE_OPERATIONS.fetch_sub(1, Ordering::Release);
            return None;
        }
        Some(Self { kind: ResourceKind::Database })
    }
}

impl Drop for ResourcePermit {
    fn drop(&mut self) {
        if matches!(self.kind, ResourceKind::Database) {
            ACTIVE_DATABASE_QUERIES.fetch_sub(1, Ordering::Release);
        }
        ACTIVE_RESOURCE_OPERATIONS.fetch_sub(1, Ordering::Release);
    }
}

fn reserve(counter: &AtomicUsize, limit: usize) -> bool {
    let mut current = counter.load(Ordering::Acquire);
    loop {
        if current >= limit {
            return false;
        }
        match counter.compare_exchange_weak(
            current,
            current + 1,
            Ordering::AcqRel,
            Ordering::Acquire,
        ) {
            Ok(_) => return true,
            Err(observed) => current = observed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atomic_reservation_never_exceeds_the_configured_limit() {
        let counter = AtomicUsize::new(0);
        assert!(reserve(&counter, 2));
        assert!(reserve(&counter, 2));
        assert!(!reserve(&counter, 2));
        assert_eq!(counter.load(Ordering::Acquire), 2);
        counter.fetch_sub(1, Ordering::Release);
        assert!(reserve(&counter, 2));
        assert_eq!(counter.load(Ordering::Acquire), 2);
    }

    #[test]
    fn configured_database_quota_is_stricter_than_global_operation_quota() {
        assert!(MAX_ACTIVE_DATABASE_QUERIES > 0);
        assert!(MAX_ACTIVE_DATABASE_QUERIES < MAX_ACTIVE_RESOURCE_OPERATIONS);
    }
}
