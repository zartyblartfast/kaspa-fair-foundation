# ENV-075 — Roulette fairness sequence and Toccata trust mapping

Result: PASS

## Deliverable

Primary spec file:
- `docs/roulette-poc-architecture.md`

Artifact directory:
- `spikes/kaspa-foundation/artifacts/env-075-roulette-fairness-sequence/`

## Scope completed

The specification now defines:
- a correct roulette round sequence from round open through published proof
- the explicit rule that `spin animation != result finalisation`
- the wheel-spins-while-bets-open fairness model with a hard no-more-bets boundary
- deterministic result derivation using domain-separated BLAKE3 candidate generation plus rejection sampling for the European `0..36` range
- the fixed European colour table
- deterministic settlement requirements
- a threat-model table mapping roulette-stage cheating risks to controls, current foundation support, and remaining adapter work
- an honest mapping of current TN10 Toccata/foundation capabilities and limits
- the first roulette PoC boundary as an adapter on top of the foundation
- an acceptance checklist for future roulette implementation

## Foundation trust mapping used by the spec

The spec maps the current foundation layer to roulette as follows:
- readiness input: `scripts/env074-toccata-layer-ready.sh`
- readiness result cited: `TOCCATA_LAYER_READY=PASS`
- app-facing verifier JSON source: `cargo run -p kaspa-fair-cli -- verify-live-tn10-canonical --json`
- canonical live trust facts cited: accepted ENV-064 spend, confirmed ENV-063 input relationship, confirmed continuing output/value/covenant id, and explicit read-only/no-signing/no-broadcast/no-wallet flags

## Safety boundary preserved

ENV-075 remained specification-only work.

Confirmed out of scope:
- no roulette implementation
- no web app
- no signing
- no transaction creation
- no submitting or broadcasting
- no wallet or private-key access
- no mainnet
- no secrets

## Verification result

All required verification commands passed:
- `cargo fmt --check`
- `cargo test -p kaspa-fair-cli`
- `cargo test -p kaspa-foundation`
- `cargo check -p kaspa-fair-cli`
- `cargo check -p kaspa-foundation`
- `git diff --check`
