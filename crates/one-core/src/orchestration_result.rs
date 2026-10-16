use crate::adapter::AdapterResult;
use crate::orchestration::AdapterId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NormalizationExpectation {
    Bytes,
    Utf8Text,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NormalizedValue {
    Bytes(Vec<u8>),
    Text(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceCertainty {
    Observed,
    Uncertain,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionEvidence {
    adapter_id: AdapterId,
    success: bool,
    certainty: EvidenceCertainty,
}

impl ExecutionEvidence {
    pub fn adapter_id(&self) -> &AdapterId { &self.adapter_id }
    pub const fn success(&self) -> bool { self.success }
    pub const fn certainty(&self) -> EvidenceCertainty { self.certainty }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NormalizedResult {
    value: NormalizedValue,
    evidence: ExecutionEvidence,
}

impl NormalizedResult {
    pub fn value(&self) -> &NormalizedValue { &self.value }
    pub fn evidence(&self) -> &ExecutionEvidence { &self.evidence }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrchestrationResultError {
    MalformedOutput,
}

pub fn normalize_result(
    adapter_id: AdapterId,
    result: AdapterResult,
    expectation: NormalizationExpectation,
) -> Result<NormalizedResult, OrchestrationResultError> {
    let value = match expectation {
        NormalizationExpectation::Bytes => NormalizedValue::Bytes(result.output),
        NormalizationExpectation::Utf8Text => NormalizedValue::Text(
            String::from_utf8(result.output).map_err(|_| OrchestrationResultError::MalformedOutput)?,
        ),
    };
    let certainty = if result.success { EvidenceCertainty::Observed } else { EvidenceCertainty::Uncertain };
    Ok(NormalizedResult {
        value,
        evidence: ExecutionEvidence { adapter_id, success: result.success, certainty },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::{AdapterResult, PlatformKind};
    use crate::orchestration::AdapterId;

    #[test]
    fn utf8_output_is_normalized_without_inventing_facts() {
        let result = AdapterResult { success: true, output: b"hello".to_vec() };
        let normalized = normalize_result(AdapterId::new("test.adapter"), result, NormalizationExpectation::Utf8Text).expect("valid UTF-8 should normalize");
        assert_eq!(normalized.value(), &NormalizedValue::Text("hello".into()));
        assert_eq!(normalized.evidence().adapter_id().as_str(), "test.adapter");
        assert_eq!(normalized.evidence().certainty(), EvidenceCertainty::Observed);
        let _ = PlatformKind::Linux;
    }

    #[test]
    fn malformed_utf8_is_a_deterministic_normalization_error() {
        let result = AdapterResult { success: true, output: vec![0xff, 0xfe] };
        assert_eq!(normalize_result(AdapterId::new("test.adapter"), result, NormalizationExpectation::Utf8Text), Err(OrchestrationResultError::MalformedOutput));
    }

    #[test]
    fn binary_output_preserves_bytes_and_uncertainty() {
        let result = AdapterResult { success: false, output: vec![1, 2, 3] };
        let normalized = normalize_result(AdapterId::new("test.adapter"), result, NormalizationExpectation::Bytes).expect("bytes should normalize");
        assert_eq!(normalized.value(), &NormalizedValue::Bytes(vec![1, 2, 3]));
        assert_eq!(normalized.evidence().certainty(), EvidenceCertainty::Uncertain);
    }
}
