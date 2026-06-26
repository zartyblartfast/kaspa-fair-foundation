use std::path::PathBuf;

use kaspa_foundation::transcript::{
    canonical::canonical_tn10_proof_transcript,
    verifier::{
        verify_canonical_tn10_transcript, verify_transcript, OfflineTranscriptVerifier,
        VerificationError,
    },
    CanonicalTranscriptValues, ProofTranscript, StepMode, StepPurpose, TranscriptSafetyBoundary,
    TranscriptStep,
};

const GOOD_STEP_0: TranscriptStep = TranscriptStep {
    env_id: "ENV-063",
    role: "canonical create evidence",
    purpose: StepPurpose::CovenantCreate,
    fixture_path: "fixtures/tn10-canonical-covenant-path/env-063-corrected-live-covenant-create/",
    mode: StepMode::Live,
    historical_live_evidence: true,
    read_only_evidence: false,
    evidence_only: true,
    expected_spend_txid: None,
    expected_input_outpoint: None,
    expected_continuing_output: None,
    expected_continuing_output_value_sompi: None,
    expected_covenant_id: Some("e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7"),
};

const GOOD_STEP_1: TranscriptStep = TranscriptStep {
    env_id: "ENV-064",
    role: "canonical spend evidence",
    purpose: StepPurpose::CovenantSpend,
    fixture_path: "fixtures/tn10-canonical-covenant-path/env-064-live-corrected-covenant-spend/",
    mode: StepMode::Live,
    historical_live_evidence: true,
    read_only_evidence: false,
    evidence_only: true,
    expected_spend_txid: Some("4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c"),
    expected_input_outpoint: Some(
        "2c7802ff9a6eec2828a96168d8f62a9a276176441ed8cb6086cd5d5d0cb26849:0",
    ),
    expected_continuing_output: Some(
        "4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c:0",
    ),
    expected_continuing_output_value_sompi: Some(99_700_000),
    expected_covenant_id: Some("e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7"),
};

const GOOD_STEP_2: TranscriptStep = TranscriptStep {
    env_id: "ENV-065",
    role: "read-only confirmation evidence",
    purpose: StepPurpose::ReadOnlyConfirmation,
    fixture_path:
        "fixtures/tn10-canonical-covenant-path/env-065-readonly-env064-spend-confirmation/",
    mode: StepMode::ReadOnly,
    historical_live_evidence: false,
    read_only_evidence: true,
    evidence_only: true,
    expected_spend_txid: Some("4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c"),
    expected_input_outpoint: Some(
        "2c7802ff9a6eec2828a96168d8f62a9a276176441ed8cb6086cd5d5d0cb26849:0",
    ),
    expected_continuing_output: Some(
        "4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c:0",
    ),
    expected_continuing_output_value_sompi: Some(99_700_000),
    expected_covenant_id: Some("e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7"),
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn good_transcript() -> ProofTranscript {
    canonical_tn10_proof_transcript()
}

fn assert_rejected(transcript: ProofTranscript, expected: VerificationError) {
    let err = verify_transcript(&transcript, repo_root())
        .expect_err("mutated transcript should be rejected");
    assert_eq!(err, expected);
}

#[test]
fn canonical_transcript_verifies_successfully() {
    let report =
        verify_canonical_tn10_transcript(repo_root()).expect("canonical transcript verifies");

    assert_eq!(report.transcript_id, "canonical-tn10-covenant-path");
    assert_eq!(
        report.checked_env_sequence,
        ["ENV-063", "ENV-064", "ENV-065"]
    );
    assert_eq!(report.checked_fixture_paths, 3);
    assert!(report.offline_only);

    let verifier = OfflineTranscriptVerifier::new(repo_root());
    assert!(verifier.verify(&good_transcript()).is_ok());
}

#[test]
fn wrong_schema_versions_are_rejected() {
    assert_rejected(
        ProofTranscript {
            transcript_schema_version: "kaspa-fair-transcript-v2",
            ..good_transcript()
        },
        VerificationError::WrongTranscriptSchemaVersion,
    );

    assert_rejected(
        ProofTranscript {
            evidence_schema_version: "kaspa-fair-evidence-v2",
            ..good_transcript()
        },
        VerificationError::WrongEvidenceSchemaVersion,
    );
}

#[test]
fn wrong_env_sequence_is_rejected() {
    static WRONG_STEPS: [TranscriptStep; 3] = [GOOD_STEP_0, GOOD_STEP_2, GOOD_STEP_1];

    assert_rejected(
        ProofTranscript {
            steps: &WRONG_STEPS,
            ..good_transcript()
        },
        VerificationError::WrongEnvSequence,
    );
}

#[test]
fn wrong_canonical_values_are_rejected() {
    assert_rejected(
        ProofTranscript {
            canonical: CanonicalTranscriptValues {
                env064_spend_txid: "bad-txid",
                ..good_transcript().canonical
            },
            ..good_transcript()
        },
        VerificationError::WrongCanonicalSpendTxid,
    );

    assert_rejected(
        ProofTranscript {
            canonical: CanonicalTranscriptValues {
                env063_input_outpoint: "bad-utxo:0",
                ..good_transcript().canonical
            },
            ..good_transcript()
        },
        VerificationError::WrongCanonicalInputOutpoint,
    );

    assert_rejected(
        ProofTranscript {
            canonical: CanonicalTranscriptValues {
                continuing_output: "bad-output:0",
                ..good_transcript().canonical
            },
            ..good_transcript()
        },
        VerificationError::WrongContinuingOutput,
    );

    assert_rejected(
        ProofTranscript {
            canonical: CanonicalTranscriptValues {
                continuing_output_value_sompi: 1,
                ..good_transcript().canonical
            },
            ..good_transcript()
        },
        VerificationError::WrongContinuingOutputValue,
    );

    assert_rejected(
        ProofTranscript {
            canonical: CanonicalTranscriptValues {
                covenant_id: "bad-covenant-id",
                ..good_transcript().canonical
            },
            ..good_transcript()
        },
        VerificationError::WrongCovenantId,
    );
}

#[test]
fn missing_fixture_path_is_rejected() {
    static MISSING_FIXTURE_STEPS: [TranscriptStep; 3] = [
        TranscriptStep {
            fixture_path: "fixtures/tn10-canonical-covenant-path/does-not-exist/",
            ..GOOD_STEP_0
        },
        GOOD_STEP_1,
        GOOD_STEP_2,
    ];

    assert_rejected(
        ProofTranscript {
            steps: &MISSING_FIXTURE_STEPS,
            ..good_transcript()
        },
        VerificationError::MissingFixturePath,
    );
}

#[test]
fn mainnet_and_unsafe_requirements_are_rejected() {
    assert_rejected(
        ProofTranscript {
            mainnet_supported: true,
            ..good_transcript()
        },
        VerificationError::MainnetSupported,
    );

    for unsafe_safety in [
        TranscriptSafetyBoundary {
            requires_no_secrets: false,
            ..good_transcript().safety
        },
        TranscriptSafetyBoundary {
            requires_no_wallet: false,
            ..good_transcript().safety
        },
        TranscriptSafetyBoundary {
            requires_no_signing: false,
            ..good_transcript().safety
        },
        TranscriptSafetyBoundary {
            requires_no_network: false,
            ..good_transcript().safety
        },
        TranscriptSafetyBoundary {
            requires_no_broadcast: false,
            ..good_transcript().safety
        },
        TranscriptSafetyBoundary {
            mainnet_supported: true,
            ..good_transcript().safety
        },
    ] {
        assert_rejected(
            ProofTranscript {
                safety: unsafe_safety,
                ..good_transcript()
            },
            VerificationError::UnsafeTranscriptBoundary,
        );
    }
}
