# ENV-092 — Verifiable TN10 entropy for KIP-17-enforced roulette rounds

Result: PASS

ENV-092 replaces the explicit demo-seed-only result path with a live TN10 future-entropy transcript for a KIP-17-enforced roulette round.

Core evidence:

- commitment transaction: `dc1759cda016bf8a9b314daf7b6f82b628187affc2d58c4af8b5437810acf332`
- NoMoreBets transaction: `fd02e7d66ebe06aa50a106b02b3fad976a4e700323b40a4e48a9574108bf34c0`
- reveal transaction: `31bb9a588e83b5299b5453f6451772e5c24b7ad8d02c9f0ba005b571a2efa5aa`
- entropy target blue score: `492892499`
- TN10 entropy value: `76b09cd0f4eaaaaa668df0af324a920fd44b4d5f75a0ef327df9e5d41c24cbe3`
- result: `34 red`
- claim level: `full_kip17_covenant_enforced_transition_with_live_tn10_entropy`

The operator seed is committed by hash before the future entropy value is known. The NoMoreBets transaction fixes the target formula `no_more_bets_accepting_blue_score + entropy_delay_blue_score`. The future TN10 block hash at or after the fixed target is read directly from TN10 and included in the final BLAKE3 transcript. The roulette number is derived by the existing domain-separated rejection-sampling algorithm.

Safety boundary:

- TN10/testnet-only
- no mainnet
- no real betting
- no real payouts
- no backend/custody casino operation
- no production randomness claim
- no UI result generation
- no wallet/private-key artifact stored in the repo
- no secrets added

Verification:

Run:

```bash
scripts/env092-tn10-verifiable-entropy-round-smoke.sh
```

Expected readiness line:

```text
TN10_VERIFIABLE_ENTROPY_ROUND_READY=PASS
```
