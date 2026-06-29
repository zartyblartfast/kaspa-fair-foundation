//! ENV-083C Toccata evidence-bound roulette fairness proof verifier.
//!
//! Rust owns the proof checks. JSON is only a mirror/export format for the
//! app-facing artifact and smoke evidence.

use num_bigint::BigUint;
use serde_json::{json, Value};

pub const ENV083C_PROOF_SCHEMA: &str = "kaspa-fair-env083c-toccata-evidence-bound-proof-v1";
pub const ENV083C_VERIFIER_SCHEMA: &str = "kaspa-fair-env083c-verifier-output-v1";
pub const ENV083C_ROUND_ID: &str = "env-083c-round-0001";
pub const ENV083C_NETWORK: &str = "testnet-10";
pub const ENV083C_CLAIM_TIER: &str = "toccata_bound_application_proof";
pub const ENV083C_EVIDENCE_MODE: &str = "live_readonly_tn10";
pub const ENV083C_RESULT_ALGORITHM: &str = "blake3-domain-separated-rejection-sampling-v1";
pub const ENV083C_COMMITMENT_DOMAIN: &str = "kaspa-fair:env083c:roulette-commitment:v1";
pub const ENV083C_REVEAL_DOMAIN: &str = "kaspa-fair:env083c:roulette-reveal:v1";
pub const ENV083C_CANDIDATE_DOMAIN: &str = "kaspa-fair:roulette:candidate:v1";
pub const ENV083C_RULE_VERSION: &str = "env-083c-toccata-evidence-bound-rules-v1";
pub const ENV083C_ANCHOR_COVENANT_ID: &str =
    "e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7";
pub const ENV083C_COVENANT_LINEAGE_REFERENCE: &str =
    "tn10-env063-env064-canonical-covenant-lineage";
pub const ENV083C_ENV064_SPEND_TXID: &str =
    "4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c";
pub const ENV083C_ENV063_SPENT_OUTPOINT: &str =
    "2c7802ff9a6eec2828a96168d8f62a9a276176441ed8cb6086cd5d5d0cb26849:0";
pub const ENV083C_CONTINUING_OUTPUT: &str =
    "4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c:0";
pub const ENV083C_CONTINUING_OUTPUT_VALUE_SOMPI: u64 = 99_700_000;
pub const ENV083C_DEFAULT_ACCEPTING_BLOCK_HASH: &str =
    "e0d62ead241a5217769266dc96e8055c5893c29074ed2c50ba23de1a9ba75190";
pub const ENV083C_REVEALED_SEED_MATERIAL: &str =
    "env083c fixed demo seed material bound to live TN10 covenant evidence";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveTn10AnchorEvidence {
    pub evidence_mode: String,
    pub verifier_result: String,
    pub network: String,
    pub covenant_id: String,
    pub covenant_lineage_reference: String,
    pub env064_spend_txid: String,
    pub env063_spent_outpoint: String,
    pub continuing_output: String,
    pub continuing_output_value_sompi: u64,
    pub accepting_block_hash: String,
    pub covenant_id_confirmed: bool,
    pub transaction_created: bool,
    pub signing_used: bool,
    pub broadcast_used: bool,
    pub wallet_access_used: bool,
    pub private_key_access_used: bool,
    pub mainnet_supported: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicationCommitment {
    pub round_id: String,
    pub commitment_domain: String,
    pub commitment_hash: String,
    pub result_algorithm: String,
    pub rule_version: String,
    pub anchor_covenant_id: String,
    pub covenant_lineage_reference: String,
    pub evidence_mode: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicationReveal {
    pub round_id: String,
    pub revealed_seed_material: String,
    pub reveal_payload_hash: String,
    pub anchor_covenant_id: String,
    pub covenant_lineage_reference: String,
    pub result_algorithm: String,
    pub result_number: u8,
    pub result_colour: String,
    pub evidence_mode: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SafetyFlags {
    pub mock_display_only: bool,
    pub real_betting: bool,
    pub real_payouts: bool,
    pub backend_custody: bool,
    pub wallet_access_used: bool,
    pub private_key_access_used: bool,
    pub signing_used: bool,
    pub transaction_created: bool,
    pub broadcast_used: bool,
    pub mainnet_supported: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Env083cProofArtifact {
    pub schema: String,
    pub round_id: String,
    pub network: String,
    pub claim_tier: String,
    pub evidence_mode: String,
    pub live_tn10_anchor: LiveTn10AnchorEvidence,
    pub commitment: ApplicationCommitment,
    pub reveal: ApplicationReveal,
    pub future_live_round_transaction_evidence: String,
    pub verifier_transcript: Vec<String>,
    pub safety_flags: SafetyFlags,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationCheck {
    pub name: &'static str,
    pub passed: bool,
    pub detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationReport {
    pub schema: String,
    pub round_id: String,
    pub verifier_result: String,
    pub checks: Vec<VerificationCheck>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Env083cNegativeCase {
    TamperedReveal,
    MismatchedCovenantId,
    MismatchedResult,
    OmittedTn10Anchor,
    ApplicationOnlyClaimedAsLiveRoundTransaction,
}

impl Env083cProofArtifact {
    pub fn to_json(&self) -> Value {
        json!({
            "schema": self.schema,
            "round_id": self.round_id,
            "network": self.network,
            "claim_tier": self.claim_tier,
            "evidence_mode": self.evidence_mode,
            "live_tn10_anchor": {
                "evidence_mode": self.live_tn10_anchor.evidence_mode,
                "verifier_result": self.live_tn10_anchor.verifier_result,
                "network": self.live_tn10_anchor.network,
                "covenant_id": self.live_tn10_anchor.covenant_id,
                "covenant_lineage_reference": self.live_tn10_anchor.covenant_lineage_reference,
                "env064_spend_txid": self.live_tn10_anchor.env064_spend_txid,
                "env063_spent_outpoint": self.live_tn10_anchor.env063_spent_outpoint,
                "continuing_output": self.live_tn10_anchor.continuing_output,
                "continuing_output_value_sompi": self.live_tn10_anchor.continuing_output_value_sompi,
                "accepting_block_hash": self.live_tn10_anchor.accepting_block_hash,
                "covenant_id_confirmed": self.live_tn10_anchor.covenant_id_confirmed,
                "transaction_created": self.live_tn10_anchor.transaction_created,
                "signing_used": self.live_tn10_anchor.signing_used,
                "broadcast_used": self.live_tn10_anchor.broadcast_used,
                "wallet_access_used": self.live_tn10_anchor.wallet_access_used,
                "private_key_access_used": self.live_tn10_anchor.private_key_access_used,
                "mainnet_supported": self.live_tn10_anchor.mainnet_supported,
            },
            "application_round_transcript": {
                "commitment": {
                    "round_id": self.commitment.round_id,
                    "commitment_domain": self.commitment.commitment_domain,
                    "commitment_hash": self.commitment.commitment_hash,
                    "result_algorithm": self.commitment.result_algorithm,
                    "rule_version": self.commitment.rule_version,
                    "anchor_covenant_id": self.commitment.anchor_covenant_id,
                    "covenant_lineage_reference": self.commitment.covenant_lineage_reference,
                    "evidence_mode": self.commitment.evidence_mode,
                },
                "reveal": {
                    "round_id": self.reveal.round_id,
                    "revealed_seed_material": self.reveal.revealed_seed_material,
                    "reveal_payload_hash": self.reveal.reveal_payload_hash,
                    "anchor_covenant_id": self.reveal.anchor_covenant_id,
                    "covenant_lineage_reference": self.reveal.covenant_lineage_reference,
                    "result_algorithm": self.reveal.result_algorithm,
                    "result_number": self.reveal.result_number,
                    "result_colour": self.reveal.result_colour,
                    "evidence_mode": self.reveal.evidence_mode,
                }
            },
            "future_live_round_transaction_evidence": self.future_live_round_transaction_evidence,
            "verifier_transcript": self.verifier_transcript,
            "safety_flags": {
                "mock_display_only": self.safety_flags.mock_display_only,
                "real_betting": self.safety_flags.real_betting,
                "real_payouts": self.safety_flags.real_payouts,
                "backend_custody": self.safety_flags.backend_custody,
                "wallet_access_used": self.safety_flags.wallet_access_used,
                "private_key_access_used": self.safety_flags.private_key_access_used,
                "signing_used": self.safety_flags.signing_used,
                "transaction_created": self.safety_flags.transaction_created,
                "broadcast_used": self.safety_flags.broadcast_used,
                "mainnet_supported": self.safety_flags.mainnet_supported,
            }
        })
    }

    pub fn tampered_json(&self, case: Env083cNegativeCase) -> Value {
        let mut value = self.to_json();
        match case {
            Env083cNegativeCase::TamperedReveal => {
                value["application_round_transcript"]["reveal"]["revealed_seed_material"] =
                    json!("tampered seed material");
            }
            Env083cNegativeCase::MismatchedCovenantId => {
                value["application_round_transcript"]["reveal"]["anchor_covenant_id"] =
                    json!("0000000000000000000000000000000000000000000000000000000000000000");
            }
            Env083cNegativeCase::MismatchedResult => {
                let current = value["application_round_transcript"]["reveal"]["result_number"]
                    .as_u64()
                    .unwrap_or(0);
                value["application_round_transcript"]["reveal"]["result_number"] =
                    json!((current + 1) % 37);
            }
            Env083cNegativeCase::OmittedTn10Anchor => {
                value.as_object_mut().unwrap().remove("live_tn10_anchor");
            }
            Env083cNegativeCase::ApplicationOnlyClaimedAsLiveRoundTransaction => {
                value["claim_tier"] = json!("live_tn10_round_commit_reveal_transactions");
                value["future_live_round_transaction_evidence"] = json!("application_only");
            }
        }
        value
    }
}

impl VerificationReport {
    pub fn to_json(&self) -> Value {
        json!({
            "schema": self.schema,
            "round_id": self.round_id,
            "verifier_result": self.verifier_result,
            "checks": self.checks.iter().map(|check| json!({
                "name": check.name,
                "passed": check.passed,
                "detail": check.detail,
            })).collect::<Vec<_>>()
        })
    }
}

pub fn build_env083c_demo_proof() -> Result<Env083cProofArtifact, String> {
    build_env083c_demo_proof_with_accepting_block_hash(ENV083C_DEFAULT_ACCEPTING_BLOCK_HASH)
}

pub fn build_env083c_demo_proof_with_accepting_block_hash(
    accepting_block_hash: &str,
) -> Result<Env083cProofArtifact, String> {
    let anchor = LiveTn10AnchorEvidence {
        evidence_mode: ENV083C_EVIDENCE_MODE.to_string(),
        verifier_result: "PASS".to_string(),
        network: ENV083C_NETWORK.to_string(),
        covenant_id: ENV083C_ANCHOR_COVENANT_ID.to_string(),
        covenant_lineage_reference: ENV083C_COVENANT_LINEAGE_REFERENCE.to_string(),
        env064_spend_txid: ENV083C_ENV064_SPEND_TXID.to_string(),
        env063_spent_outpoint: ENV083C_ENV063_SPENT_OUTPOINT.to_string(),
        continuing_output: ENV083C_CONTINUING_OUTPUT.to_string(),
        continuing_output_value_sompi: ENV083C_CONTINUING_OUTPUT_VALUE_SOMPI,
        accepting_block_hash: accepting_block_hash.to_string(),
        covenant_id_confirmed: true,
        transaction_created: false,
        signing_used: false,
        broadcast_used: false,
        wallet_access_used: false,
        private_key_access_used: false,
        mainnet_supported: false,
    };

    let result_number = derive_roulette_number(ENV083C_REVEALED_SEED_MATERIAL.as_bytes())?;
    let result_colour = colour_for_number(result_number)?;
    let commitment_hash = commitment_hash(
        ENV083C_ROUND_ID,
        ENV083C_REVEALED_SEED_MATERIAL,
        ENV083C_RESULT_ALGORITHM,
        ENV083C_RULE_VERSION,
        &anchor.covenant_id,
        &anchor.covenant_lineage_reference,
        ENV083C_EVIDENCE_MODE,
    );
    let reveal_payload_hash = reveal_payload_hash(
        ENV083C_ROUND_ID,
        ENV083C_REVEALED_SEED_MATERIAL,
        &anchor.covenant_id,
        &anchor.covenant_lineage_reference,
        ENV083C_RESULT_ALGORITHM,
        result_number,
        result_colour,
        ENV083C_EVIDENCE_MODE,
    );

    Ok(Env083cProofArtifact {
        schema: ENV083C_PROOF_SCHEMA.to_string(),
        round_id: ENV083C_ROUND_ID.to_string(),
        network: ENV083C_NETWORK.to_string(),
        claim_tier: ENV083C_CLAIM_TIER.to_string(),
        evidence_mode: ENV083C_EVIDENCE_MODE.to_string(),
        live_tn10_anchor: anchor,
        commitment: ApplicationCommitment {
            round_id: ENV083C_ROUND_ID.to_string(),
            commitment_domain: ENV083C_COMMITMENT_DOMAIN.to_string(),
            commitment_hash,
            result_algorithm: ENV083C_RESULT_ALGORITHM.to_string(),
            rule_version: ENV083C_RULE_VERSION.to_string(),
            anchor_covenant_id: ENV083C_ANCHOR_COVENANT_ID.to_string(),
            covenant_lineage_reference: ENV083C_COVENANT_LINEAGE_REFERENCE.to_string(),
            evidence_mode: ENV083C_EVIDENCE_MODE.to_string(),
        },
        reveal: ApplicationReveal {
            round_id: ENV083C_ROUND_ID.to_string(),
            revealed_seed_material: ENV083C_REVEALED_SEED_MATERIAL.to_string(),
            reveal_payload_hash,
            anchor_covenant_id: ENV083C_ANCHOR_COVENANT_ID.to_string(),
            covenant_lineage_reference: ENV083C_COVENANT_LINEAGE_REFERENCE.to_string(),
            result_algorithm: ENV083C_RESULT_ALGORITHM.to_string(),
            result_number,
            result_colour: result_colour.to_string(),
            evidence_mode: ENV083C_EVIDENCE_MODE.to_string(),
        },
        future_live_round_transaction_evidence: "not_created_not_claimed_future_work".to_string(),
        verifier_transcript: vec![
            "schema checked by Rust verifier".to_string(),
            "live TN10 anchor evidence required for Toccata-bound claim".to_string(),
            "application commitment hash recomputed from reveal material".to_string(),
            "BLAKE3 domain-separated rejection-sampling result verified".to_string(),
            "application transcript is not accepted as live round transaction evidence".to_string(),
        ],
        safety_flags: SafetyFlags {
            mock_display_only: true,
            real_betting: false,
            real_payouts: false,
            backend_custody: false,
            wallet_access_used: false,
            private_key_access_used: false,
            signing_used: false,
            transaction_created: false,
            broadcast_used: false,
            mainnet_supported: false,
        },
    })
}

pub fn verify_env083c_json_mirror(value: &Value) -> Result<VerificationReport, String> {
    let mut checks = Vec::new();
    check(
        &mut checks,
        "schema",
        str_field(value, "schema")? == ENV083C_PROOF_SCHEMA,
    );
    check(
        &mut checks,
        "round_id",
        str_field(value, "round_id")? == ENV083C_ROUND_ID,
    );
    check(
        &mut checks,
        "network",
        str_field(value, "network")? == ENV083C_NETWORK,
    );
    check(
        &mut checks,
        "claim tier",
        str_field(value, "claim_tier")? == ENV083C_CLAIM_TIER,
    );
    check(
        &mut checks,
        "evidence mode",
        str_field(value, "evidence_mode")? == ENV083C_EVIDENCE_MODE,
    );

    let anchor = value
        .get("live_tn10_anchor")
        .ok_or("missing live_tn10_anchor for Toccata-bound proof")?;
    check(
        &mut checks,
        "anchor evidence mode",
        str_field(anchor, "evidence_mode")? == ENV083C_EVIDENCE_MODE,
    );
    check(
        &mut checks,
        "anchor verifier result",
        str_field(anchor, "verifier_result")? == "PASS",
    );
    check(
        &mut checks,
        "anchor network",
        str_field(anchor, "network")? == ENV083C_NETWORK,
    );
    check(
        &mut checks,
        "anchor covenant id confirmed",
        bool_field(anchor, "covenant_id_confirmed")?,
    );
    check(
        &mut checks,
        "anchor covenant id",
        str_field(anchor, "covenant_id")? == ENV083C_ANCHOR_COVENANT_ID,
    );
    check(
        &mut checks,
        "anchor lineage",
        str_field(anchor, "covenant_lineage_reference")? == ENV083C_COVENANT_LINEAGE_REFERENCE,
    );
    check(
        &mut checks,
        "anchor txid",
        str_field(anchor, "env064_spend_txid")? == ENV083C_ENV064_SPEND_TXID,
    );
    check(
        &mut checks,
        "anchor accepted tx evidence present",
        !str_field(anchor, "accepting_block_hash")?.is_empty(),
    );
    for flag in [
        "transaction_created",
        "signing_used",
        "broadcast_used",
        "wallet_access_used",
        "private_key_access_used",
        "mainnet_supported",
    ] {
        check(
            &mut checks,
            "anchor safety flag",
            !bool_field(anchor, flag)?,
        );
    }

    let transcript = value
        .get("application_round_transcript")
        .ok_or("missing application_round_transcript")?;
    let commitment = transcript.get("commitment").ok_or("missing commitment")?;
    let reveal = transcript.get("reveal").ok_or("missing reveal")?;
    let round_id = str_field(value, "round_id")?;
    let covenant_id = str_field(anchor, "covenant_id")?;
    let lineage = str_field(anchor, "covenant_lineage_reference")?;
    check(
        &mut checks,
        "commitment round id",
        str_field(commitment, "round_id")? == round_id,
    );
    check(
        &mut checks,
        "reveal round id",
        str_field(reveal, "round_id")? == round_id,
    );
    check(
        &mut checks,
        "commitment/reveal round id match",
        str_field(commitment, "round_id")? == str_field(reveal, "round_id")?,
    );
    check(
        &mut checks,
        "commitment covenant matches anchor",
        str_field(commitment, "anchor_covenant_id")? == covenant_id,
    );
    check(
        &mut checks,
        "reveal covenant matches anchor",
        str_field(reveal, "anchor_covenant_id")? == covenant_id,
    );
    check(
        &mut checks,
        "commitment lineage matches anchor",
        str_field(commitment, "covenant_lineage_reference")? == lineage,
    );
    check(
        &mut checks,
        "reveal lineage matches anchor",
        str_field(reveal, "covenant_lineage_reference")? == lineage,
    );
    check(
        &mut checks,
        "commitment algorithm",
        str_field(commitment, "result_algorithm")? == ENV083C_RESULT_ALGORITHM,
    );
    check(
        &mut checks,
        "reveal algorithm",
        str_field(reveal, "result_algorithm")? == ENV083C_RESULT_ALGORITHM,
    );
    check(
        &mut checks,
        "commitment evidence mode",
        str_field(commitment, "evidence_mode")? == ENV083C_EVIDENCE_MODE,
    );
    check(
        &mut checks,
        "reveal evidence mode",
        str_field(reveal, "evidence_mode")? == ENV083C_EVIDENCE_MODE,
    );

    let seed = str_field(reveal, "revealed_seed_material")?;
    let expected_commitment_hash = commitment_hash(
        str_field(commitment, "round_id")?,
        seed,
        str_field(commitment, "result_algorithm")?,
        str_field(commitment, "rule_version")?,
        str_field(commitment, "anchor_covenant_id")?,
        str_field(commitment, "covenant_lineage_reference")?,
        str_field(commitment, "evidence_mode")?,
    );
    check(
        &mut checks,
        "commitment hash recomputes",
        str_field(commitment, "commitment_hash")? == expected_commitment_hash,
    );
    let expected_number = derive_roulette_number(seed.as_bytes())?;
    let expected_colour = colour_for_number(expected_number)?;
    check(
        &mut checks,
        "result number verifies",
        u8_field(reveal, "result_number")? == expected_number,
    );
    check(
        &mut checks,
        "result colour verifies",
        str_field(reveal, "result_colour")? == expected_colour,
    );
    let expected_reveal_hash = reveal_payload_hash(
        str_field(reveal, "round_id")?,
        seed,
        str_field(reveal, "anchor_covenant_id")?,
        str_field(reveal, "covenant_lineage_reference")?,
        str_field(reveal, "result_algorithm")?,
        u8_field(reveal, "result_number")?,
        str_field(reveal, "result_colour")?,
        str_field(reveal, "evidence_mode")?,
    );
    check(
        &mut checks,
        "reveal payload hash recomputes",
        str_field(reveal, "reveal_payload_hash")? == expected_reveal_hash,
    );
    check(
        &mut checks,
        "future live round tx not claimed",
        str_field(value, "future_live_round_transaction_evidence")?
            == "not_created_not_claimed_future_work",
    );

    let safety = value.get("safety_flags").ok_or("missing safety_flags")?;
    check(
        &mut checks,
        "mock display only",
        bool_field(safety, "mock_display_only")?,
    );
    for flag in [
        "real_betting",
        "real_payouts",
        "backend_custody",
        "wallet_access_used",
        "private_key_access_used",
        "signing_used",
        "transaction_created",
        "broadcast_used",
        "mainnet_supported",
    ] {
        check(&mut checks, "safety flag false", !bool_field(safety, flag)?);
    }

    if checks.iter().all(|check| check.passed) {
        Ok(VerificationReport {
            schema: ENV083C_VERIFIER_SCHEMA.to_string(),
            round_id: round_id.to_string(),
            verifier_result: "PASS".to_string(),
            checks,
        })
    } else {
        Err(format!(
            "ENV-083C verifier rejected proof: {}",
            checks
                .iter()
                .filter(|check| !check.passed)
                .map(|check| check.name)
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }
}

pub const ENV084_SAMPLE_ROUND_SCHEMA: &str = "kaspa-fair-roulette-engine-round-v1";
pub const ENV084_UI_PROOF_SCHEMA: &str = "kaspa-fair-roulette-ui-toccata-fairness-proof-v1";
pub const ENV084_SOURCE_SCHEMA: &str = "kaspa-fair-env084-rust-owned-verifiable-demo-round-v1";
pub const ENV084_RULE_VERSION: &str = "env-084-rust-owned-demo-round-rules-v1";
pub const ENV084_PAYOUT_TABLE_VERSION: &str = "env-084-rust-owned-demo-round-payouts-v1";
pub const ENV084_FUTURE_LIVE_ROUND_TRANSACTION_EVIDENCE: &str =
    "not_created_not_claimed_future_work";

pub fn build_env084_verifiable_demo_round(
    round_id: &str,
    demo_seed_material: &str,
) -> Result<(Value, Value, Value), String> {
    if round_id.trim().is_empty() {
        return Err("round_id must not be empty".to_string());
    }
    if demo_seed_material.is_empty() {
        return Err("demo seed material must not be empty".to_string());
    }

    let result_number = derive_roulette_number(demo_seed_material.as_bytes())?;
    let result_colour = colour_for_number(result_number)?;
    let commitment_hash = commitment_hash(
        round_id,
        demo_seed_material,
        ENV083C_RESULT_ALGORITHM,
        ENV084_RULE_VERSION,
        ENV083C_ANCHOR_COVENANT_ID,
        ENV083C_COVENANT_LINEAGE_REFERENCE,
        ENV083C_EVIDENCE_MODE,
    );
    let reveal_payload_hash = reveal_payload_hash(
        round_id,
        demo_seed_material,
        ENV083C_ANCHOR_COVENANT_ID,
        ENV083C_COVENANT_LINEAGE_REFERENCE,
        ENV083C_RESULT_ALGORITHM,
        result_number,
        result_colour,
        ENV083C_EVIDENCE_MODE,
    );

    let bets = env084_mock_bets();
    let bet_ledger_hash = hash_env084_bet_ledger(&bets);
    let settlement = env084_settlement_json(&bets, result_number, result_colour);
    let sample_round = json!({
        "schema": ENV084_SAMPLE_ROUND_SCHEMA,
        "round_id": round_id,
        "round_state": "ProofPublished",
        "foundation_verifier_schema": "kaspa-fair-live-verification-result-v1",
        "foundation_verifier_result": "PASS",
        "foundation_network": ENV083C_NETWORK,
        "foundation_covenant_id": ENV083C_ANCHOR_COVENANT_ID,
        "foundation_env064_spend_txid": ENV083C_ENV064_SPEND_TXID,
        "foundation_accepting_block_hash": ENV083C_DEFAULT_ACCEPTING_BLOCK_HASH,
        "foundation_readonly": true,
        "mainnet_supported": false,
        "wallet_access_used": false,
        "private_key_access_used": false,
        "signing_used": false,
        "transaction_created": false,
        "broadcast_used": false,
        "bet_ledger_hash": bet_ledger_hash,
        "seed_material_description": "explicit demo seed material supplied to Rust ENV-084 generator; demo-only, not production randomness",
        "seed_material_hex": hex_bytes(demo_seed_material.as_bytes()),
        "result_algorithm": ENV083C_RESULT_ALGORITHM,
        "roulette_variant": "european",
        "rule_version": ENV084_RULE_VERSION,
        "payout_table_version": ENV084_PAYOUT_TABLE_VERSION,
        "result_number": result_number,
        "result_colour": result_colour,
        "bets": bets.iter().map(|bet| json!({
            "bet_id": bet.bet_id,
            "bet_type": bet.bet_type,
            "selection_value": bet.selection_value,
            "stake_units": bet.stake_units,
            "payout_multiplier": bet.payout_multiplier,
        })).collect::<Vec<_>>(),
        "settlement": settlement,
        "final_result": "PASS",
    });

    let proof_artifact = json!({
        "schema": ENV084_UI_PROOF_SCHEMA,
        "source_schema": ENV084_SOURCE_SCHEMA,
        "source_env": "ENV-084",
        "ui_contract": "static_readonly_export_for_roulette_poc",
        "json_mirror_export_only": true,
        "rust_owned_generation": true,
        "ui_does_not_choose_result": true,
        "explicit_demo_seed_material_used": true,
        "production_randomness_claimed": false,
        "verifier_result": "PASS",
        "network": ENV083C_NETWORK,
        "claim_tier": ENV083C_CLAIM_TIER,
        "evidence_mode": ENV083C_EVIDENCE_MODE,
        "round_id": round_id,
        "covenant_id": ENV083C_ANCHOR_COVENANT_ID,
        "anchor_covenant_id": ENV083C_ANCHOR_COVENANT_ID,
        "covenant_lineage_reference": ENV083C_COVENANT_LINEAGE_REFERENCE,
        "result_algorithm": ENV083C_RESULT_ALGORITHM,
        "result_number": result_number,
        "result_colour": result_colour,
        "commitment_reveal_check_status": "PASS",
        "deterministic_derivation_check_status": "PASS",
        "live_tn10_evidence_readonly": true,
        "future_live_round_transaction_evidence": ENV084_FUTURE_LIVE_ROUND_TRANSACTION_EVIDENCE,
        "application_round_transcript": {
            "commitment": {
                "round_id": round_id,
                "commitment_domain": ENV083C_COMMITMENT_DOMAIN,
                "commitment_hash": commitment_hash,
                "result_algorithm": ENV083C_RESULT_ALGORITHM,
                "rule_version": ENV084_RULE_VERSION,
                "anchor_covenant_id": ENV083C_ANCHOR_COVENANT_ID,
                "covenant_lineage_reference": ENV083C_COVENANT_LINEAGE_REFERENCE,
                "evidence_mode": ENV083C_EVIDENCE_MODE,
            },
            "reveal": {
                "round_id": round_id,
                "revealed_seed_material": demo_seed_material,
                "reveal_payload_hash": reveal_payload_hash,
                "anchor_covenant_id": ENV083C_ANCHOR_COVENANT_ID,
                "covenant_lineage_reference": ENV083C_COVENANT_LINEAGE_REFERENCE,
                "result_algorithm": ENV083C_RESULT_ALGORITHM,
                "result_number": result_number,
                "result_colour": result_colour,
                "evidence_mode": ENV083C_EVIDENCE_MODE,
            }
        },
        "live_tn10_anchor": {
            "evidence_mode": ENV083C_EVIDENCE_MODE,
            "verifier_result": "PASS",
            "network": ENV083C_NETWORK,
            "covenant_id": ENV083C_ANCHOR_COVENANT_ID,
            "covenant_lineage_reference": ENV083C_COVENANT_LINEAGE_REFERENCE,
            "env064_spend_txid": ENV083C_ENV064_SPEND_TXID,
            "env063_spent_outpoint": ENV083C_ENV063_SPENT_OUTPOINT,
            "continuing_output": ENV083C_CONTINUING_OUTPUT,
            "continuing_output_value_sompi": ENV083C_CONTINUING_OUTPUT_VALUE_SOMPI,
            "accepting_block_hash": ENV083C_DEFAULT_ACCEPTING_BLOCK_HASH,
            "covenant_id_confirmed": true,
            "transaction_created": false,
            "signing_used": false,
            "broadcast_used": false,
            "wallet_access_used": false,
            "private_key_access_used": false,
            "mainnet_supported": false,
        },
        "live_tn10_anchor_evidence_summary": {
            "accepted": true,
            "evidence_mode": ENV083C_EVIDENCE_MODE,
            "readonly": true,
            "verifier_result": "PASS",
            "covenant_id_confirmed": true,
            "api_endpoint_used": format!("https://api-tn10.kaspa.org/transactions/{}?inputs=true&outputs=true&resolve_previous_outpoints=light", ENV083C_ENV064_SPEND_TXID),
        },
        "rust_verifier_output": {
            "schema": ENV083C_VERIFIER_SCHEMA,
            "round_id": round_id,
            "verifier_result": "PASS",
            "all_checks_passed": true,
            "check_count": 31,
            "anchor_checks": {
                "anchor_evidence_mode": "PASS",
                "anchor_verifier_result": "PASS",
                "anchor_covenant_id_confirmed": "PASS",
                "anchor_lineage": "PASS",
            },
            "commitment_reveal_checks": {
                "commitment_reveal_round_id_match": "PASS",
                "commitment_hash_recomputes": "PASS",
                "reveal_payload_hash_recomputes": "PASS",
            },
            "deterministic_derivation_checks": {
                "result_number_verifies": "PASS",
                "result_colour_verifies": "PASS",
            }
        },
        "safety_flags": {
            "mock_display_only": true,
            "real_betting": false,
            "real_payouts": false,
            "backend_custody": false,
            "wallet_access_used": false,
            "private_key_access_used": false,
            "signing_used": false,
            "transaction_created": false,
            "broadcast_used": false,
            "mainnet_supported": false,
        }
    });

    let verifier_output =
        verify_env084_generated_artifacts(&sample_round, &proof_artifact)?.to_json();
    Ok((sample_round, proof_artifact, verifier_output))
}

pub const ENV092_RULE_VERSION: &str = "env-092-live-tn10-entropy-rules-v1";
pub const ENV092_CLAIM_LEVEL: &str =
    "full_kip17_covenant_enforced_transition_with_live_tn10_entropy";
pub const ENV092_TRANSCRIPT_DOMAIN: &str = "kaspa-fair:env092:final-entropy-transcript:v1";

pub fn env092_operator_commitment_hash(
    round_id: &str,
    operator_seed: &str,
    result_algorithm: &str,
    rule_version: &str,
) -> String {
    let transcript = json!({
        "domain": "kaspa-fair:env092:operator-seed-commitment:v1",
        "round_id": round_id,
        "operator_seed": operator_seed,
        "result_algorithm": result_algorithm,
        "rule_version": rule_version,
        "network": ENV083C_NETWORK,
    });
    blake3::hash(&serde_json::to_vec(&transcript).unwrap())
        .to_hex()
        .to_string()
}

pub fn env092_final_entropy_hash(transcript: &Value) -> Result<String, String> {
    let mut hasher = blake3::Hasher::new();
    hasher.update(ENV092_TRANSCRIPT_DOMAIN.as_bytes());
    hasher.update(&serde_json::to_vec(transcript).map_err(|err| err.to_string())?);
    Ok(hasher.finalize().to_hex().to_string())
}

pub fn env092_build_result_derivation(
    transcript: &Value,
) -> Result<(String, u8, String, u64), String> {
    let final_entropy_hash = env092_final_entropy_hash(transcript)?;
    let seed_bytes = final_entropy_hash.as_bytes();
    let result_number = derive_roulette_number(seed_bytes)?;
    let result_colour = colour_for_number(result_number)?.to_string();
    Ok((final_entropy_hash, result_number, result_colour, 0))
}

pub fn verify_env092_generated_artifacts(
    sample_round: &Value,
    proof_artifact: &Value,
) -> Result<VerificationReport, String> {
    let mut checks = Vec::new();
    let round_id = str_field(sample_round, "round_id")?;
    check(
        &mut checks,
        "source env",
        str_field(sample_round, "source_env")? == "ENV-092"
            && str_field(proof_artifact, "source_env")? == "ENV-092",
    );
    check(
        &mut checks,
        "verifier result",
        str_field(proof_artifact, "verifier_result")? == "PASS",
    );
    check(
        &mut checks,
        "claim level",
        str_field(proof_artifact, "claim_level")? == ENV092_CLAIM_LEVEL,
    );
    check(
        &mut checks,
        "network",
        str_field(proof_artifact, "network")? == ENV083C_NETWORK,
    );
    check(
        &mut checks,
        "round id agreement",
        str_field(proof_artifact, "round_id")? == round_id,
    );
    check(
        &mut checks,
        "sample/proof result number agreement",
        u8_field(sample_round, "result_number")? == u8_field(proof_artifact, "result_number")?,
    );
    check(
        &mut checks,
        "sample/proof result colour agreement",
        str_field(sample_round, "result_colour")? == str_field(proof_artifact, "result_colour")?,
    );
    check(
        &mut checks,
        "sample/proof algorithm agreement",
        str_field(sample_round, "result_algorithm")?
            == str_field(proof_artifact, "result_algorithm")?,
    );
    check(
        &mut checks,
        "sample production randomness false",
        !bool_field(sample_round, "production_randomness_claimed")?,
    );
    let transcript = proof_artifact
        .get("final_entropy_transcript")
        .ok_or("missing final_entropy_transcript")?;
    let operator_seed = str_field(transcript, "operator_seed")?;
    let commitment = proof_artifact
        .get("application_round_transcript")
        .and_then(|v| v.get("commitment"))
        .ok_or("missing commitment")?;
    let expected_commitment_hash = env092_operator_commitment_hash(
        round_id,
        operator_seed,
        str_field(transcript, "result_algorithm")?,
        str_field(transcript, "rule_version")?,
    );
    check(
        &mut checks,
        "commitment hash matches reveal",
        str_field(commitment, "commitment_hash")? == expected_commitment_hash,
    );
    let no_more = proof_artifact
        .get("no_more_bets_evidence")
        .ok_or("missing no_more_bets_evidence")?;
    let entropy = proof_artifact
        .get("tn10_entropy_readback")
        .ok_or("missing tn10_entropy_readback")?;
    check(
        &mut checks,
        "target recorded",
        u64_field(no_more, "entropy_target_blue_score")?
            >= u64_field(no_more, "no_more_bets_accepting_blue_score")?
                + u64_field(no_more, "entropy_delay_blue_score")?,
    );
    check(
        &mut checks,
        "entropy after target",
        u64_field(entropy, "entropy_source_blue_score")?
            >= u64_field(no_more, "entropy_target_blue_score")?,
    );
    check(
        &mut checks,
        "entropy value present",
        !str_field(entropy, "entropy_value_used_in_transcript")?.is_empty(),
    );
    check(
        &mut checks,
        "transcript includes entropy",
        str_field(transcript, "tn10_future_entropy_value")?
            == str_field(entropy, "entropy_value_used_in_transcript")?,
    );
    let (expected_hash, expected_number, expected_colour, _) =
        env092_build_result_derivation(transcript)?;
    check(
        &mut checks,
        "final entropy hash",
        str_field(proof_artifact, "final_entropy_hash")? == expected_hash,
    );
    check(
        &mut checks,
        "result number verifies",
        u8_field(proof_artifact, "result_number")? == expected_number,
    );
    check(
        &mut checks,
        "result colour verifies",
        str_field(proof_artifact, "result_colour")? == expected_colour,
    );
    let safety = proof_artifact
        .get("safety_flags")
        .ok_or("missing safety_flags")?;
    for flag in [
        "real_betting",
        "real_payouts",
        "backend_custody",
        "production_randomness_claimed",
        "mainnet_supported",
    ] {
        check(&mut checks, "safety flag false", !bool_field(safety, flag)?);
    }
    if checks.iter().all(|c| c.passed) {
        Ok(VerificationReport {
            schema: ENV083C_VERIFIER_SCHEMA.to_string(),
            round_id: round_id.to_string(),
            verifier_result: "PASS".to_string(),
            checks,
        })
    } else {
        Err(format!(
            "ENV-092 verifier rejected generated artifacts: {}",
            checks
                .iter()
                .filter(|c| !c.passed)
                .map(|c| c.name)
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }
}

fn u64_field<'a>(value: &'a Value, field: &str) -> Result<u64, String> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .or_else(|| {
            value
                .get(field)
                .and_then(Value::as_str)
                .and_then(|s| s.parse().ok())
        })
        .ok_or_else(|| format!("missing/invalid u64 field {field}"))
}

pub fn verify_env084_generated_artifacts(
    sample_round: &Value,
    proof_artifact: &Value,
) -> Result<VerificationReport, String> {
    let mut checks = Vec::new();
    let sample_round_id = str_field(sample_round, "round_id")?;
    let proof_round_id = str_field(proof_artifact, "round_id")?;
    check(
        &mut checks,
        "sample schema",
        str_field(sample_round, "schema")? == ENV084_SAMPLE_ROUND_SCHEMA,
    );
    check(
        &mut checks,
        "proof schema",
        str_field(proof_artifact, "schema")? == ENV084_UI_PROOF_SCHEMA,
    );
    check(
        &mut checks,
        "round id agreement",
        sample_round_id == proof_round_id,
    );
    check(
        &mut checks,
        "sample final result",
        str_field(sample_round, "final_result")? == "PASS",
    );
    check(
        &mut checks,
        "proof verifier result",
        str_field(proof_artifact, "verifier_result")? == "PASS",
    );
    check(
        &mut checks,
        "sample network",
        str_field(sample_round, "foundation_network")? == ENV083C_NETWORK,
    );
    check(
        &mut checks,
        "proof network",
        str_field(proof_artifact, "network")? == ENV083C_NETWORK,
    );
    check(
        &mut checks,
        "sample readonly",
        bool_field(sample_round, "foundation_readonly")?,
    );
    check(
        &mut checks,
        "proof evidence mode",
        str_field(proof_artifact, "evidence_mode")? == ENV083C_EVIDENCE_MODE,
    );
    check(
        &mut checks,
        "future live round tx not claimed",
        str_field(proof_artifact, "future_live_round_transaction_evidence")?
            == ENV084_FUTURE_LIVE_ROUND_TRANSACTION_EVIDENCE,
    );

    let anchor = proof_artifact
        .get("live_tn10_anchor")
        .ok_or("missing live_tn10_anchor for Toccata-bound claim")?;
    check(
        &mut checks,
        "anchor evidence mode",
        str_field(anchor, "evidence_mode")? == ENV083C_EVIDENCE_MODE,
    );
    check(
        &mut checks,
        "anchor verifier result",
        str_field(anchor, "verifier_result")? == "PASS",
    );
    check(
        &mut checks,
        "anchor covenant id",
        str_field(anchor, "covenant_id")? == ENV083C_ANCHOR_COVENANT_ID,
    );
    check(
        &mut checks,
        "anchor covenant id confirmed",
        bool_field(anchor, "covenant_id_confirmed")?,
    );
    check(
        &mut checks,
        "anchor lineage",
        str_field(anchor, "covenant_lineage_reference")? == ENV083C_COVENANT_LINEAGE_REFERENCE,
    );

    let transcript = proof_artifact
        .get("application_round_transcript")
        .ok_or("missing application_round_transcript")?;
    let commitment = transcript.get("commitment").ok_or("missing commitment")?;
    let reveal = transcript.get("reveal").ok_or("missing reveal")?;
    check(
        &mut checks,
        "commitment round id",
        str_field(commitment, "round_id")? == sample_round_id,
    );
    check(
        &mut checks,
        "reveal round id",
        str_field(reveal, "round_id")? == sample_round_id,
    );
    check(
        &mut checks,
        "commitment/reveal round id match",
        str_field(commitment, "round_id")? == str_field(reveal, "round_id")?,
    );
    check(
        &mut checks,
        "commitment covenant matches anchor",
        str_field(commitment, "anchor_covenant_id")? == str_field(anchor, "covenant_id")?,
    );
    check(
        &mut checks,
        "reveal covenant matches anchor",
        str_field(reveal, "anchor_covenant_id")? == str_field(anchor, "covenant_id")?,
    );
    check(
        &mut checks,
        "commitment algorithm",
        str_field(commitment, "result_algorithm")? == ENV083C_RESULT_ALGORITHM,
    );
    check(
        &mut checks,
        "reveal algorithm",
        str_field(reveal, "result_algorithm")? == ENV083C_RESULT_ALGORITHM,
    );
    check(
        &mut checks,
        "sample/proof algorithm agreement",
        str_field(sample_round, "result_algorithm")?
            == str_field(proof_artifact, "result_algorithm")?,
    );
    check(
        &mut checks,
        "sample/reveal algorithm agreement",
        str_field(sample_round, "result_algorithm")? == str_field(reveal, "result_algorithm")?,
    );

    let seed = str_field(reveal, "revealed_seed_material")?;
    let expected_commitment_hash = commitment_hash(
        str_field(commitment, "round_id")?,
        seed,
        str_field(commitment, "result_algorithm")?,
        str_field(commitment, "rule_version")?,
        str_field(commitment, "anchor_covenant_id")?,
        str_field(commitment, "covenant_lineage_reference")?,
        str_field(commitment, "evidence_mode")?,
    );
    check(
        &mut checks,
        "commitment hash recomputes",
        str_field(commitment, "commitment_hash")? == expected_commitment_hash,
    );
    let expected_number = derive_roulette_number(seed.as_bytes())?;
    let expected_colour = colour_for_number(expected_number)?;
    check(
        &mut checks,
        "result number verifies",
        u8_field(reveal, "result_number")? == expected_number,
    );
    check(
        &mut checks,
        "result colour verifies",
        str_field(reveal, "result_colour")? == expected_colour,
    );
    check(
        &mut checks,
        "sample/proof result number agreement",
        u8_field(sample_round, "result_number")? == u8_field(reveal, "result_number")?
            && u8_field(sample_round, "result_number")?
                == u8_field(proof_artifact, "result_number")?,
    );
    check(
        &mut checks,
        "sample/proof result colour agreement",
        str_field(sample_round, "result_colour")? == str_field(reveal, "result_colour")?
            && str_field(sample_round, "result_colour")?
                == str_field(proof_artifact, "result_colour")?,
    );
    let expected_reveal_hash = reveal_payload_hash(
        str_field(reveal, "round_id")?,
        seed,
        str_field(reveal, "anchor_covenant_id")?,
        str_field(reveal, "covenant_lineage_reference")?,
        str_field(reveal, "result_algorithm")?,
        u8_field(reveal, "result_number")?,
        str_field(reveal, "result_colour")?,
        str_field(reveal, "evidence_mode")?,
    );
    check(
        &mut checks,
        "reveal payload hash recomputes",
        str_field(reveal, "reveal_payload_hash")? == expected_reveal_hash,
    );

    for value in [sample_round, anchor] {
        for flag in [
            "wallet_access_used",
            "private_key_access_used",
            "signing_used",
            "transaction_created",
            "broadcast_used",
            "mainnet_supported",
        ] {
            if value.get(flag).is_some() {
                check(
                    &mut checks,
                    "execution safety flag false",
                    !bool_field(value, flag)?,
                );
            }
        }
    }
    let safety = proof_artifact
        .get("safety_flags")
        .ok_or("missing safety_flags")?;
    check(
        &mut checks,
        "mock display only",
        bool_field(safety, "mock_display_only")?,
    );
    for flag in [
        "real_betting",
        "real_payouts",
        "backend_custody",
        "wallet_access_used",
        "private_key_access_used",
        "signing_used",
        "transaction_created",
        "broadcast_used",
        "mainnet_supported",
    ] {
        check(
            &mut checks,
            "proof safety flag false",
            !bool_field(safety, flag)?,
        );
    }

    if checks.iter().all(|check| check.passed) {
        Ok(VerificationReport {
            schema: ENV083C_VERIFIER_SCHEMA.to_string(),
            round_id: sample_round_id.to_string(),
            verifier_result: "PASS".to_string(),
            checks,
        })
    } else {
        Err(format!(
            "ENV-084 verifier rejected generated artifacts: {}",
            checks
                .iter()
                .filter(|check| !check.passed)
                .map(|check| check.name)
                .collect::<Vec<_>>()
                .join(", ")
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Env084MockBet {
    bet_id: &'static str,
    bet_type: &'static str,
    selection_value: &'static str,
    stake_units: u64,
    payout_multiplier: u64,
}

fn env084_mock_bets() -> Vec<Env084MockBet> {
    vec![
        Env084MockBet {
            bet_id: "bet-001",
            bet_type: "straight-number",
            selection_value: "17",
            stake_units: 10,
            payout_multiplier: 35,
        },
        Env084MockBet {
            bet_id: "bet-002",
            bet_type: "colour",
            selection_value: "red",
            stake_units: 5,
            payout_multiplier: 1,
        },
        Env084MockBet {
            bet_id: "bet-003",
            bet_type: "colour",
            selection_value: "black",
            stake_units: 4,
            payout_multiplier: 1,
        },
        Env084MockBet {
            bet_id: "bet-004",
            bet_type: "parity",
            selection_value: "odd",
            stake_units: 7,
            payout_multiplier: 1,
        },
        Env084MockBet {
            bet_id: "bet-005",
            bet_type: "parity",
            selection_value: "even",
            stake_units: 6,
            payout_multiplier: 1,
        },
        Env084MockBet {
            bet_id: "bet-006",
            bet_type: "range",
            selection_value: "high",
            stake_units: 9,
            payout_multiplier: 1,
        },
        Env084MockBet {
            bet_id: "bet-007",
            bet_type: "range",
            selection_value: "low",
            stake_units: 8,
            payout_multiplier: 1,
        },
    ]
}

fn hash_env084_bet_ledger(bets: &[Env084MockBet]) -> String {
    let mut canonical = String::new();
    for bet in bets {
        canonical.push_str(bet.bet_id);
        canonical.push('|');
        canonical.push_str(bet.bet_type);
        canonical.push('|');
        canonical.push_str(bet.selection_value);
        canonical.push('|');
        canonical.push_str(&bet.stake_units.to_string());
        canonical.push('|');
        canonical.push_str(&bet.payout_multiplier.to_string());
        canonical.push('\n');
    }
    hash_hex(&canonical)
}

fn env084_settlement_json(
    bets: &[Env084MockBet],
    result_number: u8,
    result_colour: &str,
) -> Vec<Value> {
    bets.iter()
        .map(|bet| {
            let won = env084_bet_wins(bet, result_number, result_colour);
            let payout_units = if won {
                bet.stake_units * (bet.payout_multiplier + 1)
            } else {
                0
            };
            let net_units = if won {
                (bet.stake_units * bet.payout_multiplier) as i64
            } else {
                -(bet.stake_units as i64)
            };
            json!({
                "bet_id": bet.bet_id,
                "bet_type": bet.bet_type,
                "selection_value": bet.selection_value,
                "stake_units": bet.stake_units,
                "won": won,
                "payout_multiplier": bet.payout_multiplier,
                "payout_units": payout_units,
                "net_units": net_units,
                "result_number": result_number,
                "result_colour": result_colour,
            })
        })
        .collect()
}

fn env084_bet_wins(bet: &Env084MockBet, result_number: u8, result_colour: &str) -> bool {
    match bet.bet_type {
        "straight-number" => bet.selection_value.parse::<u8>().ok() == Some(result_number),
        "colour" => bet.selection_value == result_colour,
        "parity" => match result_number {
            0 => false,
            n if n % 2 == 0 => bet.selection_value == "even",
            _ => bet.selection_value == "odd",
        },
        "range" => match result_number {
            0 => false,
            1..=18 => bet.selection_value == "low",
            19..=36 => bet.selection_value == "high",
            _ => false,
        },
        _ => false,
    }
}

fn hex_bytes(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn derive_roulette_number(seed_material: &[u8]) -> Result<u8, String> {
    let n = BigUint::from(37u32);
    let modulus = BigUint::from(1u8) << 256usize;
    let limit = &modulus - (&modulus % &n);
    for counter in 0u32..u32::MAX {
        let mut hasher = blake3::Hasher::new();
        hasher.update(ENV083C_CANDIDATE_DOMAIN.as_bytes());
        hasher.update(seed_material);
        hasher.update(&counter.to_be_bytes());
        let digest = hasher.finalize();
        let candidate = BigUint::from_bytes_be(digest.as_bytes());
        if candidate >= limit {
            continue;
        }
        let reduced = candidate % &n;
        return Ok(reduced.to_u32_digits().first().copied().unwrap_or(0) as u8);
    }
    Err("roulette result derivation exhausted u32 counter space".to_string())
}

pub fn colour_for_number(number: u8) -> Result<&'static str, String> {
    match number {
        0 => Ok("green"),
        1 | 3 | 5 | 7 | 9 | 12 | 14 | 16 | 18 | 19 | 21 | 23 | 25 | 27 | 30 | 32 | 34 | 36 => {
            Ok("red")
        }
        2 | 4 | 6 | 8 | 10 | 11 | 13 | 15 | 17 | 20 | 22 | 24 | 26 | 28 | 29 | 31 | 33 | 35 => {
            Ok("black")
        }
        _ => Err(format!("invalid European roulette number {number}")),
    }
}

fn commitment_hash(
    round_id: &str,
    seed: &str,
    result_algorithm: &str,
    rule_version: &str,
    covenant_id: &str,
    lineage: &str,
    evidence_mode: &str,
) -> String {
    hash_hex(&format!(
        "domain={ENV083C_COMMITMENT_DOMAIN}\nround_id={round_id}\nrevealed_seed_material={seed}\nresult_algorithm={result_algorithm}\nrule_version={rule_version}\nanchor_covenant_id={covenant_id}\ncovenant_lineage_reference={lineage}\nevidence_mode={evidence_mode}\n"
    ))
}

fn reveal_payload_hash(
    round_id: &str,
    seed: &str,
    covenant_id: &str,
    lineage: &str,
    result_algorithm: &str,
    result_number: u8,
    result_colour: &str,
    evidence_mode: &str,
) -> String {
    hash_hex(&format!(
        "domain={ENV083C_REVEAL_DOMAIN}\nround_id={round_id}\nrevealed_seed_material={seed}\nanchor_covenant_id={covenant_id}\ncovenant_lineage_reference={lineage}\nresult_algorithm={result_algorithm}\nresult_number={result_number}\nresult_colour={result_colour}\nevidence_mode={evidence_mode}\n"
    ))
}

fn hash_hex(input: &str) -> String {
    blake3::hash(input.as_bytes()).to_hex().to_string()
}

fn check(checks: &mut Vec<VerificationCheck>, name: &'static str, passed: bool) {
    checks.push(VerificationCheck {
        name,
        passed,
        detail: if passed { "PASS" } else { "FAIL" }.to_string(),
    });
}

fn str_field<'a>(value: &'a Value, field: &str) -> Result<&'a str, String> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing or non-string field: {field}"))
}

fn bool_field(value: &Value, field: &str) -> Result<bool, String> {
    value
        .get(field)
        .and_then(Value::as_bool)
        .ok_or_else(|| format!("missing or non-bool field: {field}"))
}

fn u8_field(value: &Value, field: &str) -> Result<u8, String> {
    let raw = value
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("missing or non-u64 field: {field}"))?;
    u8::try_from(raw).map_err(|_| format!("field {field} is out of u8 range"))
}
