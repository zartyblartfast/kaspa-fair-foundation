use std::path::Path;

use kaspa_foundation::transcript::{
    canonical::canonical_tn10_proof_transcript,
    schema::{EVIDENCE_SCHEMA_VERSION, TRANSCRIPT_SCHEMA_VERSION},
    StepMode, StepPurpose,
};

#[test]
fn transcript_schema_versions_are_stable() {
    let transcript = canonical_tn10_proof_transcript();

    assert_eq!(TRANSCRIPT_SCHEMA_VERSION, "kaspa-fair-transcript-v1");
    assert_eq!(EVIDENCE_SCHEMA_VERSION, "kaspa-fair-evidence-v1");
    assert_eq!(
        transcript.transcript_schema_version,
        TRANSCRIPT_SCHEMA_VERSION
    );
    assert_eq!(transcript.evidence_schema_version, EVIDENCE_SCHEMA_VERSION);
}

#[test]
fn canonical_env_sequence_is_exactly_env063_env064_env065() {
    let transcript = canonical_tn10_proof_transcript();
    let env_ids: Vec<&str> = transcript.steps.iter().map(|step| step.env_id).collect();

    assert_eq!(env_ids, vec!["ENV-063", "ENV-064", "ENV-065"]);
    assert_eq!(transcript.steps[0].purpose, StepPurpose::CovenantCreate);
    assert_eq!(transcript.steps[1].purpose, StepPurpose::CovenantSpend);
    assert_eq!(
        transcript.steps[2].purpose,
        StepPurpose::ReadOnlyConfirmation
    );
    assert_eq!(transcript.steps[0].mode, StepMode::Live);
    assert_eq!(transcript.steps[1].mode, StepMode::Live);
    assert_eq!(transcript.steps[2].mode, StepMode::ReadOnly);
}

#[test]
fn canonical_transcript_records_proven_tn10_values() {
    let transcript = canonical_tn10_proof_transcript();

    assert_eq!(
        transcript.canonical.env064_spend_txid,
        "4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c"
    );
    assert_eq!(
        transcript.canonical.env063_input_outpoint,
        "2c7802ff9a6eec2828a96168d8f62a9a276176441ed8cb6086cd5d5d0cb26849:0"
    );
    assert_eq!(
        transcript.canonical.continuing_output,
        "4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c:0"
    );
    assert_eq!(
        transcript.canonical.continuing_output_value_sompi,
        99_700_000
    );
    assert_eq!(
        transcript.canonical.covenant_id,
        "e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7"
    );
}

#[test]
fn canonical_transcript_links_to_fixture_paths_that_exist() {
    let transcript = canonical_tn10_proof_transcript();
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

    assert_eq!(
        transcript.steps[0].fixture_path,
        "fixtures/tn10-canonical-covenant-path/env-063-corrected-live-covenant-create/"
    );
    assert_eq!(
        transcript.steps[1].fixture_path,
        "fixtures/tn10-canonical-covenant-path/env-064-live-corrected-covenant-spend/"
    );
    assert_eq!(
        transcript.steps[2].fixture_path,
        "fixtures/tn10-canonical-covenant-path/env-065-readonly-env064-spend-confirmation/"
    );

    for step in transcript.steps {
        assert!(
            repo_root.join(step.fixture_path).exists(),
            "missing fixture path for {}",
            step.env_id
        );
    }
}

#[test]
fn canonical_transcript_is_offline_readonly_evidence_only_and_safe() {
    let transcript = canonical_tn10_proof_transcript();

    assert_eq!(transcript.network, "TN10/testnet-10");
    assert!(!transcript.mainnet_supported);
    assert!(transcript.offline_verifier_first);
    assert!(transcript.online_verifier_later);
    assert!(transcript.app_agnostic_foundation_layer);
    assert!(!transcript.includes_roulette_adapter);

    assert!(!transcript.safety.requires_secrets);
    assert!(!transcript.safety.requires_wallet);
    assert!(!transcript.safety.requires_signing);
    assert!(!transcript.safety.requires_network);
    assert!(!transcript.safety.requires_broadcast);
    assert!(!transcript.safety.mainnet_supported);

    assert!(transcript.steps[0].historical_live_evidence);
    assert!(transcript.steps[1].historical_live_evidence);
    assert!(!transcript.steps[2].historical_live_evidence);
    assert!(transcript.steps[2].read_only_evidence);
}
