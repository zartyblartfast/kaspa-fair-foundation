# ENV-093 — Verifiable TN10 entropy milestone

Result packaged from ENV-092: PASS — a KIP-17 covenant-enforced roulette round lifecycle whose final result transcript includes a live future TN10 chain value.

## Purpose

ENV-093 packages the current verifiable TN10 entropy checkpoint as a clean, reviewable, demo-ready milestone. It is documentation, artifact packaging, and checks only.

ENV-093 does not add implementation, create transactions, sign transactions, broadcast transactions, access wallets or private keys, touch mainnet, add real betting, add real payouts, add production randomness claims, or add UI result generation.

## What ENV-092 achieved

ENV-092 advanced the roulette proof from explicit demo-seed-only derivation to a live TN10 future-entropy transcript while preserving the full KIP-17 covenant-enforced round lifecycle.

The achieved state is:

- live TN10 KIP-17 covenant-enforced round lifecycle evidence exists for commitment, NoMoreBets, and reveal transitions;
- the operator seed was committed by hash before reveal;
- NoMoreBets fixed a future TN10 entropy target blue score;
- a TN10 block hash at that future target was read back and included in the final result transcript;
- BLAKE3 domain-separated rejection sampling derived the roulette result;
- the Rust verifier recomputed and validated the proof/result;
- the static UI displays the proof/result from app-facing artifacts and does not generate the result.

The proof layer remains the product. The roulette UI remains the demonstration surface.

## Why this is stronger than explicit demo seed only

Earlier demo-round milestones used explicit demo seed material to produce repeatable verifier-friendly outcomes. That proves the displayed result follows from the published transcript, but it does not add an external future entropy value after bets close.

ENV-092 is stronger because the final transcript mixes the revealed operator seed with a TN10 block hash selected by a target fixed only after the NoMoreBets transition. The operator seed commitment is made before reveal, and the future chain value is not known at the time the operator seed is committed.

This is still not a production casino randomness certification. It is a verifiable entropy milestone for a PoC.

## Operator seed commitment

The operator seed used by ENV-092 is:

```text
env092-operator-seed-0001
```

Before the seed is revealed, the commitment transition records the commitment hash:

```text
293c769ee41d89ecd830175504a086faa6bc4ff0b57351cad49b99f19ebab0d2
```

The commitment evidence records `operator_seed_revealed = false`. At reveal time, the verifier recomputes the commitment hash from the revealed seed and checks that it matches the earlier commitment.

## NoMoreBets fixes the future entropy target

The NoMoreBets transition fixes the future entropy target with this formula:

```text
entropy_target_blue_score = no_more_bets_accepting_blue_score + entropy_delay_blue_score
```

ENV-092 values:

```text
no_more_bets_txid: fd02e7d66ebe06aa50a106b02b3fad976a4e700323b40a4e48a9574108bf34c0
no_more_bets_accepting_blue_score: 492892469
entropy_delay_blue_score: 30
entropy_target_blue_score: 492892499
```

The target is therefore fixed by the accepted NoMoreBets transaction before the future TN10 entropy value is used.

## Future TN10 value readback

The entropy source is a TN10 block hash at or after the fixed target blue score.

ENV-092 readback values:

```text
entropy_source_type: tn10_block_hash_at_or_after_target_blue_score
entropy_source_block_hash: 76b09cd0f4eaaaaa668df0af324a920fd44b4d5f75a0ef327df9e5d41c24cbe3
entropy_source_blue_score: 492892499
entropy_source_daa_score: 503799370
```

The readback endpoint recorded in the ENV-092 artifact is:

```text
https://api-tn10.kaspa.org/blocks-from-bluescore?blueScoreGte=492892499&includeTransactions=false
```

## Final entropy transcript

The final entropy transcript records the commitment, NoMoreBets target, revealed operator seed, reveal transaction, and future TN10 entropy value under the domain:

```text
kaspa-fair:env092:final-entropy-transcript:v1
```

Core final transcript/result values:

```text
final_entropy_hash: f14b87fafbb6d5b8fd3bc1126bf78a87205c1e0e5830543cf13c091eb139df52
result_algorithm: blake3-domain-separated-rejection-sampling-v1
result_number: 34
result_colour: red
verifier_result: PASS
source_env: ENV-092
```

## BLAKE3 rejection sampling result derivation

The result is derived by the existing roulette PoC algorithm:

1. Build the final entropy transcript from the committed/revealed operator seed plus the future TN10 entropy value.
2. Hash the transcript to produce `final_entropy_hash`.
3. Use BLAKE3 domain-separated candidate generation for European roulette.
4. Interpret each candidate as a 256-bit integer.
5. Reject candidates outside the unbiased limit for 37 outcomes.
6. Use the accepted candidate modulo 37 to derive the roulette number.
7. Map the number through the fixed European colour table.

For ENV-092 the derived result is `34 red`.

## Rust verifier check

The Rust verifier recomputes and validates the milestone evidence. It checks:

- source environment is ENV-092;
- verifier result is PASS;
- claim level includes live TN10 entropy;
- sample/proof result number and colour agree;
- commitment hash matches reveal;
- NoMoreBets target is recorded;
- entropy source blue score is at or after the fixed target;
- final transcript includes the future TN10 entropy value;
- final entropy hash is present and valid;
- result number and colour verify;
- safety flags reject mainnet, real betting, real payouts, custody, wallet/private-key use, signing, and production randomness claims.

The persistent ENV-092 smoke command is:

```bash
scripts/env092-tn10-verifiable-entropy-round-smoke.sh
```

Expected readiness line:

```text
TN10_VERIFIABLE_ENTROPY_ROUND_READY=PASS
```

## UI boundary

The UI displays the app-facing sample/proof artifacts. It does not generate the result, source entropy, call browser randomness APIs for roulette result selection, sign, broadcast, access wallets, or create transactions.

The app-facing proof carries:

```text
ui_contract: static_readonly_export_for_roulette_poc
ui_does_not_choose_result: true
json_mirror_export_only: true
```

The result shown by the UI is the verifier-backed artifact result: `34 red`.

## Exact reviewer verification commands

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
scripts/env092-tn10-verifiable-entropy-round-smoke.sh
scripts/env093-verifiable-tn10-entropy-milestone-smoke.sh
```

The ENV-093 smoke must finish with exactly:

```text
VERIFIABLE_TN10_ENTROPY_MILESTONE_READY=PASS
```

## What is still not claimed

This milestone does not claim:

- production casino randomness;
- real betting;
- real payouts;
- custody/backend casino operation;
- mainnet;
- production casino operation;
- UI-generated results;
- wallet integration;
- private-key exposure;
- signing or broadcast in ENV-093.

ENV-092 did create/sign/broadcast authorised TN10 testnet transactions. ENV-093 itself does not create, sign, or broadcast anything.

## Known remaining risk and future work

Known remaining risks and optional strengthening paths:

- operator abort/griefing remains future work;
- user/multi-party entropy remains future work;
- KIP-21 sequencing/lane proof remains future optional strengthening;
- production randomness/casino certification is not claimed.

Next real development options are:

A. KIP-21 sequencing/lane proof strengthening;
B. user/multi-party entropy;
C. UX/performance cleanup;
D. stop and review product/demo positioning.
