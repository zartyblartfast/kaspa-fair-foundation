# Roulette PoC milestone summary

The current Roulette PoC is packaged as a static/mock verifiable demo.

## Plain-English summary

The spin animation is not trusted. It is only presentation.

The UI does not choose the result. The browser loads generated JSON artifacts and displays what those artifacts say.

Rust generates and verifies the demo result. A Rust CLI command accepts an explicit round ID and explicit demo seed material, derives the roulette number with BLAKE3 rejection sampling, derives the colour from the fixed European roulette mapping, and writes the sample round and proof artifact together.

The proof artifact is tied to live read-only TN10 Toccata covenant evidence. That anchor evidence proves the project can inspect the relevant TN10 covenant lineage fields without creating, signing, or broadcasting transactions.

The current version is mock-only. It has a static UI, mock chips, mock ledger display, generated demo JSON, and verifier output. It does not operate a casino.

## Safety boundary

No money is involved.

No wallet is involved.

No payout is executed.

No signing happens.

No transaction is created, submitted, or broadcast.

No mainnet is used.

No production randomness is claimed.

No real betting, custody, backend account system, or house bankroll exists.

## What is demonstrated

- The UI can display a roulette round without being trusted to choose the result.
- Rust owns demo round generation and proof validation.
- The sample round and Toccata proof artifact agree.
- The proof references live read-only TN10 Toccata covenant evidence.
- A reviewer can re-run the generator and smoke checks to verify the current package.

## What remains future work

Live round-specific commitment/reveal transactions are not implemented yet. They require explicit authorisation because they cross into transaction creation, wallet/faucet/signing/broadcast concerns.

Production entropy design is also future work. Explicit demo seed material is useful for repeatable demos, but it is not production randomness.

## Next real decision

A. Stop/package as demo.

B. Request explicit authorisation for a live TN10 round-specific commitment/reveal transaction spike.
