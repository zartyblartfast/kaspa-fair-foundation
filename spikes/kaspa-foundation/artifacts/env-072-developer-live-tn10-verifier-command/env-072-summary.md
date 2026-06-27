# ENV-072 — Developer-facing live TN10 verifier command

## Result

PASS.

ENV-072 adds an operational developer command for the canonical live TN10 read-only proof verifier:

```bash
cargo run -p kaspa-fair-cli -- verify-live-tn10-canonical
```

A convenience wrapper is also available:

```bash
scripts/env072-live-tn10-verify.sh
```

## What changed

- `kaspa-fair-cli` now has a real command instead of a placeholder binary.
- The command reads public TN10 state through read-only wRPC and transaction-detail API calls.
- The command calls the `kaspa-foundation` online verifier library directly with normalized live evidence.
- The command prints a clear `PASS` / `PARTIAL` / `FAIL` result and key checks.
- Normal tests remain deterministic; no normal test depends on live TN10 availability.

## Live command evidence

Command:

```bash
cargo run -p kaspa-fair-cli -- verify-live-tn10-canonical
```

Exit status: 0

Final verifier result: PASS

Observed live values:

- wRPC endpoint used by resolver: `wss://boson-10.kaspa.red/kaspa/testnet-10/wrpc/borsh`
- transaction detail API: `https://api-tn10.kaspa.org/transactions/4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c?inputs=true&outputs=true&resolve_previous_outpoints=light`
- ENV-064 accepted: `true`
- accepting block hash: `e0d62ead241a5217769266dc96e8055c5893c29074ed2c50ba23de1a9ba75190`
- ENV-063 input relationship confirmed: `true`
- continuing output exists: `true`
- continuing output value: `99700000 sompi`
- covenant id confirmed: `e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7`

Full output is in `env-072-live-command-output.txt`.

## Safety

ENV-072 remains inside the limited Toccata foundation boundary:

- read-only TN10 only
- no signing
- no transaction creation
- no submitting or broadcasting
- no wallet/private-key access
- no secrets
- no mainnet
- no roulette implementation

## Verification

See `env-072-test-output.txt` for:

- `cargo fmt --check`
- `cargo test -p kaspa-foundation`
- `cargo test -p kaspa-fair-cli`
- `cargo check -p kaspa-foundation`
- `cargo check -p kaspa-fair-cli`
- `git diff --check`

See `env-072-live-command-output.txt` for the developer-facing live TN10 command run.
