use kaspa_foundation::fairness::{
    build_env084_verifiable_demo_round, verify_env084_generated_artifacts,
    ENV084_FUTURE_LIVE_ROUND_TRANSACTION_EVIDENCE,
};

#[test]
fn env084_generator_builds_matching_sample_and_proof_from_explicit_seed() {
    let (sample, proof, verifier) =
        build_env084_verifiable_demo_round("env-084-demo-round-0001", "env084-demo-seed-0001")
            .expect("ENV-084 generated artifacts build");

    assert_eq!(sample["round_id"], "env-084-demo-round-0001");
    assert_eq!(proof["round_id"], "env-084-demo-round-0001");
    assert_eq!(sample["result_number"], proof["result_number"]);
    assert_eq!(sample["result_colour"], proof["result_colour"]);
    assert_eq!(sample["result_algorithm"], proof["result_algorithm"]);
    assert_eq!(proof["evidence_mode"], "live_readonly_tn10");
    assert_eq!(
        proof["future_live_round_transaction_evidence"],
        ENV084_FUTURE_LIVE_ROUND_TRANSACTION_EVIDENCE
    );
    assert_eq!(proof["safety_flags"]["transaction_created"], false);
    assert_eq!(proof["safety_flags"]["signing_used"], false);
    assert_eq!(proof["safety_flags"]["broadcast_used"], false);
    assert_eq!(proof["safety_flags"]["wallet_access_used"], false);
    assert_eq!(verifier["verifier_result"], "PASS");
}

#[test]
fn env084_negative_checks_reject_tampering_and_false_live_claims() {
    let (sample, proof, _) =
        build_env084_verifiable_demo_round("env-084-demo-round-0001", "env084-demo-seed-0001")
            .expect("ENV-084 generated artifacts build");

    let mut tampered_number_sample = sample.clone();
    let current_number = tampered_number_sample["result_number"].as_u64().unwrap();
    tampered_number_sample["result_number"] = serde_json::json!((current_number + 1) % 37);
    assert!(verify_env084_generated_artifacts(&tampered_number_sample, &proof).is_err());

    let mut tampered_colour_sample = sample.clone();
    tampered_colour_sample["result_colour"] =
        serde_json::json!(if sample["result_colour"] == "red" {
            "black"
        } else {
            "red"
        });
    assert!(verify_env084_generated_artifacts(&tampered_colour_sample, &proof).is_err());

    let mut changed_seed_proof = proof.clone();
    changed_seed_proof["application_round_transcript"]["reveal"]["revealed_seed_material"] =
        serde_json::json!("env084-demo-seed-0001 changed without recompute");
    assert!(verify_env084_generated_artifacts(&sample, &changed_seed_proof).is_err());

    let mut omitted_anchor_proof = proof.clone();
    omitted_anchor_proof
        .as_object_mut()
        .unwrap()
        .remove("live_tn10_anchor");
    assert!(verify_env084_generated_artifacts(&sample, &omitted_anchor_proof).is_err());

    let mut mismatched_covenant_proof = proof.clone();
    mismatched_covenant_proof["application_round_transcript"]["reveal"]["anchor_covenant_id"] =
        serde_json::json!("0000000000000000000000000000000000000000000000000000000000000000");
    assert!(verify_env084_generated_artifacts(&sample, &mismatched_covenant_proof).is_err());

    let mut false_live_claim_proof = proof.clone();
    false_live_claim_proof["future_live_round_transaction_evidence"] =
        serde_json::json!("live_round_transaction_created");
    false_live_claim_proof["safety_flags"]["transaction_created"] = serde_json::json!(true);
    assert!(verify_env084_generated_artifacts(&sample, &false_live_claim_proof).is_err());
}
