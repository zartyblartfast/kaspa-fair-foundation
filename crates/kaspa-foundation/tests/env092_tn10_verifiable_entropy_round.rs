use kaspa_foundation::fairness::{
    env092_build_result_derivation, env092_operator_commitment_hash,
    verify_env092_generated_artifacts, ENV092_CLAIM_LEVEL, ENV092_RULE_VERSION,
};
use serde_json::json;

fn fixture() -> (serde_json::Value, serde_json::Value) {
    let round_id = "env-092-entropy-round-0001";
    let operator_seed = "env092-operator-seed-0001";
    let algorithm = "blake3-domain-separated-rejection-sampling-v1";
    let commitment_hash =
        env092_operator_commitment_hash(round_id, operator_seed, algorithm, ENV092_RULE_VERSION);
    let no_more = json!({
        "no_more_bets_txid":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        "no_more_bets_accepting_blue_score":1000,
        "entropy_delay_blue_score":30,
        "entropy_target_blue_score":1030
    });
    let entropy = json!({
        "entropy_source_type":"tn10_block_hash_at_or_after_target_blue_score",
        "entropy_source_block_hash":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "entropy_source_blue_score":1031,
        "entropy_source_daa_score":2000,
        "entropy_readback_endpoint":"https://api-tn10.kaspa.org/blocks-from-bluescore?blueScoreGte=1030&includeTransactions=false",
        "entropy_value_used_in_transcript":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    });
    let transcript = json!({
        "round_id":round_id,
        "operator_seed":operator_seed,
        "commitment_hash":commitment_hash,
        "commitment_txid":"cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
        "no_more_bets_txid":no_more["no_more_bets_txid"],
        "reveal_txid":"dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
        "tn10_future_entropy_value":entropy["entropy_value_used_in_transcript"],
        "entropy_target_blue_score":1030,
        "entropy_source_blue_score":1031,
        "entropy_source_type":entropy["entropy_source_type"],
        "result_algorithm":algorithm,
        "rule_version":ENV092_RULE_VERSION
    });
    let (final_entropy_hash, result_number, result_colour, _) =
        env092_build_result_derivation(&transcript).unwrap();
    let sample = json!({
        "source_env":"ENV-092",
        "round_id":round_id,
        "final_result":"PASS",
        "result_algorithm":algorithm,
        "result_number":result_number,
        "result_colour":result_colour,
        "production_randomness_claimed":false
    });
    let proof = json!({
        "source_env":"ENV-092",
        "network":"testnet-10",
        "round_id":round_id,
        "verifier_result":"PASS",
        "claim_level":ENV092_CLAIM_LEVEL,
        "result_algorithm":algorithm,
        "result_number":result_number,
        "result_colour":result_colour,
        "final_entropy_hash":final_entropy_hash,
        "application_round_transcript":{
            "commitment":{"round_id":round_id,"commitment_hash":commitment_hash},
            "reveal":{"round_id":round_id,"revealed_seed_material":operator_seed}
        },
        "no_more_bets_evidence":no_more,
        "tn10_entropy_readback":entropy,
        "final_entropy_transcript":transcript,
        "safety_flags":{
            "real_betting":false,
            "real_payouts":false,
            "backend_custody":false,
            "production_randomness_claimed":false,
            "mainnet_supported":false
        }
    });
    (sample, proof)
}

#[test]
fn env092_verifies_live_tn10_entropy_transcript_shape() {
    let (sample, proof) = fixture();
    let report = verify_env092_generated_artifacts(&sample, &proof).unwrap();
    assert_eq!(report.verifier_result, "PASS");
}

#[test]
fn env092_negative_checks_reject_tampering() {
    let (sample, proof) = fixture();

    let mut changed_seed = proof.clone();
    changed_seed["final_entropy_transcript"]["operator_seed"] = json!("changed");
    assert!(verify_env092_generated_artifacts(&sample, &changed_seed).is_err());

    let mut changed_commitment = proof.clone();
    changed_commitment["application_round_transcript"]["commitment"]["commitment_hash"] =
        json!("00");
    assert!(verify_env092_generated_artifacts(&sample, &changed_commitment).is_err());

    let mut changed_entropy = proof.clone();
    changed_entropy["tn10_entropy_readback"]["entropy_value_used_in_transcript"] = json!("ff");
    assert!(verify_env092_generated_artifacts(&sample, &changed_entropy).is_err());

    let mut before_target = proof.clone();
    before_target["tn10_entropy_readback"]["entropy_source_blue_score"] = json!(1029);
    assert!(verify_env092_generated_artifacts(&sample, &before_target).is_err());

    let mut missing_no_more = proof.clone();
    missing_no_more
        .as_object_mut()
        .unwrap()
        .remove("no_more_bets_evidence");
    assert!(verify_env092_generated_artifacts(&sample, &missing_no_more).is_err());

    let mut tampered_number = proof.clone();
    tampered_number["result_number"] = json!((proof["result_number"].as_u64().unwrap() + 1) % 37);
    assert!(verify_env092_generated_artifacts(&sample, &tampered_number).is_err());

    let mut tampered_colour = proof.clone();
    tampered_colour["result_colour"] = json!("green");
    assert!(verify_env092_generated_artifacts(&sample, &tampered_colour).is_err());

    let mut production_claim = proof.clone();
    production_claim["safety_flags"]["production_randomness_claimed"] = json!(true);
    assert!(verify_env092_generated_artifacts(&sample, &production_claim).is_err());

    let mut missing_sample_claim = sample.clone();
    missing_sample_claim
        .as_object_mut()
        .unwrap()
        .remove("production_randomness_claimed");
    assert!(verify_env092_generated_artifacts(&missing_sample_claim, &proof).is_err());

    let mut sample_production_claim = sample.clone();
    sample_production_claim["production_randomness_claimed"] = json!(true);
    assert!(verify_env092_generated_artifacts(&sample_production_claim, &proof).is_err());
}
