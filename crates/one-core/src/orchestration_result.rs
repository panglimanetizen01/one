#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::{AdapterResult, PlatformKind};
    use crate::orchestration::AdapterId;

    #[test]
    fn utf8_output_is_normalized_without_inventing_facts() {
        let result = AdapterResult { success: true, output: b"hello".to_vec() };
        let normalized = normalize_result(
            AdapterId::new("test.adapter"),
            result,
            NormalizationExpectation::Utf8Text,
        ).expect("valid UTF-8 should normalize");
        assert_eq!(normalized.value(), &NormalizedValue::Text("hello".into()));
        assert_eq!(normalized.evidence().adapter_id().as_str(), "test.adapter");
        assert_eq!(normalized.evidence().certainty(), EvidenceCertainty::Observed);
        let _ = PlatformKind::Linux;
    }

    #[test]
    fn malformed_utf8_is_a_deterministic_normalization_error() {
        let result = AdapterResult { success: true, output: vec![0xff, 0xfe] };
        assert_eq!(
            normalize_result(AdapterId::new("test.adapter"), result, NormalizationExpectation::Utf8Text),
            Err(OrchestrationResultError::MalformedOutput)
        );
    }

    #[test]
    fn binary_output_preserves_bytes_and_uncertainty() {
        let result = AdapterResult { success: false, output: vec![1, 2, 3] };
        let normalized = normalize_result(
            AdapterId::new("test.adapter"),
            result,
            NormalizationExpectation::Bytes,
        ).expect("bytes should normalize");
        assert_eq!(normalized.value(), &NormalizedValue::Bytes(vec![1, 2, 3]));
        assert_eq!(normalized.evidence().certainty(), EvidenceCertainty::Uncertain);
    }
}
