# ENV-083F — Verifiable demo round generation path

## Purpose

ENV-083F-FIX closes the app-facing consistency gap between the static roulette sample round and the Toccata fairness proof artifact. The Rust/Toccata-bound proof artifact is the stronger source of truth. The displayed sample round has therefore been aligned to the verified proof result instead of weakening or backfitting the proof artifact to the older placeholder result.

## A. Current fixed state after ENV-083F-FIX

- `examples/roulette-poc/ui/sample-round.json` and `examples/roulette-poc/ui/toccata-fairness-proof.json` now agree on the displayed roulette result.
- The agreed result is `18 red` under `blake3-domain-separated-rejection-sampling-v1`.
- The UI displays static/mock artifacts only.
- The UI loads the sample round from `sample-round.json`.
- The UI loads the app-facing proof snapshot from `toccata-fairness-proof.json`.
- The UI does not generate, choose, mutate, or randomise the roulette result.
- The Rust/Toccata proof artifact remains the proof source for verifier status, Toccata evidence mode, covenant lineage, deterministic derivation status, and safety flags.
- The Toccata anchor evidence remains `live_readonly_tn10`.
- Future live round-specific commitment/reveal transaction evidence remains labelled `not_created_not_claimed_future_work`.

## B. Unsafe path rejected

The project must not address the fixed-result issue by moving result selection into the browser.

Rejected paths:

- UI randomises the result.
- UI calls `Math.random`.
- UI calls browser crypto random APIs to choose a result.
- UI displays a result first and backfills proof data later.
- Proof data does not bind to Rust verifier checks and Toccata evidence.
- JSON artifacts tell conflicting stories about result number, colour, verifier status, evidence mode, or safety boundaries.

These paths would reintroduce trust in the UI/operator and would undermine the project thesis that the proof layer, not the spin animation, is the product.

## C. Acceptable next implementation path

The next safe implementation path is Rust-owned artifact generation.

Required shape:

1. A Rust CLI command generates a demo round artifact.
2. The Rust CLI either generates demo seed material or accepts explicit demo seed material from the operator/test harness.
3. The Rust verifier derives the roulette result using the existing BLAKE3 domain-separated rejection-sampling algorithm.
4. The proof artifact and sample round artifact are produced together from one transcript.
5. The generated artifacts include result number, result colour, result algorithm, verifier status, evidence mode, safety flags, and future-live-transaction boundary fields from the same source data.
6. The UI loads the produced artifacts but does not choose the result.
7. The Toccata anchor remains `live_readonly_tn10` unless and until live round-specific transactions are explicitly authorised.
8. Output is clearly labelled demo-only and must not be described as production casino randomness.

The artifact generator should fail closed if the sample round and proof artifact would disagree.

## D. Entropy position

Version 1 may use operator/demo seed material for repeatable demonstration output, but the grinding caveat must be disclosed:

- commitment/reveal proves the seed was not changed after commitment;
- deterministic derivation proves the displayed result follows from the revealed seed;
- operator/demo seed material does not prove unbiased seed selection;
- operator/demo seed material does not fully prevent seed grinding.

Version 2 should bind entropy to independent, public, or multi-party material such as:

- a future TN10 chain event selected under a carefully specified sampling rule;
- a user-provided seed;
- a multi-party seed contribution protocol;
- another independently auditable entropy source.

No implementation may claim production-grade unbiased casino randomness until the entropy design is completed and verified.

## E. Future live transaction boundary

Live round-specific commitment/reveal transaction creation requires explicit user authorisation.

The following remain out of scope until authorised:

- wallet access;
- faucet funds;
- private key access;
- signing;
- transaction creation;
- transaction submission or broadcast;
- mainnet;
- real betting;
- real payouts;
- custody/backend casino operation.

## F. Project-control conclusion

ENV-083F is the final bridge/gate ENV for the static artifact consistency phase. The next action must be one of:

- A: ENV-084 Rust-owned verifiable demo round generator.
- B: ENV-084 authorised live TN10 round-specific commitment/reveal transaction spike.
- C: stop feature development and package the current PoC as a milestone demo.

Recommended next action: A.

No further planning-only ENV should be recommended unless a concrete blocker is found.
