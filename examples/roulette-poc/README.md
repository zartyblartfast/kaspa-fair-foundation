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
scripts/env084-verifiable-demo-round-generator-smoke.sh
```

## ENV-084 demo round generation

ENV-084 adds Rust-owned verifiable demo round generation from explicit demo seed material.

Generate the app-facing demo artifacts together:

```bash
cargo run -q -p kaspa-fair-cli -- env084-generate-verifiable-demo-round \
  --round-id env-084-demo-round-0001 \
  --demo-seed "env084-demo-seed-0001" \
  --write-ui
```

Generate into a separate output directory for checks without changing the UI files:

```bash
cargo run -q -p kaspa-fair-cli -- env084-generate-verifiable-demo-round \
  --round-id env-084-demo-round-0001 \
  --demo-seed "env084-demo-seed-0001" \
  --out-dir spikes/kaspa-foundation/artifacts/env-084-verifiable-demo-round-generator/generated-check
```

The command writes matching `sample-round.json` and `toccata-fairness-proof.json` from the same proof transcript. The Rust verifier confirms that the round ID, result number, result colour, result algorithm, commitment/reveal fields, live read-only TN10 anchor evidence, future-live-transaction status, and safety flags agree.

The UI still does not choose or generate the result. It loads the generated JSON files and displays the result only from those files. The browser code must not use `Math.random` or browser crypto random APIs for roulette result selection.

The `--demo-seed` value is explicit demo seed material. It is useful for repeatable verifiable demos, and a different explicit value may produce a different visible roulette result. It is not production randomness, does not prove unbiased seed selection, and does not remove seed-grinding risk.

Round and proof artifacts must be generated together because JSON is only a mirror/export format. Rust verifier logic is the authority for whether the app-facing round and proof agree.

## ENV-092 live TN10 entropy round

ENV-092 adds a TN10/testnet-only live proof path:

```bash
cargo run -q -p kaspa-fair-cli -- env092-tn10-verifiable-entropy-round \
  --round-id env-092-entropy-round-0001 \
  --operator-seed env092-operator-seed-0001 \
  --network tn10 \
  --entropy-delay-blue-score 30 \
  --preflight-only \
  --json
```

The live `--broadcast` mode is an authorised transaction path only. It records a KIP-17 commitment transition, a NoMoreBets transition that fixes a future TN10 blue-score target, a direct TN10 future block-hash readback, and a reveal transition. The app-facing JSON displays the resulting proof and result; the UI still does not choose entropy or generate the result.

This is not production casino randomness, real betting, real payouts, custody, mainnet support, or a production casino deployment.
