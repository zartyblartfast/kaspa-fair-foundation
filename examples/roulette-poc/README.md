# Roulette PoC

ENV-080 upgrades the roulette UI at `examples/roulette-poc/ui/` with mock UI bet placement and reset flow.

What it does:
- adds visible mock bet placement with a `Place Mock Bet` control
- supports mock choices for straight number, red, black, odd, even, high, and low
- allows bets to be placed before wheel start in `BetsOpen`
- allows bets to be placed during `SpinVisualStarted`
- blocks bets only after `NoMoreBets`
- lets the user trigger `Start Wheel`, `No More Bets`, `Reveal Result`, `Show Settlement`, `Publish Proof`, and `Reset Round`
- resets the UI to `BetsOpen` without page refresh
- keeps the deterministic result, settlement, and proof sourced from `sample-round.json`
- clearly states that UI-added bets are mock-only and deterministic settlement is from the engine sample round
- explicitly states `spin animation != result finalisation`

What it does not do:
- decide, generate, or randomise the result
- real betting
- real payouts
- wallet integration
- signing
- transaction creation
- submitting/broadcasting
- backend custody or accounts
- mainnet
- production casino functionality

Readiness command:

```bash
scripts/env080-roulette-ui-bet-flow-smoke.sh
```
