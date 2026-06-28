# Roulette PoC

ENV-083E connects the roulette PoC UI to a static app-facing Toccata fairness proof artifact while preserving the ENV-082 table-first Start Wheel / Reset Round workflow and the ENV-083D proof explanation.

Current milestone:

Rust verifier binds a roulette proof transcript to real TN10 Toccata covenant evidence. Live round-specific commitment/reveal transactions remain future work and require explicit authorisation.

Primary UI files:
- `examples/roulette-poc/ui/index.html`
- `examples/roulette-poc/ui/styles.css`
- `examples/roulette-poc/ui/app.js`
- `examples/roulette-poc/ui/roulette-table-schema.json`
- `examples/roulette-poc/ui/roulette-table-renderer.js`
- `examples/roulette-poc/ui/toccata-fairness-proof.json`

What the current UI does:
- renders the visible roulette betting surface from the ENV-081A schema in `roulette-table-schema.json`
- uses an SVG table layout with the standard European shape
- keeps `0` as the dedicated green region on the left
- renders the 12 × 3 number grid with rows `3,6,9,...,36`, `2,5,8,...,35`, and `1,4,7,...,34`
- renders dozens, outside bets, and column selector regions from schema geometry
- keeps straight number cells clickable on the table
- keeps dozens, columns, and outside bets clickable on the table
- uses compact table hotspot selectors for split, street, corner, and six-line bets
- adds visible chip markers and a UI ledger for mock bets only
- keeps the table before workflow, ledger, settlement, proof, verifier, and safety information
- uses a status label rather than status-like workflow buttons
- automatically advances from wheel spin to no-more-bets, result reveal, settlement display, and proof publication
- preserves the mock roulette round/result display from `sample-round.json`
- loads Rust verifier/Toccata proof snapshot fields from `toccata-fairness-proof.json`
- explains that the UI displays a mock roulette round and does not choose the result
- displays a compact verifier proof snapshot with verifier result, evidence mode, covenant ID confirmation, result algorithm, result number/colour, future live transaction evidence, and safety flags
- explains that the proof is checked by Rust verifier logic
- explains commitment/reveal consistency and deterministic BLAKE3 result derivation
- explains that the proof transcript is bound to live TN10 Toccata covenant evidence
- explains the two-tier thesis: Kaspa public PoW DAG evidence first, Toccata covenant lineage/state-transition evidence second
- warns users not to trust the UI alone or the operator alone, and to verify the proof
- discloses that commitment/reveal does not by itself prove production-grade unbiased randomness and that seed/entropy hardening remains future work

What the current UI explicitly avoids:
- manual result reveal controls
- old wheel visual / multi-button round-control workflow
- giant inside-zone lists
- dropdown-only inside-zone betting
- result generation or randomisation
- live round-specific commitment/reveal transaction creation
- real betting
- real payouts
- wallet integration
- backend custody or accounts
- signing
- transaction creation
- submitting or broadcasting
- faucet funds
- mainnet
- secrets
- production casino functionality

Readiness commands:

```bash
scripts/env083d-user-facing-fairness-proof-explanation-smoke.sh
scripts/env083e-app-facing-fairness-proof-artifact-smoke.sh
```
