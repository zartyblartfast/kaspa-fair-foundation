# Proof Transcript Format

Status: ENV-067 design baseline
Scope: documentation/design only
Target result: READY_FOR_REFACTOR

## 1. Purpose

Proof transcripts are first-class project artifacts. They make covenant actions, read-only confirmations, and app-level fairness checks independently reviewable.

Every meaningful action should produce both:

```text
proof-transcript.json
proof-transcript.md
```

The JSON file is the canonical machine-verifiable artifact. The Markdown file is the human-readable rendering of the same evidence.

## 2. Result Vocabulary

Verifier outputs should use a small stable vocabulary:

```text
PASS
FAIL
AMBIGUOUS
UNAVAILABLE
```

Meanings:

- `PASS`: required evidence is present and independently verifies.
- `FAIL`: evidence is present and contradicts the claimed result.
- `AMBIGUOUS`: evidence is incomplete or insufficient to prove/falsify the claim.
- `UNAVAILABLE`: required external data cannot currently be queried.

## 3. Top-level JSON Shape

Draft schema shape:

```json
{
  "schema_version": "kaspa-fair-proof-transcript-v0",
  "transcript_id": "env-067-example",
  "created_at_utc": "2026-06-26T00:00:00Z",
  "project": {
    "repo": "kaspa-fair-lab",
    "git_commit": null,
    "git_dirty": null
  },
  "network": {
    "network_id": "testnet-10",
    "mainnet": false,
    "server_info": null,
    "sync_status": null,
    "has_utxo_index": null
  },
  "app": {
    "app_id": null,
    "round_id": null,
    "action_type": "design-only"
  },
  "covenant": {
    "recipe_id": "toccata-v1-keyed-blake3-state-transition",
    "tx_version": 1,
    "covenant_id": null,
    "state_before": null,
    "state_after": null
  },
  "transactions": {
    "input_utxos": [],
    "output_utxos": [],
    "txids": [],
    "submit_result": null,
    "postcheck_result": null
  },
  "local_verification": {
    "vm_proof_result": null,
    "shape_validation_result": null,
    "state_transition_result": null
  },
  "fairness": null,
  "safety": {
    "transaction_created": false,
    "transaction_signed": false,
    "transaction_submitted": false,
    "submission_count": 0,
    "mainnet_used": false,
    "wallet_secrets_accessed": false,
    "helper_private_key_exposed": false,
    "secret_redaction_checked": true
  },
  "verification_summary": {
    "result": "PASS",
    "checks": []
  }
}
```

## 4. Required Fields by Transcript Type

### 4.1 Design-only / Planning Transcript

Required:

- `schema_version`
- `transcript_id`
- `created_at_utc`
- `app.action_type`
- `safety.transaction_created=false`
- `safety.transaction_signed=false`
- `safety.transaction_submitted=false`
- `safety.submission_count=0`
- `safety.mainnet_used=false`
- `verification_summary.result`

### 4.2 Covenant Create Transcript

Required:

- network id
- server info or explicit reason unavailable
- input UTXOs
- planned outputs
- fee
- tx version
- covenant recipe id
- covenant id if created/observable
- local transaction id reconstruction
- local shape validation result
- submit result when submitted
- exact rejection when rejected
- postcheck result
- safety block

### 4.3 Covenant Spend Transcript

Required:

- network id
- previous covenant outpoint
- previous covenant id/state
- intended next output/state
- covenant recipe id
- local VM proof result
- state transition verification result
- submit result when submitted
- exact rejection when rejected
- postcheck result
- safety block

### 4.4 Read-only Confirmation Transcript

Required:

- network id
- server info
- sync status
- UTXO index availability when relevant
- transaction/mempool query result
- original input status
- continuing output status
- value/covenant id where observable
- explicit `transaction_created=false`
- explicit `transaction_submitted=false`

### 4.5 Roulette Round Transcript

Required:

- app id
- round id
- player address or public player identifier
- stake
- bet type
- bet selection
- house commitment
- player entropy
- reveal value after reveal
- outcome encoding version
- outcome number
- payout rule version
- expected payout
- actual settlement output(s)
- covenant create txid
- covenant spend/settlement txid
- independent verifier result

## 5. Roulette Fairness Object

Draft shape:

```json
{
  "model": "commit-reveal-v0",
  "round_id": "round-001",
  "house_commitment": "blake3:<hex>",
  "house_reveal": "redacted-until-reveal-or-hex-after-reveal",
  "player_entropy": "hex-or-public-value",
  "covenant_create_txid": "<txid>",
  "encoding": "kaspa-fair-roulette-v0",
  "seed_formula": "blake3(round_id || house_secret || player_entropy || covenant_create_txid)",
  "seed_hex": "<hex>",
  "outcome_formula": "integer(seed) mod 37",
  "outcome_number": 17,
  "wheel": "european-0-36",
  "bet": {
    "type": "red_black",
    "selection": "red",
    "stake_sompi": 100000000
  },
  "payout": {
    "rule_id": "european-roulette-v0",
    "expected_multiplier": "2x",
    "expected_payout_sompi": 200000000,
    "actual_payout_sompi": 200000000,
    "result": "PASS"
  },
  "checks": [
    {
      "id": "commitment-fixed-before-reveal",
      "result": "PASS"
    },
    {
      "id": "reveal-matches-commitment",
      "result": "PASS"
    },
    {
      "id": "outcome-calculation",
      "result": "PASS"
    },
    {
      "id": "payout-rule",
      "result": "PASS"
    }
  ]
}
```

## 6. Markdown Rendering

The Markdown transcript should be generated from the JSON transcript and should include:

```text
# Proof Transcript

Result: PASS / FAIL / AMBIGUOUS / UNAVAILABLE
Network: testnet-10
Action: <action>
App: <app id or none>
Round: <round id or none>
Covenant recipe: <recipe id>

## Summary
...

## Safety
- Transaction created: true/false
- Transaction signed: true/false
- Transaction submitted: true/false
- Submission count: N
- Mainnet used: false
- Wallet secrets accessed: false
- Helper private key exposed: false

## Covenant Evidence
...

## Local Verification
...

## Live Submission / Read-only Confirmation
...

## App/Fairness Evidence
...

## Independent Verification Result
...
```

## 7. Verification Rules

The verifier should fail closed for critical mismatches:

- wrong network
- mainnet evidence when mainnet is disabled
- tx version not matching recipe
- missing local VM proof for a submitted covenant spend
- covenant id mismatch
- state transition mismatch
- reveal does not match commitment
- outcome calculation mismatch
- payout mismatch
- submitted transaction without explicit submit authorization evidence

The verifier should return `AMBIGUOUS` instead of `PASS` when:

- postcheck evidence is missing
- current RPC cannot answer the needed read-only query
- settlement is not yet indexed
- mempool state is inconclusive
- transcript omits non-critical but relevant evidence

## 8. Secret Redaction Requirements

A transcript must not contain:

- private keys
- mnemonic phrases
- wallet passwords
- seed phrases
- raw house secret before reveal
- unrelated local filesystem secrets

After reveal, the house secret can be included only if it is required to verify the round and the protocol defines it as public at that phase.

## 9. ENV-063/064/065 Golden Transcript Targets

Initial golden transcript fixtures should reconstruct and verify:

- ENV-063 corrected create txid: `2c7802ff9a6eec2828a96168d8f62a9a276176441ed8cb6086cd5d5d0cb26849`
- ENV-064 corrected spend txid: `4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c`
- continuing output: `4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c:0`
- continuing output value: `99700000` sompi
- covenant id: `e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7`

## 10. First Implementation Recommendation

Implement transcript generation before a polished UI.

Recommended no-live sequence:

1. define Rust structs for the transcript schema
2. generate JSON for ENV-063/064/065 evidence fixtures
3. render Markdown from JSON
4. write verifier that returns PASS/FAIL/AMBIGUOUS/UNAVAILABLE
5. add tamper tests for txid, network, covenant id, reveal, and payout

No private keys are needed for verifier work.
