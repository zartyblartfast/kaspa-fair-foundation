# Proof Transcript Format

Status: ENV-069 foundation transcript baseline
Scope: offline/read-only foundation layer only
Target result: stable versioned transcript schema for the canonical TN10 covenant evidence path

## 1. Purpose

The proof transcript format turns the already-proven corrected TN10 covenant path into a stable, app-agnostic artifact that later tools can verify without re-running live workflow steps.

For the foundation repository, a proof transcript exists to:

- preserve the canonical evidence trail from ENV-063, ENV-064, and ENV-065
- expose a stable schema that later offline and online verifiers can share
- keep covenant proof data separate from roulette or any future app adapter
- keep live submit/signing concerns outside the transcript layer

The transcript is a foundation-level description of what was proven, where the evidence lives, and which canonical values later verifiers must confirm.

## 2. Scope

ENV-069 is intentionally modest.

In scope:

- versioned transcript metadata
- versioned evidence-bundle metadata
- ordered ENV steps for the canonical TN10 path
- canonical txid / outpoint / continuing output / value / covenant id fields
- fixture path links into committed evidence directories
- explicit safety boundary fields for offline verification work

Out of scope:

- live TN10 activity
- RPC/network requirements
- signing or wallet access
- transaction submission or rebroadcast
- mainnet support
- roulette implementation
- app-specific fairness objects beyond future adapter layers

## 3. Evidence Bundle Schema vs Proof Transcript Schema

The repository should treat evidence bundles and proof transcripts as related but separate schemas.

### 3.1 Evidence bundle schema

The evidence bundle schema names the raw committed evidence set:

```text
kaspa-fair-evidence-v1
```

Examples of evidence-bundle contents:

- ENV summaries
- preflight notes
- submit outputs
- postcheck outputs
- read-only confirmation outputs
- final summary indexes

The evidence bundle is source material.

### 3.2 Proof transcript schema

The proof transcript schema names the stable interpreted layer built on top of evidence bundles:

```text
kaspa-fair-transcript-v1
```

The transcript does not replace evidence bundles. It points at them, orders them, and extracts the canonical facts later verifiers must check.

In short:

```text
evidence bundle = raw committed evidence
proof transcript = stable, verifier-oriented interpretation of that evidence
```

## 4. Canonical TN10 transcript instance

ENV-069 defines the first canonical foundation transcript around the corrected TN10 covenant path proven earlier.

Network:

```text
TN10/testnet-10 only
mainnet_supported = false
```

Canonical values:

- ENV-064 spend txid:
  `4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c`
- ENV-063 covenant input spent by ENV-064:
  `2c7802ff9a6eec2828a96168d8f62a9a276176441ed8cb6086cd5d5d0cb26849:0`
- continuing output:
  `4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c:0`
- continuing output value:
  `99700000 sompi`
- covenant id:
  `e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7`

## 5. Canonical ENV step ordering

The canonical transcript order is fixed:

```text
ENV-063 create
-> ENV-064 spend
-> ENV-065 confirmation
```

Step-by-step meaning:

1. `ENV-063`
   - role: canonical create evidence
   - character of original action: live TN10 create
   - transcript use: evidence-only historical reference
   - fixture path:
     `fixtures/tn10-canonical-covenant-path/env-063-corrected-live-covenant-create/`

2. `ENV-064`
   - role: canonical spend evidence
   - character of original action: live TN10 spend
   - transcript use: evidence-only historical reference
   - fixture path:
     `fixtures/tn10-canonical-covenant-path/env-064-live-corrected-covenant-spend/`

3. `ENV-065`
   - role: canonical read-only confirmation evidence
   - character of original action: read-only settlement confirmation
   - transcript use: evidence-only historical reference
   - fixture path:
     `fixtures/tn10-canonical-covenant-path/env-065-readonly-env064-spend-confirmation/`

The transcript itself is offline evidence. It records that ENV-063 and ENV-064 were historically live steps, but using the transcript later must not require replaying them.

## 6. Foundation transcript shape

At the Rust layer, the foundation transcript should remain simple and strongly typed.

Recommended top-level fields:

- transcript schema version
- evidence schema version
- transcript id
- network
- mainnet supported flag
- verifier direction flags
- app-agnostic / roulette flags
- safety boundary flags
- canonical proven TN10 values
- ordered transcript steps

Recommended step fields:

- ENV id
- role/purpose
- fixture path
- mode of original step (`live`, `read-only`, `offline`, or `evidence-only`)
- whether the transcript is using that step only as historical evidence
- expected txid / input outpoint / continuing output / value / covenant id where relevant

This is enough for foundation verifiers without over-engineering application logic into the crate.

## 7. Safety boundary

The canonical ENV-069 transcript must explicitly state that later offline transcript work requires:

- no secrets
- no wallet
- no signing
- no network
- no broadcast
- no mainnet

At the schema level, these appear as explicit positive safety markers:

- `requires_no_secrets = true`
- `requires_no_wallet = true`
- `requires_no_signing = true`
- `requires_no_network = true`
- `requires_no_broadcast = true`
- `mainnet_supported = false`

This is a hard separation boundary between:

```text
foundation transcript and verifier work
vs
future live submit / app adapter / demo workflows
```

## 8. Verifier direction

Verifier priority should be:

1. offline verifier first as a deterministic scaffold and safety prerequisite
2. online TN10 read-only verifier for actual operational settlement checks

Meaning:

- first, build tooling that can read committed evidence and the proof transcript without requiring a node or wallet
- then add online/read-only helpers that compare transcript claims against current TN10 state
- submit/broadcast remains out of scope for the transcript layer

The transcript should therefore be usable by a deterministic offline verifier before any richer network-connected verifier runs. The offline verifier is not the end goal; it is a sanity layer before read-only online verification proves whether the canonical evidence bundle still matches live TN10 chain state.

## 9. Application direction

The foundation layer is app-agnostic.

That means:

- the transcript names covenant facts, not roulette round logic
- the transcript can later support multiple adapters
- roulette remains future adapter/demo work only

Roulette-specific fairness data can be layered on top later, but it must not leak backward into the foundation transcript baseline.

## 10. Minimal stable JSON example

The ENV-069 sample JSON artifact should stay readable and stable. A canonical shape is:

```json
{
  "transcript_schema_version": "kaspa-fair-transcript-v1",
  "evidence_schema_version": "kaspa-fair-evidence-v1",
  "transcript_id": "canonical-tn10-covenant-path",
  "network": "TN10/testnet-10",
  "mainnet_supported": false,
  "offline_verifier_first": true,
  "online_verifier_later": true,
  "app_agnostic_foundation_layer": true,
  "includes_roulette_adapter": false,
  "safety": {
    "requires_no_secrets": true,
    "requires_no_wallet": true,
    "requires_no_signing": true,
    "requires_no_network": true,
    "requires_no_broadcast": true,
    "mainnet_supported": false
  },
  "canonical": {
    "env064_spend_txid": "4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c",
    "env063_input_outpoint": "2c7802ff9a6eec2828a96168d8f62a9a276176441ed8cb6086cd5d5d0cb26849:0",
    "continuing_output": "4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c:0",
    "continuing_output_value_sompi": 99700000,
    "covenant_id": "e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7"
  },
  "steps": [
    {
      "env_id": "ENV-063",
      "purpose": "covenant-create",
      "fixture_path": "fixtures/tn10-canonical-covenant-path/env-063-corrected-live-covenant-create/"
    },
    {
      "env_id": "ENV-064",
      "purpose": "covenant-spend",
      "fixture_path": "fixtures/tn10-canonical-covenant-path/env-064-live-corrected-covenant-spend/"
    },
    {
      "env_id": "ENV-065",
      "purpose": "read-only-confirmation",
      "fixture_path": "fixtures/tn10-canonical-covenant-path/env-065-readonly-env064-spend-confirmation/"
    }
  ]
}
```

## 11. ENV-069 result

ENV-069 should leave the foundation repository with:

- a stable transcript schema identifier: `kaspa-fair-transcript-v1`
- a stable evidence schema identifier: `kaspa-fair-evidence-v1`
- a transcript module in `crates/kaspa-foundation/src/transcript/`
- fixture-backed tests covering the canonical TN10 proof path
- a committed sample transcript artifact under `spikes/kaspa-foundation/artifacts/env-069-proof-transcript-format/`

That gives later work a clean base for offline verification first, then online verification later, while keeping submit/broadcast and roulette logic out of the foundation transcript layer.

## 12. ENV-070 offline proof transcript verifier

ENV-070 adds the first foundation offline verifier for the canonical TN10 proof transcript. The verifier is deliberately modest: it validates the transcript model and the committed fixture-path links without re-running live Kaspa workflow steps.

The offline verifier checks:

- transcript schema version is `kaspa-fair-transcript-v1`
- evidence schema version is `kaspa-fair-evidence-v1`
- network is `TN10/testnet-10`
- `mainnet_supported = false`
- ENV sequence is exactly `ENV-063 -> ENV-064 -> ENV-065`
- canonical ENV-064 spend txid, ENV-063 input outpoint, continuing output, continuing output value, and covenant id match the proven TN10 values
- transcript step expectations agree with the same canonical values
- referenced fixture paths exist relative to the repository root
- safety boundary requires no secrets, no wallet, no signing, no network, no broadcast, and no mainnet
- transcript remains app-agnostic and does not include a roulette adapter

The verifier intentionally does not check every Kaspa consensus rule. It does not execute scripts, build or sign transactions, query node state, fetch UTXOs, submit transactions, or prove that current TN10 state still matches the historical fixture evidence. It is a deterministic local consistency checker over the foundation transcript and committed fixture tree.

The verifier does not contact TN10 or mainnet. It requires no RPC endpoint, no network access, no wallet files, no helper keys, and no secrets. Submit, sign, broadcast, and transaction creation remain out of scope for the transcript layer.

## 13. ENV-071 online TN10 read-only proof transcript verifier

ENV-071 adds the first online verifier scaffold for the canonical ENV-063/064/065 transcript. It keeps the ENV-070 offline verifier as a prerequisite, then compares normalized read-only TN10 observations against the canonical transcript values.

The online verifier is the operational direction for a limited Toccata layer on TN10. It is intended to answer whether a transcript/evidence bundle matches live chain state without trusting the app server and without replaying historical live actions.

The ENV-071 verifier can represent each live-chain check as:

- supported
- not yet supported, with a reason
- skipped, with a reason

It must not fake full verification when a public API or adapter cannot expose a field yet.

The configured public TN10 approach is the one already used by the reference spike: rusty-kaspa wRPC via `kaspa_wrpc_client::KaspaRpcClient` with `WrpcEncoding::Borsh`, `Resolver::default()`, `NetworkId::with_suffix(NetworkType::Testnet, 10)`, and `ConnectStrategy::Fallback`. ENV-071B exercised that path from an ignored, explicitly gated live integration test while the default verifier path remained deterministic/offline. ENV-071C adds a second read-only transaction-detail source, `https://api-tn10.kaspa.org/transactions/<txid>?inputs=true&outputs=true&resolve_previous_outpoints=light`, to retrieve accepted mined transaction structure by txid. The live adapter imports only read-only calls; it does not import signing, wallet, transaction creation, or submit paths into the default verifier flow.

ENV-071C live status: public rusty-kaspa wRPC confirmed TN10 node readiness and continuing UTXO visibility, and the public TN10 transaction-detail API confirmed ENV-064 is accepted, has accepting block `e0d62ead241a5217769266dc96e8055c5893c29074ed2c50ba23de1a9ba75190`, spends ENV-063 outpoint `2c7802ff9a6eec2828a96168d8f62a9a276176441ed8cb6086cd5d5d0cb26849:0`, creates output `4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c:0` with value `99700000` sompi, and exposes covenant id `e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7`. With these fields supported, the live online verifier result is `Pass`.

The verifier is limited to TN10/testnet-10. Mainnet portability remains future work and is disabled until explicit approval.

Safety boundary for ENV-071:

- read-only TN10 queries are allowed
- no signing
- no transaction creation
- no submitting or broadcasting
- no wallet or private-key access
- no secrets
- no mainnet
- no roulette implementation

The target live checks are:

- server/network is TN10/testnet-10
- node sync and UTXO-index readiness where available
- ENV-064 spend txid is known or accepted/visible where available
- ENV-063 covenant input outpoint is spent by ENV-064 where available
- ENV-064 continuing output exists
- continuing output value is `99700000` sompi
- covenant id is `e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7` where available

Roulette remains future app adapter work layered above the foundation verifier.

## 14. ENV-072 developer-facing live TN10 verifier command

ENV-072 promotes the ENV-071 live verifier from ignored-test scaffolding into an operational developer command:

```bash
cargo run -p kaspa-fair-cli -- verify-live-tn10-canonical
```

The command calls the foundation online verifier library directly. It performs only read-only TN10 queries, then prints a clear `PASS`, `PARTIAL`, or `FAIL` result with the wRPC endpoint, transaction-detail API URL, ENV-064 acceptance status, accepting block hash, ENV-063 input relationship, continuing output existence/value, covenant id, and final verifier result.

Exit behavior:

- `PASS`: exit 0
- `FAIL`: non-zero
- `PARTIAL`: non-zero ambiguous/partial result

Normal cargo tests remain deterministic and do not require TN10 availability. The offline verifier is a prerequisite/regression safety layer, not the main operational goal of ENV-072. The live CLI command is the explicit path for developers to confirm that the canonical transcript still matches public TN10 chain data.

ENV-072 does not add signing, transaction creation, submission/broadcast, wallet/private-key access, secrets, mainnet support, or roulette logic. Mainnet remains disabled/not supported; roulette remains future app adapter work above the limited Toccata foundation verifier.