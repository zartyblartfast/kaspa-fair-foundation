# Roulette PoC

ENV-078 adds a simple static UI prototype at `examples/roulette-poc/ui/`.

What it does:
- displays the existing deterministic ENV-077 round JSON
- shows trust/safety status from the foundation verifier fields
- shows the round sequence, roulette table, mock bets, settlement, and proof panel
- treats wheel/spin as visual only

What it does not do:
- decide or randomise the result
- real betting
- real payouts
- wallet integration
- signing
- transaction creation
- submitting/broadcasting
- custody or accounts
- mainnet
- production casino functionality

Readiness command:

```bash
scripts/env078-roulette-ui-smoke.sh
```
