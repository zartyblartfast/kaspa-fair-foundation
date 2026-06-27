# Roulette PoC

ENV-080B cleans up the roulette UI at `examples/roulette-poc/ui/` so the current mock bet control is explicitly temporary and intentionally limited before ENV-081 table-zone work.

What it does:
- relabels the current control as a temporary simple/prototype straight-number mock bet only path
- visibly states that full roulette table bet zones are not implemented yet
- lists the future table-driven bet zones deferred to ENV-081: straight, split, street, corner, six-line, dozens, columns, red/black, odd/even, high/low
- allows temporary simple mock bets to be placed before wheel start in `BetsOpen`
- allows temporary simple mock bets to be placed during `SpinVisualStarted`
- blocks temporary simple mock bets only after `NoMoreBets`
- lets the user trigger `Start Wheel`, `No More Bets`, `Reveal Result`, `Show Settlement`, `Publish Proof`, and `Reset Round`
- resets the UI to `BetsOpen` without page refresh and clears UI-added mock bets
- keeps the deterministic result, settlement, and proof sourced from `sample-round.json`
- clearly states that UI-added bets are temporary prototype display bets only and deterministic settlement is from the engine sample round
- explicitly states `spin animation != result finalisation`
- defers proper table-driven roulette bet zones to ENV-081

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
scripts/env080b-roulette-ui-bet-cleanup-smoke.sh
```
