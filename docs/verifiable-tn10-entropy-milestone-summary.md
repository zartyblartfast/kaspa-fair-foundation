# Verifiable TN10 entropy milestone summary

The roulette PoC result is no longer based only on a fixed demo seed.

The current milestone uses a live TN10 future chain value as part of the result transcript. In ENV-092, the operator seed was committed first, then the NoMoreBets transaction fixed a future TN10 blue-score target, and then the TN10 block hash at that future target was read back and used in the final entropy transcript.

Plain-English flow:

1. The operator commits to a seed before the future TN10 chain value is known.
2. The round reaches NoMoreBets.
3. NoMoreBets fixes the future entropy target blue score.
4. A TN10 block hash at that target is read back.
5. The verifier combines the revealed operator seed and TN10 future block hash in the transcript.
6. BLAKE3 rejection sampling derives the roulette result.
7. The Rust verifier recomputes the result and reports PASS.
8. The UI displays the resulting proof and result; it does not choose the result.

ENV-092 result:

```text
result_number: 34
result_colour: red
verifier_result: PASS
```

This is still a proof-of-concept checkpoint, not real gambling. There is no real betting, no real payouts, no custody/backend casino operation, no wallet integration, and no mainnet claim.

This is verifiable entropy for the PoC, not a production casino randomness certification. Operator abort/griefing, user or multi-party entropy, and optional KIP-21 sequencing/lane proof strengthening remain future work.
