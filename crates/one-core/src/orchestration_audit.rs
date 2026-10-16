#[cfg(test)]
mod tests {
    use super::*;
    use crate::orchestration::{AdapterId, SemanticCapabilityId};

    #[test]
    fn denied_operation_is_not_recorded_as_attempted_execution() {
        let record = AuditRecord::denied(1, SemanticCapabilityId::NetworkConnect, "policy");
        assert_eq!(record.event(), AuditEvent::Denied);
        assert!(!record.attempted_execution());
        assert_eq!(record.evidence(), AuditEvidence::DecisionOnly);
    }

    #[test]
    fn adapter_invocation_is_recorded_as_attempted_not_automatically_verified() {
        let record = AuditRecord::invocation_finished(
            2,
            SemanticCapabilityId::NetworkConnect,
            AdapterId::new("network.native"),
            false,
        );
        assert_eq!(record.event(), AuditEvent::InvocationFinished);
        assert!(record.attempted_execution());
        assert_eq!(record.evidence(), AuditEvidence::ObservedFailure);
    }

    #[test]
    fn uncertain_results_remain_uncertain_in_audit() {
        let record = AuditRecord::uncertain(3, SemanticCapabilityId::NetworkConnect, AdapterId::new("network.native"));
        assert_eq!(record.evidence(), AuditEvidence::Uncertain);
        assert!(record.attempted_execution());
    }
}
