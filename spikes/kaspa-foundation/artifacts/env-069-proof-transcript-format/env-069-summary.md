# ENV-069 — Proof transcript format

## What

ENV-069 defines a stable, versioned proof transcript layer for the canonical corrected TN10 covenant path already proven by ENV-063, ENV-064, and ENV-065.

It adds:

- a foundation transcript module under `crates/kaspa-foundation/src/transcript/`
- stable schema identifiers `kaspa-fair-transcript-v1` and `kaspa-fair-evidence-v1`
- a canonical TN10 proof transcript model with ordered ENV steps and safety flags
- fixture-backed integration tests for canonical values and fixture links
- a sample JSON proof transcript artifact

## Why

This separates reusable covenant proof evidence from product/demo logic.

The result is a foundation-first transcript layer that later work can use for:

- offline verifier implementation first
- online/read-only verifier work later
- future app adapters without embedding roulette logic into the covenant core

## Canonical facts captured

- ENV-064 spend txid: `4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c`
- ENV-063 input outpoint: `2c7802ff9a6eec2828a96168d8f62a9a276176441ed8cb6086cd5d5d0cb26849:0`
- continuing output: `4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c:0`
- continuing output value: `99700000 sompi`
- covenant id: `e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7`

## Ordered transcript steps

1. `ENV-063` create evidence
2. `ENV-064` spend evidence
3. `ENV-065` read-only confirmation evidence

## Safety boundary

The transcript layer remains offline/read-only evidence only:

- no secrets
- no wallet
- no signing
- no network requirement
- no broadcast/submission
- no mainnet
- no roulette implementation
