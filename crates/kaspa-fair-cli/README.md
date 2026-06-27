# kaspa-fair-cli

Developer-facing commands for the Kaspa Fair Foundation workspace.

## Live TN10 canonical verifier

```bash
cargo run -p kaspa-fair-cli -- verify-live-tn10-canonical
```

This command verifies the canonical ENV-063/064/065 proof transcript against public TN10 read-only chain data. It calls the `kaspa-foundation` online verifier library directly and prints the wRPC endpoint, transaction-detail API URL, ENV-064 acceptance status, accepting block hash, ENV-063 input relationship, continuing output existence/value, covenant id, and final verifier result.

Exit behavior:

- `PASS`: exit 0
- `FAIL`: non-zero
- `PARTIAL`: non-zero ambiguous/partial result

Safety boundary: read-only TN10 only; no signing; no transaction creation; no submitting/broadcasting; no wallet/private-key access; no secrets; no mainnet; no roulette implementation.

A convenience wrapper is available from the repository root:

```bash
scripts/env072-live-tn10-verify.sh
```
