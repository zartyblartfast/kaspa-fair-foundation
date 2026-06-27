# Roulette PoC

ENV-079 upgrades the roulette UI at `examples/roulette-poc/ui/` into an interactive static UI flow prototype.

What it does:
- demonstrates the correct round-state sequence in the browser UI with intentionally minimal visual polish
- starts in `BetsOpen`
- lets the user trigger `Start Wheel`, `No More Bets`, `Reveal Result`, `Show Settlement`, and `Publish Proof`
- keeps bets visually open while the wheel is spinning
- finalises the displayed result only after `NoMoreBets`
- displays settlement and proof panels from deterministic engine JSON
- shows trust/safety status from the foundation verifier fields
- explicitly states `spin animation != result finalisation`
- consumes deterministic engine JSON from `sample-round.json`

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
scripts/env079-roulette-ui-flow-smoke.sh
```
