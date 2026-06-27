# ENV-080 — UI mock bet placement and reset flow

Result: PASS

Concrete deliverable:
- UI path: `/root/kaspa-fair-foundation/examples/roulette-poc/ui`
- smoke script: `/root/kaspa-fair-foundation/scripts/env080-roulette-ui-bet-flow-smoke.sh`
- smoke script exit status: `0`
- final readiness line: `ROULETTE_UI_BET_FLOW_READY=PASS`
- sample round JSON: `/root/kaspa-fair-foundation/examples/roulette-poc/ui/sample-round.json`
- UI bet-flow smoke artifact: `/root/kaspa-fair-foundation/spikes/kaspa-foundation/artifacts/env-080-roulette-ui-bet-flow/env-080-ui-bet-flow-smoke-output.txt`

UI bet-flow contract:
- Place Mock Bet control implemented: yes
- bets allowed before wheel start: yes
- bets allowed during SpinVisualStarted: yes
- bets blocked after NoMoreBets: yes
- blocked-bet message implemented: yes
- reset/new-round implemented: yes
- reset works without page refresh: yes
- result still loaded from sample JSON only: yes
- Math.random absent: yes
- crypto random APIs absent: yes

Roulette UI result:
- round_id: `env-077-round-0001`
- round_state: `ProofPublished`
- result_number: `21`
- result_colour: `red`
- final_result: `PASS`

Tests/checks:
- `cargo fmt --check` — PASS
- `cargo test -p kaspa-fair-cli` — PASS
- `cargo test -p kaspa-foundation` — PASS
- `cargo check -p kaspa-fair-cli` — PASS
- `cargo check -p kaspa-foundation` — PASS
- `git diff --check` — PASS
- `node --check examples/roulette-poc/ui/app.js` — PASS
- `scripts/env080-roulette-ui-bet-flow-smoke.sh` — PASS

Safety confirmation:
- mock display only
- no real betting
- no real payouts
- no backend/custody
- no signing
- no transaction creation
- no submitting/broadcasting
- no wallet/private key access
- no mainnet
- no secrets added
