//! Process-wide resource admission budget shared by subprocess, filesystem, DNS/HTTP, and SQLite adapters.
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
    Operation,
}

pub(crate) struct ResourcePermit {
    kind: ResourceKind,
}

impl ResourcePermit {
    pub(crate) fn operation() -> Option<Self> {
        reserve(&ACTIVE_RESOURCE_OPERATIONS, MAX_ACTIVE_RESOURCE_OPERATIONS)
            .then(|| Self { kind: ResourceKind::Operation })
    }

    pub(crate) fn process() -> Option<Self> {
        reserve(&ACTIVE_RESOURCE_OPERATIONS, MAX_ACTIVE_RESOURCE_OPERATIONS)
            .then(|| Self { kind: ResourceKind::Process })
    }

    pub(crate) fn database() -> Option<Self> {
        reserve_database(
            &ACTIVE_RESOURCE_OPERATIONS,
            &ACTIVE_DATABASE_QUERIES,
            MAX_ACTIVE_RESOURCE_OPERATIONS,
            MAX_ACTIVE_DATABASE_QUERIES,
        )
        .then(|| Self { kind: ResourceKind::Database })
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

fn reserve_database(
    active_operations: &AtomicUsize,
    active_database_queries: &AtomicUsize,
    operation_limit: usize,
    database_limit: usize,
) -> bool {
    if !reserve(active_operations, operation_limit) {
        return false;
    }
    if !reserve(active_database_queries, database_limit) {
        active_operations.fetch_sub(1, Ordering::Release);
        return false;
    }
    true
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
    fn database_quota_denial_releases_global_reservation_without_exceeding_limits() {
        let active_operations = AtomicUsize::new(0);
        let active_database_queries = AtomicUsize::new(0);

        assert!(reserve_database(&active_operations, &active_database_queries, 2, 1));
        assert_eq!(active_operations.load(Ordering::Acquire), 1);
        assert_eq!(active_database_queries.load(Ordering::Acquire), 1);

        assert!(!reserve_database(&active_operations, &active_database_queries, 2, 1));
        assert_eq!(active_operations.load(Ordering::Acquire), 1);
        assert_eq!(active_database_queries.load(Ordering::Acquire), 1);

        active_database_queries.fetch_sub(1, Ordering::Release);
        active_operations.fetch_sub(1, Ordering::Release);
        assert!(reserve_database(&active_operations, &active_database_queries, 2, 1));
    }

    #[test]
    fn global_operation_quota_denies_process_database_and_adapter_admission_when_full() {
        let active_operations = AtomicUsize::new(2);
        let active_database_queries = AtomicUsize::new(0);

        assert!(!reserve(&active_operations, 2));
        assert!(!reserve_database(&active_operations, &active_database_queries, 2, 1));
        // Filesystem/DNS/HTTP adapters use the same global operation counter.
        assert!(!reserve(&active_operations, 2));
        assert_eq!(active_operations.load(Ordering::Acquire), 2);
        assert_eq!(active_database_queries.load(Ordering::Acquire), 0);
    }
}
