use crate::orchestration::{AdapterId, SemanticCapabilityId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditEvent {
    Denied,
    InvocationFinished,
    Uncertain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditEvidence {
    DecisionOnly,
    ObservedSuccess,
    ObservedFailure,
    Uncertain,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditRecord {
    sequence: u64,
    semantic: SemanticCapabilityId,
    adapter: Option<AdapterId>,
    event: AuditEvent,
    attempted_execution: bool,
    evidence: AuditEvidence,
    detail: String,
}

impl AuditRecord {
    pub fn denied(sequence: u64, semantic: SemanticCapabilityId, detail: impl Into<String>) -> Self {
        Self { sequence, semantic, adapter: None, event: AuditEvent::Denied, attempted_execution: false, evidence: AuditEvidence::DecisionOnly, detail: detail.into() }
    }

    pub fn invocation_finished(sequence: u64, semantic: SemanticCapabilityId, adapter: AdapterId, success: bool) -> Self {
        Self { sequence, semantic, adapter: Some(adapter), event: AuditEvent::InvocationFinished, attempted_execution: true, evidence: if success { AuditEvidence::ObservedSuccess } else { AuditEvidence::ObservedFailure }, detail: String::new() }
    }

    pub fn uncertain(sequence: u64, semantic: SemanticCapabilityId, adapter: AdapterId) -> Self {
        Self { sequence, semantic, adapter: Some(adapter), event: AuditEvent::Uncertain, attempted_execution: true, evidence: AuditEvidence::Uncertain, detail: String::new() }
    }

    pub const fn sequence(&self) -> u64 { self.sequence }
    pub const fn semantic(&self) -> SemanticCapabilityId { self.semantic }
    pub const fn event(&self) -> AuditEvent { self.event }
    pub const fn attempted_execution(&self) -> bool { self.attempted_execution }
    pub const fn evidence(&self) -> AuditEvidence { self.evidence }
    pub fn adapter(&self) -> Option<&AdapterId> { self.adapter.as_ref() }
    pub fn detail(&self) -> &str { &self.detail }
}

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
        let record = AuditRecord::invocation_finished(2, SemanticCapabilityId::NetworkConnect, AdapterId::new("network.native"), false);
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
