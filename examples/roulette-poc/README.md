# Roulette PoC

ENV-081B rebuilds the roulette PoC betting surface as a schema-driven SVG renderer.

Primary UI files:
- `examples/roulette-poc/ui/index.html`
- `examples/roulette-poc/ui/styles.css`
- `examples/roulette-poc/ui/app.js`
- `examples/roulette-poc/ui/roulette-table-schema.json`
- `examples/roulette-poc/ui/roulette-table-renderer.js`

What ENV-081B does:
- renders the visible roulette betting surface from the ENV-081A schema in `roulette-table-schema.json`
- uses an SVG table layout with the standard European shape
- keeps `0` as the dedicated green region on the left
- renders the 12 × 3 number grid with rows `3,6,9,...,36`, `2,5,8,...,35`, and `1,4,7,...,34`
- renders dozens, outside bets, and column selector regions from schema geometry
- keeps straight number cells clickable on the table
- keeps dozens, columns, and outside bets clickable on the table
- uses compact table-overlay modes for split, street, corner, and six-line hotspots
- adds visible chip markers and a UI ledger for mock bets only
- preserves the deterministic result, settlement, and proof from `sample-round.json` only

What ENV-081B explicitly avoids:
- giant inside-zone lists
- dropdown-only inside-zone betting
- result generation or randomisation
- real betting
- real payouts
- wallet integration
- backend custody or accounts
- signing
- transaction creation
- submitting or broadcasting
- mainnet
- secrets
- production casino functionality

Readiness command:

```bash
scripts/env081b-svg-table-renderer-smoke.sh
```
