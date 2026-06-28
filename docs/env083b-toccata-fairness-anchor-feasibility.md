# ENV-083B — TN10 Toccata fairness-anchor feasibility and toolchain gate

## Result

ENV-083B is a feasibility/design gate only. It does not implement roulette randomisation, covenant transactions, transaction creation, signing, broadcasting, wallet access, real betting, payouts, custody, UI changes, or mainnet support.

Gate result: PASS for feasibility/toolchain completion.

Hard-gate claim result: Toccata covenant fairness anchor is allowed as the next design claim because a concrete read-only TN10 evidence path is available for the canonical covenant lineage fields already proven in this repository:

- transaction output covenant binding: `outputs[0].covenant_id` and `outputs[0].covenant_authorizing_input`
- transaction input covenant lineage evidence: `inputs[0].covenant_id`
- accepted transaction evidence: `is_accepted`, `accepting_block_hash`, and input previous outpoint fields
- public TN10 API evidence: `https://api-tn10.kaspa.org/transactions/<txid>?inputs=true&outputs=true&resolve_previous_outpoints=light`
- Rust verifier evidence path: `kaspa-fair-cli verify-live-tn10-canonical --json`
- wRPC evidence transport for node readiness and UTXO visibility: public rusty-kaspa wRPC on testnet-10 with Borsh encoding

This is not a live roulette anchor implementation. It is a feasibility gate proving that the required covenant-id / lineage evidence can be observed read-only on TN10 for the canonical covenant path and can be used as the evidence pattern for the next offline covenant-state model.

## Binding source documents inspected

Primary project guidance:

- `docs/toccata-fairness-anchor-architecture.md`
- `docs/threat-model.md`

Official upstream sources fetched and inspected:

- rusty-kaspa Toccata guide: `https://raw.githubusercontent.com/kaspanet/rusty-kaspa/master/docs/toccata-guide.md`
- KIP-16: `https://raw.githubusercontent.com/kaspanet/kips/master/kip-0016.md`
- KIP-17: `https://raw.githubusercontent.com/kaspanet/kips/master/kip-0017.md`
- KIP-20: `https://raw.githubusercontent.com/kaspanet/kips/master/kip-0020.md`
- KIP-21: `https://raw.githubusercontent.com/kaspanet/kips/master/kip-0021.md`
- SilverScript README: `https://raw.githubusercontent.com/kaspanet/silverscript/master/README.md`

Local repo evidence inspected:

- `crates/kaspa-foundation/src/transcript/online_verifier.rs`
- `crates/kaspa-fair-cli/src/main.rs`
- `crates/kaspa-foundation/src/covenant/constants.rs`
- `crates/kaspa-foundation/src/covenant/recipe.rs`
- `docs/kaspa-foundation-architecture.md`
- `docs/proof-transcript-format.md`
- existing ENV-071/072/074 verifier paths and readiness scripts

## Core thesis preserved

- Tier 1: Kaspa beats a trusted operator/private database because a public PoW DAG anchor is independently inspectable.
- Tier 2: Toccata covenant enforcement/lineage adds more than a bare anchored hash by making covenant-labelled state membership consensus-tracked and by exposing covenant bindings/IDs for verifier evidence.
- JSON is a mirror/export format, not the proof source.
- Rust is the truth layer.
- RPC/API is evidence transport.
- SilverScript is optional until proven compatible.
- The UI is not trusted.
- No wallet/signing/broadcast/mainnet is allowed without explicit authorization.

## 1. Strongest immediate fairness-anchor candidate

### Recommendation: KIP-20 covenant IDs / covenant lineage, backed by KIP-17 transition modelling

KIP-20 is the strongest immediate fairness-anchor candidate because it introduces consensus-tracked covenant identifiers carried by UTXOs and declared by transaction outputs. For fairness anchoring, this gives the verifier a stable covenant lineage reference that a JSON proof cannot forge by itself and that a bare payload hash cannot enforce.

KIP-17 is required as the covenant transition mechanism: transaction introspection and payload/output inspection can model or enforce commitment-to-reveal state transitions. But KIP-17 alone leaves lineage identity more dependent on script logic and witness structure. KIP-20 turns the lineage/member identity into a first-class consensus field.

KIP-21 and KIP-16 are useful later, not first:

- KIP-21 sequencing commitments/lane proofs may later strengthen ordering/inclusion proofs, especially for proving commitment-before-reveal activity in an app lane.
- KIP-16 ZK precompile may later compress or verify complex off-chain computation, but it is too large and unnecessary for the first fairness-anchor milestone.

## 2. What Toccata adds beyond a bare anchored hash

A bare anchored hash can prove that some bytes existed at or before a chain point. It cannot, by itself:

- prove that later reveal evidence belongs to the same covenant/application lineage as the earlier commitment;
- prevent arbitrary unrelated JSON files from claiming to represent a round;
- enforce or model a state transition in the UTXO/script layer;
- expose consensus-level covenant membership for verifier checks;
- prove that a continuing output preserves the same covenant identity;
- provide native covenant-introspection hooks for scripts to inspect input/output covenant membership.

Toccata adds the following specific evidence and constraints:

- KIP-20 output covenant binding: an output may declare `covenant_id` and `authorizing_input`.
- KIP-20 UTXO covenant ID: accepted covenant-bound UTXOs carry `covenant_id`.
- KIP-20 non-forgeability: covenant-labelled UTXOs are valid only as authorized continuations or genesis initializations.
- KIP-20 lineage: commitment and reveal states can be linked to one covenant instance rather than merely to unrelated payload hashes.
- KIP-17 introspection: scripts can inspect transaction fields, inputs, outputs, payload substrings, and covenant-related fields to constrain valid transitions.

For the PoC, the useful claim is therefore:

> The proof is not just a hash in a payload. The commitment/reveal evidence is linked to a TN10 Toccata covenant lineage with observable covenant IDs and output/input covenant bindings, and the Rust verifier checks that the app-facing JSON mirror matches that evidence.

## 3. Hard gate: covenant lineage / covenant IDs read-only on TN10

### Gate answer

`kip20_covenant_lineage_readonly_tn10_verified`: true

The available tooling can verify covenant lineage/covenant IDs read-only on TN10 for the canonical covenant path already present in the repository.

Evidence path:

1. Official Toccata guide states Toccata RPC/protobuf changes include `RpcTransactionOutput.covenant` and `RpcUtxoEntry.covenant_id`.
2. KIP-20 specifies transaction output covenant binding and UTXO entry covenant IDs.
3. The local Rust verifier already models `LiveTn10Evidence.covenant_id` and checks it against the canonical transcript.
4. The local CLI public TN10 transaction-detail adapter extracts `outputs[0].covenant_id`, `outputs[0].covenant_authorizing_input`, and fallback `inputs[0].covenant_id` from the public transaction-detail API.
5. A live read-only command returned `verifier_result: PASS`, `readonly: true`, `transaction_created: false`, `signing_used: false`, `broadcast_used: false`, `wallet_access_used: false`, and `covenant_id_confirmed: true`.
6. Direct public TN10 transaction-detail API evidence exposes `inputs[0].covenant_id`, `outputs[0].covenant_authorizing_input`, and `outputs[0].covenant_id` for the canonical ENV-064 transaction.

### Public TN10 RPC/API sufficiency

`public_tn10_rpc_sufficient`: true for the current canonical read-only covenant lineage evidence path.

The current project uses two public read-only transports:

- public rusty-kaspa wRPC for server readiness, network, sync status, UTXO-index status, and UTXO visibility;
- public TN10 transaction-detail REST API for accepted transaction structure and covenant ID fields.

This combination is sufficient for the ENV-083B hard gate. A future per-round covenant anchoring implementation must re-check whether the same public endpoints expose the required fields for the new round transactions.

### Local TN10 `--utxoindex` requirement

`local_tn10_utxoindex_required`: false for this gate; unknown/conditional for future custom per-round covenant workflows.

Public wRPC reports UTXO-index status and public transaction-detail API exposes covenant IDs for the canonical evidence. A local `kaspad --utxoindex --testnet` remains a fallback if public endpoints stop exposing covenant fields, if historical indexing is needed beyond public API retention, or if future verifier checks need richer raw RPC/protobuf fields than the public API provides.

## 4. Tool decision matrix

| Tool/layer | Role now | Role later | Do not use for |
|---|---|---|---|
| Rust | Truth layer. Covenant-state model, commitment/reveal model, deterministic result derivation, verifier checks, JSON mirror validation, tests. | Offline covenant-state model in ENV-083C; future transaction construction modelling only if authorized. | UI-generated proof truth; wallet/signing/broadcast in this ENV. |
| RPC / gRPC / node API | Read-only evidence transport. Fetch TN10 readiness, UTXO visibility, accepted transaction detail, output covenant fields, input/UTXO covenant IDs. | Lane proof / sequencing evidence if KIP-21 APIs are needed; local indexed node fallback. | Core proof logic; private database substitute; transaction submission. |
| SilverScript | Candidate covenant-authoring tool only. Not a dependency for TN10 PoC now. | Separate compatibility spike if it targets TN10 and emits inspectable scripts compatible with rusty-kaspa/TN10. | Required build dependency; proof source; TN10 dependency before compatibility is proven. |
| Static UI | Demonstration and explanation surface only. Displays proof mirror and safety warnings. | Explain proof/check status after Rust emits verified artifacts. | Source of randomness/result/proof truth; wallet; signing; broadcast; betting/custody. |
| JSON mirror files | Export/mirror of Rust-verified proof and TN10 evidence. Useful for UI and audit readability. | Stable app-facing proof format after Rust verifier owns mappings. | Proof source; independent authority; substitute for covenant evidence. |
| Optional local TN10 indexed node | Not required for ENV-083B because public read-only path is sufficient. | Fallback if public RPC/API cannot expose covenant IDs, UTXO lineage, or future lane evidence. | Default dependency; wallet/signing/broadcast/mainnet. |

## 5. SilverScript suitability for this TN10 PoC now

SilverScript is not suitable as a required dependency for this TN10 PoC today.

Official SilverScript README evidence:

- status is experimental and unstable;
- compiled scripts are valid only on Kaspa Testnet 12;
- compatibility with other Kaspa networks or mainnet must not be assumed.

Because the PoC target is TN10, SilverScript must remain optional. The next step should use Rust-native covenant-state modelling and existing rusty-kaspa/TN10 evidence types. SilverScript can be revisited only after a separate compatibility ENV proves TN10 support and inspectable output compatibility.

## 6. Minimum viable Toccata-backed fairness anchor

### Minimum covenant-state model

One covenant lineage representing the application/fairness anchor, with per-round states represented as covenant-state transitions or modelled state records under that lineage:

```text
AppFairnessLineage(covenant_id)
  RoundOpened(round_id, commitment_hash, rules_hash, algorithm_id)
  -> CommitmentAnchored(commitment_txid, output_index, covenant_id)
  -> RevealPublished(reveal_txid, revealed_seed_material, reveal_hash)
  -> ResultDerived(result_number, result_colour, derivation_transcript_hash)
  -> ProofPublished(verifier_result, mirror_hash)
```

### Minimum round fields

- `round_id`
- `round_state`
- `network = testnet-10`
- `covenant_id`
- `covenant_lineage_reference`
- `rule_version`
- `result_algorithm`
- `commitment_txid` or modelled commitment anchor reference
- `commitment_output_index`
- `reveal_txid` or modelled reveal anchor reference
- `result_number`
- `result_colour`

### Minimum commitment fields

- `round_id`
- `commitment_hash`
- `commitment_domain`
- `result_algorithm`
- `rule_version`
- `commitment_payload_hash`
- `commitment_txid`
- `commitment_output_index`
- `commitment_covenant_id`
- `commitment_authorizing_input` if applicable

### Minimum reveal fields

- `round_id`
- `revealed_seed_material` or disclosed seed bytes
- `reveal_payload_hash`
- `reveal_txid`
- `reveal_output_index` or input reference
- `reveal_covenant_id`
- deterministic derivation transcript hash
- derived result number and colour

### Minimum verifier checks

- offline Rust model/schema validation passes;
- JSON mirror fields match Rust proof model;
- network is `testnet-10`;
- no mainnet/wallet/signing/broadcast flags are present;
- commitment hash equals hash of revealed seed/material under the expected domain;
- deterministic roulette result derivation from revealed seed/material matches result fields;
- commitment and reveal references carry the expected `covenant_id`;
- covenant ID matches the expected app lineage;
- transaction evidence is accepted or otherwise explicitly marked model-only;
- output covenant binding and/or input covenant ID are present where the claim requires them;
- no claim upgrades from Tier 1 to Toccata covenant fairness unless covenant evidence is present.

### Minimum TN10 evidence

- accepted commitment transaction or accepted model/reference transaction with `is_accepted = true`;
- accepting block hash / blue score where available;
- commitment/reveal transaction IDs and relevant output indices;
- output covenant binding: `covenant_id` and `covenant_authorizing_input`;
- input/UTXO covenant ID evidence where available;
- continuing output value and script public key where relevant;
- read-only verifier safety flags.

## 7. First architecture recommendation

Recommendation: A. one covenant lineage for the app with per-round state transitions.

Reasons:

- It preserves a stable application/fairness identity across rounds.
- It aligns with KIP-20 covenant IDs as lineage/membership identifiers.
- It reduces proof fragmentation versus one covenant instance per round.
- It lets the verifier check that every round belongs to the same app lineage.
- It provides a clean route to future app-level sequencing/lane proofs.

One covenant instance per roulette round is a later option for isolation or scalability experiments, but it weakens the immediate app-lineage story and creates more indexing/proof-management overhead. For the first PoC, per-round state under one app lineage is the clearer architecture.

## 8. Fallback plan

### Fallback A — Tier-1 bare TN10 commitment anchor + Rust verifier

If KIP-20 covenant lineage cannot be verified for future round transactions, continue with a bare TN10 commitment anchor and Rust verifier. Label the claim as Tier 1 only: Kaspa public anchor plus commitment/reveal verification, not full Toccata covenant fairness.

### Fallback B — Offline Rust covenant-state model pending live evidence

Build the covenant-state lifecycle in Rust without transaction creation/signing/broadcasting. Use deterministic artifacts and tests to define the expected covenant fields, state transitions, and verifier checks. Label this as offline covenant model only until live TN10 evidence exists.

### Fallback C — Local TN10 indexed node if public RPC/API is insufficient

Run a local TN10 node with UTXO index, following the rusty-kaspa Toccata guide pattern (`kaspad --utxoindex --testnet`), if public RPC/API cannot expose covenant IDs, covenant bindings, UTXO lineage, or future KIP-21 evidence. Still keep it read-only unless explicit authorization is granted.

## 9. Deferred work: what should not be built yet

Do not build yet:

- random demo rounds;
- wallet/faucet/signing/broadcast flows;
- production entropy;
- real betting;
- real payouts;
- custody/backend casino functionality;
- ZK proof implementation;
- mainnet support;
- SilverScript dependency unless TN10 compatibility is proven;
- covenant transaction creation/signing/broadcasting;
- roulette UI changes;
- verifier or roulette engine changes in ENV-083B.

## Recommended next ENV

ENV-083C — Offline covenant-state artifact and verifier model.

The next ENV should implement only Rust-native offline modelling of the fairness lifecycle and verifier checks. It should not create, sign, or broadcast transactions. It should produce a modelled commitment artifact, reveal artifact, derivation transcript, covenant-state transition model, verifier checks, and JSON mirror mapping.

## Hard-gate JSON summary

The machine-readable result is recorded in:

`spikes/kaspa-foundation/artifacts/env-083b-toccata-fairness-anchor-feasibility/env-083b-hard-gate-result.json`

## Safety boundary

ENV-083B remains a documentation/script/artifact feasibility gate only.

Confirmed:

- no roulette behaviour changed;
- no `examples/roulette-poc/ui/sample-round.json` change;
- no randomisation implemented;
- no real betting;
- no real payouts;
- no backend/custody;
- no wallet/private-key access;
- no signing;
- no transaction creation;
- no submitting/broadcasting;
- no mainnet;
- no secrets added.
