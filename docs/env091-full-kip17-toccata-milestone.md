# ENV-091 — Full KIP-17 Toccata milestone

Result packaged from ENV-090: PASS — live TN10 full KIP-17 covenant-enforced state transition, with app-facing UI acceptance of the authorised proof artifact.

## Purpose

ENV-091 packages the current full live Toccata milestone as a clean, reviewable, demo-ready checkpoint. It is documentation, artifact packaging, and verification only.

ENV-091 does not add implementation, create transactions, sign transactions, broadcast transactions, access wallets, touch mainnet, add real betting, add payouts, add production randomness, or add UI result generation.

## What this milestone proves

The current milestone proves that the project has moved beyond a static proof mirror and beyond a bare live TN10 anchor. The accepted ENV-090 evidence demonstrates:

- a live TN10 commitment transaction;
- a live TN10 reveal/continuation transaction;
- KIP-20 covenant lineage visible in direct TN10 transaction/readback evidence;
- a KIP-17 covenant-enforced state transition from commitment state counter 0 to reveal/continuation state counter 1;
- verifier output with `verifier_result = PASS`;
- claim level `full_kip17_covenant_enforced_transition`;
- negative transition evidence showing an invalid transition is rejected;
- a static app-facing UI that accepts the authorised ENV-090 full KIP-17 proof artifact while rejecting unsafe proof states.

The proof layer is the product. The roulette UI is the demonstration surface.

## Core live TN10 identifiers

Commitment txid:

```text
050bbe398ff7e8f7511697c65b511ab23bf1548bcba1ed0fb29380d1e582ec26
```

Reveal/continuation txid:

```text
269abfe10635d666d0c5b7624550a4abee5a47a8bd08d6a0e0b1a09dc2cf0620
```

Claim level:

```text
full_kip17_covenant_enforced_transition
```

## What ENV-090 added beyond ENV-088

ENV-088 proved live TN10 KIP-20/Toccata covenant-linked lineage. It showed that a covenant-bound commitment output existed and that a reveal/continuation transaction spent it while preserving covenant evidence in direct TN10 transaction fields.

ENV-090 added the deeper KIP-17 claim. It did not stop at covenant lineage. It added covenant-enforced state-transition evidence for the round lifecycle:

- commitment state counter `0` is represented in the commitment evidence;
- reveal/continuation state counter `1` is represented in the reveal evidence;
- the KIP-17 rule requires the transition to advance exactly one authorised next-state output;
- VM/covenant enforcement evidence records that the valid increment transition passes;
- negative checks record that invalid transitions are rejected;
- verifier output rejects bare TN10 anchor and KIP-20 lineage-only proof levels for ENV-090 PASS.

## How KIP-17 enforcement is represented

KIP-17 enforcement is represented in the ENV-090 artifacts, especially:

- `spikes/kaspa-foundation/artifacts/env-090-kip17-covenant-enforced-transition/env-090-kip17-enforcement-verification.json`
- `spikes/kaspa-foundation/artifacts/env-090-kip17-covenant-enforced-transition/env-090-verifier-output.json`
- `examples/roulette-poc/ui/toccata-fairness-proof.json`

Required fields include:

```text
claim_level = full_kip17_covenant_enforced_transition
verifier_result = PASS
kip17_rule_enforced_on_transition = true
invalid_transition_rejected = true
```

The enforcement artifact records the enforced rule:

```text
commitment output P2SH redeem script counter must increment by exactly one and authorized output SPK must match reconstructed next-state P2SH script
```

The app-facing proof mirrors the same proof contract under `kip17_enforcement` and under the live commitment/reveal evidence. JSON remains a mirror/export. The Rust verifier and TN10/KIP-17 evidence are the authority.

## How invalid transition rejection is recorded

Invalid transition rejection is recorded by ENV-090 in two layers:

1. `env-090-kip17-enforcement-verification.json` records:
   - `invalid_no_increment_rejected = true`
   - `invalid_reuse_previous_state_rejected = true`
   - `invalid_skip_state_rejected = true`
   - `valid_increment_transition_passed = true`

2. `env-090-verifier-output.json` records:
   - `invalid_transition_rejected = true`
   - `kip17_rule_enforced_on_transition = true`
   - `verifier_result = PASS`

The ENV-090 UI proof smoke also validates that unsafe proof states remain rejected, including missing live evidence, unsupported claim levels, verifier failure, mainnet flags, real betting, real payouts, backend custody, production randomness claims, mismatched result data, and secret-like UI material.

## How to verify the milestone

From the repository root, run:

```bash
git status --short
git log --oneline -n 12
cargo fmt --check
cargo check -p kaspa-foundation
cargo check -p kaspa-fair-cli
cargo test -p kaspa-foundation
cargo test -p kaspa-fair-cli
node --check examples/roulette-poc/ui/app.js
node --check examples/roulette-poc/ui/roulette-table-renderer.js
git diff --check
scripts/env082-roulette-ui-workflow-rebuild-smoke.sh
scripts/env083a-roulette-randomness-source-audit.sh
scripts/env083d-user-facing-fairness-proof-explanation-smoke.sh
scripts/env083e-app-facing-fairness-proof-artifact-smoke.sh
scripts/env083f-round-proof-consistency-smoke.sh
scripts/env084-verifiable-demo-round-generator-smoke.sh
scripts/env085-static-mock-verifiable-demo-milestone-smoke.sh
scripts/env086-live-toccata-workaround-inventory-smoke.sh
scripts/env087-tn10-round-commit-reveal-spike-smoke.sh
scripts/env088-tn10-covenant-lineage-commit-reveal-smoke.sh
scripts/env089-live-toccata-covenant-lineage-milestone-smoke.sh
scripts/env090-kip17-covenant-enforced-transition-smoke.sh
scripts/env090-ui-accepts-kip17-proof-smoke.sh
scripts/env091-full-kip17-toccata-milestone-smoke.sh
```

The ENV-091 smoke must finish with exactly:

```text
FULL_KIP17_TOCCATA_MILESTONE_READY=PASS
```

## How to verify the UI acceptance contract

Run:

```bash
scripts/env090-ui-accepts-kip17-proof-smoke.sh
```

That smoke proves the static UI accepts the authorised ENV-090 proof artifact and rejects unsafe mutations. It verifies the proof contract includes:

- `source_env = ENV-090`
- `verifier_result = PASS`
- `claim_level = full_kip17_covenant_enforced_transition`
- live commitment/reveal evidence present and linked
- KIP-17 enforcement represented as true
- invalid-transition rejection represented as true
- sample round result agreement with the proof artifact
- no mainnet, real betting, real payouts, backend custody, or production randomness claim

The UI displays proof. It does not choose the result.

## Direct TN10 readback support

Direct TN10 readback artifacts supporting ENV-090 are preserved under:

```text
spikes/kaspa-foundation/artifacts/env-090-kip17-covenant-enforced-transition/
```

Relevant files include:

- `env-090-direct-tn10-commitment-tx.json`
- `env-090-direct-tn10-reveal-tx.json`
- `env-090-commitment-tx-evidence.json`
- `env-090-reveal-tx-evidence.json`
- `env-090-covenant-field-verification.json`
- `env-090-kip17-enforcement-verification.json`
- `env-090-verifier-output.json`

The direct readback evidence records the commitment and reveal/continuation transactions as accepted TN10 transactions, with covenant fields present in transaction inputs/outputs and the reveal/continuation spending the commitment output.

## What is still not claimed

This milestone does not claim:

- production randomness;
- real betting;
- real payouts;
- custody/backend casino operation;
- wallet integration;
- private-key exposure;
- mainnet;
- production casino operation;
- UI-generated results;
- unbiased real-money randomness;
- payout execution or custody.

The ENV-090 proof uses explicit demo seed material. Commitment/reveal and deterministic derivation prove the result follows from the published material; they do not prove production-grade unbiased entropy or eliminate seed-grinding risk.

## Next real development area

The next real development area is verifiable entropy/randomisation.

The project should now move toward a design where roulette result derivation is bound to verifiable entropy, such as carefully selected public-chain entropy, user/operator multi-party seed material, or another auditable anti-grinding construction. The next technical step is not another bridge/planning packaging ENV and not UI random numbers.
