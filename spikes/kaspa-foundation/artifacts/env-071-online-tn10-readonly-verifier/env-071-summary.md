# ENV-071C — Live TN10 transaction-detail verification

Result: PASS

ENV-071C completes the live read-only TN10 verifier by adding a read-only transaction-detail API path for mined/accepted ENV-064 transaction structure. The previous wRPC path remains useful for TN10 node readiness and live UTXO visibility, but the public transaction-detail API provides the mined transaction input/output/covenant fields needed for a decisive verifier result.

## Read-only sources used

- Public rusty-kaspa wRPC resolver defaults:
  - `kaspa_wrpc_client::KaspaRpcClient`
  - `WrpcEncoding::Borsh`
  - `Resolver::default()`
  - `NetworkId::with_suffix(NetworkType::Testnet, 10)`
  - `ConnectStrategy::Fallback`
- Resolved wRPC endpoint in the captured run: `wss://vector-10.kaspa.green/kaspa/testnet-10/wrpc/borsh`
- Public TN10 transaction-detail API:
  - `https://api-tn10.kaspa.org/transactions/4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c?inputs=true&outputs=true&resolve_previous_outpoints=light`

## Live verification result

- ENV-064 mined/accepted transaction retrieved: yes
  - HTTP status: `200 OK`
  - `is_accepted=true`
  - accepting block hash: `e0d62ead241a5217769266dc96e8055c5893c29074ed2c50ba23de1a9ba75190`
- ENV-063 input relationship confirmed: yes
  - previous outpoint hash: `2c7802ff9a6eec2828a96168d8f62a9a276176441ed8cb6086cd5d5d0cb26849`
  - previous outpoint index: `0`
- Continuing output confirmed: yes
  - output index: `0`
  - outpoint: `4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c:0`
- Continuing output value confirmed: yes, `99700000` sompi
- Covenant id confirmed: yes, `e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7`
- Final verifier result: `Pass`

## Unsupported / skipped fields

None in the ENV-071C live verifier result. The old mempool lookup still returns not found for ENV-064, which is expected because ENV-064 is mined/non-mempool; it is no longer used as the source of transaction-detail truth.

## Safety confirmation

- read-only TN10 only
- no signing
- no transaction creation
- no submitting or broadcasting
- no wallet/private-key access
- no secrets
- no mainnet
- no roulette implementation

## Verification

All requested ENV-071C checks passed:

- `cargo fmt --check`
- `cargo test -p kaspa-foundation --test env071_online_tn10_readonly_verifier`
- `KASPA_FOUNDATION_RUN_LIVE_TN10=1 cargo test -p kaspa-foundation --test env071_online_tn10_readonly_verifier -- --ignored --nocapture live_tn10_readonly_verification_is_optional_and_gated`
- `cargo check -p kaspa-foundation`
- `git diff --check`
