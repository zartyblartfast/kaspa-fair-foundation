# ENV-090-UI-FIX — Accept authorised live KIP-17 proof artifact in roulette UI

Result: PASS

Scope:
- Updated `examples/roulette-poc/ui/app.js` proof validation to use a version-aware app-facing proof safety contract.
- Added `scripts/env090-ui-accepts-kip17-proof-smoke.sh` to validate the ENV-090 UI contract and unsafe-proof rejection.
- Updated legacy UI/proof smoke scripts `env083e` and `env083f` so they validate the current ENV-090 app-facing proof instead of rejecting it as stale live evidence.

Current accepted proof requirements:
- `source_env = ENV-090`
- `verifier_result = PASS`
- `claim_level = full_kip17_covenant_enforced_transition`
- `network = testnet-10` / `evidence_mode = live_readonly_tn10`
- `mainnet_supported = false`
- no real betting, real payouts, backend custody, or production randomness claim
- live commitment/reveal transaction evidence is present and linked
- KIP-17 enforcement and invalid-transition rejection are represented as true
- `sample-round.json` agrees with `toccata-fairness-proof.json` on result number, colour, and algorithm

Unsafe proof cases validated by the new smoke:
- mainnet enabled
- real betting
- real payouts
- backend custody
- production randomness claim
- verifier failure
- unknown source_env
- unsupported live claim_level
- missing live commitment evidence
- mismatched result
- secret-like UI material

Safety boundary:
This was a UI validation fix only. No transaction creation, signing, broadcasting, wallet/private-key access, mainnet use, real betting, real payouts, backend/custody, or production randomness was added.
