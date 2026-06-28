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
