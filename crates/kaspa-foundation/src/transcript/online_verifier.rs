//! Online read-only TN10 verifier scaffolding for canonical proof transcripts.
//!
//! ENV-071 intentionally keeps live-chain access behind a narrow read-only trait.
//! The verifier can be exercised deterministically with mocked observations, while
//! a future wRPC adapter can supply the same observations from public TN10 rusty
//! nodes using `kaspa_wrpc_client` resolver defaults.

use std::path::Path;

use super::canonical::{canonical_tn10_proof_transcript, CANONICAL_NETWORK};
use super::verifier::{verify_transcript, VerificationError};
use super::ProofTranscript;

/// Documented public rusty-kaspa TN10 API approach from the reference spike.
pub const PUBLIC_RUSTY_TN10_READONLY_API_APPROACH: &str = concat!(
    "kaspa_wrpc_client::KaspaRpcClient with WrpcEncoding::Borsh, ",
    "Resolver::default(), NetworkId::with_suffix(NetworkType::Testnet, 10), ",
    "ConnectStrategy::Fallback; read-only calls only"
);

const TN10_NETWORK_ID: &str = "testnet-10";

/// Read-only online verifier configuration. Mainnet is deliberately disabled.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tn10ReadOnlyConfig {
    pub network: String,
    pub endpoint: Option<String>,
    pub mainnet_enabled: bool,
}

impl Tn10ReadOnlyConfig {
    /// Uses the public rusty-kaspa resolver path already used by the reference
    /// TN10 covenant spike; no endpoint secret is required.
    pub fn public_tn10() -> Self {
        Self {
            network: TN10_NETWORK_ID.to_string(),
            endpoint: None,
            mainnet_enabled: false,
        }
    }

    pub fn is_read_only_tn10(&self) -> bool {
        self.network == TN10_NETWORK_ID && !self.mainnet_enabled
    }
}

/// Positive safety declaration required from any live adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReadOnlyClientSafety {
    pub read_only: bool,
    pub no_signing: bool,
    pub no_transaction_creation: bool,
    pub no_broadcast: bool,
    pub no_wallet_access: bool,
    pub no_secrets: bool,
    pub no_mainnet: bool,
}

impl ReadOnlyClientSafety {
    pub const fn strict_read_only() -> Self {
        Self {
            read_only: true,
            no_signing: true,
            no_transaction_creation: true,
            no_broadcast: true,
            no_wallet_access: true,
            no_secrets: true,
            no_mainnet: true,
        }
    }

    pub const fn is_safe(self) -> bool {
        self.read_only
            && self.no_signing
            && self.no_transaction_creation
            && self.no_broadcast
            && self.no_wallet_access
            && self.no_secrets
            && self.no_mainnet
    }
}

/// One live-chain observation from a read-only TN10 client.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Tn10ReadOnlyObservation<T> {
    Supported(T),
    NotYetSupported(&'static str),
    Skipped(&'static str),
}

impl<T> Tn10ReadOnlyObservation<T> {
    pub fn supported(value: T) -> Self {
        Self::Supported(value)
    }

    pub fn not_yet_supported(reason: &'static str) -> Self {
        Self::NotYetSupported(reason)
    }

    pub fn skipped(reason: &'static str) -> Self {
        Self::Skipped(reason)
    }
}

/// Normalized live TN10 observations needed by the canonical verifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveTn10Evidence {
    pub network_id: Tn10ReadOnlyObservation<String>,
    pub is_synced: Tn10ReadOnlyObservation<bool>,
    pub has_utxo_index: Tn10ReadOnlyObservation<bool>,
    pub env064_spend_txid_known: Tn10ReadOnlyObservation<bool>,
    pub env063_input_spent_by_env064: Tn10ReadOnlyObservation<bool>,
    pub continuing_output_exists: Tn10ReadOnlyObservation<bool>,
    pub continuing_output_value_sompi: Tn10ReadOnlyObservation<u64>,
    pub covenant_id: Tn10ReadOnlyObservation<String>,
}

/// A live adapter must only read public TN10 state and return normalized evidence.
pub trait ReadOnlyTn10Client {
    fn safety(&self) -> ReadOnlyClientSafety;

    fn read_tn10_evidence(
        &self,
        transcript: &ProofTranscript,
    ) -> Result<LiveTn10Evidence, OnlineVerificationError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OnlineCheckStatus {
    Supported,
    NotYetSupported,
    Skipped,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OnlineVerificationCheck {
    pub name: &'static str,
    pub status: OnlineCheckStatus,
    pub passed: Option<bool>,
    pub reason: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OnlineVerificationResult {
    Pass,
    Fail,
    Ambiguous,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OnlineVerificationReport {
    pub transcript_id: &'static str,
    pub network: String,
    pub endpoint: Option<String>,
    pub api_approach: &'static str,
    pub offline_prerequisite_passed: bool,
    pub read_only: bool,
    pub mainnet_enabled: bool,
    pub result: OnlineVerificationResult,
    pub checks: Vec<OnlineVerificationCheck>,
}

impl OnlineVerificationReport {
    pub fn passed(&self) -> bool {
        self.result == OnlineVerificationResult::Pass
    }

    pub fn failed(&self) -> bool {
        self.result == OnlineVerificationResult::Fail
    }

    pub fn ambiguous(&self) -> bool {
        self.result == OnlineVerificationResult::Ambiguous
    }

    pub fn failed_checks(&self) -> impl Iterator<Item = &OnlineVerificationCheck> {
        self.checks
            .iter()
            .filter(|check| check.passed == Some(false))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OnlineVerificationError {
    MainnetDisabled,
    UnsafeClient,
    OfflinePrerequisiteFailed(VerificationError),
    ClientUnavailable(&'static str),
}

/// Online verifier that first enforces the offline transcript prerequisite, then
/// compares read-only live TN10 evidence against canonical transcript values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OnlineTn10Verifier {
    config: Tn10ReadOnlyConfig,
}

impl OnlineTn10Verifier {
    pub fn new(config: Tn10ReadOnlyConfig) -> Result<Self, OnlineVerificationError> {
        if !config.is_read_only_tn10() {
            return Err(OnlineVerificationError::MainnetDisabled);
        }
        Ok(Self { config })
    }

    pub fn config(&self) -> &Tn10ReadOnlyConfig {
        &self.config
    }

    pub fn verify(
        &self,
        transcript: &ProofTranscript,
        repo_root: impl AsRef<Path>,
        client: &impl ReadOnlyTn10Client,
    ) -> Result<OnlineVerificationReport, OnlineVerificationError> {
        verify_transcript(transcript, repo_root)
            .map_err(OnlineVerificationError::OfflinePrerequisiteFailed)?;

        let safety = client.safety();
        if !safety.is_safe() {
            return Err(OnlineVerificationError::UnsafeClient);
        }

        let evidence = client.read_tn10_evidence(transcript)?;
        let checks = build_checks(transcript, evidence);
        let result = summarize_result(&checks);

        Ok(OnlineVerificationReport {
            transcript_id: transcript.transcript_id,
            network: self.config.network.clone(),
            endpoint: self.config.endpoint.clone(),
            api_approach: PUBLIC_RUSTY_TN10_READONLY_API_APPROACH,
            offline_prerequisite_passed: true,
            read_only: safety.read_only,
            mainnet_enabled: self.config.mainnet_enabled,
            result,
            checks,
        })
    }
}

/// Verifies the built-in canonical TN10 transcript against read-only live evidence.
pub fn verify_canonical_tn10_transcript_online(
    repo_root: impl AsRef<Path>,
    client: &impl ReadOnlyTn10Client,
    config: Tn10ReadOnlyConfig,
) -> Result<OnlineVerificationReport, OnlineVerificationError> {
    OnlineTn10Verifier::new(config)?.verify(&canonical_tn10_proof_transcript(), repo_root, client)
}

fn build_checks(
    transcript: &ProofTranscript,
    evidence: LiveTn10Evidence,
) -> Vec<OnlineVerificationCheck> {
    vec![
        check_eq("network", evidence.network_id, TN10_NETWORK_ID.to_string()),
        check_bool("node synced", evidence.is_synced),
        check_bool("utxo index", evidence.has_utxo_index),
        check_bool("ENV-064 spend txid known", evidence.env064_spend_txid_known),
        check_bool(
            "ENV-063 input spent by ENV-064",
            evidence.env063_input_spent_by_env064,
        ),
        check_bool(
            "continuing output exists",
            evidence.continuing_output_exists,
        ),
        check_eq(
            "continuing output value",
            evidence.continuing_output_value_sompi,
            transcript.canonical.continuing_output_value_sompi,
        ),
        check_eq(
            "covenant id",
            evidence.covenant_id,
            transcript.canonical.covenant_id.to_string(),
        ),
        check_eq(
            "transcript network",
            Tn10ReadOnlyObservation::supported(transcript.network),
            CANONICAL_NETWORK,
        ),
    ]
}

fn check_bool(
    name: &'static str,
    observation: Tn10ReadOnlyObservation<bool>,
) -> OnlineVerificationCheck {
    check_eq(name, observation, true)
}

fn check_eq<T>(
    name: &'static str,
    observation: Tn10ReadOnlyObservation<T>,
    expected: T,
) -> OnlineVerificationCheck
where
    T: Eq + std::fmt::Debug,
{
    match observation {
        Tn10ReadOnlyObservation::Supported(actual) => {
            let passed = actual == expected;
            OnlineVerificationCheck {
                name,
                status: OnlineCheckStatus::Supported,
                passed: Some(passed),
                reason: if passed {
                    None
                } else {
                    Some(format!("expected {expected:?}, observed {actual:?}"))
                },
            }
        }
        Tn10ReadOnlyObservation::NotYetSupported(reason) => OnlineVerificationCheck {
            name,
            status: OnlineCheckStatus::NotYetSupported,
            passed: None,
            reason: Some(reason.to_string()),
        },
        Tn10ReadOnlyObservation::Skipped(reason) => OnlineVerificationCheck {
            name,
            status: OnlineCheckStatus::Skipped,
            passed: None,
            reason: Some(reason.to_string()),
        },
    }
}

fn summarize_result(checks: &[OnlineVerificationCheck]) -> OnlineVerificationResult {
    if checks.iter().any(|check| check.passed == Some(false)) {
        OnlineVerificationResult::Fail
    } else if checks.iter().any(|check| check.passed.is_none()) {
        OnlineVerificationResult::Ambiguous
    } else {
        OnlineVerificationResult::Pass
    }
}
