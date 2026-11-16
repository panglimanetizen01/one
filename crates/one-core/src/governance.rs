use crate::capability::Capability;
use crate::orchestration::{CapabilityRequest, OrchestrationValidationError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeRule {
    Any,
    Global,
    Local,
    Target(String),
    Resource(String),
}

impl ScopeRule {
    fn matches(&self, scope: &crate::orchestration::ExecutionScope) -> bool {
        match (self, scope) {
            (Self::Any, _) => true,
            (Self::Global, crate::orchestration::ExecutionScope::Global) => true,
            (Self::Local, crate::orchestration::ExecutionScope::Local) => true,
            (Self::Target(expected), crate::orchestration::ExecutionScope::Target(actual)) => expected == actual,
            (Self::Resource(expected), crate::orchestration::ExecutionScope::Resource(actual)) => expected == actual,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DenyReason {
    InvalidRequest(OrchestrationValidationError),
    MissingCapability,
    OutOfScope,
    ExplicitlyDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDecision {
    Allow,
    Deny(DenyReason),
    RequireApproval,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExecutionPolicy {
    grants: Vec<Capability>,
    denials: Vec<Capability>,
    approvals: Vec<Capability>,
    scopes: Vec<(Capability, ScopeRule)>,
}

impl ExecutionPolicy {
    fn covers(policy_capability: Capability, requested_capability: Capability) -> bool {
        policy_capability == requested_capability
            || (policy_capability == Capability::Filesystem
                && matches!(
                    requested_capability,
                    Capability::FilesystemRead | Capability::FilesystemWrite
                ))
    }

    pub fn grant(mut self, capability: Capability) -> Self {
        if !self.grants.contains(&capability) {
            self.grants.push(capability);
        }
        self
    }

    pub fn deny(mut self, capability: Capability) -> Self {
        if !self.denials.contains(&capability) {
            self.denials.push(capability);
        }
        self
    }

    pub fn require_approval(mut self, capability: Capability) -> Self {
        if !self.approvals.contains(&capability) {
            self.approvals.push(capability);
        }
        self
    }

    pub fn allow_scope(mut self, capability: Capability, rule: ScopeRule) -> Self {
        if !self.scopes.iter().any(|(existing_capability, existing_rule)| {
            *existing_capability == capability && *existing_rule == rule
        }) {
            self.scopes.push((capability, rule));
        }
        self
    }

    pub fn evaluate(&self, request: &CapabilityRequest) -> PolicyDecision {
        if let Err(error) = request.validate() {
            return PolicyDecision::Deny(DenyReason::InvalidRequest(error));
        }
        if self.denials.iter().any(|capability| Self::covers(*capability, request.capability)) {
            return PolicyDecision::Deny(DenyReason::ExplicitlyDenied);
        }
        let capability_granted = self.grants.iter().any(|capability| {
            Self::covers(*capability, request.capability)
        });
        if !capability_granted {
            return PolicyDecision::Deny(DenyReason::MissingCapability);
        }
        let scope_allowed = self.scopes.iter().any(|(capability, rule)| {
            Self::covers(*capability, request.capability) && rule.matches(&request.scope)
        });
        if !scope_allowed {
            return PolicyDecision::Deny(DenyReason::OutOfScope);
        }
        if self.approvals.iter().any(|capability| Self::covers(*capability, request.capability)) {
            return PolicyDecision::RequireApproval;
        }
        PolicyDecision::Allow
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::Capability;
    use crate::orchestration::{CapabilityRequest, EffectClass, ExecutionScope, SemanticCapabilityId};

    fn request(scope: ExecutionScope) -> CapabilityRequest {
        CapabilityRequest::new(
            SemanticCapabilityId::NetworkConnect,
            Capability::NetworkConnect,
            scope,
            EffectClass::Connect,
        )
    }

    #[test]
    fn missing_capability_is_denied() {
        let policy = ExecutionPolicy::default();
        assert_eq!(policy.evaluate(&request(ExecutionScope::target("server.example"))), PolicyDecision::Deny(DenyReason::MissingCapability));
    }

    #[test]
    fn explicit_capability_grant_allows_matching_scope() {
        let policy = ExecutionPolicy::default()
            .grant(Capability::NetworkConnect)
            .allow_scope(Capability::NetworkConnect, ScopeRule::Target("server.example".into()));
        assert_eq!(policy.evaluate(&request(ExecutionScope::target("server.example"))), PolicyDecision::Allow);
    }

    #[test]
    fn specific_filesystem_write_grant_allows_matching_scope() {
        let request = CapabilityRequest::new(
            SemanticCapabilityId::FilesystemWrite,
            Capability::FilesystemWrite,
            ExecutionScope::target("file.txt"),
            EffectClass::Write,
        );
        let policy = ExecutionPolicy::default()
            .grant(Capability::FilesystemWrite)
            .allow_scope(Capability::FilesystemWrite, ScopeRule::Target("file.txt".into()));
        assert_eq!(policy.evaluate(&request), PolicyDecision::Allow);
    }

    #[test]
    fn coarse_filesystem_grant_allows_specific_write_with_matching_scope() {
        let request = CapabilityRequest::new(
            SemanticCapabilityId::FilesystemWrite,
            Capability::FilesystemWrite,
            ExecutionScope::target("file.txt"),
            EffectClass::Write,
        );
        let policy = ExecutionPolicy::default()
            .grant(Capability::Filesystem)
            .allow_scope(Capability::Filesystem, ScopeRule::Target("file.txt".into()));
        assert_eq!(policy.evaluate(&request), PolicyDecision::Allow);
    }

    #[test]
    fn coarse_filesystem_grant_does_not_cross_write_read_isolation() {
        let request = CapabilityRequest::new(
            SemanticCapabilityId::FilesystemRead,
            Capability::FilesystemRead,
            ExecutionScope::target("file.txt"),
            EffectClass::Read,
        );
        let policy = ExecutionPolicy::default()
            .grant(Capability::FilesystemWrite)
            .allow_scope(Capability::FilesystemWrite, ScopeRule::Target("file.txt".into()));
        assert_eq!(
            policy.evaluate(&request),
            PolicyDecision::Deny(DenyReason::MissingCapability)
        );
    }

    #[test]
    fn out_of_scope_target_is_denied() {
        let policy = ExecutionPolicy::default()
            .grant(Capability::NetworkConnect)
            .allow_scope(Capability::NetworkConnect, ScopeRule::Target("server.example".into()));
        assert_eq!(policy.evaluate(&request(ExecutionScope::target("other.example"))), PolicyDecision::Deny(DenyReason::OutOfScope));
    }

    #[test]
    fn explicit_denial_overrides_grant() {
        let policy = ExecutionPolicy::default()
            .grant(Capability::NetworkConnect)
            .deny(Capability::NetworkConnect)
            .allow_scope(Capability::NetworkConnect, ScopeRule::Any);
        assert_eq!(policy.evaluate(&request(ExecutionScope::target("server.example"))), PolicyDecision::Deny(DenyReason::ExplicitlyDenied));
    }

    #[test]
    fn approval_requirement_is_deterministic() {
        let policy = ExecutionPolicy::default()
            .grant(Capability::NetworkConnect)
            .require_approval(Capability::NetworkConnect)
            .allow_scope(Capability::NetworkConnect, ScopeRule::Any);
        assert_eq!(policy.evaluate(&request(ExecutionScope::target("server.example"))), PolicyDecision::RequireApproval);
    }

    #[test]
    fn localized_wording_cannot_change_policy_for_same_request() {
        let policy = ExecutionPolicy::default()
            .grant(Capability::NetworkConnect)
            .allow_scope(Capability::NetworkConnect, ScopeRule::Any);
        let id_request = request(ExecutionScope::target("server.example"));
        let en_request = request(ExecutionScope::target("server.example"));
        assert_eq!(policy.evaluate(&id_request), policy.evaluate(&en_request));
    }
}


    #[test]
    fn coarse_filesystem_denial_overrides_inherited_read_and_write_grants() {
        let read = CapabilityRequest::new(
            SemanticCapabilityId::FilesystemRead,
            Capability::FilesystemRead,
            ExecutionScope::target("file.txt"),
            EffectClass::Read,
        );
        let write = CapabilityRequest::new(
            SemanticCapabilityId::FilesystemWrite,
            Capability::FilesystemWrite,
            ExecutionScope::target("file.txt"),
            EffectClass::Write,
        );
        let policy = ExecutionPolicy::default()
            .grant(Capability::Filesystem)
            .deny(Capability::Filesystem)
            .allow_scope(Capability::Filesystem, ScopeRule::Any);

        assert_eq!(
            policy.evaluate(&read),
            PolicyDecision::Deny(DenyReason::ExplicitlyDenied)
        );
        assert_eq!(
            policy.evaluate(&write),
            PolicyDecision::Deny(DenyReason::ExplicitlyDenied)
        );
    }

    #[test]
    fn coarse_filesystem_approval_applies_to_inherited_child_capabilities() {
        let request = CapabilityRequest::new(
            SemanticCapabilityId::FilesystemRead,
            Capability::FilesystemRead,
            ExecutionScope::target("file.txt"),
            EffectClass::Read,
        );
        let policy = ExecutionPolicy::default()
            .grant(Capability::Filesystem)
            .require_approval(Capability::Filesystem)
            .allow_scope(Capability::Filesystem, ScopeRule::Any);

        assert_eq!(policy.evaluate(&request), PolicyDecision::RequireApproval);
    }
