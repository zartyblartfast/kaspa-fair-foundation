use kaspa_foundation::fairness::{
    build_env083c_demo_proof, verify_env083c_json_mirror, Env083cNegativeCase,
    ENV083C_RESULT_ALGORITHM,
};

#[test]
fn env083c_demo_proof_binds_round_transcript_to_live_tn10_anchor() {
    let proof = build_env083c_demo_proof().expect("demo proof builds from fixed transcript");
    let report = verify_env083c_json_mirror(&proof.to_json()).expect("proof verifies");

    assert_eq!(proof.network, "testnet-10");
    assert_eq!(proof.live_tn10_anchor.evidence_mode, "live_readonly_tn10");
    assert_eq!(proof.live_tn10_anchor.verifier_result, "PASS");
    assert!(proof.live_tn10_anchor.covenant_id_confirmed);
    assert_eq!(proof.commitment.round_id, proof.reveal.round_id);
    assert_eq!(
        proof.commitment.anchor_covenant_id,
        proof.live_tn10_anchor.covenant_id
    );
    assert_eq!(
        proof.reveal.anchor_covenant_id,
        proof.live_tn10_anchor.covenant_id
    );
    assert_eq!(proof.commitment.result_algorithm, ENV083C_RESULT_ALGORITHM);
    assert_eq!(proof.reveal.result_algorithm, ENV083C_RESULT_ALGORITHM);
    assert_eq!(report.verifier_result, "PASS");
    assert!(report.checks.iter().all(|check| check.passed));
}

#[test]
fn env083c_negative_cases_reject_tampering_and_claim_upgrades() {
    let proof = build_env083c_demo_proof().expect("demo proof builds");
    for case in [
        Env083cNegativeCase::TamperedReveal,
        Env083cNegativeCase::MismatchedCovenantId,
        Env083cNegativeCase::MismatchedResult,
        Env083cNegativeCase::OmittedTn10Anchor,
        Env083cNegativeCase::ApplicationOnlyClaimedAsLiveRoundTransaction,
    ] {
        let tampered = proof.tampered_json(case);
        assert!(
            verify_env083c_json_mirror(&tampered).is_err(),
            "negative case {case:?} must fail"
        );
    }
}
