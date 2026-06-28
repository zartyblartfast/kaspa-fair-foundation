# Toccata Fairness Anchor Architecture

## Purpose

This document resets and clarifies the development path for the Kaspa Fair Foundation / Roulette PoC.

The project must not drift into a generic roulette UI with JSON proof files. The core product thesis is:

> Kaspa/Toccata can strengthen proof-of-fairness by anchoring round commitments, reveal evidence, and verifier data to a covenant-aware trust layer.

The roulette app is only the demonstration surface. The proof layer is the product.

## Current project position

The project currently has:

* A limited TN10 Toccata read-only trust layer.
* A live TN10 read-only verifier.
* An app-facing JSON verifier.
* A readiness gate.
* A deterministic roulette round engine.
* A simple static roulette UI.
* A schema-driven roulette table renderer.
* Mock-only bet selection and mock ledger display.
* A fixed sample result currently loaded from `sample-round.json`.

The project currently does not have:

* Real betting.
* Real payouts.
* Wallet integration.
* Signing.
* Transaction creation.
* Transaction broadcasting.
* Backend custody.
* Mainnet usage.
* Production casino operation.
* Toccata covenant-backed round anchoring.

The current JSON path is useful as a scaffold, but it must not become the centre of the PoC.

## Strategic correction

The project direction should change from:

```text
Improve JSON proof model first, maybe add Toccata later.
```

to:

```text
Use JSON only as a local mirror/export format for a Toccata-backed fairness proof model.
```

The PoC must deliberately answer:

```text
Can Kaspa/Toccata strengthen the proof-of-fairness claim in a way that ordinary off-chain JSON cannot?
```

If Toccata covenant functionality can sensibly support the fairness story, the PoC should use it sooner rather than later.

## Core fairness claim

The desired user-facing claim is not:

```text
Trust this roulette UI.
```

It is:

```text
Do not trust the roulette UI.

The UI displays a result, but the fairness proof can be independently checked.

The round commitment is anchored through a Toccata-aware covenant flow. After the reveal, the verifier checks that the revealed seed matches the earlier commitment and that the public deterministic algorithm derives the displayed roulette result.
```

The intended product message is:

```text
The spin animation is theatre.
The proof is the product.
```

## Important distinction: randomisation vs verifiability

Randomisation does not inherently conflict with proof-of-fairness.

The risk is hidden or unverifiable randomisation.

Bad model:

```text
UI calls Math.random()
UI chooses roulette number
UI displays result
```

This weakens the fairness story because the result is client-side behaviour and cannot prove that the result was fixed fairly before or during the betting window.

Better model:

```text
Seed or entropy material is defined.
A commitment is published before result reveal.
Betting closes.
Seed/reveal material is disclosed.
The roulette number is derived deterministically using a public algorithm.
The verifier recomputes the result.
The UI only displays the verified result.
```

The existing deterministic BLAKE3 domain-separated rejection-sampling result derivation is useful. It is the derivation step, not the entropy source.

Do not call deterministic derivation “randomisation” unless the ENV also defines and verifies the entropy or commitment/reveal material.

## Toccata opportunity

Toccata is relevant because it introduces features that can support a fairness anchor:

* Covenant-capable scripting.
* Transaction introspection.
* Payload substring access.
* Output inspection.
* BLAKE3 hash opcodes.
* Byte/string operations.
* Arithmetic/modulo-style operations.
* Stateful UTXO transitions.
* Covenant IDs / consensus-tracked covenant lineage.
* Lane-based sequencing commitments that may later help prove ordering or activity.

The most important missed opportunity is covenant lineage.

A plain JSON proof can say:

```text
Here is a round.
Here is a seed.
Here is a derived result.
Trust that this file represents the real round.
```

A Toccata covenant model can aim to say:

```text
Here is a round state anchored to a covenant lineage.
Here is the commitment state.
Here is the reveal state.
Here is the deterministic derivation.
Here is verifier evidence linking the app proof to the covenant flow.
```

That is much stronger and better aligned with Kaspa Fair Foundation.

## Recommended architecture

The preferred architecture is:

```text
Static Roulette UI
  ↓
App-facing proof JSON
  ↓
Rust verifier / deterministic proof model
  ↓
Toccata covenant-state model
  ↓
TN10 read-only evidence layer
  ↓
Later, only if authorised: TN10 testnet transaction creation/broadcast
```

The JSON file should be treated as a mirror/export of proof evidence, not as the source of truth.

## Toolchain decision

RPC, Rust, and SilverScript are not competing choices. They should have separate roles.

### Rust

Rust should be the canonical truth layer.

Use Rust for:

* Fairness proof model.
* Deterministic roulette result derivation.
* BLAKE3 rejection-sampling checks.
* Commitment/reveal verification.
* Covenant-state modelling.
* Covenant ID / payload / script evidence modelling.
* App-facing proof JSON validation.
* Future transaction construction modelling.
* Tests and deterministic smoke checks.

Rust should not be bypassed by UI-generated result logic.

### RPC / node API

RPC should be the read-only evidence transport layer.

Use RPC for:

* TN10 node readiness.
* DAG/block/transaction/UTXO evidence retrieval.
* Toccata transaction field inspection.
* Covenant output / UTXO evidence where available.
* Later covenant UTXO lookup.
* Later sequencing or lane evidence retrieval where available.

RPC should not become the core proof logic layer.

### SilverScript

SilverScript should be treated as a covenant-authoring candidate, not the foundation of the PoC.

Use SilverScript only if a separate feasibility ENV confirms:

* It targets the correct testnet.
* It supports the required Toccata features.
* Its output can be inspected and verified against rusty-kaspa/TN10 expectations.
* It does not introduce toolchain instability.

If SilverScript is not compatible with the target network or exact covenant features, the PoC should proceed with Rust-native modelling and script construction.

### UI

The UI should remain a static mock display and explanation layer.

Use the UI for:

* Roulette table interaction.
* Mock bet ledger display.
* Status flow.
* Result display.
* Proof explanation.
* Verifier summary.
* Safety warnings.

The UI must not:

* Generate the result.
* Source randomness for the result.
* Sign transactions.
* Access wallets.
* Broadcast transactions.
* Claim production casino capability.

## Current safety boundary

The project remains mock-only unless explicitly authorised otherwise.

The following remain out of scope:

* Real betting.
* Real payouts.
* Backend custody.
* Wallet integration.
* Private key access.
* Signing.
* Transaction creation.
* Transaction submission/broadcast.
* Mainnet usage.
* Production casino operation.

Any ENV that crosses into faucet, wallet, signing, transaction creation, or broadcast must ask for explicit authorisation first.

## Toccata-backed fairness model

The preferred fairness lifecycle is:

```text
RoundOpened(commitment)
→ BetsOpen
→ WheelSpinning
→ NoMoreBets
→ Seed/Revealed
→ ResultDerived
→ SettlementShown
→ ProofPublished
```

The covenant-backed version should aim for:

```text
Covenant lineage exists.
Round commitment is anchored.
Reveal material is checked against commitment.
Result is deterministically derived.
Verifier confirms the app-facing result matches the covenant-backed proof evidence.
```

## Candidate round-state fields

The round proof model should eventually include fields such as:

```text
round_id
round_state
covenant_id
covenant_lineage_reference
commitment_tx_id
commitment_output_index
commitment_payload_hash
commitment_payload_fields
no_more_bets_marker
reveal_tx_id
reveal_payload_hash
revealed_seed_material
result_algorithm
result_number
result_colour
derivation_transcript
verifier_result
tn10_network_reference
tn10_node_evidence
```

Not all of these need to exist immediately. ENV-083B should determine the minimum useful set.

## Commitment/reveal fairness

Commitment/reveal proves:

* The reveal material matches an earlier commitment.
* The result source was fixed before reveal.
* The displayed result can be recomputed from the revealed material.
* The operator cannot change the result after reveal without detection.

Commitment/reveal alone does not prove:

* The seed was unbiased.
* The operator did not search for a favourable seed before committing.
* The system is production casino-grade.

Future improvements may include independent entropy, public entropy, user-contributed entropy, block-derived entropy, or multi-party seed contribution. These must be designed carefully so the system is not manipulable by any party.

## Toccata feature opportunities

### KIP-17 covenants

Likely use:

* Enforce or model round-state transitions.
* Inspect transaction payload fields.
* Check output shape.
* Use BLAKE3 hash opcodes where appropriate.
* Anchor state in UTXO transitions.

### KIP-20 covenant IDs

Likely use:

* Establish stable covenant lineage.
* Prove that fairness states belong to a specific covenant instance.
* Avoid arbitrary JSON files masquerading as proof.
* Make covenant-labelled state harder to forge.

This is probably the strongest immediate Toccata opportunity for the PoC.

### KIP-21 sequencing commitments

Possible later use:

* Prove ordering or inclusion of commitment/reveal activity.
* Support “commitment before reveal” evidence.
* Support app-lane-specific proof ideas.
* Improve future ZK/lane proof architecture.

This should be researched, not assumed.

### KIP-16 ZK precompile

Possible later use:

* Compress or verify more complex off-chain computation.
* Support advanced settlement proofs.
* Support privacy-preserving or batched proof systems.

This should probably be deferred. It is powerful but likely too much for the first fairness-anchor milestone.

## Recommended near-term roadmap

### ENV-083B — TN10 Toccata fairness-anchor architecture and toolchain sanity check

Goal:

Define the correct Toccata-backed fairness architecture before implementing random demo rounds or deeper UI changes.

Required outputs:

* Identify exact Toccata features relevant to fairness anchoring.
* Confirm whether KIP-17, KIP-20, KIP-21, and KIP-16 are applicable now, later, or not at all.
* Decide whether the first anchor should be:

  * one covenant lineage for the app, with per-round state transitions, or
  * one covenant instance per roulette round.
* Decide the minimum useful covenant state fields.
* Decide which fields remain off-chain JSON mirror fields.
* Identify what can be verified read-only on TN10.
* Identify what requires a local TN10 node with UTXO index.
* Identify what public RPC can and cannot provide.
* Evaluate SilverScript compatibility but do not depend on it.
* Produce a no-wallet/no-signing/no-broadcast implementation plan.
* Clearly mark what later requires explicit transaction authorisation.

Expected result:

```text
TOCCATA_FAIRNESS_ANCHOR_ARCHITECTURE_READY=PASS
```

### ENV-083C — Offline covenant-state artifact and verifier model

Goal:

Implement a Rust-native model of the proposed covenant fairness lifecycle without signing or broadcasting transactions.

Expected outputs:

* Round commitment artifact.
* Reveal artifact.
* Deterministic result derivation.
* Covenant-state model.
* Verifier checks.
* App-facing JSON mirror.
* Tests proving commitment/reveal/result consistency.

No wallet, signing, broadcast, or mainnet.

### ENV-083D — UI explanation and proof narrative

Goal:

Update the UI/docs so users can understand “random but verifiable”.

Expected outputs:

* “How this result is verified” section.
* Clear distinction between spin animation and proof.
* Clear statement that the UI does not choose the result.
* Clear statement that the proof is mock/testnet only.
* Safety warnings preserved.

### ENV-084 — Authorised TN10 covenant transaction spike

Goal:

Only if explicitly authorised, create/sign/broadcast a minimal TN10 testnet covenant transaction flow.

This ENV must not start without user approval because it crosses into transaction creation/signing/broadcasting.

## Development standards for Hermes

Hermes must follow these standards:

* Use concrete ENV deliverables only.
* Final ENV responses must use PASS/FAIL against explicit commands.
* Avoid vague language.
* Avoid temporary-script-centred reporting.
* Use persistent scripts for checks.
* Do not commit or push unless explicitly authorised.
* If commit/push is authorised, stage only relevant ENV files.
* Run staged secret/file checks before commit.
* Report final git status and latest commit hash.
* Keep the safety boundary explicit.

## Required final response style for future ENVs

Every ENV final response should include:

```text
ENV-XXX — Title

Result:
PASS / FAIL

Concrete deliverable:
* path(s):
* smoke/check script:
* script exit status:
* final readiness line:

Contract:
* requirement: yes/no
* requirement: yes/no

Tests/checks:
* command: PASS/FAIL
* command: PASS/FAIL

Files changed:
* source/docs/scripts/artifacts grouped clearly

Safety confirmation:
* no real betting
* no real payouts
* no backend/custody
* no wallet/private key access
* no signing
* no transaction creation
* no submitting/broadcasting
* no mainnet
* no secrets added

Git handoff:
* ask before commit/push
* provide exact commit message if applicable
```

## Non-negotiable architectural principles

1. Toccata is not decoration.
   The PoC must use Toccata where it sensibly improves proof-of-fairness.

2. JSON is not the proof source.
   JSON may mirror/export evidence, but the model should lead toward covenant-backed proof anchoring.

3. Rust is the truth layer.
   Deterministic verification and proof logic belong in Rust.

4. RPC is evidence transport.
   RPC observes TN10 and retrieves evidence. It should not become the proof model.

5. SilverScript is optional until proven.
   It may help author covenant scripts later, but it must pass a compatibility spike first.

6. UI is not trusted.
   The UI displays and explains proof. It must not choose the result.

7. Randomness must be verifiable.
   Random-looking outcomes are acceptable only when tied to explicit seed/commitment/reveal evidence.

8. No real casino functionality.
   The PoC remains mock-only unless explicitly authorised otherwise.

9. No silent scope expansion.
   Wallet, signing, broadcasting, faucet, custody, and mainnet work require explicit authorisation.

10. The proof layer is the product.
    Roulette is the demonstration vehicle.