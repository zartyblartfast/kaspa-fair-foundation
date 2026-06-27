# kaspa-fair-cli

Developer-facing commands for the Kaspa Fair Foundation workspace.

## Live TN10 canonical verifier

Human-readable developer output:

```bash
cargo run -p kaspa-fair-cli -- verify-live-tn10-canonical
```

Stable app-facing JSON output:

```bash
cargo run -p kaspa-fair-cli -- verify-live-tn10-canonical --json
```

This command verifies the canonical ENV-063/064/065 proof transcript against public TN10 read-only chain data. It calls the `kaspa-foundation` online verifier library directly and prints either human-readable status or the stable `kaspa-fair-live-verification-result-v1` JSON contract. Both modes perform the same read-only verification and return success only when the final verifier result is `PASS`.

Expected JSON result shape:

```json
{
  "schema": "kaspa-fair-live-verification-result-v1",
  "network": "testnet-10",
  "mainnet_supported": false,
  "verifier_result": "PASS",
  "env064_spend_txid": "4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c",
  "env063_spent_outpoint": "2c7802ff9a6eec2828a96168d8f62a9a276176441ed8cb6086cd5d5d0cb26849:0",
  "continuing_output": "4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c:0",
  "continuing_output_value_sompi": 99700000,
  "covenant_id": "e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7",
  "accepted": true,
  "accepting_block_hash": "e0d62ead241a5217769266dc96e8055c5893c29074ed2c50ba23de1a9ba75190",
  "input_relationship_confirmed": true,
  "continuing_output_confirmed": true,
  "continuing_output_value_confirmed": true,
  "covenant_id_confirmed": true,
  "readonly": true,
  "signing_used": false,
  "transaction_created": false,
  "broadcast_used": false,
  "wallet_access_used": false,
  "api_endpoint_used": "https://api-tn10.kaspa.org/transactions/4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c?inputs=true&outputs=true&resolve_previous_outpoints=light",
  "wrpc_endpoint_observed": "..."
}
```

Exit behavior:

- `PASS`: exit 0
- `FAIL`: non-zero
- `AMBIGUOUS` / human `PARTIAL`: non-zero ambiguous/partial result

Safety boundary: read-only TN10 only; no signing; no transaction creation; no submitting/broadcasting; no wallet/private-key access; no secrets; no mainnet; no roulette implementation. Roulette remains future app adapter work above this foundation verifier contract.

A convenience wrapper is available from the repository root:

```bash
scripts/env072-live-tn10-verify.sh
```

## Pre-roulette readiness gate

Before the roulette PoC consumes the limited TN10 Toccata layer, run the
repository readiness command:

```bash
scripts/env074-toccata-layer-ready.sh
```

`TOCCATA_LAYER_READY=PASS` means the live TN10 verifier JSON contract passed all
readiness assertions and the limited read-only TN10 Toccata layer is ready to be
consumed by the roulette PoC. The gate remains foundation-only: no signing, no
transaction creation, no submitting/broadcasting, no wallet/private-key access,
no mainnet, no roulette implementation, and no secrets.

## Roulette PoC dry-run adapter skeleton

ENV-076 adds the first roulette adapter skeleton above the foundation verifier
contract.

Adapter JSON command:

```bash
cargo run -p kaspa-fair-cli -- roulette-poc-dry-run --json
```

Persistent dry-run wrapper:

```bash
scripts/env076-roulette-poc-dry-run.sh
```

The adapter:
- consumes the live TN10 verifier JSON contract
- requires `verifier_result = PASS`
- enforces the read-only/no-wallet/no-signing/no-broadcast/no-mainnet boundary
- uses mock bets only
- derives a deterministic European roulette result with domain-separated BLAKE3
  rejection sampling
- calculates deterministic mock settlement
- emits the stable `kaspa-fair-roulette-poc-round-v1` JSON contract

ENV-076 is not a web app, wallet integration, real betting flow, real payout
system, signing flow, broadcasting flow, mainnet flow, or production roulette.
