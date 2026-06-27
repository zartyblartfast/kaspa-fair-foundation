use num_bigint::BigUint;
use serde_json::{json, Value};
use std::error::Error;

pub const ROULETTE_POC_SCHEMA_V1: &str = "kaspa-fair-roulette-poc-round-v1";
pub const ROULETTE_ENGINE_SCHEMA_V1: &str = "kaspa-fair-roulette-engine-round-v1";
pub const ROULETTE_RESULT_ALGORITHM_V1: &str = "blake3-domain-separated-rejection-sampling-v1";
pub const ROULETTE_VARIANT_EUROPEAN: &str = "european";
pub const ROULETTE_CANDIDATE_DOMAIN_V1: &str = "kaspa-fair:roulette:candidate:v1";
pub const ENV076_ROUND_ID: &str = "env-076-round-0001";
pub const ENV077_ROUND_ID: &str = "env-077-round-0001";
pub const ENV076_RULE_VERSION: &str = "env-076-roulette-poc-rules-v1";
pub const ENV076_PAYOUT_TABLE_VERSION: &str = "env-076-roulette-poc-payouts-v1";
pub const ENV077_RULE_VERSION: &str = "env-077-roulette-engine-rules-v1";
pub const ENV077_PAYOUT_TABLE_VERSION: &str = "env-077-roulette-engine-payouts-v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FoundationVerifierContract {
    pub schema: String,
    pub verifier_result: String,
    pub network: String,
    pub covenant_id: String,
    pub env064_spend_txid: String,
    pub accepting_block_hash: String,
    pub readonly: bool,
    pub mainnet_supported: bool,
    pub wallet_access_used: bool,
    pub signing_used: bool,
    pub transaction_created: bool,
    pub broadcast_used: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoundState {
    Created,
    BetsOpen,
    SpinVisualStarted,
    NoMoreBets,
    ResultFinalised,
    Settled,
    ProofPublished,
}

impl RoundState {
    pub fn as_str(self) -> &'static str {
        match self {
            RoundState::Created => "Created",
            RoundState::BetsOpen => "BetsOpen",
            RoundState::SpinVisualStarted => "SpinVisualStarted",
            RoundState::NoMoreBets => "NoMoreBets",
            RoundState::ResultFinalised => "ResultFinalised",
            RoundState::Settled => "Settled",
            RoundState::ProofPublished => "ProofPublished",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Colour {
    Green,
    Red,
    Black,
}

impl Colour {
    pub fn as_str(self) -> &'static str {
        match self {
            Colour::Green => "green",
            Colour::Red => "red",
            Colour::Black => "black",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Parity {
    Odd,
    Even,
}

impl Parity {
    fn as_str(self) -> &'static str {
        match self {
            Parity::Odd => "odd",
            Parity::Even => "even",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NumberRange {
    Low,
    High,
}

impl NumberRange {
    fn as_str(self) -> &'static str {
        match self {
            NumberRange::Low => "low",
            NumberRange::High => "high",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BetKind {
    StraightNumber(u8),
    Colour(Colour),
    Parity(Parity),
    Range(NumberRange),
}

impl BetKind {
    fn bet_type(self) -> &'static str {
        match self {
            BetKind::StraightNumber(_) => "straight-number",
            BetKind::Colour(_) => "colour",
            BetKind::Parity(_) => "parity",
            BetKind::Range(_) => "range",
        }
    }

    fn selection_value(self) -> String {
        match self {
            BetKind::StraightNumber(number) => number.to_string(),
            BetKind::Colour(colour) => colour.as_str().to_string(),
            BetKind::Parity(parity) => parity.as_str().to_string(),
            BetKind::Range(range) => range.as_str().to_string(),
        }
    }

    fn payout_multiplier(self) -> u64 {
        match self {
            BetKind::StraightNumber(_) => 35,
            BetKind::Colour(_) | BetKind::Parity(_) | BetKind::Range(_) => 1,
        }
    }

    fn wins(self, result_number: u8, result_colour: Colour) -> bool {
        match self {
            BetKind::StraightNumber(number) => number == result_number,
            BetKind::Colour(colour) => colour == result_colour,
            BetKind::Parity(parity) => match result_number {
                0 => false,
                n if n % 2 == 0 => parity == Parity::Even,
                _ => parity == Parity::Odd,
            },
            BetKind::Range(range) => match result_number {
                0 => false,
                1..=18 => range == NumberRange::Low,
                19..=36 => range == NumberRange::High,
                _ => false,
            },
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MockBet {
    bet_id: &'static str,
    kind: BetKind,
    stake_units: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SettlementEntry {
    bet_id: &'static str,
    bet_type: &'static str,
    selection_value: String,
    stake_units: u64,
    won: bool,
    payout_units: u64,
    net_units: i64,
}

impl SettlementEntry {
    fn to_json(&self) -> Value {
        json!({
            "bet_id": self.bet_id,
            "bet_type": self.bet_type,
            "selection_value": self.selection_value,
            "stake_units": self.stake_units,
            "won": self.won,
            "payout_units": self.payout_units,
            "net_units": self.net_units,
        })
    }
}

#[derive(Clone, Debug)]
struct RouletteRoundEngine {
    foundation: FoundationVerifierContract,
    round_id: &'static str,
    rule_version: &'static str,
    payout_table_version: &'static str,
    state: RoundState,
    bets: Vec<MockBet>,
    bet_ledger_hash: Option<String>,
    result_number: Option<u8>,
    result_colour: Option<Colour>,
    settlement: Vec<SettlementEntry>,
}

impl RouletteRoundEngine {
    fn new(
        foundation: FoundationVerifierContract,
        round_id: &'static str,
        rule_version: &'static str,
        payout_table_version: &'static str,
    ) -> Self {
        Self {
            foundation,
            round_id,
            rule_version,
            payout_table_version,
            state: RoundState::Created,
            bets: Vec::new(),
            bet_ledger_hash: None,
            result_number: None,
            result_colour: None,
            settlement: Vec::new(),
        }
    }

    fn open_bets(&mut self) -> Result<(), String> {
        self.require_state(RoundState::Created, "open_bets")?;
        self.state = RoundState::BetsOpen;
        Ok(())
    }

    fn start_spin_visual(&mut self) -> Result<(), String> {
        self.require_state(RoundState::BetsOpen, "start_spin_visual")?;
        self.state = RoundState::SpinVisualStarted;
        Ok(())
    }

    fn accept_bet(&mut self, bet: MockBet) -> Result<(), String> {
        match self.state {
            RoundState::BetsOpen | RoundState::SpinVisualStarted => {
                self.bets.push(bet);
                Ok(())
            }
            _ => Err(format!(
                "accept_bet rejected in state {}; no-more-bets boundary already crossed",
                self.state.as_str()
            )),
        }
    }

    fn close_bets(&mut self) -> Result<(), String> {
        match self.state {
            RoundState::BetsOpen | RoundState::SpinVisualStarted => {
                self.bet_ledger_hash = Some(bet_ledger_hash(&self.bets));
                self.state = RoundState::NoMoreBets;
                Ok(())
            }
            _ => Err(format!(
                "close_bets invalid from state {}",
                self.state.as_str()
            )),
        }
    }

    fn finalise_result(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        if self.state != RoundState::NoMoreBets {
            return Err(format!(
                "result finalisation requires state NoMoreBets, got {}",
                self.state.as_str()
            )
            .into());
        }

        let bet_ledger_hash = self
            .bet_ledger_hash
            .as_deref()
            .ok_or("bet ledger hash missing at result finalisation")?;
        let seed_material =
            roulette_seed_material(&self.foundation, self.round_id, bet_ledger_hash);
        let result_number = derive_roulette_number(&seed_material)?;
        let result_colour = colour_for_number(result_number)
            .ok_or_else(|| format!("invalid roulette number {result_number}"))?;

        self.result_number = Some(result_number);
        self.result_colour = Some(result_colour);
        self.state = RoundState::ResultFinalised;
        Ok(())
    }

    fn settle(&mut self) -> Result<(), String> {
        self.require_state(RoundState::ResultFinalised, "settle")?;
        let result_number = self
            .result_number
            .ok_or("result number missing before settlement")?;
        let result_colour = self
            .result_colour
            .ok_or("result colour missing before settlement")?;
        self.settlement = settle_mock_bets(&self.bets, result_number, result_colour);
        self.state = RoundState::Settled;
        Ok(())
    }

    fn publish_proof(&mut self) -> Result<(), String> {
        self.require_state(RoundState::Settled, "publish_proof")?;
        self.state = RoundState::ProofPublished;
        Ok(())
    }

    fn build_engine_json(&self) -> Result<Value, Box<dyn Error + Send + Sync>> {
        self.require_state(RoundState::ProofPublished, "build_engine_json")
            .map_err(|err| -> Box<dyn Error + Send + Sync> { err.into() })?;
        let bet_ledger_hash = self
            .bet_ledger_hash
            .as_deref()
            .ok_or("bet ledger hash missing")?;
        let result_number = self.result_number.ok_or("result number missing")?;
        let result_colour = self.result_colour.ok_or("result colour missing")?;
        let seed_material =
            roulette_seed_material(&self.foundation, self.round_id, bet_ledger_hash);

        Ok(json!({
            "schema": ROULETTE_ENGINE_SCHEMA_V1,
            "round_id": self.round_id,
            "round_state": self.state.as_str(),
            "foundation_verifier_schema": self.foundation.schema,
            "foundation_verifier_result": self.foundation.verifier_result,
            "foundation_network": self.foundation.network,
            "foundation_covenant_id": self.foundation.covenant_id,
            "foundation_env064_spend_txid": self.foundation.env064_spend_txid,
            "foundation_accepting_block_hash": self.foundation.accepting_block_hash,
            "foundation_readonly": self.foundation.readonly,
            "mainnet_supported": self.foundation.mainnet_supported,
            "wallet_access_used": self.foundation.wallet_access_used,
            "signing_used": self.foundation.signing_used,
            "transaction_created": self.foundation.transaction_created,
            "broadcast_used": self.foundation.broadcast_used,
            "bet_ledger_hash": bet_ledger_hash,
            "seed_material_description": "utf8 lines: round_id, covenant_id, env064_spend_txid, accepting_block_hash, final_mock_bet_ledger_hash",
            "seed_material_hex": hex_string(&seed_material),
            "result_algorithm": ROULETTE_RESULT_ALGORITHM_V1,
            "roulette_variant": ROULETTE_VARIANT_EUROPEAN,
            "rule_version": self.rule_version,
            "payout_table_version": self.payout_table_version,
            "result_number": result_number,
            "result_colour": result_colour.as_str(),
            "bets": bets_to_json(&self.bets),
            "settlement": self.settlement.iter().map(SettlementEntry::to_json).collect::<Vec<_>>(),
            "final_result": "PASS",
        }))
    }

    fn require_state(&self, expected: RoundState, action: &str) -> Result<(), String> {
        if self.state == expected {
            Ok(())
        } else {
            Err(format!(
                "{action} requires state {}, got {}",
                expected.as_str(),
                self.state.as_str()
            ))
        }
    }
}

pub fn validate_foundation_verifier_contract(
    value: &Value,
    live_schema: &str,
    continuing_output_value_sompi_required: u64,
) -> Result<FoundationVerifierContract, String> {
    let schema = required_string_field(value, "schema")?;
    if schema != live_schema {
        return Err(format!("schema must be {live_schema}, got {schema}"));
    }

    let network = required_string_field(value, "network")?;
    if network != "testnet-10" {
        return Err(format!("network must be testnet-10, got {network}"));
    }

    let verifier_result = required_string_field(value, "verifier_result")?;
    if verifier_result != "PASS" {
        return Err(format!(
            "verifier_result must be PASS, got {verifier_result}"
        ));
    }

    for field in [
        "accepted",
        "input_relationship_confirmed",
        "continuing_output_confirmed",
        "continuing_output_value_confirmed",
        "covenant_id_confirmed",
        "readonly",
    ] {
        if !required_bool_field(value, field)? {
            return Err(format!("{field} must be true"));
        }
    }

    let continuing_output_value_sompi = required_u64_field(value, "continuing_output_value_sompi")?;
    if continuing_output_value_sompi != continuing_output_value_sompi_required {
        return Err(format!(
            "continuing_output_value_sompi must be {continuing_output_value_sompi_required}, got {continuing_output_value_sompi}"
        ));
    }

    let mainnet_supported = required_bool_field(value, "mainnet_supported")?;
    let wallet_access_used = required_bool_field(value, "wallet_access_used")?;
    let signing_used = required_bool_field(value, "signing_used")?;
    let transaction_created = required_bool_field(value, "transaction_created")?;
    let broadcast_used = required_bool_field(value, "broadcast_used")?;

    for (field, current) in [
        ("mainnet_supported", mainnet_supported),
        ("wallet_access_used", wallet_access_used),
        ("signing_used", signing_used),
        ("transaction_created", transaction_created),
        ("broadcast_used", broadcast_used),
    ] {
        if current {
            return Err(format!("{field} must be false"));
        }
    }

    Ok(FoundationVerifierContract {
        schema,
        verifier_result,
        network,
        covenant_id: required_string_field(value, "covenant_id")?,
        env064_spend_txid: required_string_field(value, "env064_spend_txid")?,
        accepting_block_hash: required_string_field(value, "accepting_block_hash")?,
        readonly: true,
        mainnet_supported,
        wallet_access_used,
        signing_used,
        transaction_created,
        broadcast_used,
    })
}

pub fn roulette_poc_round_json(
    foundation: &FoundationVerifierContract,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let bets = env076_mock_bets();
    let bet_ledger_hash = bet_ledger_hash(&bets);
    let seed_material = roulette_seed_material(foundation, ENV076_ROUND_ID, &bet_ledger_hash);
    let result_number = derive_roulette_number(&seed_material)?;
    let result_colour = colour_for_number(result_number)
        .ok_or_else(|| format!("invalid roulette number {result_number}"))?;
    let settlement = settle_mock_bets(&bets, result_number, result_colour);

    Ok(json!({
        "schema": ROULETTE_POC_SCHEMA_V1,
        "round_id": ENV076_ROUND_ID,
        "foundation_verifier_schema": foundation.schema,
        "foundation_verifier_result": foundation.verifier_result,
        "foundation_network": foundation.network,
        "foundation_covenant_id": foundation.covenant_id,
        "foundation_env064_spend_txid": foundation.env064_spend_txid,
        "foundation_accepting_block_hash": foundation.accepting_block_hash,
        "foundation_readonly": foundation.readonly,
        "mainnet_supported": foundation.mainnet_supported,
        "wallet_access_used": foundation.wallet_access_used,
        "signing_used": foundation.signing_used,
        "transaction_created": foundation.transaction_created,
        "broadcast_used": foundation.broadcast_used,
        "bet_ledger_hash": bet_ledger_hash,
        "seed_material_description": "utf8 lines: round_id, covenant_id, env064_spend_txid, accepting_block_hash, final_mock_bet_ledger_hash",
        "seed_material_hex": hex_string(&seed_material),
        "result_algorithm": ROULETTE_RESULT_ALGORITHM_V1,
        "roulette_variant": ROULETTE_VARIANT_EUROPEAN,
        "rule_version": ENV076_RULE_VERSION,
        "payout_table_version": ENV076_PAYOUT_TABLE_VERSION,
        "result_number": result_number,
        "result_colour": result_colour.as_str(),
        "bets": bets_to_json(&bets),
        "settlement": settlement.iter().map(SettlementEntry::to_json).collect::<Vec<_>>(),
        "final_result": "PASS",
    }))
}

pub fn roulette_engine_round_json(
    foundation: &FoundationVerifierContract,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let mut engine = RouletteRoundEngine::new(
        foundation.clone(),
        ENV077_ROUND_ID,
        ENV077_RULE_VERSION,
        ENV077_PAYOUT_TABLE_VERSION,
    );
    engine.open_bets()?;

    let mut bets = env077_mock_bets().into_iter();
    for _ in 0..4 {
        engine.accept_bet(
            bets.next()
                .ok_or("mock bet ledger underflow before spin start")?,
        )?;
    }
    engine.start_spin_visual()?;
    for bet in bets {
        engine.accept_bet(bet)?;
    }
    engine.close_bets()?;
    engine.finalise_result()?;
    engine.settle()?;
    engine.publish_proof()?;
    engine.build_engine_json()
}

pub fn colour_for_number(number: u8) -> Option<Colour> {
    match number {
        0 => Some(Colour::Green),
        1 | 3 | 5 | 7 | 9 | 12 | 14 | 16 | 18 | 19 | 21 | 23 | 25 | 27 | 30 | 32 | 34 | 36 => {
            Some(Colour::Red)
        }
        2 | 4 | 6 | 8 | 10 | 11 | 13 | 15 | 17 | 20 | 22 | 24 | 26 | 28 | 29 | 31 | 33 | 35 => {
            Some(Colour::Black)
        }
        _ => None,
    }
}

pub fn derive_roulette_number(seed_material: &[u8]) -> Result<u8, Box<dyn Error + Send + Sync>> {
    let n = BigUint::from(37u32);
    let modulus = BigUint::from(1u8) << 256usize;
    let limit = &modulus - (&modulus % &n);

    for counter in 0u32..u32::MAX {
        let candidate = candidate_biguint(seed_material, counter);
        if candidate >= limit {
            continue;
        }
        let reduced = candidate % &n;
        let digits = reduced.to_u32_digits();
        return Ok(digits.first().copied().unwrap_or(0) as u8);
    }

    Err("roulette result derivation exhausted u32 counter space".into())
}

#[cfg(test)]
pub fn candidate_hex(seed_material: &[u8], counter: u32) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(ROULETTE_CANDIDATE_DOMAIN_V1.as_bytes());
    hasher.update(seed_material);
    hasher.update(&counter.to_be_bytes());
    hasher.finalize().to_hex().to_string()
}

#[cfg(test)]
pub fn algorithm_label() -> &'static str {
    ROULETTE_RESULT_ALGORITHM_V1
}

#[cfg(test)]
pub fn stable_mock_bet_ledger_hash_for_tests() -> String {
    bet_ledger_hash(&env077_mock_bets())
}

fn required_string_field(value: &Value, field: &str) -> Result<String, String> {
    value
        .get(field)
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| format!("missing or non-string field: {field}"))
}

fn required_bool_field(value: &Value, field: &str) -> Result<bool, String> {
    value
        .get(field)
        .and_then(Value::as_bool)
        .ok_or_else(|| format!("missing or non-bool field: {field}"))
}

fn required_u64_field(value: &Value, field: &str) -> Result<u64, String> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("missing or non-u64 field: {field}"))
}

fn env076_mock_bets() -> Vec<MockBet> {
    vec![
        MockBet {
            bet_id: "bet-001",
            kind: BetKind::StraightNumber(17),
            stake_units: 10,
        },
        MockBet {
            bet_id: "bet-002",
            kind: BetKind::Colour(Colour::Red),
            stake_units: 5,
        },
        MockBet {
            bet_id: "bet-003",
            kind: BetKind::Parity(Parity::Odd),
            stake_units: 7,
        },
        MockBet {
            bet_id: "bet-004",
            kind: BetKind::Range(NumberRange::High),
            stake_units: 9,
        },
    ]
}

fn env077_mock_bets() -> Vec<MockBet> {
    vec![
        MockBet {
            bet_id: "bet-001",
            kind: BetKind::StraightNumber(17),
            stake_units: 10,
        },
        MockBet {
            bet_id: "bet-002",
            kind: BetKind::Colour(Colour::Red),
            stake_units: 5,
        },
        MockBet {
            bet_id: "bet-003",
            kind: BetKind::Colour(Colour::Black),
            stake_units: 4,
        },
        MockBet {
            bet_id: "bet-004",
            kind: BetKind::Parity(Parity::Odd),
            stake_units: 7,
        },
        MockBet {
            bet_id: "bet-005",
            kind: BetKind::Parity(Parity::Even),
            stake_units: 6,
        },
        MockBet {
            bet_id: "bet-006",
            kind: BetKind::Range(NumberRange::High),
            stake_units: 9,
        },
        MockBet {
            bet_id: "bet-007",
            kind: BetKind::Range(NumberRange::Low),
            stake_units: 8,
        },
    ]
}

fn bets_to_json(bets: &[MockBet]) -> Vec<Value> {
    bets.iter()
        .map(|bet| {
            json!({
                "bet_id": bet.bet_id,
                "bet_type": bet.kind.bet_type(),
                "selection_value": bet.kind.selection_value(),
                "stake_units": bet.stake_units,
                "payout_multiplier": bet.kind.payout_multiplier(),
            })
        })
        .collect()
}

fn bet_ledger_hash(bets: &[MockBet]) -> String {
    let mut canonical = String::new();
    for bet in bets {
        canonical.push_str(bet.bet_id);
        canonical.push('|');
        canonical.push_str(bet.kind.bet_type());
        canonical.push('|');
        canonical.push_str(&bet.kind.selection_value());
        canonical.push('|');
        canonical.push_str(&bet.stake_units.to_string());
        canonical.push('|');
        canonical.push_str(&bet.kind.payout_multiplier().to_string());
        canonical.push('\n');
    }
    hex_string(blake3::hash(canonical.as_bytes()).as_bytes())
}

fn roulette_seed_material(
    foundation: &FoundationVerifierContract,
    round_id: &str,
    bet_ledger_hash: &str,
) -> Vec<u8> {
    format!(
        "round_id={round_id}\nfoundation_covenant_id={}\nfoundation_env064_spend_txid={}\nfoundation_accepting_block_hash={}\nfinal_mock_bet_ledger_hash={}\n",
        foundation.covenant_id,
        foundation.env064_spend_txid,
        foundation.accepting_block_hash,
        bet_ledger_hash,
    )
    .into_bytes()
}

fn candidate_biguint(seed_material: &[u8], counter: u32) -> BigUint {
    let mut hasher = blake3::Hasher::new();
    hasher.update(ROULETTE_CANDIDATE_DOMAIN_V1.as_bytes());
    hasher.update(seed_material);
    hasher.update(&counter.to_be_bytes());
    let candidate_bytes = hasher.finalize();
    BigUint::from_bytes_be(candidate_bytes.as_bytes())
}

fn settle_mock_bets(
    bets: &[MockBet],
    result_number: u8,
    result_colour: Colour,
) -> Vec<SettlementEntry> {
    bets.iter()
        .map(|bet| {
            let won = bet.kind.wins(result_number, result_colour);
            let payout_units = if won {
                bet.stake_units * (bet.kind.payout_multiplier() + 1)
            } else {
                0
            };
            let net_units = if won {
                (bet.stake_units * bet.kind.payout_multiplier()) as i64
            } else {
                -(bet.stake_units as i64)
            };
            SettlementEntry {
                bet_id: bet.bet_id,
                bet_type: bet.kind.bet_type(),
                selection_value: bet.kind.selection_value(),
                stake_units: bet.stake_units,
                won,
                payout_units,
                net_units,
            }
        })
        .collect()
}

fn hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn foundation_fixture() -> FoundationVerifierContract {
        FoundationVerifierContract {
            schema: "kaspa-fair-live-verification-result-v1".to_string(),
            verifier_result: "PASS".to_string(),
            network: "testnet-10".to_string(),
            covenant_id: "e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7"
                .to_string(),
            env064_spend_txid: "4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c"
                .to_string(),
            accepting_block_hash:
                "e0d62ead241a5217769266dc96e8055c5893c29074ed2c50ba23de1a9ba75190".to_string(),
            readonly: true,
            mainnet_supported: false,
            wallet_access_used: false,
            signing_used: false,
            transaction_created: false,
            broadcast_used: false,
        }
    }

    fn foundation_json_fixture() -> Value {
        json!({
            "schema": "kaspa-fair-live-verification-result-v1",
            "network": "testnet-10",
            "mainnet_supported": false,
            "verifier_result": "PASS",
            "accepted": true,
            "input_relationship_confirmed": true,
            "continuing_output_confirmed": true,
            "continuing_output_value_sompi": 99_700_000,
            "continuing_output_value_confirmed": true,
            "covenant_id_confirmed": true,
            "readonly": true,
            "signing_used": false,
            "transaction_created": false,
            "broadcast_used": false,
            "wallet_access_used": false,
            "covenant_id": foundation_fixture().covenant_id,
            "env064_spend_txid": foundation_fixture().env064_spend_txid,
            "accepting_block_hash": foundation_fixture().accepting_block_hash,
        })
    }

    #[test]
    fn engine_round_reaches_proof_published_and_pass() {
        let round = roulette_engine_round_json(&foundation_fixture()).unwrap();
        assert_eq!(round["schema"], ROULETTE_ENGINE_SCHEMA_V1);
        assert_eq!(round["round_state"], "ProofPublished");
        assert_eq!(round["final_result"], "PASS");
    }

    #[test]
    fn state_machine_rejects_invalid_transitions() {
        let mut engine = RouletteRoundEngine::new(
            foundation_fixture(),
            ENV077_ROUND_ID,
            ENV077_RULE_VERSION,
            ENV077_PAYOUT_TABLE_VERSION,
        );
        assert!(engine.finalise_result().is_err());
        engine.open_bets().unwrap();
        assert!(engine.settle().is_err());
        engine.start_spin_visual().unwrap();
        engine.accept_bet(env077_mock_bets()[0].clone()).unwrap();
        engine.close_bets().unwrap();
        assert!(engine.accept_bet(env077_mock_bets()[1].clone()).is_err());
        assert!(engine.publish_proof().is_err());
    }

    #[test]
    fn same_ledger_hash_is_stable_and_changed_ledger_differs() {
        let original = env077_mock_bets();
        let same = env077_mock_bets();
        let mut changed = env077_mock_bets();
        changed[0].stake_units += 1;

        assert_eq!(bet_ledger_hash(&original), bet_ledger_hash(&same));
        assert_ne!(bet_ledger_hash(&original), bet_ledger_hash(&changed));
    }

    #[test]
    fn deterministic_algorithm_label_is_stable() {
        assert_eq!(algorithm_label(), ROULETTE_RESULT_ALGORITHM_V1);
    }

    #[test]
    fn same_seed_material_gives_same_result_and_result_is_in_range() {
        let seed = b"env077-seed";
        let a = derive_roulette_number(seed).unwrap();
        let b = derive_roulette_number(seed).unwrap();
        assert_eq!(a, b);
        assert!(a <= 36, "expected 0..36, got {a}");
    }

    #[test]
    fn candidate_stream_changes_when_seed_changes() {
        assert_ne!(candidate_hex(b"seed-a", 0), candidate_hex(b"seed-b", 0));
    }

    #[test]
    fn european_colour_table_is_complete_and_exact() {
        assert_eq!(colour_for_number(0), Some(Colour::Green));
        for red in [
            1u8, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36,
        ] {
            assert_eq!(colour_for_number(red), Some(Colour::Red), "{red}");
        }
        for black in [
            2u8, 4, 6, 8, 10, 11, 13, 15, 17, 20, 22, 24, 26, 28, 29, 31, 33, 35,
        ] {
            assert_eq!(colour_for_number(black), Some(Colour::Black), "{black}");
        }
        for number in 0u8..=36 {
            assert!(colour_for_number(number).is_some(), "missing {number}");
        }
    }

    #[test]
    fn mock_settlement_is_reproducible() {
        let first = settle_mock_bets(&env077_mock_bets(), 7, Colour::Red)
            .into_iter()
            .map(|entry| entry.to_json())
            .collect::<Vec<_>>();
        let second = settle_mock_bets(&env077_mock_bets(), 7, Colour::Red)
            .into_iter()
            .map(|entry| entry.to_json())
            .collect::<Vec<_>>();
        assert_eq!(first, second);
    }

    #[test]
    fn foundation_contract_validator_rejects_unsafe_flags() {
        let mut foundation = foundation_json_fixture();
        foundation["wallet_access_used"] = json!(true);
        let error = validate_foundation_verifier_contract(
            &foundation,
            "kaspa-fair-live-verification-result-v1",
            99_700_000,
        )
        .unwrap_err();
        assert!(error.contains("wallet_access_used"));
    }

    #[test]
    fn foundation_contract_validator_accepts_passing_contract() {
        let validated = validate_foundation_verifier_contract(
            &foundation_json_fixture(),
            "kaspa-fair-live-verification-result-v1",
            99_700_000,
        )
        .unwrap();
        assert_eq!(validated.verifier_result, "PASS");
        assert_eq!(validated.network, "testnet-10");
    }

    #[test]
    fn mock_ledger_hash_is_stable() {
        let hash_a = stable_mock_bet_ledger_hash_for_tests();
        let hash_b = stable_mock_bet_ledger_hash_for_tests();
        assert_eq!(hash_a, hash_b);
        assert_eq!(hash_a.len(), 64);
    }

    #[test]
    fn roulette_engine_json_contains_required_contract_fields() {
        let round = roulette_engine_round_json(&foundation_fixture()).unwrap();
        assert_eq!(round["foundation_verifier_result"], "PASS");
        assert_eq!(round["foundation_network"], "testnet-10");
        assert_eq!(round["mainnet_supported"], false);
        assert_eq!(round["wallet_access_used"], false);
        assert_eq!(round["signing_used"], false);
        assert_eq!(round["transaction_created"], false);
        assert_eq!(round["broadcast_used"], false);
        assert_eq!(round["result_algorithm"], ROULETTE_RESULT_ALGORITHM_V1);
        assert_eq!(round["roulette_variant"], ROULETTE_VARIANT_EUROPEAN);
        assert_eq!(round["round_state"], "ProofPublished");
        assert_eq!(round["bets"].as_array().unwrap().len(), 7);
        assert_eq!(round["settlement"].as_array().unwrap().len(), 7);
    }
}
