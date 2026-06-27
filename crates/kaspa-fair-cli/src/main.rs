use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::ExitCode;
use std::str::FromStr;
use std::time::Duration;

use kaspa_foundation::transcript::online_verifier::{
    verify_canonical_tn10_transcript_online, LiveTn10Evidence, OnlineVerificationReport,
    OnlineVerificationResult, ReadOnlyClientSafety, ReadOnlyTn10Client, Tn10ReadOnlyConfig,
    Tn10ReadOnlyObservation,
};

const COMMAND_VERIFY_LIVE_TN10_CANONICAL: &str = "verify-live-tn10-canonical";
const FLAG_JSON: &str = "--json";
const LIVE_VERIFICATION_SCHEMA_V1: &str = "kaspa-fair-live-verification-result-v1";
const ENV063_COVENANT_OUTPOINT_TXID: &str =
    "2c7802ff9a6eec2828a96168d8f62a9a276176441ed8cb6086cd5d5d0cb26849";
const ENV063_COVENANT_OUTPOINT_INDEX: u32 = 0;
const ENV064_SPEND_TXID: &str = "4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c";
const ENV064_CONTINUING_OUTPUT_INDEX: u32 = 0;
const ENV064_CONTINUING_OUTPUT_VALUE_SOMPI: u64 = 99_700_000;
const ENV063_ENV064_COVENANT_ID: &str =
    "e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7";
const ENV063_ENV064_COVENANT_ADDRESS: &str =
    "kaspatest:prn06m02u6aygczhzjuttgclymxlsrj7tdhy0ptye3u8gcdwl2grc0kk4fn7r";
const PUBLIC_TN10_TRANSACTION_API_BASE: &str = "https://api-tn10.kaspa.org";
const USER_AGENT: &str = "kaspa-fair-cli-env072-live-tn10-verifier/0.1";

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("ERROR: {err}");
            ExitCode::from(1)
        }
    }
}

fn run(args: Vec<String>) -> Result<ExitCode, Box<dyn Error + Send + Sync>> {
    match parse_command(&args) {
        CliCommand::Help => {
            print_help();
            Ok(ExitCode::SUCCESS)
        }
        CliCommand::VerifyLiveTn10Canonical { output } => run_verify_live_tn10_canonical(output),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CliCommand {
    Help,
    VerifyLiveTn10Canonical { output: OutputMode },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OutputMode {
    Human,
    Json,
}

fn parse_command(args: &[String]) -> CliCommand {
    match args.first().map(String::as_str) {
        None | Some("-h") | Some("--help") | Some("help") => CliCommand::Help,
        Some(COMMAND_VERIFY_LIVE_TN10_CANONICAL) => {
            if args.get(1).map(String::as_str) == Some(FLAG_JSON) {
                CliCommand::VerifyLiveTn10Canonical {
                    output: OutputMode::Json,
                }
            } else {
                CliCommand::VerifyLiveTn10Canonical {
                    output: OutputMode::Human,
                }
            }
        }
        Some(_) => CliCommand::Help,
    }
}

fn print_help() {
    println!("kaspa-fair-cli");
    println!();
    println!("Commands:");
    println!("  {COMMAND_VERIFY_LIVE_TN10_CANONICAL}");
    println!("      Run the canonical ENV-063/064/065 proof transcript verifier against live TN10 read-only data.");
    println!("  {COMMAND_VERIFY_LIVE_TN10_CANONICAL} {FLAG_JSON}");
    println!("      Emit the stable {LIVE_VERIFICATION_SCHEMA_V1} machine-readable contract.");
    println!();
    println!("Safety:");
    println!("  read-only TN10 only; no signing; no transaction creation; no submit/broadcast;");
    println!("  no wallet/private-key access; no secrets; no mainnet; no roulette implementation.");
}

fn run_verify_live_tn10_canonical(
    output: OutputMode,
) -> Result<ExitCode, Box<dyn Error + Send + Sync>> {
    let runtime = tokio::runtime::Runtime::new()?;
    let summary = runtime.block_on(read_and_verify_live_tn10_canonical())?;
    match output {
        OutputMode::Human => print_live_tn10_summary(&summary),
        OutputMode::Json => println!("{}", live_tn10_summary_json(&summary)),
    }

    Ok(match summary.report.result {
        OnlineVerificationResult::Pass => ExitCode::SUCCESS,
        OnlineVerificationResult::Fail => ExitCode::from(2),
        OnlineVerificationResult::Ambiguous => ExitCode::from(3),
    })
}

fn canonical_env063_spent_outpoint() -> String {
    format!("{ENV063_COVENANT_OUTPOINT_TXID}:{ENV063_COVENANT_OUTPOINT_INDEX}")
}

fn canonical_continuing_output() -> String {
    format!("{ENV064_SPEND_TXID}:{ENV064_CONTINUING_OUTPUT_INDEX}")
}

fn live_tn10_summary_json(summary: &LiveTn10VerifierSummary) -> serde_json::Value {
    let detail = &summary.readout.transaction_detail;
    let mut value = serde_json::json!({
        "schema": LIVE_VERIFICATION_SCHEMA_V1,
        "network": "testnet-10",
        "mainnet_supported": false,
        "verifier_result": json_result_label(summary.report.result),
        "env064_spend_txid": ENV064_SPEND_TXID,
        "env063_spent_outpoint": canonical_env063_spent_outpoint(),
        "continuing_output": canonical_continuing_output(),
        "continuing_output_value_sompi": ENV064_CONTINUING_OUTPUT_VALUE_SOMPI,
        "covenant_id": ENV063_ENV064_COVENANT_ID,
        "accepted": detail.is_accepted,
        "accepting_block_hash": detail.accepting_block_hash,
        "input_relationship_confirmed": detail.input_relationship_confirmed,
        "continuing_output_confirmed": detail.output0_exists,
        "continuing_output_value_confirmed": detail.output0_value_sompi == Some(ENV064_CONTINUING_OUTPUT_VALUE_SOMPI),
        "covenant_id_confirmed": detail.covenant_id.as_deref() == Some(ENV063_ENV064_COVENANT_ID),
        "readonly": true,
        "signing_used": false,
        "transaction_created": false,
        "broadcast_used": false,
        "wallet_access_used": false,
        "api_endpoint_used": detail.url,
    });

    if let Some(endpoint) = &summary.readout.connected_endpoint {
        value["wrpc_endpoint_observed"] = serde_json::Value::String(endpoint.clone());
    }

    value
}

async fn read_and_verify_live_tn10_canonical(
) -> Result<LiveTn10VerifierSummary, Box<dyn Error + Send + Sync>> {
    let readout = read_live_tn10_readout().await?;
    let client = ObservedReadOnlyClient {
        evidence: readout.evidence.clone(),
    };
    let report = verify_canonical_tn10_transcript_online(
        repo_root(),
        &client,
        Tn10ReadOnlyConfig::public_tn10(),
    )
    .map_err(|err| format!("online verifier rejected live TN10 evidence: {err:?}"))?;

    Ok(LiveTn10VerifierSummary { readout, report })
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[derive(Clone, Debug)]
struct ObservedReadOnlyClient {
    evidence: LiveTn10Evidence,
}

impl ReadOnlyTn10Client for ObservedReadOnlyClient {
    fn safety(&self) -> ReadOnlyClientSafety {
        ReadOnlyClientSafety::strict_read_only()
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

#[derive(Clone, Debug)]
struct LiveTn10VerifierSummary {
    readout: LiveTn10Readout,
    report: OnlineVerificationReport,
}

#[derive(Clone, Debug)]
struct LiveTn10Readout {
    connected_endpoint: Option<String>,
    server_network_id: String,
    server_is_synced: bool,
    server_has_utxo_index: bool,
    mempool_entry_observed: bool,
    covenant_address_utxo_count: usize,
    continuing_output_observed_in_utxo_set: bool,
    continuing_output_utxo_value_sompi: Option<u64>,
    transaction_detail: PublicTn10TransactionDetail,
    evidence: LiveTn10Evidence,
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

async fn read_live_tn10_readout() -> Result<LiveTn10Readout, Box<dyn Error + Send + Sync>> {
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

    let connected_endpoint = client.url().map(|url| url.to_string());
    let server_info = client.get_server_info().await?;
    let spend_txid = Hash::from_str(ENV064_SPEND_TXID)?;
    let covenant_input_txid = Hash::from_str(ENV063_COVENANT_OUTPOINT_TXID)?;

    let mempool_entry = client.get_mempool_entry(spend_txid, false, false).await;
    let covenant_address = Address::try_from(ENV063_ENV064_COVENANT_ADDRESS)?;
    let covenant_utxos = client
        .get_utxos_by_addresses(vec![covenant_address])
        .await?;

    let continuing_utxo = covenant_utxos.iter().find(|entry| {
        entry.outpoint.transaction_id == spend_txid
            && entry.outpoint.index == ENV064_CONTINUING_OUTPUT_INDEX
    });
    let continuing_output_utxo_value_sompi = continuing_utxo.map(|entry| entry.utxo_entry.amount);

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

    let transaction_detail = fetch_public_tn10_transaction_detail().await?;
    client.disconnect().await.ok();

    let tx_known = mempool_entry.is_ok() || continuing_utxo.is_some();
    let continuing_output_exists = continuing_utxo.is_some() || mempool_output0_value.is_some();
    let continuing_output_value = continuing_output_utxo_value_sompi.or(mempool_output0_value);

    let evidence = LiveTn10Evidence {
        network_id: Tn10ReadOnlyObservation::supported(server_info.network_id.to_string()),
        is_synced: Tn10ReadOnlyObservation::supported(server_info.is_synced),
        has_utxo_index: Tn10ReadOnlyObservation::supported(server_info.has_utxo_index),
        env064_spend_txid_known: Tn10ReadOnlyObservation::supported(
            tx_known && transaction_detail.is_accepted,
        ),
        env063_input_spent_by_env064: Tn10ReadOnlyObservation::supported(
            transaction_detail.input_relationship_confirmed
                || mempool_spends_env063.unwrap_or(false),
        ),
        continuing_output_exists: Tn10ReadOnlyObservation::supported(
            continuing_output_exists && transaction_detail.output0_exists,
        ),
        continuing_output_value_sompi: match (
            transaction_detail.output0_value_sompi,
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
        covenant_id: match transaction_detail.covenant_id.clone() {
            Some(covenant_id) => Tn10ReadOnlyObservation::supported(covenant_id),
            None => Tn10ReadOnlyObservation::not_yet_supported(
                "public TN10 transaction detail API did not expose Toccata covenant_id",
            ),
        },
    };

    Ok(LiveTn10Readout {
        connected_endpoint,
        server_network_id: server_info.network_id.to_string(),
        server_is_synced: server_info.is_synced,
        server_has_utxo_index: server_info.has_utxo_index,
        mempool_entry_observed: mempool_entry.is_ok(),
        covenant_address_utxo_count: covenant_utxos.len(),
        continuing_output_observed_in_utxo_set: continuing_utxo.is_some(),
        continuing_output_utxo_value_sompi,
        transaction_detail,
        evidence,
    })
}

async fn fetch_public_tn10_transaction_detail(
) -> Result<PublicTn10TransactionDetail, Box<dyn Error + Send + Sync>> {
    let url = format!(
        "{}/transactions/{}?inputs=true&outputs=true&resolve_previous_outpoints=light",
        PUBLIC_TN10_TRANSACTION_API_BASE, ENV064_SPEND_TXID
    );
    let response = reqwest::Client::new()
        .get(&url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
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

fn print_live_tn10_summary(summary: &LiveTn10VerifierSummary) {
    let detail = &summary.readout.transaction_detail;
    println!("ENV-072 live TN10 canonical verifier");
    println!("result={}", result_label(summary.report.result));
    println!("network={}", summary.report.network);
    println!(
        "wrpc_endpoint={}",
        summary
            .readout
            .connected_endpoint
            .as_deref()
            .unwrap_or("unavailable")
    );
    println!("transaction_detail_api_url={}", detail.url);
    println!("transaction_detail_http_status={}", detail.http_status);
    println!("api_approach={}", summary.report.api_approach);
    println!(
        "offline_prerequisite_passed={}",
        summary.report.offline_prerequisite_passed
    );
    println!("read_only={}", summary.report.read_only);
    println!("mainnet_enabled={}", summary.report.mainnet_enabled);
    println!("server_network_id={}", summary.readout.server_network_id);
    println!("server_is_synced={}", summary.readout.server_is_synced);
    println!(
        "server_has_utxo_index={}",
        summary.readout.server_has_utxo_index
    );
    println!(
        "mempool_entry_for_env064_observed={}",
        summary.readout.mempool_entry_observed
    );
    println!(
        "covenant_address_utxo_count={}",
        summary.readout.covenant_address_utxo_count
    );
    println!(
        "continuing_output_observed_in_utxo_set={}",
        summary.readout.continuing_output_observed_in_utxo_set
    );
    println!(
        "continuing_output_utxo_value_sompi={:?}",
        summary.readout.continuing_output_utxo_value_sompi
    );
    println!("ENV-064 accepted={}", detail.is_accepted);
    println!(
        "accepting_block_hash={}",
        detail
            .accepting_block_hash
            .as_deref()
            .unwrap_or("unavailable")
    );
    println!(
        "ENV-063 input relationship confirmed={}",
        detail.input_relationship_confirmed
    );
    println!("continuing_output_exists={}", detail.output0_exists);
    println!(
        "continuing_output_value_sompi={}",
        detail
            .output0_value_sompi
            .map(|value| value.to_string())
            .unwrap_or_else(|| "unavailable".to_string())
    );
    println!(
        "covenant_id_confirmed={}",
        detail.covenant_id.as_deref().unwrap_or("unavailable")
    );
    println!(
        "final_verifier_result={}",
        result_label(summary.report.result)
    );
    for check in &summary.report.checks {
        println!(
            "check name={:?} status={:?} passed={:?} reason={:?}",
            check.name, check.status, check.passed, check.reason
        );
    }
}

fn result_label(result: OnlineVerificationResult) -> &'static str {
    match result {
        OnlineVerificationResult::Pass => "PASS",
        OnlineVerificationResult::Fail => "FAIL",
        OnlineVerificationResult::Ambiguous => "PARTIAL",
    }
}

fn json_result_label(result: OnlineVerificationResult) -> &'static str {
    match result {
        OnlineVerificationResult::Pass => "PASS",
        OnlineVerificationResult::Fail => "FAIL",
        OnlineVerificationResult::Ambiguous => "AMBIGUOUS",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_parser_selects_live_verifier_command() {
        assert_eq!(
            parse_command(&[COMMAND_VERIFY_LIVE_TN10_CANONICAL.to_string()]),
            CliCommand::VerifyLiveTn10Canonical {
                output: OutputMode::Human
            }
        );
        assert_eq!(
            parse_command(&[
                COMMAND_VERIFY_LIVE_TN10_CANONICAL.to_string(),
                FLAG_JSON.to_string()
            ]),
            CliCommand::VerifyLiveTn10Canonical {
                output: OutputMode::Json
            }
        );
        assert_eq!(parse_command(&[]), CliCommand::Help);
        assert_eq!(parse_command(&["unknown".to_string()]), CliCommand::Help);
    }

    #[test]
    fn result_labels_match_cli_contract() {
        assert_eq!(result_label(OnlineVerificationResult::Pass), "PASS");
        assert_eq!(result_label(OnlineVerificationResult::Fail), "FAIL");
        assert_eq!(result_label(OnlineVerificationResult::Ambiguous), "PARTIAL");
    }

    #[test]
    fn json_result_labels_match_app_contract() {
        assert_eq!(json_result_label(OnlineVerificationResult::Pass), "PASS");
        assert_eq!(json_result_label(OnlineVerificationResult::Fail), "FAIL");
        assert_eq!(
            json_result_label(OnlineVerificationResult::Ambiguous),
            "AMBIGUOUS"
        );
    }

    #[test]
    fn live_verifier_json_schema_and_safety_fields_are_stable() {
        let summary = passing_summary_for_tests();
        let value = live_tn10_summary_json(&summary);

        assert_eq!(value["schema"], LIVE_VERIFICATION_SCHEMA_V1);
        assert_eq!(value["network"], "testnet-10");
        assert_eq!(value["mainnet_supported"], false);
        assert_eq!(value["verifier_result"], "PASS");
        assert_eq!(value["env064_spend_txid"], ENV064_SPEND_TXID);
        assert_eq!(
            value["env063_spent_outpoint"],
            canonical_env063_spent_outpoint()
        );
        assert_eq!(value["continuing_output"], canonical_continuing_output());
        assert_eq!(
            value["continuing_output_value_sompi"],
            ENV064_CONTINUING_OUTPUT_VALUE_SOMPI
        );
        assert_eq!(value["covenant_id"], ENV063_ENV064_COVENANT_ID);
        assert_eq!(value["accepted"], true);
        assert_eq!(value["input_relationship_confirmed"], true);
        assert_eq!(value["continuing_output_confirmed"], true);
        assert_eq!(value["continuing_output_value_confirmed"], true);
        assert_eq!(value["covenant_id_confirmed"], true);
        assert_eq!(value["readonly"], true);
        assert_eq!(value["signing_used"], false);
        assert_eq!(value["transaction_created"], false);
        assert_eq!(value["broadcast_used"], false);
        assert_eq!(value["wallet_access_used"], false);
        assert_eq!(value["api_endpoint_used"], passing_transaction_api_url());
        assert_eq!(value["wrpc_endpoint_observed"], "wss://example.testnet-10");
    }

    fn passing_summary_for_tests() -> LiveTn10VerifierSummary {
        LiveTn10VerifierSummary {
            readout: LiveTn10Readout {
                connected_endpoint: Some("wss://example.testnet-10".to_string()),
                server_network_id: "testnet-10".to_string(),
                server_is_synced: true,
                server_has_utxo_index: true,
                mempool_entry_observed: false,
                covenant_address_utxo_count: 1,
                continuing_output_observed_in_utxo_set: true,
                continuing_output_utxo_value_sompi: Some(ENV064_CONTINUING_OUTPUT_VALUE_SOMPI),
                transaction_detail: PublicTn10TransactionDetail {
                    url: passing_transaction_api_url(),
                    http_status: reqwest::StatusCode::OK,
                    is_accepted: true,
                    accepting_block_hash: Some(
                        "e0d62ead241a5217769266dc96e8055c5893c29074ed2c50ba23de1a9ba75190"
                            .to_string(),
                    ),
                    input_relationship_confirmed: true,
                    output0_exists: true,
                    output0_value_sompi: Some(ENV064_CONTINUING_OUTPUT_VALUE_SOMPI),
                    covenant_id: Some(ENV063_ENV064_COVENANT_ID.to_string()),
                },
                evidence: LiveTn10Evidence {
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
                        ENV063_ENV064_COVENANT_ID.to_string(),
                    ),
                },
            },
            report: OnlineVerificationReport {
                transcript_id: "canonical-tn10-covenant-path",
                network: "testnet-10".to_string(),
                endpoint: None,
                api_approach: "read-only test",
                offline_prerequisite_passed: true,
                read_only: true,
                mainnet_enabled: false,
                result: OnlineVerificationResult::Pass,
                checks: vec![],
            },
        }
    }

    fn passing_transaction_api_url() -> String {
        format!(
            "{PUBLIC_TN10_TRANSACTION_API_BASE}/transactions/{ENV064_SPEND_TXID}?inputs=true&outputs=true&resolve_previous_outpoints=light"
        )
    }
}
