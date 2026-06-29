# ENV-091 claim boundary

## Claimed

ENV-091 documents and verifies the current achieved state:

- live TN10 transaction flow exists for the ENV-090 round evidence;
- KIP-20 covenant lineage is present in the direct TN10 covenant fields;
- full KIP-17 covenant-enforced state transition is the ENV-090 claim level;
- the valid commitment-to-reveal/continuation transition is represented as passing;
- invalid transition states are rejected;
- direct TN10 readback supports the ENV-090 artifacts;
- app-facing UI accepts the authorised ENV-090 full KIP-17 proof artifact;
- unsafe proof states remain rejected by the UI proof validation smoke.

## Evidence identifiers

- commitment txid: `050bbe398ff7e8f7511697c65b511ab23bf1548bcba1ed0fb29380d1e582ec26`
- reveal/continuation txid: `269abfe10635d666d0c5b7624550a4abee5a47a8bd08d6a0e0b1a09dc2cf0620`
- claim level: `full_kip17_covenant_enforced_transition`
- verifier_result: `PASS`
- `kip17_rule_enforced_on_transition = true`
- `invalid_transition_rejected = true`

## Not claimed

ENV-091 does not claim:

- production randomness;
- unbiased production-grade casino entropy;
- real betting;
- real payouts;
- wallet integration;
- backend custody;
- production casino operation;
- mainnet;
- UI-generated roulette results;
- new transaction creation during ENV-091;
- signing during ENV-091;
- broadcast during ENV-091.

## Safety boundary

ENV-091 is packaging/documentation/checks only. It preserves the existing ENV-090 proof state and creates a reviewable milestone bundle. It does not modify Rust, CLI, UI, app-facing JSON, wallet/signing/broadcast code, or mainnet configuration.

## Next development area

The next real development area is verifiable entropy/randomisation: binding the roulette result derivation to auditable entropy instead of relying on explicit demo seed material or UI random numbers.
