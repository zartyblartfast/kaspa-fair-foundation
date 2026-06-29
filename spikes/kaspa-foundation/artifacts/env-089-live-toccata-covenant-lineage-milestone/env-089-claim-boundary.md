# ENV-089 claim boundary

## Claimed

- Live TN10 KIP-20 covenant-linked lineage.
- Commitment transaction has non-null covenant evidence.
- Reveal/continuation transaction spends the covenant-bound commitment output.
- Covenant evidence comes from direct TN10 transaction fields, not payload JSON.

## Identifiers

- Commitment transaction: `ebb28c6b34532cb97ae3a0a135fda74a0566b336df4dbf248283c5cad8c9ff65`.
- Reveal/continuation transaction: `f8fe14932071ac49cdac9e4f3df1177b9655dffbd0ad66b0e7491d6f78e5654b`.
- Covenant ID: `9931b78d93e1019ed132d52ccc8dc0b812b7fb5fa41cb561342c184afd11735c`.

## Not claimed

- Full KIP-17 covenant-enforced state transition.
- Production randomness.
- Real betting.
- Real payouts.
- Mainnet.
- Backend custody.
- Any new transaction, signing, broadcast, wallet/private-key access, or live action during ENV-089.

## Future work

Full KIP-17 covenant-enforced state transition remains future work because this milestone proves covenant-linked lineage and transaction-field continuity, not complete covenant-enforced application state-machine rules.
