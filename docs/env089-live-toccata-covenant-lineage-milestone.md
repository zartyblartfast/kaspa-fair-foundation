# ENV-089 — Live Toccata covenant lineage milestone

Result documented from ENV-088: PASS — live TN10 KIP-20 covenant-linked lineage commitment/reveal flow.

## Milestone purpose

ENV-089 packages the current live Toccata milestone as a clean, reviewable checkpoint. It does not create new live transactions and does not extend the implementation. It preserves the ENV-088 evidence and its claim boundary so reviewers can verify exactly what was proven and what remains future work.

## What ENV-088 achieved

ENV-088 created and verified a live TN10 commitment/reveal lineage using KIP-20/Toccata covenant fields:

- Network: TN10 / testnet-10.
- Commitment transaction: `ebb28c6b34532cb97ae3a0a135fda74a0566b336df4dbf248283c5cad8c9ff65`.
- Reveal/continuation transaction: `f8fe14932071ac49cdac9e4f3df1177b9655dffbd0ad66b0e7491d6f78e5654b`.
- Covenant ID: `9931b78d93e1019ed132d52ccc8dc0b812b7fb5fa41cb561342c184afd11735c`.
- Claim level: `covenant-linked lineage`.

The commitment transaction has non-null covenant evidence in its TN10 transaction output fields. The reveal/continuation transaction spends the covenant-bound commitment output and carries the same covenant ID through its TN10 input/output covenant fields.

## Why this is stronger than static JSON

Static JSON can mirror a proof transcript, but it cannot itself prove that a live accepted TN10 transaction contained covenant evidence. ENV-088 is stronger because the milestone evidence includes accepted TN10 transaction readback for the commitment and reveal/continuation transactions. The covenant evidence is taken from direct transaction and UTXO fields returned by TN10 readback, not from application payload JSON.

Static payload JSON remains useful as a transcript mirror, but ENV-088 rejects payload-only `covenant_id` as sufficient covenant evidence. The accepted TN10 transaction fields are the evidence source for this milestone.

## Why this is stronger than ENV-087 bare TN10 anchoring

ENV-087 proved a live TN10 commitment/reveal anchor, but its covenant fields were null. That meant ENV-087 could claim live accepted TN10 anchoring only, not covenant-linked lineage.

ENV-088 advances beyond the ENV-087 bare anchor by proving non-null KIP-20/Toccata covenant evidence in direct TN10 transaction fields:

- the commitment output has covenant evidence;
- the reveal/continuation input references the commitment output covenant ID;
- the reveal/continuation output continues the same covenant ID;
- the reveal/continuation spends the covenant-bound commitment output.

## Toccata / KIP-20 evidence present

The packaged ENV-088 evidence includes:

- `env-088-commitment-tx-evidence.json`: accepted commitment tx with non-null output covenant ID.
- `env-088-reveal-tx-evidence.json`: accepted reveal/continuation tx with matching input/output covenant ID and linkage to the commitment txid.
- `env-088-direct-tn10-commitment-tx.json`: direct TN10 readback for the commitment tx.
- `env-088-direct-tn10-reveal-tx.json`: direct TN10 readback for the reveal/continuation tx.
- `env-088-covenant-field-verification.json`: verification that required covenant evidence is non-null and comes from direct TN10 transaction readback fields, not payload JSON.
- `env-088-verifier-output.json`: verifier result with `claim_level` set to `covenant-linked lineage`.

Core identifiers:

- Commitment txid: `ebb28c6b34532cb97ae3a0a135fda74a0566b336df4dbf248283c5cad8c9ff65`.
- Reveal/continuation txid: `f8fe14932071ac49cdac9e4f3df1177b9655dffbd0ad66b0e7491d6f78e5654b`.
- Covenant ID: `9931b78d93e1019ed132d52ccc8dc0b812b7fb5fa41cb561342c184afd11735c`.

## Evidence source boundary

Claimed covenant evidence comes from direct TN10 transaction-field evidence, not payload-only JSON. The payload can describe the application transcript, but the covenant lineage claim depends on TN10 readback fields such as transaction inputs, outputs, covenant IDs, and the output spent by the reveal/continuation transaction.

## Not claimed

ENV-089 preserves the ENV-088 claim boundary. It does not claim:

- full KIP-17 covenant-enforced state transition;
- production randomness;
- real betting;
- real payouts;
- mainnet;
- backend custody;
- wallet/private-key material in artifacts;
- any new transaction, signing, or broadcast during ENV-089 packaging.

## Why full KIP-17 covenant-enforced transition remains future work

The milestone proves a live KIP-20/Toccata covenant-linked lineage: a commitment output is covenant-bound, and a reveal/continuation transaction spends that output while continuing the covenant ID. That is not the same as full KIP-17 covenant-enforced roulette state transition logic.

Full KIP-17 enforcement would require the transaction rules themselves to enforce the allowed state transition, validation conditions, and application-specific transition constraints. ENV-088 proves lineage and field-level covenant continuity; it does not prove the complete state machine is enforced by covenant logic.

## Reviewer commands and checks

From the repository root, reviewers should run:

```bash
git status --short
git log --oneline -n 10
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
```

The ENV-089 smoke must finish with exactly:

```text
LIVE_TOCCATA_COVENANT_LINEAGE_MILESTONE_READY=PASS
```
