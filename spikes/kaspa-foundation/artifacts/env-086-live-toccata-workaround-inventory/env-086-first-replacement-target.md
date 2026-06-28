# ENV-086 — First live TN10/Toccata replacement target

## Recommended next implementation ENV

ENV-087 — Authorised TN10 round-specific commitment/reveal transaction spike

Exactly one next implementation ENV is recommended.

## Target name

Authorised TN10 round-specific commitment/reveal transaction spike.

## Workaround removed

This removes the highest-priority current workaround: round commitment and reveal evidence are local/demo proof transcript fields rather than live round-specific TN10/Toccata evidence.

It also begins replacing the live TN10 anchor limitation by moving from a reusable canonical `live_readonly_tn10` anchor to per-round commitment/reveal evidence.

## Exact TN10/Toccata feature involved

- TN10 testnet transaction creation for a minimal round commitment transaction.
- TN10 testnet transaction creation for a linked reveal transaction.
- KIP-20 covenant ID / covenant lineage evidence for proving the commitment and reveal belong to the expected Toccata covenant lineage.
- KIP-17 covenant transition/introspection path where available for constraining or modelling the commitment-to-reveal state transition.
- Public/read-only TN10 evidence retrieval after creation, using the existing verifier transport pattern for transaction detail, accepted status, covenant_id, authorizing input/output, and accepting block evidence.

## Exact safety boundary

ENV-087 must not start transaction work until the user explicitly authorises crossing the tx/sign/broadcast boundary.

Allowed only after authorisation:

- TN10/testnet-10 only.
- Minimal faucet-funded or explicitly provided testnet funding path only.
- Minimal round-specific commitment/reveal transaction spike only.
- Read-only verifier evidence retrieval after transactions are created.

Still forbidden:

- mainnet;
- real betting;
- real payouts;
- custody/backend casino operation;
- production randomisation claim;
- UI result generation;
- wallet/private-key persistence in the repo;
- committing secrets;
- broad wallet/product integration beyond the minimal authorised spike.

## Explicit user authorisation required

Yes. ENV-087 requires explicit user authorisation because it crosses transaction creation, signing, and broadcast/submission. It may also require faucet funds and wallet/private-key handling for TN10 only.

## Minimum success criteria

ENV-087 should PASS only if it proves all of the following without changing the safety boundary:

1. A round-specific commitment transaction is created on TN10 after explicit authorisation.
2. The commitment transaction carries or links the round commitment hash and expected round_id.
3. The commitment transaction has accepted/readable TN10 evidence.
4. A reveal transaction is created on TN10 after the commitment transaction.
5. Reveal evidence is linked to the commitment by round_id, commitment hash, covenant_id/lineage, and transaction order or accepted evidence.
6. Read-only verifier output proves commitment-before-reveal relationship for the live round evidence.
7. Verifier-owned artifacts can export app-facing JSON mirrors from retrieved live evidence.
8. All safety flags remain false for mainnet, real betting, payouts, custody, and production casino claims.

## Commands/checks that should prove PASS

Exact commands should be finalised in ENV-087 after the authorised transaction approach is chosen, but the minimum proof suite should include:

```bash
cargo fmt --check
cargo check -p kaspa-foundation
cargo check -p kaspa-fair-cli
cargo test -p kaspa-foundation
cargo test -p kaspa-fair-cli
cargo run -q -p kaspa-fair-cli -- <env087-live-round-commitment-reveal-verifier> --json
scripts/env087-authorised-tn10-round-commitment-reveal-smoke.sh
```

The smoke script should assert a readiness line equivalent to:

```text
AUTHORISED_TN10_ROUND_COMMITMENT_REVEAL_READY=PASS
```

## What must not be included

ENV-087 must not include another bridge/planning/package/UI-polish step. It must not implement production entropy ahead of live commitment/reveal anchoring. It must not add real betting, real payouts, custody, mainnet, or UI-owned proof generation. It must not commit wallet/private-key material or secrets.

## Why this is the highest-value next target

The inventory shows that the core remaining proof gap is not display or packaging. The central missing capability is live round-specific commitment/reveal evidence under TN10/Toccata covenant lineage. Replacing that gap moves the PoC from static/demo/local proof transcript handling toward the project thesis: Toccata-linked proof-of-fairness evidence that a verifier can inspect independently.

## Why this is not another bridge/planning ENV

The next ENV is an implementation spike with concrete live TN10/Toccata evidence as its output. It must either create authorised round-specific commitment/reveal transactions and verify them read-only, or fail with exact transaction/verifier evidence. It is not a planning, packaging, or UI-polish milestone.
