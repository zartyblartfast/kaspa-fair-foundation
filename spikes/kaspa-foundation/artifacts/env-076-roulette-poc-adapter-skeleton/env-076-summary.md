# ENV-076 — Roulette PoC adapter skeleton

Result: PASS

## Deliverable

Dry-run command:
- `scripts/env076-roulette-poc-dry-run.sh`

CLI adapter command:
- `cargo run -p kaspa-fair-cli -- roulette-poc-dry-run --json`

Artifacts written:
- live foundation verifier JSON: `spikes/kaspa-foundation/artifacts/env-076-roulette-poc-adapter-skeleton/env-076-live-foundation-verifier.json`
- roulette round JSON: `spikes/kaspa-foundation/artifacts/env-076-roulette-poc-adapter-skeleton/env-076-roulette-round-output.json`
- dry-run output: `spikes/kaspa-foundation/artifacts/env-076-roulette-poc-adapter-skeleton/env-076-dry-run-output.txt`

## What ENV-076 proves

ENV-076 adds the first non-web, no-wallet, dry-run roulette adapter skeleton on top of the existing foundation layer.

The adapter now:
- consumes the live TN10 foundation verifier JSON contract
- requires `verifier_result = PASS`
- enforces the safety boundary: read-only only, no wallet access, no signing, no transaction creation, no broadcasting, and no mainnet support
- builds deterministic seed material from the round id, verifier covenant id, verifier ENV-064 spend txid, verifier accepting block hash, and final mock bet-ledger hash
- derives a deterministic European roulette number with the ENV-075 BLAKE3 domain-separated rejection-sampling algorithm
- derives colour from the fixed European roulette table
- calculates deterministic settlement from fixed mock bets only
- emits stable machine-readable round JSON with `final_result = PASS`

## Safety boundary preserved

Confirmed:
- mock bets only
- no real betting
- no real payouts
- no web app
- no signing
- no transaction creation
- no submitting or broadcasting
- no wallet or private-key access
- no mainnet
- no secrets added

## Verification result

All required ENV-076 checks passed:
- `cargo fmt --check`
- `cargo test -p kaspa-fair-cli`
- `cargo test -p kaspa-foundation`
- `cargo check -p kaspa-fair-cli`
- `cargo check -p kaspa-foundation`
- `git diff --check`
- `scripts/env076-roulette-poc-dry-run.sh`
