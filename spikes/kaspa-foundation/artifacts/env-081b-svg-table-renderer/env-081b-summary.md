# ENV-081B — Finalise roulette table selection UI

Result: PASS

## Concrete deliverable

The roulette table is the betting input surface. All valid roulette table zones use one consistent additive click-to-chip interaction model.

## UI contract

- Bet type buttons above the table are removed.
- No giant inside-zone list is present.
- No dropdown-only inside betting workflow is present.
- Straight, zero, split, corner, street, six-line, outside, dozen, and column bet regions are represented by table zones.
- Column selectors render as `2 to 1` labels.
- Placed bets create chip markers and ledger entries.
- Repeated bets at the same anchor stack/fan instead of replacing each other.
- Chips do not intercept further table clicks.
- Large selected/focus/doughnut marker behavior is removed.
- The mock-bet ledger has a fixed-height scroll region so the table does not jump vertically when bets are added.
- The SVG viewBox has extra top padding so top-row street selectors are not clipped.

## Required command results

- PASS `cargo fmt --check`
- PASS `cargo test -p kaspa-fair-cli`
- PASS `cargo test -p kaspa-foundation`
- PASS `cargo check -p kaspa-fair-cli`
- PASS `cargo check -p kaspa-foundation`
- PASS `git diff --check`
- PASS `node --check examples/roulette-poc/ui/app.js`
- PASS `node --check examples/roulette-poc/ui/roulette-table-renderer.js`
- PASS `scripts/env081b-svg-table-renderer-smoke.sh`

Final readiness line:

```text
ROULETTE_SVG_TABLE_RENDERER_READY=PASS
```

## Safety

Mock display only. No real betting, no real payouts, no backend/custody, no signing, no transaction creation, no broadcasting/submitting, no wallet/private-key access, no mainnet, and no secrets.
