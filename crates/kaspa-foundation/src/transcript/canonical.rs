//! Canonical TN10 transcript metadata derived from committed ENV-063/064/065 fixtures.

use crate::covenant::constants::{
    CANONICAL_CONTINUING_OUTPUT, CANONICAL_CONTINUING_OUTPUT_VALUE_SOMPI, CANONICAL_COVENANT_ID,
    CANONICAL_ENV063_INPUT_OUTPOINT, CANONICAL_ENV064_SPEND_TXID,
};
use crate::evidence::canonical_fixtures::canonical_fixture_set;
use crate::safety::default_capabilities;

use super::schema::{
    StepMode, StepPurpose, TranscriptSafetyBoundary, TranscriptStep, EVIDENCE_SCHEMA_VERSION,
    TRANSCRIPT_SCHEMA_VERSION,
};

pub const CANONICAL_TRANSCRIPT_ID: &str = "canonical-tn10-covenant-path";
pub const CANONICAL_NETWORK: &str = "TN10/testnet-10";

/// Canonical TN10 values proven by ENV-063/064/065 and reused by later verifiers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanonicalTranscriptValues {
    pub env064_spend_txid: &'static str,
    pub env063_input_outpoint: &'static str,
    pub continuing_output: &'static str,
    pub continuing_output_value_sompi: u64,
    pub covenant_id: &'static str,
}

/// Stable foundation transcript for the proven TN10 corrected covenant path.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProofTranscript {
    pub transcript_schema_version: &'static str,
    pub evidence_schema_version: &'static str,
    pub transcript_id: &'static str,
    pub network: &'static str,
    pub mainnet_supported: bool,
    pub offline_verifier_first: bool,
    pub online_verifier_later: bool,
    pub app_agnostic_foundation_layer: bool,
    pub includes_roulette_adapter: bool,
    pub safety: TranscriptSafetyBoundary,
    pub canonical: CanonicalTranscriptValues,
    pub steps: &'static [TranscriptStep],
}

const CANONICAL_STEPS: [TranscriptStep; 3] = [
    TranscriptStep {
        env_id: "ENV-063",
        role: "canonical create evidence",
        purpose: StepPurpose::CovenantCreate,
        fixture_path:
            "fixtures/tn10-canonical-covenant-path/env-063-corrected-live-covenant-create/",
        mode: StepMode::Live,
        historical_live_evidence: true,
        read_only_evidence: false,
        evidence_only: true,
        expected_spend_txid: None,
        expected_input_outpoint: None,
        expected_continuing_output: None,
        expected_continuing_output_value_sompi: None,
        expected_covenant_id: Some(CANONICAL_COVENANT_ID),
    },
    TranscriptStep {
        env_id: "ENV-064",
        role: "canonical spend evidence",
        purpose: StepPurpose::CovenantSpend,
        fixture_path:
            "fixtures/tn10-canonical-covenant-path/env-064-live-corrected-covenant-spend/",
        mode: StepMode::Live,
        historical_live_evidence: true,
        read_only_evidence: false,
        evidence_only: true,
        expected_spend_txid: Some(CANONICAL_ENV064_SPEND_TXID),
        expected_input_outpoint: Some(CANONICAL_ENV063_INPUT_OUTPOINT),
        expected_continuing_output: Some(CANONICAL_CONTINUING_OUTPUT),
        expected_continuing_output_value_sompi: Some(CANONICAL_CONTINUING_OUTPUT_VALUE_SOMPI),
        expected_covenant_id: Some(CANONICAL_COVENANT_ID),
    },
    TranscriptStep {
        env_id: "ENV-065",
        role: "read-only confirmation evidence",
        purpose: StepPurpose::ReadOnlyConfirmation,
        fixture_path:
            "fixtures/tn10-canonical-covenant-path/env-065-readonly-env064-spend-confirmation/",
        mode: StepMode::ReadOnly,
        historical_live_evidence: false,
        read_only_evidence: true,
        evidence_only: true,
        expected_spend_txid: Some(CANONICAL_ENV064_SPEND_TXID),
        expected_input_outpoint: Some(CANONICAL_ENV063_INPUT_OUTPOINT),
        expected_continuing_output: Some(CANONICAL_CONTINUING_OUTPUT),
        expected_continuing_output_value_sompi: Some(CANONICAL_CONTINUING_OUTPUT_VALUE_SOMPI),
        expected_covenant_id: Some(CANONICAL_COVENANT_ID),
    },
];

pub fn canonical_tn10_proof_transcript() -> ProofTranscript {
    let fixtures = canonical_fixture_set();
    let capabilities = default_capabilities();

    ProofTranscript {
        transcript_schema_version: TRANSCRIPT_SCHEMA_VERSION,
        evidence_schema_version: EVIDENCE_SCHEMA_VERSION,
        transcript_id: CANONICAL_TRANSCRIPT_ID,
        network: CANONICAL_NETWORK,
        mainnet_supported: capabilities.mainnet_enabled,
        offline_verifier_first: true,
        online_verifier_later: true,
        app_agnostic_foundation_layer: true,
        includes_roulette_adapter: false,
        safety: TranscriptSafetyBoundary {
            requires_no_secrets: true,
            requires_no_wallet: true,
            requires_no_signing: !capabilities.signing_api_exposed,
            requires_no_network: true,
            requires_no_broadcast: !capabilities.submit_api_exposed,
            mainnet_supported: capabilities.mainnet_enabled,
        },
        canonical: CanonicalTranscriptValues {
            env064_spend_txid: fixtures.env064_spend_txid,
            env063_input_outpoint: fixtures.env063_input_outpoint,
            continuing_output: fixtures.continuing_output,
            continuing_output_value_sompi: fixtures.continuing_output_value_sompi,
            covenant_id: fixtures.covenant_id,
        },
        steps: &CANONICAL_STEPS,
    }
}
