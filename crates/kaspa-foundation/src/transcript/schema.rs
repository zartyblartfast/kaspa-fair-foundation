//! Versioned transcript schema types.

/// Stable schema identifier/version for foundation proof transcripts.
pub const TRANSCRIPT_SCHEMA_VERSION: &str = "kaspa-fair-transcript-v1";

/// Stable schema identifier/version for the evidence bundle layer referenced by
/// proof transcripts.
pub const EVIDENCE_SCHEMA_VERSION: &str = "kaspa-fair-evidence-v1";

/// Indicates the original character of an ENV step, while still allowing the
/// transcript itself to remain offline evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StepMode {
    Live,
    ReadOnly,
    Offline,
    EvidenceOnly,
}

/// Stable purpose label for an ENV transcript step.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StepPurpose {
    CovenantCreate,
    CovenantSpend,
    ReadOnlyConfirmation,
    EvidenceSummary,
}

/// Safety constraints required to work with a transcript offline.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TranscriptSafetyBoundary {
    pub requires_no_secrets: bool,
    pub requires_no_wallet: bool,
    pub requires_no_signing: bool,
    pub requires_no_network: bool,
    pub requires_no_broadcast: bool,
    pub mainnet_supported: bool,
}

/// One ordered ENV step in a canonical transcript.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TranscriptStep {
    pub env_id: &'static str,
    pub role: &'static str,
    pub purpose: StepPurpose,
    pub fixture_path: &'static str,
    pub mode: StepMode,
    pub historical_live_evidence: bool,
    pub read_only_evidence: bool,
    pub evidence_only: bool,
    pub expected_spend_txid: Option<&'static str>,
    pub expected_input_outpoint: Option<&'static str>,
    pub expected_continuing_output: Option<&'static str>,
    pub expected_continuing_output_value_sompi: Option<u64>,
    pub expected_covenant_id: Option<&'static str>,
}
