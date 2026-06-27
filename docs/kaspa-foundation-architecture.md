# Kaspa Foundation Architecture Draft

## 1. Purpose

This document defines the proposed technical foundation for building applications on Kaspa Toccata features, starting with a roulette proof-of-concept but designed for reuse across future apps and dApps.

The goal is to avoid building a one-off roulette implementation. Instead, the project should produce a reusable Kaspa foundation layer that provides:

- covenant transaction construction
- local covenant verification
- live TN10 covenant submission workflows using Toccata features
- read-only settlement confirmation
- proof transcript generation
- app-specific fairness and state-machine adapters
- independent verification tools

The roulette PoC should be the first consumer of this foundation, not the foundation itself.

## 2. Core Design Principle

Separate the project into clear layers:

```text
Kaspa covenant mechanics
    ↓
Reusable proof/transcript layer
    ↓
Application state-machine layer
    ↓
Roulette/future app adapters
    ↓
CLI / API / UI
```

The covenant layer must not know anything about roulette.

The roulette layer must not manually reimplement covenant mechanics.

The proof layer must be usable by both developers and users.

The CLI and eventual web/API service should be thin wrappers around the same reusable foundation library.

## 3. Proposed Module Structure

A possible long-term structure is:

```text
kaspa-fair-lab/
  crates/
    kaspa-foundation/
      src/
        network/
        covenant/
        transaction/
        verifier/
        transcript/
        evidence/
        safety/
        errors/

    kaspa-app-core/
      src/
        app_adapter/
        state_machine/
        fairness/
        settlement/

    kaspa-roulette/
      src/
        round/
        bet/
        outcome/
        payout/
        fairness_proof/

    kaspa-fair-cli/
      src/
        commands/

  docs/
    kaspa-foundation-architecture.md
    proof-transcript-format.md
    roulette-poc-architecture.md
    safety-and-secrets.md
    threat-model.md

  spikes/
    kaspa-foundation/
      artifacts/
    tn12-minimal-covenant/
      artifacts/
      # historical spike area; TN10 became the proven live path during this work

  examples/
    roulette-poc/
    coin-flip-poc/
```

This does not need to be implemented all at once. The first refactor can simply move the proven TN10 covenant code into clearer internal modules.

## 4. Foundation Responsibilities

The reusable foundation should provide the technical building blocks required by roulette and future applications.

### 4.1 Network Layer

Responsible for:

- connecting to TN10 / future supported networks
- verifying network id
- checking synced status
- checking UTXO index availability
- querying UTXOs
- querying mempool entries
- submitting transactions only when explicitly allowed

Initial hard rule:

```text
Mainnet disabled by default.
TN10/testnet-10 only until explicitly changed.
```

### 4.2 Covenant Layer

Responsible for:

- building corrected v1 covenant scripts
- using `TX_VERSION_TOCCATA`
- using the corrected `OpBlake3WithKey` path
- using 32-byte padded domain keys
- applying covenant bindings correctly
- constructing covenant create transactions
- constructing covenant spend transactions
- preserving covenant id/state invariants

The proven ENV-063 / ENV-064 / ENV-065 path should become the canonical implementation.

The old ENV-060C path should remain documented as historical evidence only, not as the current recipe.

The proven corrected TN10 path is traceable through the public evidence bundles under:

```text
spikes/tn12-minimal-covenant/artifacts/env-063-corrected-live-covenant-create/
spikes/tn12-minimal-covenant/artifacts/env-064-live-corrected-covenant-spend/
spikes/tn12-minimal-covenant/artifacts/env-065-readonly-env064-spend-confirmation/
```

The directory name contains historical TN12 wording because the spike originally started there. The canonical live path is now TN10/testnet-10. Future foundation-planning artifacts should use a neutral foundation path rather than adding new work under the TN12-labelled spike directory.

### 4.3 Transaction Layer

Responsible for:

- input selection
- output creation
- fee calculation
- change handling
- transaction signing through controlled key interfaces
- local transaction id reconstruction
- transaction shape validation before submit

This layer should provide explicit transaction plans before signing or submission.

Example concept:

```rust
pub struct TransactionPlan {
    pub network: NetworkId,
    pub action: ActionKind,
    pub inputs: Vec<PlannedInput>,
    pub outputs: Vec<PlannedOutput>,
    pub fee_sompi: u64,
    pub tx_version: u16,
    pub covenant_recipe_id: Option<String>,
}
```

### 4.4 Local Verifier

Responsible for proving locally that a transaction is valid before any live submission.

For covenant spends, this means:

- reconstruct previous output/state
- reconstruct intended next output/state
- run the local script VM proof
- confirm covenant id
- confirm state transition
- fail closed if local proof does not pass

No transaction should be submitted unless local verification passes.

### 4.5 Submitter

Responsible for controlled live submission.

Rules:

- submission must be explicitly requested
- no automatic retries
- submit exactly once unless a later environment explicitly authorises otherwise
- use `allow_orphan=false`
- record txid or exact rejection
- stop after submit result

Submit functionality should not be enabled by default in the foundation library. Offline verification and transcript parsing should be available without live-submit capability.

### 4.6 Read-only Confirmation Layer

Responsible for post-submit confirmation:

- check whether transaction is in mempool
- check whether original UTXO is spent or absent
- check whether continuing output is visible as UTXO
- check value and covenant id where observable
- never sign or submit

ENV-065 is the model for this layer.

## 5. Covenant Recipe Abstraction

The foundation should introduce a recipe abstraction so future apps do not hand-code covenant details.

Example:

```rust
pub struct CovenantRecipe {
    pub id: String,
    pub name: String,
    pub tx_version: u16,
    pub script_family: ScriptFamily,
    pub hash_mode: HashMode,
    pub domain_key_policy: DomainKeyPolicy,
    pub state_transition_policy: StateTransitionPolicy,
}
```

For the proven path:

```text
Recipe ID:
toccata-v1-keyed-blake3-state-transition

Properties:
- TX_VERSION_TOCCATA
- OpBlake3WithKey
- 32-byte padded domain keys
- corrected v1 script path
- covenant id preserved across transition
- continuing output carries next payload/state
```

Roulette and future apps should reference recipes by ID.

## 6. Threat Model and Trust Boundaries

The architecture should not imply that “provably fair” means “trustless in every respect.”

The correct claim is narrower:

```text
The protocol should make post-commit outcome manipulation detectable and operationally impractical, while allowing independent verification of each completed round.
```

### 6.1 Threats the Foundation Should Help Prevent

For roulette and similar apps, the foundation should help detect or prevent:

- house changing its secret after the player bet is fixed
- server displaying an outcome inconsistent with the transcript
- outcome calculation mismatch
- payout calculation mismatch
- settlement transaction mismatch
- covenant state transition mismatch
- historical proof artifacts being silently altered

### 6.2 Threats Not Solved by the Foundation Alone

The foundation does not prove:

- the UI displayed the correct transcript to the user
- the operator is solvent beyond funds already committed on-chain
- the operator is legally compliant
- the server will always be available
- the house will always reveal when required
- the user’s device is uncompromised
- the player’s entropy source is strong
- custody, AML/KYC or responsible-gaming controls are adequate

These are product, operational, regulatory, and UX concerns outside the low-level covenant foundation.

## 7. App Adapter Interface

Each app should provide an adapter that defines its own rules while reusing the foundation.

Example responsibilities:

```text
App adapter:
- defines app state
- defines allowed transitions
- defines fairness rules
- defines payout rules
- defines timeout/refund rules
- defines settlement intent
- defines user-visible proof format
- calls foundation for covenant create/spend/verify
```

Suggested trait shape:

```rust
pub trait AppAdapter {
    type State;
    type Action;
    type Proof;

    fn app_id(&self) -> &'static str;

    fn validate_action(
        &self,
        current_state: &Self::State,
        action: &Self::Action,
    ) -> Result<(), AppError>;

    fn next_state(
        &self,
        current_state: &Self::State,
        action: &Self::Action,
    ) -> Result<Self::State, AppError>;

    fn build_settlement_intent(
        &self,
        current_state: &Self::State,
        action: &Self::Action,
    ) -> Result<SettlementIntent, AppError>;

    fn verify_settlement(
        &self,
        transcript: &ProofTranscript,
    ) -> Result<AppVerificationReport, AppError>;

    fn build_fairness_proof(
        &self,
        state: &Self::State,
        action: &Self::Action,
    ) -> Result<Self::Proof, AppError>;
}
```

The foundation verifies the Kaspa/covenant side.

The adapter verifies the app/game side.

## 8. Settlement and Funds Model

The foundation should not prematurely decide the real-money funds model.

For the roulette PoC, these remain open decisions:

- who funds covenant create
- where the player stake is locked
- where house payout liquidity lives
- whether the app ever custodies funds
- whether player keys sign transactions directly
- whether rounds are fully on-chain or partially server-mediated
- how failed or abandoned rounds unwind
- what happens if the house does not reveal
- what happens if public RPC is unavailable

For now, the safe assumption is:

```text
TN10/testnet-only.
No real funds.
No customer custody.
No mainnet.
```

The foundation layer should be useful independently of any future real-money roulette deployment.

## 9. Roulette PoC Adapter

The roulette PoC should be implemented as an app adapter.

It should define:

- round id
- player address
- stake
- selected bet
- house commitment
- player entropy
- reveal value
- outcome number
- payout rule
- settlement transaction
- proof transcript

The fairness model should use commit-reveal:

```text
1. House commits to a secret.
2. Player bet and entropy are fixed.
3. Covenant locks the round state.
4. House reveals the secret.
5. Outcome is calculated deterministically.
6. Payout/settlement is verified.
```

### 9.1 Roulette Round States

The roulette state machine should model normal and abnormal paths explicitly.

Suggested round states:

```text
Created
Committed
BetPlaced
Locked
Revealed
Settled
Expired
Refundable
Failed
Ambiguous
```

### 9.2 Failure Cases to Model

The design should explicitly handle:

- house commits but never reveals
- player starts but does not complete
- player entropy missing or malformed
- reveal does not match commitment
- settlement transaction rejected
- settlement confirmation delayed
- UTXO index unavailable
- RPC unavailable
- round expires before settlement
- transcript incomplete

Verifier results should distinguish:

```text
FAIL        = evidence proves something is wrong
AMBIGUOUS   = evidence is incomplete or chain state unavailable
UNAVAILABLE = required external data could not be queried
```

## 10. Exact Fairness Encoding

The roulette PoC must avoid vague outcome rules.

Avoid:

```text
outcome = hash(...) mod 37
```

Instead, the encoding must be canonical and documented.

### 10.1 Canonical Seed Input

A roulette round should derive its outcome from a domain-separated byte encoding:

```text
domain_tag           = "kaspa-fair:roulette:v1"
round_id             = canonical 32-byte round id
house_secret         = raw 32-byte value
player_entropy       = raw 32-byte value
covenant_create_txid = canonical 32-byte transaction id
bet_commitment       = canonical hash of selected bet
```

Seed material:

```text
seed_material =
    domain_tag ||
    round_id ||
    house_secret ||
    player_entropy ||
    covenant_create_txid ||
    bet_commitment
```

The exact encoding must define:

- byte order
- txid byte order
- whether values are raw bytes or hex text before hashing
- field lengths
- domain separation tag
- version string
- rejection-sampling procedure

### 10.2 Avoiding Modulo Bias

For European roulette:

```text
wheel_size = 37
valid outcomes = 0..36
```

The PoC should use deterministic rejection sampling so the independent verifier can reproduce the exact outcome calculation.

First compute canonical seed material as defined in Section 10.1.

For candidate counter `i`:

```text
candidate_i = BLAKE3(
    "kaspa-fair:roulette:candidate:v1" ||
    seed_material ||
    counter_u32_be(i)
)
```

Where:

- `counter_u32_be(i)` is a 4-byte unsigned big-endian integer
- `i` starts at 0
- `candidate_i` is interpreted as an unsigned 256-bit integer using the documented byte order
- the domain tag is a fixed byte string, not user-controlled text

Let:

```text
limit = floor(2^256 / 37) * 37
```

Algorithm:

```text
1. Set i = 0.
2. Compute candidate_i.
3. Interpret candidate_i as an unsigned 256-bit integer.
4. If candidate_i >= limit, increment i and repeat.
5. Otherwise outcome = candidate_i % 37.
6. The outcome maps to roulette number 0..36.
```

The rejection probability is negligible, but the procedure removes modulo bias and is exactly replayable by an offline verifier.

## 11. Proof Transcript Layer

Proof transcripts should be first-class artifacts.

Every meaningful action should produce both:

```text
proof-transcript.json
proof-transcript.md
```

The JSON version is for machines.

The Markdown version is for humans.

A proof transcript should include:

- transcript schema version
- network
- app id
- round id
- action type
- covenant recipe id
- git commit
- git dirty status
- server info
- input UTXOs
- output UTXOs
- transaction ids
- covenant id
- state before
- state after
- local VM proof result
- submit result
- postcheck result
- fairness proof
- secret-redaction confirmation

For roulette, a round proof should answer:

- was the house commitment fixed before the result?
- was the player input fixed before reveal?
- does the reveal match the commitment?
- does the hash produce the displayed roulette result?
- does the payout match the rules?
- was the covenant transaction accepted/confirmed?
- can the user independently verify the transcript?

## 12. Evidence Bundle Standard

The existing environment pattern should become a formal evidence bundle standard.

The evidence bundle schema and the proof transcript schema are versioned independently.

For example:

```text
evidence bundle schema: kaspa-fair-evidence-v1
proof transcript schema: kaspa-fair-transcript-v1
```

They may evolve at different speeds. A future evidence bundle format change should not automatically imply a proof transcript format change, and vice versa.

Each live or read-only environment should produce:

```text
manifest.json
proof-transcript.json
proof-transcript.md
commands.txt
artifact-hashes.txt
summary.txt
preflight.txt
submit.txt or action.txt
postcheck.txt
```

For no-live-action environments:

```text
manifest.json
summary.txt
read-only-checks.txt
proof-transcript.json
artifact-hashes.txt
```

### 12.1 manifest.json

The manifest should include:

```json
{
  "schema_version": "kaspa-fair-evidence-v1",
  "app_id": "roulette-poc",
  "network": "testnet-10",
  "git_commit": "...",
  "git_dirty": false,
  "created_at_utc": "...",
  "artifact_files": [],
  "artifact_hashes": {},
  "canonical_replay_command": "...",
  "live_action_performed": false
}
```

### 12.2 Append-only Evidence Principle

Evidence bundles should be treated as append-only once committed.

If a correction is required, create a new artifact folder rather than silently rewriting historical evidence.

## 13. Independent Verifier

The project should provide a verifier before building a polished UI.

Example commands:

```bash
kaspa-fair verify-transcript proof-transcript.json
kaspa-fair verify-round round-proof.json
kaspa-fair inspect-utxo --network testnet-10 <outpoint>
kaspa-fair verify-roulette-outcome <round-id>
```

The verifier should return simple results:

```text
PASS
FAIL
AMBIGUOUS
UNAVAILABLE
```

### 13.1 Offline Verifier

The offline verifier should require no RPC connection and no private keys.

It verifies:

- transcript schema validity
- artifact hashes
- commitment/reveal consistency
- deterministic outcome calculation
- payout rule calculation
- covenant recipe id
- local state transition consistency
- expected transaction ids if raw transaction data is included

ENV-070 implements the first version of this layer as a scaffold and sanity check over the canonical ENV-063/064/065 transcript and committed fixture paths. It is useful, but it is not the final developer value by itself.

### 13.2 Online Verifier

The online verifier adds chain/RPC checks:

- network id
- node synced status
- UTXO index availability
- mempool entry status
- transaction visibility
- original UTXO spent/absent status
- continuing output UTXO visibility

ENV-071 starts this layer for TN10/testnet-10 only. It first requires the offline transcript verifier to pass, then compares normalized read-only live-chain observations against the canonical transcript facts. The read-only TN10 API approach follows the already-proven reference spike path using public rusty-kaspa wRPC resolver defaults for testnet-10; endpoint override can be added later without requiring secrets.

ENV-071B proved the wRPC path with an explicitly gated live read-only test. Public rusty-kaspa wRPC confirmed the ENV-064 continuing output outpoint and value from the live UTXO set, but left accepted transaction input details and covenant id unsupported through that model alone.

ENV-071C closes that gap with the public TN10 transaction-detail API at `https://api-tn10.kaspa.org/transactions/<txid>?inputs=true&outputs=true&resolve_previous_outpoints=light`. That read-only response exposes mined/accepted ENV-064 transaction structure, including `is_accepted`, accepting block hash, previous outpoint hash/index, output 0 value, and covenant id. The live verifier now reaches `Pass` when those fields match the canonical transcript and still fails closed on contradictions.

ENV-072 turns the ENV-071 live verifier into a developer-facing operational command instead of requiring a long ignored-test invocation:

```bash
cargo run -p kaspa-fair-cli -- verify-live-tn10-canonical
```

The command is a thin CLI wrapper around the foundation online verifier library. It reads public TN10 state through read-only wRPC/API calls, prints the endpoint/API used and each key canonical check, and exits successfully only when the final verifier result is `PASS`. `PARTIAL` and `FAIL` are non-zero outcomes. A convenience wrapper is also available at `scripts/env072-live-tn10-verify.sh`.

This makes the limited Toccata layer operational for developers: offline/unit tests remain deterministic regression and safety checks, while the live command is the explicit way to confirm the canonical ENV-063/064/065 proof transcript against current TN10 chain data before any roulette PoC adapter is built.

ENV-073 adds the app-facing contract for the same live read-only verifier:

```bash
cargo run -p kaspa-fair-cli -- verify-live-tn10-canonical --json
```

The JSON mode emits schema `kaspa-fair-live-verification-result-v1`, keeps the result enum stable as `PASS`, `FAIL`, or `AMBIGUOUS`, and includes canonical txid/outpoint/output/value/covenant-id fields plus explicit safety fields:

```json
{
  "schema": "kaspa-fair-live-verification-result-v1",
  "network": "testnet-10",
  "mainnet_supported": false,
  "verifier_result": "PASS",
  "readonly": true,
  "signing_used": false,
  "transaction_created": false,
  "broadcast_used": false,
  "wallet_access_used": false
}
```

Apps, starting with the future roulette PoC adapter, can consume this contract without scraping human text. ENV-073 does not implement roulette; it only defines the stable foundation verifier output that app adapters can call later.

The initial online verifier must stay narrow:

- read-only TN10 queries only
- no signing
- no transaction creation
- no transaction submission or broadcast
- no wallet or private-key access
- no secrets
- no mainnet
- no roulette implementation

Mainnet portability is future work and remains disabled until explicit approval. Roulette remains future app adapter work above the foundation verifier.

Online verification should never be required to verify the mathematical fairness of a completed transcript, but it is required to verify live chain settlement status.

## 14. Safety Model

The project should keep the safety discipline that made the spike successful.

Required guardrails:

```text
mainnet disabled by default
explicit network required
submit requires explicit flag
no automatic retries
local proof required before submit
secret paths ignored by git
artifact redaction checks
staged secret checks before commit
old incompatible covenant paths guarded
```

There should be a permanent command:

```bash
kaspa-fair safety-check
```

It should check:

- no local-secrets staged
- no private key strings in artifacts
- no mnemonic/password/seed strings in docs
- network explicitly testnet-10
- mainnet disabled
- known dangerous paths ignored

## 15. Foundation Feature Separation

The reusable foundation should not require live submit functionality by default.

Suggested crate or feature layout:

```text
foundation-core
foundation-transcript
foundation-verifier
foundation-submit
foundation-cli
roulette-adapter
```

Possible Rust feature strategy:

```toml
[features]
default = ["offline"]
offline = []
submit = []
roulette = []
```

The default build should support:

- transcript parsing
- offline verification
- local proof checks
- no live submission

Live submission should require an explicit feature and explicit command flag.

## 16. Testing Strategy

The foundation should have several levels of tests.

### 16.1 Unit Tests

Examples:

- builds corrected v1 covenant script
- pads domain keys to 32 bytes
- rejects old v0 script for v1 spend
- calculates roulette outcome deterministically
- calculates payout correctly

### 16.2 Golden Tests

Use the proven ENV evidence as fixtures.

Examples:

- reconstruct ENV-063 create txid
- reconstruct ENV-064 spend txid
- verify ENV-064 continuing output
- verify ENV-065 confirmed settlement

### 16.3 Transcript Tests

Examples:

- valid transcript verifies as PASS
- tampered txid fails
- tampered reveal fails
- tampered payout fails
- wrong network fails
- missing postcheck is AMBIGUOUS

### 16.4 App Adapter Tests

For roulette:

- red/black payout
- straight number payout
- zero outcome
- invalid bet rejected
- commit-reveal mismatch rejected
- late reveal rejected
- expired round enters correct state

## 17. API Surface Draft

The initial Rust API could expose:

```rust
pub fn create_covenant_plan(...)
    -> Result<CovenantCreatePlan, FoundationError>;

pub fn verify_covenant_create_plan(...)
    -> Result<VerificationReport, FoundationError>;

pub fn submit_covenant_create_once(...)
    -> Result<SubmitReport, FoundationError>;

pub fn create_covenant_spend_plan(...)
    -> Result<CovenantSpendPlan, FoundationError>;

pub fn verify_covenant_spend_plan(...)
    -> Result<VerificationReport, FoundationError>;

pub fn submit_covenant_spend_once(...)
    -> Result<SubmitReport, FoundationError>;

pub fn confirm_transaction_readonly(...)
    -> Result<ConfirmationReport, FoundationError>;

pub fn generate_proof_transcript(...)
    -> Result<ProofTranscript, FoundationError>;

pub fn verify_proof_transcript_offline(...)
    -> Result<TranscriptVerificationReport, FoundationError>;

pub fn verify_proof_transcript_online(...)
    -> Result<TranscriptVerificationReport, FoundationError>;
```

The CLI should be a thin wrapper around this API.

## 18. Public API Principles

The public API should be:

```text
deterministic
explicit
safe by default
network-aware
evidence-producing
testable
app-agnostic
```

Avoid hidden side effects.

Bad:

```rust
play_round()
```

Better:

```rust
build_round_commit_plan()
verify_round_commit_plan()
submit_round_commit_once()
confirm_round_commit_readonly()
generate_round_proof()
```

This keeps actions auditable.

## 19. CLI Design

Near-term CLI commands could be:

```bash
kaspa-fair covenant create-plan
kaspa-fair covenant create-submit
kaspa-fair covenant spend-plan
kaspa-fair covenant spend-submit
kaspa-fair covenant confirm

kaspa-fair proof generate
kaspa-fair proof verify-offline
kaspa-fair proof verify-online

kaspa-fair roulette new-round
kaspa-fair roulette place-bet
kaspa-fair roulette reveal
kaspa-fair roulette settle
kaspa-fair roulette verify-round
```

All live commands should require:

```bash
--network testnet-10
--submit
--confirm-one-submission
```

## 20. HTTP/API Service Later

A web/API service can come later.

Initial endpoints might be:

```text
POST /api/rounds
POST /api/rounds/{id}/bet
POST /api/rounds/{id}/reveal
POST /api/rounds/{id}/settle
GET  /api/rounds/{id}/proof
GET  /api/rounds/{id}/verify
GET  /api/tx/{txid}/status
```

However, this should not be built until the CLI and verifier are stable.

The web service should call the same library as the CLI.

## 21. User-facing Proof of Fairness

For roulette, the user should see:

```text
Round result: 17 red
Your bet: red
Payout: 2x
Fairness proof: PASS
Settlement: confirmed on TN10
```

Expandable proof:

```text
House commitment
Player entropy
Reveal
Hash calculation
Outcome mapping
Covenant create tx
Covenant spend tx
Continuing output
Independent verifier result
```

Simple message:

```text
Do not trust the server. Verify the round.
```

## 22. Regulatory and Commercial Boundary

The Kaspa foundation layer is not gambling-specific. It may be reusable for many applications that need covenant state transitions, proof transcripts, and independent verification.

The roulette PoC is gambling-adjacent and must remain testnet/demo-only unless legal, compliance, custody, KYC/AML, jurisdictional, and responsible-gaming requirements are reviewed separately.

Technical provability supports transparency, but it does not by itself establish legal deployability.

## 23. Non-goals for the Foundation

The foundation should not initially include:

```text
full web app
user account system
real-money compliance system
mainnet support
custodial wallet service
affiliate system
casino lobby
multiple games
mobile app
```

Those can come later if justified.

## 24. Recommended Development Sequence

### ENV-067 — Foundation Refactor Plan

Goal:

Document and plan how the current helper evolves into reusable modules.

No live action.

### ENV-068 — Extract Covenant Core

Goal:

Move corrected v1 covenant construction/proof code into reusable internal module.

No live action.

### ENV-069 — Proof Transcript Format

Goal:

Define and implement JSON/Markdown proof transcript generation.

No live action.

### ENV-070 — Golden Transcript Fixtures for ENV-063/064/065

Goal:

Convert the proven covenant create/spend/confirmation path into stable verification fixtures.

No live action.

### ENV-071 — Standalone Offline Verifier CLI

Goal:

Verify proof transcripts independently without RPC access.

No live action.

### ENV-072 — Roulette State Machine and Threat Model

Goal:

Implement roulette round state, bet types, outcome calculation, payout calculation, timeout states, and app-level threat model.

No live action.

### ENV-073 — Roulette Fairness Simulation

Goal:

Offline commit-reveal roulette round with proof transcript.

No live action.

### ENV-074 — TN10 Roulette PoC Round

Goal:

Use the foundation to run one TN10 testnet roulette round.

Live TN10 only, explicit approval required.

## 25. Definition of Success

The architecture succeeds if:

```text
A future app can use the Kaspa foundation without copying low-level covenant code.

A user can verify a roulette round without trusting the app server.

A developer can reconstruct and verify historical transactions from transcripts.

Offline verification works without RPC access.

Online verification confirms chain settlement when RPC is available.

A reviewer can see exactly what was submitted, when, why, and with what proof.

Every live action produces versioned, hashable evidence.

Dangerous live submit functionality is opt-in.

Mainnet remains disabled until explicitly and separately approved.

The project can add new apps without compromising the proven covenant foundation.
```

## 26. ENV-068 Extraction Status

ENV-068 started the clean reusable foundation crate extraction from the proven corrected TN10 path.

The crate now represents the canonical corrected recipe as offline metadata and fixture-backed constants:

```text
recipe id: toccata-v1-keyed-blake3-state-transition
network: TN10/testnet-10
transaction version: TX_VERSION_TOCCATA = 1
hash opcode: OpBlake3WithKey
domain key policy: 32-byte padded domain keys
canonical evidence: ENV-063 create, ENV-064 spend, ENV-065 read-only settlement confirmation
```

The old ENV-060C path is retained only as historical/non-canonical evidence. It must not be used as the canonical implementation.

The extraction deliberately avoids copying the reference helper's rusty-kaspa path dependencies into the foundation crate. The reference helper depends on historical local source paths under `tools/rusty-kaspa-source/...`; the foundation crate should integrate official Kaspa crates later through a clean dependency strategy rather than through hidden links to the old lab workspace.

ENV-068 does not expose default live signing, wallet, RPC submit, mainnet, roulette, or web-app APIs. It is an offline/refactor step only.

## 27. Immediate Next Recommendation

The next concrete task should be either:

```text
ENV-069: Proof transcript format implementation
```

or, if reviewers want a deeper crate extraction before transcript work:

```text
ENV-068B: official Kaspa crate dependency integration plan / spike
```

Both should preserve the existing safety boundary: no live TN10 action, no signing, no transaction submission, no wallet/helper secret access, no mainnet, and no roulette/web app unless explicitly approved in a later environment.
