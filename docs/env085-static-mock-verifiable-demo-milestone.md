# ENV-085 — Static/mock verifiable demo milestone

## Purpose

This milestone packages the current Roulette PoC as a static/mock verifiable demo. It adds no proof logic and no UI behaviour. It documents how a reviewer can generate, inspect, and verify the current demo artifacts.

## What the demo proves

- Rust can generate `sample-round.json` and `toccata-fairness-proof.json` together from one explicit demo seed transcript.
- The generated sample round and proof artifact agree on round ID, result number, result colour, result algorithm, commitment/reveal transcript fields, safety flags, evidence mode, and future live transaction status.
- The deterministic roulette result is derived by Rust using BLAKE3 domain-separated rejection sampling and the fixed European colour mapping.
- Rust verifier logic validates the generated JSON mirror before it is accepted.
- The proof artifact is tied to live read-only TN10 Toccata covenant evidence as anchor evidence.
- The UI loads and displays the generated artifacts; the UI does not choose, mutate, randomise, or generate the roulette result.

## What the demo does not prove

- It does not prove production-grade unbiased randomness.
- It does not prevent seed grinding from operator-chosen demo seed material.
- It does not create live round-specific commitment/reveal transactions.
- It does not create, sign, submit, or broadcast any transaction.
- It does not use wallets, private keys, faucet funds, or mainnet.
- It does not implement real betting, real payouts, custody, accounts, or backend casino operations.

## Source architecture references

- [Toccata fairness anchor architecture](docs/toccata-fairness-anchor-architecture.md)
- [Toccata fairness anchor feasibility](docs/env083b-toccata-fairness-anchor-feasibility.md)
- [Verifiable demo round generation path (ENV-083F)](docs/env083f-verifiable-demo-round-generation-path.md)
- [Roulette PoC architecture](docs/roulette-poc-architecture.md)
- [Roulette PoC README](examples/roulette-poc/README.md)

## How the pieces fit together

```text
Rust ENV-084 generator
  -> generated sample-round.json
  -> generated toccata-fairness-proof.json
  -> Rust verifier output
  -> static Roulette UI displays generated artifacts only
  -> proof artifact references live_readonly_tn10 anchor evidence
  -> live round-specific commitment/reveal transactions remain future authorised work
```

The JSON files are app-facing mirror/export artifacts. Rust verifier logic is the proof authority. The browser is a display surface and mock betting surface only.

## Generate a demo round

Write to a separate output directory without changing app-facing files:

```bash
cargo run -q -p kaspa-fair-cli -- env084-generate-verifiable-demo-round \
  --round-id env-085-demo-round-0001 \
  --demo-seed "env085-demo-seed-0001" \
  --out-dir spikes/kaspa-foundation/artifacts/env-085-static-mock-verifiable-demo-milestone/generated-check
```

Write to the app-facing UI artifacts when intentionally refreshing the demo:

```bash
cargo run -q -p kaspa-fair-cli -- env084-generate-verifiable-demo-round \
  --round-id env-084-demo-round-0001 \
  --demo-seed "env084-demo-seed-0001" \
  --write-ui
```

## Verify generated artifacts

```bash
scripts/env083f-round-proof-consistency-smoke.sh
scripts/env084-verifiable-demo-round-generator-smoke.sh
scripts/env085-static-mock-verifiable-demo-milestone-smoke.sh
cargo run -q -p kaspa-fair-cli -- env083c-toccata-evidence-bound-fairness-proof --json
```

Required verified fields:

- `verifier_result = PASS`
- `evidence_mode = live_readonly_tn10`
- `future_live_round_transaction_evidence = not_created_not_claimed_future_work`
- sample/proof agreement on `round_id`, `result_number`, `result_colour`, and `result_algorithm`
- false safety flags for wallet, private key, signing, transaction creation, broadcast, mainnet, real betting, payouts, and custody

## Run the UI locally/static

From the repository root:

```bash
cd examples/roulette-poc/ui
python3 -m http.server 8080
```

Then open:

```text
http://127.0.0.1:8080/
```

The page loads:

- `sample-round.json`
- `toccata-fairness-proof.json`
- `roulette-table-schema.json`

The UI displays Start Wheel / Reset Round, mock table bets, generated result display, settlement display, and proof snapshot. It does not use `Math.random`, browser crypto random APIs, wallet APIs, signing APIs, or broadcast APIs to choose a result.

## Required developer checks

```bash
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
cargo run -q -p kaspa-fair-cli -- env083c-toccata-evidence-bound-fairness-proof --json
cargo run -q -p kaspa-fair-cli -- env084-generate-verifiable-demo-round \
  --round-id env-085-demo-round-0001 \
  --demo-seed "env085-demo-seed-0001" \
  --out-dir spikes/kaspa-foundation/artifacts/env-085-static-mock-verifiable-demo-milestone/generated-check
```

## Randomness and future transaction boundary

Explicit demo seed material is repeatable demo input. It is not production randomness and must not be described as casino-grade, production-grade, or on-chain random output.

Live round-specific TN10 commitment/reveal transactions remain future authorised work. Any wallet, faucet, signing, transaction creation, submission, broadcast, or mainnet work requires explicit authorisation before implementation.
