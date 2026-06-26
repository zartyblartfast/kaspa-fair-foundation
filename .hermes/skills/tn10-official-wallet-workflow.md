# TN10 Official Rusty Kaspa Wallet Workflow

Purpose:
Use the official Rusty Kaspa tn10-toc3 wallet tooling safely for TN10/Toccata testnet work in kaspa-fair-lab.

Tool path:

/root/kaspa-fair-lab/tools/rusty-kaspa-releases/tn10-toc3/bin/kaspa-wallet

Known version:

Kaspa Cli Wallet v1.2.1-toc.3

Standard TN10 connection sequence inside kaspa-wallet:

network testnet-10
server public
connect

Read-only verification commands:

rpc get-server-info
rpc get-current-network
rpc get-sync-status
rpc get-block-dag-info

Important:
get-current-network only proves Testnet class. Use get-server-info or get-block-dag-info to confirm suffix 10.

Expected good server properties:

network_id: Testnet suffix 10
is_synced: true
has_utxo_index: true

Project wallet:

env052-tn10-test-only

Funded source address:

kaspatest:qrhszwr4r2ejukpxyjp7jvn40tth5s8zy0538zvkkrvtkxvvyhlmjhe275slx

TN10 faucet used successfully:

https://faucet-tn10.kaspanet.io/

Faucet txid:

29d76273819d519bea146e881554c633bac4d30989bfc8e1862fed965d8f5116

Ordinary send command pattern:

estimate <amount> <priority fee>
send <address> <amount> <priority fee>

Proven ordinary send:

amount: 1 TKAS
priority fee: 0
txid: c9b4532f217d66987997e972963ec5dbfa5a9e7bf18f3e38910763274fb05135

Recipient used in ENV-056:

kaspatest:qpf0pc97d4vxtd99gppqtzhrjtna4t396lvu2t249p9f0rkh05pxxc5mj9yf2

Observed post-send state:

wallet/account total: 999.997964 TKAS
recipient address balance: 1.0 TKAS
recipient UTXO: c9b4532f217d66987997e972963ec5dbfa5a9e7bf18f3e38910763274fb05135:0
change UTXO: c9b4532f217d66987997e972963ec5dbfa5a9e7bf18f3e38910763274fb05135:1

Safety rules:

- Never use mainnet.
- Never expose mnemonic, seed, private key, wallet password, or wallet file contents.
- Never tee or paste wallet creation transcripts if they may include secret material.
- Never commit files from /root/.kaspa/.
- Never commit wallet files or password files.
- Public artifacts may include public addresses, txids, balances, UTXO outpoints, command names, and non-secret server/network info.
- Public artifacts must not include wallet password, mnemonic, seed phrase, private key, or wallet file contents.

Known local-only wallet files:

/root/.kaspa/env052-tn10-test-only.wallet
/root/.kaspa/env052-tn10-test-only.password

Current project boundary:

- TN10 ordinary wallet/faucet/UTXO/sign/broadcast is proven.
- TN10 corrected live covenant create/spend/settlement is proven on the ENV-063/ENV-064/ENV-065 path.
- Corrected covenant create ENV: ENV-063, txid `2c7802ff9a6eec2828a96168d8f62a9a276176441ed8cb6086cd5d5d0cb26849`, output `2c7802ff9a6eec2828a96168d8f62a9a276176441ed8cb6086cd5d5d0cb26849:0`, covenant id `e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7`.
- Corrected covenant spend ENV: ENV-064, accepted spend txid `4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c`, continuing output `4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c:0`, value `99700000` sompi.
- Settlement confirmation ENV: ENV-065, continuing output visible and original ENV-063 input absent/spent.
- Old ENV-060C/ENV-061 covenant UTXO `f4941c478e9540c477e04d0a2dff7ab1b0d0d794a3ae453c8148d25d125fe53d:0` was the superseded v0-locked path and must not be treated as the final corrected path.
- Stop conditions after ENV-065/ENV-066: no more signing, submitting, broadcasting, transaction creation, or live spend/create action is required for the completed spike; any future live action requires a new reviewed plan and explicit approval.
- Never use mainnet, expose wallet secrets/helper private key material, build roulette, or build a web app under this workflow.
- Roulette remains paused.
