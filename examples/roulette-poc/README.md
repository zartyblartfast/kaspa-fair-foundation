# Roulette PoC

ENV-081A defines the declarative European roulette table layout schema at `examples/roulette-poc/ui/roulette-table-schema.json`.

What it does:
- defines the standard European roulette layout as schema data only
- keeps `0` as a dedicated green region on the left of the main number grid
- defines all 37 straight number cells with coordinates and chip anchors
- defines dozens, outside even-money bets, and explicit column selector regions
- defines future hotspot geometry for split, street, corner, and six-line bets
- provides a stable schema that can drive future SVG/table-hotspot rendering
- keeps the current UI rebuild deferred to ENV-081B
- explicitly avoids giant inside-zone lists
- explicitly avoids dropdown-based inside-zone betting

What it does not do:
- rebuild the betting UI yet
- render the final SVG betting surface yet
- decide, generate, or randomise the result
- real betting
- real payouts
- wallet integration
- backend custody or accounts
- signing
- transaction creation
- submitting/broadcasting
- mainnet
- production casino functionality

Readiness command:

```bash
scripts/env081a-roulette-table-schema-smoke.sh
```
