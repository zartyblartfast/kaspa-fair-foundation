use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use kaspa_foundation::transcript::{
    canonical::canonical_tn10_proof_transcript,
    online_verifier::{
        verify_canonical_tn10_transcript_online, LiveTn10Evidence, OnlineCheckStatus,
        OnlineTn10Verifier, ReadOnlyClientSafety, ReadOnlyTn10Client, Tn10ReadOnlyConfig,
        Tn10ReadOnlyObservation,
    },
};

const RUN_LIVE_TN10_ENV: &str = "KASPA_FOUNDATION_RUN_LIVE_TN10";
const ENV063_COVENANT_OUTPOINT_TXID: &str =
    "2c7802ff9a6eec2828a96168d8f62a9a276176441ed8cb6086cd5d5d0cb26849";
const ENV063_COVENANT_OUTPOINT_INDEX: u32 = 0;
const ENV064_SPEND_TXID: &str = "4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c";
const ENV064_CONTINUING_OUTPUT_INDEX: u32 = 0;
const ENV064_CONTINUING_OUTPUT_VALUE_SOMPI: u64 = 99_700_000;
const ENV063_ENV064_COVENANT_ADDRESS: &str =
    "kaspatest:prn06m02u6aygczhzjuttgclymxlsrj7tdhy0ptye3u8gcdwl2grc0kk4fn7r";
const PUBLIC_TN10_TRANSACTION_API_BASE: &str = "https://api-tn10.kaspa.org";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[derive(Clone, Debug)]
struct MockReadOnlyClient {
    safety: ReadOnlyClientSafety,
    evidence: LiveTn10Evidence,
}

impl ReadOnlyTn10Client for MockReadOnlyClient {
    fn safety(&self) -> ReadOnlyClientSafety {
        self.safety
    }

    fn read_tn10_evidence(
        &self,
        _transcript: &kaspa_foundation::transcript::ProofTranscript,
    ) -> Result<
        LiveTn10Evidence,
        kaspa_foundation::transcript::online_verifier::OnlineVerificationError,
    > {
        Ok(self.evidence.clone())
    }
}

fn safe_mock(evidence: LiveTn10Evidence) -> MockReadOnlyClient {
    MockReadOnlyClient {
        safety: ReadOnlyClientSafety::strict_read_only(),
        evidence,
    }
}

fn passing_evidence() -> LiveTn10Evidence {
    LiveTn10Evidence {
        network_id: Tn10ReadOnlyObservation::supported("testnet-10".to_string()),
        is_synced: Tn10ReadOnlyObservation::supported(true),
        has_utxo_index: Tn10ReadOnlyObservation::supported(true),
        env064_spend_txid_known: Tn10ReadOnlyObservation::supported(true),
        env063_input_spent_by_env064: Tn10ReadOnlyObservation::supported(true),
        continuing_output_exists: Tn10ReadOnlyObservation::supported(true),
        continuing_output_value_sompi: Tn10ReadOnlyObservation::supported(
            ENV064_CONTINUING_OUTPUT_VALUE_SOMPI,
        ),
        covenant_id: Tn10ReadOnlyObservation::supported(
            "e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7".to_string(),
        ),
    }
}

#[test]
fn online_verifier_refuses_mainnet_config() {
    let config = Tn10ReadOnlyConfig {
        network: "mainnet".to_string(),
        endpoint: None,
        mainnet_enabled: true,
    };

    let err = OnlineTn10Verifier::new(config).expect_err("mainnet must be rejected");
    assert_eq!(
        err,
        kaspa_foundation::transcript::online_verifier::OnlineVerificationError::MainnetDisabled
    );
}

#[test]
fn online_verifier_config_and_client_are_read_only() {
    let mut unsafe_client = safe_mock(passing_evidence());
    unsafe_client.safety.no_broadcast = false;

    let verifier = OnlineTn10Verifier::new(Tn10ReadOnlyConfig::public_tn10()).unwrap();
    let err = verifier
        .verify(
            &canonical_tn10_proof_transcript(),
            repo_root(),
            &unsafe_client,
        )
        .expect_err("unsafe client must be rejected");

    assert_eq!(
        err,
        kaspa_foundation::transcript::online_verifier::OnlineVerificationError::UnsafeClient
    );
    assert!(verifier.config().is_read_only_tn10());
}

#[test]
fn online_verifier_report_represents_pass_fail_and_skipped_checks() {
    let verifier = OnlineTn10Verifier::new(Tn10ReadOnlyConfig::public_tn10()).unwrap();

    let pass = verifier
        .verify(
            &canonical_tn10_proof_transcript(),
            repo_root(),
            &safe_mock(passing_evidence()),
        )
        .expect("supported matching evidence passes");
    assert!(pass.passed());
    assert_eq!(pass.failed_checks().count(), 0);

    let mut failed_evidence = passing_evidence();
    failed_evidence.continuing_output_value_sompi = Tn10ReadOnlyObservation::supported(1);
    let fail = verifier
        .verify(
            &canonical_tn10_proof_transcript(),
            repo_root(),
            &safe_mock(failed_evidence),
        )
        .expect("mismatching evidence produces report, not transport error");
    assert!(fail.failed());
    assert_eq!(fail.failed_checks().count(), 1);

    let mut skipped_evidence = passing_evidence();
    skipped_evidence.covenant_id = Tn10ReadOnlyObservation::not_yet_supported(
        "current public read-only adapter does not expose covenant id directly",
    );
    let ambiguous = verifier
        .verify(
            &canonical_tn10_proof_transcript(),
            repo_root(),
            &safe_mock(skipped_evidence),
        )
        .expect("unsupported observable is represented as ambiguous report");
    assert!(ambiguous.ambiguous());
    assert!(ambiguous
        .checks
        .iter()
        .any(|check| check.status == OnlineCheckStatus::NotYetSupported));
}

#[test]
fn canonical_transcript_can_be_prepared_for_tn10_online_verification() {
    let report = verify_canonical_tn10_transcript_online(
        repo_root(),
        &safe_mock(passing_evidence()),
        Tn10ReadOnlyConfig::public_tn10(),
    )
    .expect("canonical transcript can be checked with read-only TN10 evidence");

    assert!(report.offline_prerequisite_passed);
    assert!(report.read_only);
    assert_eq!(report.network, "testnet-10");
    assert!(report.passed());
}

#[test]
#[ignore = "optional live TN10 read-only integration test; requires explicit KASPA_FOUNDATION_RUN_LIVE_TN10=1"]
fn live_tn10_readonly_verification_is_optional_and_gated() {
    if std::env::var(RUN_LIVE_TN10_ENV).as_deref() != Ok("1") {
        eprintln!(
            "skipping live TN10 read-only check; set {RUN_LIVE_TN10_ENV}=1 and run this ignored test explicitly"
        );
        return;
    }

    let runtime = tokio::runtime::Runtime::new().expect("create tokio runtime for live TN10 read");
    runtime.block_on(async {
        let evidence = read_live_tn10_evidence()
            .await
            .expect("read public TN10 evidence through rusty-kaspa wRPC resolver defaults");
        let report = verify_canonical_tn10_transcript_online(
            repo_root(),
            &safe_mock(evidence),
            Tn10ReadOnlyConfig::public_tn10(),
        )
        .expect("live read-only TN10 evidence is verifier-compatible");

        println!("ENV-071C live TN10 transaction-detail verifier report");
        println!("result={:?}", report.result);
        println!("network={}", report.network);
        println!("endpoint={:?}", report.endpoint);
        println!("api_approach={}", report.api_approach);
        println!(
            "offline_prerequisite_passed={}",
            report.offline_prerequisite_passed
        );
        println!("read_only={}", report.read_only);
        println!("mainnet_enabled={}", report.mainnet_enabled);
        for check in &report.checks {
            println!(
                "check name={:?} status={:?} passed={:?} reason={:?}",
                check.name, check.status, check.passed, check.reason
            );
        }

        assert!(
            !report.failed(),
            "live read-only verifier had mismatches: {report:?}"
        );
    });
}

async fn read_live_tn10_evidence(
) -> Result<LiveTn10Evidence, Box<dyn std::error::Error + Send + Sync>> {
    use kaspa_addresses::Address;
    use kaspa_hashes::Hash;
    use kaspa_rpc_core::api::rpc::RpcApi;
    use kaspa_wrpc_client::{
        client::{ConnectOptions, ConnectStrategy},
        prelude::{NetworkId, NetworkType},
        KaspaRpcClient, Resolver, WrpcEncoding,
    };

    let client = KaspaRpcClient::new(
        WrpcEncoding::Borsh,
        None,
        Some(Resolver::default()),
        Some(NetworkId::with_suffix(NetworkType::Testnet, 10)),
        None,
    )?;
    let options = ConnectOptions {
        block_async_connect: true,
        connect_timeout: Some(Duration::from_millis(10_000)),
        strategy: ConnectStrategy::Fallback,
        ..Default::default()
    };
    client.connect(Some(options)).await?;
    println!("connected_endpoint={:?}", client.url());

    let server_info = client.get_server_info().await?;
    let spend_txid = Hash::from_str(ENV064_SPEND_TXID)?;
    let covenant_input_txid = Hash::from_str(ENV063_COVENANT_OUTPOINT_TXID)?;

    let mempool_entry = client.get_mempool_entry(spend_txid, false, false).await;
    println!(
        "mempool_entry_for_env064_observed={}",
        mempool_entry.is_ok()
    );
    if let Err(err) = &mempool_entry {
        println!("mempool_entry_for_env064_error={err}");
    }

    let covenant_address = Address::try_from(ENV063_ENV064_COVENANT_ADDRESS)?;
    let covenant_utxos = client
        .get_utxos_by_addresses(vec![covenant_address])
        .await?;
    println!("covenant_address_utxo_count={}", covenant_utxos.len());

    let continuing_utxo = covenant_utxos.iter().find(|entry| {
        entry.outpoint.transaction_id == spend_txid
            && entry.outpoint.index == ENV064_CONTINUING_OUTPUT_INDEX
    });
    let continuing_output_value = continuing_utxo.map(|entry| entry.utxo_entry.amount);
    println!("continuing_output_observed={}", continuing_utxo.is_some());
    println!("continuing_output_value_sompi={continuing_output_value:?}");

    let mempool_spends_env063 = mempool_entry.as_ref().ok().map(|entry| {
        entry.transaction.inputs.iter().any(|input| {
            input.previous_outpoint.transaction_id == covenant_input_txid
                && input.previous_outpoint.index == ENV063_COVENANT_OUTPOINT_INDEX
        })
    });
    let mempool_output0_value = mempool_entry
        .as_ref()
        .ok()
        .and_then(|entry| {
            entry
                .transaction
                .outputs
                .get(ENV064_CONTINUING_OUTPUT_INDEX as usize)
        })
        .map(|output| output.value);

    let tx_known = mempool_entry.is_ok() || continuing_utxo.is_some();
    let continuing_output_exists = continuing_utxo.is_some() || mempool_output0_value.is_some();
    let continuing_output_value = continuing_output_value.or(mempool_output0_value);

    let tx_detail = fetch_public_tn10_transaction_detail().await?;
    println!("transaction_detail_api_url={}", tx_detail.url);
    println!("transaction_detail_http_status={}", tx_detail.http_status);
    println!("transaction_detail_is_accepted={}", tx_detail.is_accepted);
    println!(
        "transaction_detail_accepting_block_hash={:?}",
        tx_detail.accepting_block_hash
    );
    println!(
        "transaction_detail_input_relationship_confirmed={}",
        tx_detail.input_relationship_confirmed
    );
    println!(
        "transaction_detail_output0_exists={}",
        tx_detail.output0_exists
    );
    println!(
        "transaction_detail_output0_value_sompi={:?}",
        tx_detail.output0_value_sompi
    );
    println!("transaction_detail_covenant_id={:?}", tx_detail.covenant_id);

    client.disconnect().await.ok();

    Ok(LiveTn10Evidence {
        network_id: Tn10ReadOnlyObservation::supported(server_info.network_id.to_string()),
        is_synced: Tn10ReadOnlyObservation::supported(server_info.is_synced),
        has_utxo_index: Tn10ReadOnlyObservation::supported(server_info.has_utxo_index),
        env064_spend_txid_known: Tn10ReadOnlyObservation::supported(
            tx_known && tx_detail.is_accepted,
        ),
        env063_input_spent_by_env064: Tn10ReadOnlyObservation::supported(
            tx_detail.input_relationship_confirmed || mempool_spends_env063.unwrap_or(false),
        ),
        continuing_output_exists: Tn10ReadOnlyObservation::supported(
            continuing_output_exists && tx_detail.output0_exists,
        ),
        continuing_output_value_sompi: match (
            tx_detail.output0_value_sompi,
            continuing_output_value,
        ) {
            (Some(transaction_detail_value), Some(utxo_value))
                if transaction_detail_value == utxo_value =>
            {
                Tn10ReadOnlyObservation::supported(transaction_detail_value)
            }
            (Some(transaction_detail_value), _) => {
                Tn10ReadOnlyObservation::supported(transaction_detail_value)
            }
            (None, Some(utxo_value)) => Tn10ReadOnlyObservation::supported(utxo_value),
            (None, None) => Tn10ReadOnlyObservation::skipped(
                "continuing output was not observable in UTXO set or mempool response",
            ),
        },
        covenant_id: match tx_detail.covenant_id {
            Some(covenant_id) => Tn10ReadOnlyObservation::supported(covenant_id),
            None => Tn10ReadOnlyObservation::not_yet_supported(
                "public TN10 transaction detail API did not expose Toccata covenant_id",
            ),
        },
    })
}

#[derive(Clone, Debug)]
struct PublicTn10TransactionDetail {
    url: String,
    http_status: reqwest::StatusCode,
    is_accepted: bool,
    accepting_block_hash: Option<String>,
    input_relationship_confirmed: bool,
    output0_exists: bool,
    output0_value_sompi: Option<u64>,
    covenant_id: Option<String>,
}

async fn fetch_public_tn10_transaction_detail(
) -> Result<PublicTn10TransactionDetail, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "{}/transactions/{}?inputs=true&outputs=true&resolve_previous_outpoints=light",
        PUBLIC_TN10_TRANSACTION_API_BASE, ENV064_SPEND_TXID
    );
    let response = reqwest::Client::new()
        .get(&url)
        .header(
            reqwest::header::USER_AGENT,
            "kaspa-foundation-env071c-readonly-verifier/0.1",
        )
        .send()
        .await?;
    let http_status = response.status();
    if !http_status.is_success() {
        return Err(format!("public TN10 transaction detail API returned {http_status}").into());
    }
    let value: serde_json::Value = response.json().await?;

    let txid_matches = value
        .get("transaction_id")
        .and_then(serde_json::Value::as_str)
        == Some(ENV064_SPEND_TXID);
    let is_accepted = value
        .get("is_accepted")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let accepting_block_hash = value
        .get("accepting_block_hash")
        .and_then(serde_json::Value::as_str)
        .map(str::to_string);

    let input_relationship_confirmed = value
        .get("inputs")
        .and_then(serde_json::Value::as_array)
        .map(|inputs| {
            inputs.iter().any(|input| {
                input
                    .get("previous_outpoint_hash")
                    .and_then(serde_json::Value::as_str)
                    == Some(ENV063_COVENANT_OUTPOINT_TXID)
                    && input.get("previous_outpoint_index").and_then(value_as_u64)
                        == Some(u64::from(ENV063_COVENANT_OUTPOINT_INDEX))
            })
        })
        .unwrap_or(false);

    let output0 = value
        .get("outputs")
        .and_then(serde_json::Value::as_array)
        .and_then(|outputs| {
            outputs.iter().find(|output| {
                output.get("index").and_then(value_as_u64)
                    == Some(u64::from(ENV064_CONTINUING_OUTPUT_INDEX))
            })
        });
    let output0_value_sompi = output0
        .and_then(|output| output.get("amount"))
        .and_then(value_as_u64);
    let covenant_id = output0
        .and_then(|output| output.get("covenant_id"))
        .and_then(serde_json::Value::as_str)
        .or_else(|| {
            value
                .get("inputs")
                .and_then(serde_json::Value::as_array)
                .and_then(|inputs| inputs.first())
                .and_then(|input| input.get("covenant_id"))
                .and_then(serde_json::Value::as_str)
        })
        .map(str::to_string);

    Ok(PublicTn10TransactionDetail {
        url,
        http_status,
        is_accepted: txid_matches && is_accepted && accepting_block_hash.is_some(),
        accepting_block_hash,
        input_relationship_confirmed,
        output0_exists: output0.is_some(),
        output0_value_sompi,
        covenant_id,
    })
}

fn value_as_u64(value: &serde_json::Value) -> Option<u64> {
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|string| string.parse().ok()))
}
