use one_core::{resolve_candidates, resolve_localized_alias, CapabilityResolutionError, SemanticCapabilityId};

const LOCALIZED_EQUIVALENCE_FIXTURES: &[(&str, &str, SemanticCapabilityId)] = &[
    ("id", "hubungkan", SemanticCapabilityId::NetworkConnect),
    ("en", "connect", SemanticCapabilityId::NetworkConnect),
    ("id", "dengarkan", SemanticCapabilityId::NetworkListen),
    ("en", "listen", SemanticCapabilityId::NetworkListen),
    ("id", "cari perangkat", SemanticCapabilityId::NetworkDiscover),
    ("en", "discover devices", SemanticCapabilityId::NetworkDiscover),
    ("id", "jalankan proses", SemanticCapabilityId::ProcessExecute),
    ("en", "execute process", SemanticCapabilityId::ProcessExecute),
];

#[test]
fn localized_aliases_converge_on_the_same_semantic_ids() {
    for (locale, label, expected) in LOCALIZED_EQUIVALENCE_FIXTURES {
        assert_eq!(resolve_localized_alias(locale, label), Ok(*expected));
    }
}

#[test]
fn ambiguous_candidate_sets_are_rejected_deterministically() {
    let candidates = [SemanticCapabilityId::NetworkConnect, SemanticCapabilityId::NetworkDiscover];
    assert_eq!(resolve_candidates(&candidates), Err(CapabilityResolutionError::Ambiguous));
}

#[test]
fn repeated_equivalent_candidates_are_not_ambiguous() {
    let candidates = [SemanticCapabilityId::NetworkConnect, SemanticCapabilityId::NetworkConnect];
    assert_eq!(resolve_candidates(&candidates), Ok(SemanticCapabilityId::NetworkConnect));
}
