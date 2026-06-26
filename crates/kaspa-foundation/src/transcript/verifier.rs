//! Offline proof transcript verifier for the canonical TN10 evidence path.
//!
//! This verifier is intentionally structural and filesystem-local. It checks the
//! committed transcript model and fixture-path links without contacting TN10,
//! mainnet, wallets, signers, or broadcast APIs.

use std::path::{Path, PathBuf};

use crate::covenant::constants::{
    CANONICAL_CONTINUING_OUTPUT, CANONICAL_CONTINUING_OUTPUT_VALUE_SOMPI, CANONICAL_COVENANT_ID,
    CANONICAL_ENV063_INPUT_OUTPOINT, CANONICAL_ENV064_SPEND_TXID,
};

use super::canonical::{canonical_tn10_proof_transcript, CANONICAL_NETWORK};
use super::schema::{EVIDENCE_SCHEMA_VERSION, TRANSCRIPT_SCHEMA_VERSION};
use super::{ProofTranscript, StepMode, StepPurpose, TranscriptStep};

const EXPECTED_ENV_SEQUENCE: [&str; 3] = ["ENV-063", "ENV-064", "ENV-065"];

/// Successful offline verification evidence for a proof transcript.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationReport {
    pub transcript_id: &'static str,
    pub checked_env_sequence: [&'static str; 3],
    pub checked_fixture_paths: usize,
    pub offline_only: bool,
}

/// First failing reason found by the offline transcript verifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerificationError {
    WrongTranscriptSchemaVersion,
    WrongEvidenceSchemaVersion,
    WrongNetwork,
    MainnetSupported,
    UnsafeTranscriptBoundary,
    WrongEnvSequence,
    WrongCanonicalSpendTxid,
    WrongCanonicalInputOutpoint,
    WrongContinuingOutput,
    WrongContinuingOutputValue,
    WrongCovenantId,
    StepCanonicalMismatch,
    MissingFixturePath,
}

/// Offline verifier bound to a repository root for fixture path checks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OfflineTranscriptVerifier {
    repo_root: PathBuf,
}

impl OfflineTranscriptVerifier {
    pub fn new(repo_root: impl Into<PathBuf>) -> Self {
        Self {
            repo_root: repo_root.into(),
        }
    }

    pub fn verify(
        &self,
        transcript: &ProofTranscript,
    ) -> Result<VerificationReport, VerificationError> {
        verify_transcript(transcript, &self.repo_root)
    }
}

/// Verifies the built-in canonical TN10 proof transcript against local fixtures.
pub fn verify_canonical_tn10_transcript(
    repo_root: impl AsRef<Path>,
) -> Result<VerificationReport, VerificationError> {
    verify_transcript(&canonical_tn10_proof_transcript(), repo_root)
}

/// Verifies a proof transcript using only local, offline checks.
pub fn verify_transcript(
    transcript: &ProofTranscript,
    repo_root: impl AsRef<Path>,
) -> Result<VerificationReport, VerificationError> {
    verify_schema_and_safety(transcript)?;
    verify_env_sequence(transcript.steps)?;
    verify_canonical_values(transcript)?;
    verify_step_expectations(transcript.steps)?;
    verify_fixture_paths(transcript.steps, repo_root.as_ref())?;

    Ok(VerificationReport {
        transcript_id: transcript.transcript_id,
        checked_env_sequence: [
            transcript.steps[0].env_id,
            transcript.steps[1].env_id,
            transcript.steps[2].env_id,
        ],
        checked_fixture_paths: transcript.steps.len(),
        offline_only: true,
    })
}

fn verify_schema_and_safety(transcript: &ProofTranscript) -> Result<(), VerificationError> {
    if transcript.transcript_schema_version != TRANSCRIPT_SCHEMA_VERSION {
        return Err(VerificationError::WrongTranscriptSchemaVersion);
    }
    if transcript.evidence_schema_version != EVIDENCE_SCHEMA_VERSION {
        return Err(VerificationError::WrongEvidenceSchemaVersion);
    }
    if transcript.network != CANONICAL_NETWORK {
        return Err(VerificationError::WrongNetwork);
    }
    if transcript.mainnet_supported {
        return Err(VerificationError::MainnetSupported);
    }

    let safety = transcript.safety;
    if !safety.requires_no_secrets
        || !safety.requires_no_wallet
        || !safety.requires_no_signing
        || !safety.requires_no_network
        || !safety.requires_no_broadcast
        || safety.mainnet_supported
    {
        return Err(VerificationError::UnsafeTranscriptBoundary);
    }

    if !transcript.offline_verifier_first
        || !transcript.app_agnostic_foundation_layer
        || transcript.includes_roulette_adapter
    {
        return Err(VerificationError::UnsafeTranscriptBoundary);
    }

    Ok(())
}

fn verify_env_sequence(steps: &[TranscriptStep]) -> Result<(), VerificationError> {
    if steps.len() != EXPECTED_ENV_SEQUENCE.len() {
        return Err(VerificationError::WrongEnvSequence);
    }

    let expected_shapes = [
        ("ENV-063", StepPurpose::CovenantCreate, StepMode::Live),
        ("ENV-064", StepPurpose::CovenantSpend, StepMode::Live),
        (
            "ENV-065",
            StepPurpose::ReadOnlyConfirmation,
            StepMode::ReadOnly,
        ),
    ];

    for (step, (expected_env, expected_purpose, expected_mode)) in steps.iter().zip(expected_shapes)
    {
        if step.env_id != expected_env
            || step.purpose != expected_purpose
            || step.mode != expected_mode
            || !step.evidence_only
        {
            return Err(VerificationError::WrongEnvSequence);
        }
    }

    Ok(())
}

fn verify_canonical_values(transcript: &ProofTranscript) -> Result<(), VerificationError> {
    let canonical = transcript.canonical;

    if canonical.env064_spend_txid != CANONICAL_ENV064_SPEND_TXID {
        return Err(VerificationError::WrongCanonicalSpendTxid);
    }
    if canonical.env063_input_outpoint != CANONICAL_ENV063_INPUT_OUTPOINT {
        return Err(VerificationError::WrongCanonicalInputOutpoint);
    }
    if canonical.continuing_output != CANONICAL_CONTINUING_OUTPUT {
        return Err(VerificationError::WrongContinuingOutput);
    }
    if canonical.continuing_output_value_sompi != CANONICAL_CONTINUING_OUTPUT_VALUE_SOMPI {
        return Err(VerificationError::WrongContinuingOutputValue);
    }
    if canonical.covenant_id != CANONICAL_COVENANT_ID {
        return Err(VerificationError::WrongCovenantId);
    }

    Ok(())
}

fn verify_step_expectations(steps: &[TranscriptStep]) -> Result<(), VerificationError> {
    for step in steps {
        if let Some(txid) = step.expected_spend_txid {
            if txid != CANONICAL_ENV064_SPEND_TXID {
                return Err(VerificationError::StepCanonicalMismatch);
            }
        }
        if let Some(outpoint) = step.expected_input_outpoint {
            if outpoint != CANONICAL_ENV063_INPUT_OUTPOINT {
                return Err(VerificationError::StepCanonicalMismatch);
            }
        }
        if let Some(output) = step.expected_continuing_output {
            if output != CANONICAL_CONTINUING_OUTPUT {
                return Err(VerificationError::StepCanonicalMismatch);
            }
        }
        if let Some(value) = step.expected_continuing_output_value_sompi {
            if value != CANONICAL_CONTINUING_OUTPUT_VALUE_SOMPI {
                return Err(VerificationError::StepCanonicalMismatch);
            }
        }
        if let Some(covenant_id) = step.expected_covenant_id {
            if covenant_id != CANONICAL_COVENANT_ID {
                return Err(VerificationError::StepCanonicalMismatch);
            }
        }
    }

    Ok(())
}

fn verify_fixture_paths(
    steps: &[TranscriptStep],
    repo_root: &Path,
) -> Result<(), VerificationError> {
    for step in steps {
        if step.fixture_path.is_empty()
            || step.fixture_path.starts_with('/')
            || step.fixture_path.contains("..")
            || !repo_root.join(step.fixture_path).exists()
        {
            return Err(VerificationError::MissingFixturePath);
        }
    }

    Ok(())
}
