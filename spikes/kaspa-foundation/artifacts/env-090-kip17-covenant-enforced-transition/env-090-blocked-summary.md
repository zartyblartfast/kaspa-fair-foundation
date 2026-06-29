# ENV-090 — Full KIP-17 covenant-enforced state transition

Result: BLOCKED

ENV-090 implemented a KIP-17 covenant/introspection construction path based on the rusty-kaspa Toccata covenant counter-state example and added a CLI command plus persistent smoke script. Preflight passed: TN10-only, mainnet excluded, KIP-17 construction tooling identified, safe helper key path outside repo identified, testnet-only UTXO availability identified, local KIP-17 VM positive transition passed, and local invalid transition checks failed as expected.

The live path did not reach PASS. The commitment transaction candidate was submitted once, but direct TN10 transaction readback did not return an accepted commitment transaction. The command stopped before reveal/continuation construction and broadcast.

Blocked commitment txid reported by CLI wait loop:

`050bbe398ff7e8f7511697c65b511ab23bf1548bcba1ed0fb29380d1e582ec26`

Direct TN10 readback after failure returned HTTP 404 / transaction not found.

Claim boundary:

- No ENV-090 PASS is claimed.
- No full KIP-17 covenant-enforced state transition is claimed.
- No KIP-20 lineage-only fallback is accepted.
- No bare-anchor fallback is accepted.
- No payload-only state transition claim is accepted.
- No local/model/offline/simulated enforcement is accepted as live evidence.

Minimum unblock action:

Use tooling/node support that exposes the concrete mempool/consensus rejection reason for the KIP-17 P2SH covenant commitment candidate, then adapt the transaction/script construction to TN10 acceptance rules and rerun only after explicit live transaction approval.
