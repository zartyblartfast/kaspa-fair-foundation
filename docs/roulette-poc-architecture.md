# Roulette PoC Architecture

Status: ENV-081A declarative European roulette table layout schema on top of the deterministic round engine and foundation verifier contract

Purpose: define and now demonstrate an interactive static roulette UI flow prototype that displays the deterministic round result and proof/safety status without adding production casino features.

Safety boundary:
- no production web app or backend
- no signing
- no transaction creation
- no submitting or broadcasting
- no wallet or private-key access
- no mainnet
- no secrets

## 1. Foundation inputs available to roulette

The first roulette PoC is an adapter on top of the current foundation layer.

Available foundation inputs:
- readiness command: `scripts/env074-toccata-layer-ready.sh`
- readiness result: `TOCCATA_LAYER_READY=PASS`
- developer verifier command: `cargo run -p kaspa-fair-cli -- verify-live-tn10-canonical`
- app-facing JSON verifier command: `cargo run -p kaspa-fair-cli -- verify-live-tn10-canonical --json`

Current live verifier JSON facts available to the roulette adapter:
- `verifier_result = PASS`
- `network = testnet-10`
- `mainnet_supported = false`
- `accepted = true`
- `input_relationship_confirmed = true`
- `continuing_output_confirmed = true`
- `continuing_output_value_sompi = 99700000`
- `covenant_id_confirmed = true`
- `readonly = true`
- `signing_used = false`
- `transaction_created = false`
- `broadcast_used = false`
- `wallet_access_used = false`

Canonical TN10 proof values:
- ENV-064 spend txid: `4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c`
- ENV-063 spent input: `2c7802ff9a6eec2828a96168d8f62a9a276176441ed8cb6086cd5d5d0cb26849:0`
- continuing output: `4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c:0`
- continuing output value: `99700000 sompi`
- covenant id: `e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7`

What this proves today:
- live TN10 covenant evidence exists
- the canonical ENV-064 spend is accepted on testnet-10
- the ENV-063 input relationship is confirmed
- the continuing output, value, and covenant id match expected values
- the verifier is read-only and did not sign, create, or broadcast transactions
- a machine-readable JSON contract exists for app consumption
- a readiness gate exists before app use

What this does not prove today:
- roulette betting logic
- roulette round state machine behavior
- player accounts or wallets
- payout execution
- house bankroll management
- mainnet support
- generic all-Toccata feature coverage
- vProgs or ZK support
- a web UI

## 2. Roulette fairness objective

The roulette PoC must let an independent verifier reconstruct one round from fixed inputs and reach the same outcome.

The fairness claim for the first PoC is:
- round rules are fixed before bets open
- the accepted bet ledger is fixed at bet close
- the result seed material is fixed after bet close, or revealed from a prior commitment that cannot change
- the roulette number and colour are derived deterministically
- settlement is derived deterministically from the fixed ledger and fixed result
- the round publishes proof data plus the foundation verifier JSON

The fairness claim is not:
- trustless bankroll custody
- automatic payout enforcement
- mainnet payout support
- censorship resistance
- immunity from fake UI animation without proof review

## 3. Correct roulette round sequence

A normal online-roulette-compatible round sequence for this PoC is:

1. Round opened
   - round id, rule version, payout table version, wheel type, and colour table are fixed
   - any pre-round commitment format is fixed

2. Bets open
   - bets are accepted into an append-only round ledger
   - each accepted bet gets a stable bet id, timestamp, stake, selection, and player reference

3. Wheel animation or spin starts
   - the visual wheel animation may start before betting closes
   - this animation is presentation only at this stage

4. Bets remain open briefly while the wheel appears to spin
   - accepted bets continue to append to the ledger until the close boundary
   - no result seed is finalised during this interval

5. No more bets / betting closes
   - a hard close timestamp or event cuts off acceptance
   - any later submissions are rejected and logged as rejected

6. Final bet ledger fixed
   - the accepted-bets-only ledger is canonicalised and hashed
   - this hash becomes the round bet-ledger commitment

7. Result seed material fixed
   - option A: seed material is constructed only after bet close
   - option B: a pre-round seed commitment is revealed after bet close and verified against the commitment
   - in both options, the operator cannot change the final seed after seeing the fixed ledger

8. Roulette number derived
   - deterministic rejection sampling derives a number in the fixed European range `0..36`

9. Colour derived
   - colour is derived from the fixed published colour table, not from UI state

10. Settlement calculated
   - each accepted bet is evaluated against the fixed result and fixed payout table
   - gross win, net win, and losing stakes are computed deterministically

11. Winnings paid or recorded
   - for this PoC, settlement may be recorded only
   - if later payout execution exists, the recorded settlement must remain identical to the published calculation

12. Proof published
   - the round proof publishes the rule version, payout table version, bet ledger hash, seed material or verified reveal, deterministic result derivation transcript, settlement summary, and foundation live verifier JSON

Critical rule:
- spin animation != result finalisation

The visual spin may start before betting closes, but the final result must not be knowable or selectable by the operator while bets are still being accepted.

## 4. Recommended PoC fairness model

The default ENV-075 model is:
- wheel animation starts while bets are still open
- bets close at a hard no-more-bets boundary
- final bet ledger hash is fixed
- result seed material is fixed only after bets close, or revealed from a pre-round commitment that cannot be changed
- roulette result is derived deterministically
- proof is published with foundation verifier JSON

This model preserves a familiar online roulette sequence while removing the classic operator advantage of waiting for the final bet ledger and then choosing a convenient outcome.

## 5. Deterministic result derivation

### 5.1 Seed material

The round uses domain-separated seed material dedicated to roulette result derivation. The published round proof must identify every byte source used to build `seed_material`.

Permitted PoC seed-material models:
- post-close fixed seed material assembled only after the final bet ledger hash exists
- pre-round commitment revealed after bet close and verified against the earlier commitment
- mixed model that includes both a pre-round commitment reveal and the final bet-ledger hash

For the first PoC, the round proof must publish enough material for an independent verifier to reconstruct the exact seed bytes.

### 5.2 Candidate generation

Use deterministic rejection sampling with BLAKE3 and domain separation.

Candidate formula:

`candidate_i = BLAKE3("kaspa-fair:roulette:candidate:v1" || seed_material || counter_u32_be(i))`

Interpretation rules:
- `BLAKE3(...)` returns 32 bytes
- `counter_u32_be(i)` is a 4-byte big-endian unsigned integer
- each candidate uses a new counter value starting from `i = 0`
- the candidate bytes are interpreted as a 256-bit unsigned integer in big-endian form

### 5.3 Rejection sampling to European roulette numbers

Fixed roulette number range:
- European roulette numbers: `0..36`
- outcome count: `37`

Let:
- `M = 2^256`
- `N = 37`
- `limit = M - (M mod N)`

Algorithm:
1. compute `x_i` from `candidate_i`
2. if `x_i >= limit`, reject and continue with `i + 1`
3. otherwise accept and set `number = x_i mod 37`

This removes modulo bias. Direct `mod 37` on all 256-bit values is not permitted.

### 5.4 Fixed colour mapping

Colour is derived from the fixed European table below.

- `0 = green`
- red numbers: `1,3,5,7,9,12,14,16,18,19,21,23,25,27,30,32,34,36`
- black numbers: `2,4,6,8,10,11,13,15,17,20,22,24,26,28,29,31,33,35`

A proof verifier must reject any round artifact that changes this mapping without a new explicit rule version declared before bets open.

## 6. Deterministic settlement model

The first PoC records settlement deterministically even if payout execution remains out of scope.

Required settlement inputs:
- round id
- rule version
- payout table version
- fixed accepted-bet ledger
- final bet-ledger hash
- deterministic result number
- deterministic result colour

Required settlement outputs:
- per-bet outcome: win or loss
- per-bet gross payout
- per-bet net payout
- per-bet reason code tied to the rule set
- round settlement summary totals

Settlement must be reproducible from the published round proof without hidden server state.

## 7. Threat model and control mapping

| roulette stage | cheating/rigging risk | required control | current foundation support | roulette adapter work still needed |
|---|---|---|---|---|
| round definition before open | changing rules after bets | publish immutable rule version and payout table version before bets open | foundation can carry app-facing proof artifacts and JSON attachments; it does not define roulette rules | define round schema, rule versioning, payout table versioning, and proof fields |
| bets open | altering accepted bets | append-only accepted-bet ledger with canonical serialisation and final hash at close | no current bet-ledger support | implement accepted-bet ledger model, canonical encoding, and hash publication |
| no-more-bets boundary | accepting late bets after no more bets | hard close timestamp or event; reject and log late submissions | readiness gate exists for foundation use; no betting-close logic exists | implement close boundary, rejection logging, and proof inclusion |
| post-close seed fixation | choosing result after seeing bet ledger | seed material fixed only after close, or reveal a prior commitment that cannot change | foundation provides read-only TN10 proof JSON, not roulette seed handling | implement seed-material schema, commitment or post-close fixation path, and publication rules |
| visual spin | fake spin animation | UI animation treated as non-authoritative; published proof decides the authoritative result | foundation proof can anchor independent verification outside UI claims | mark animation as cosmetic and verify displayed result against proof |
| result derivation | biased RNG or modulo bias | deterministic BLAKE3 domain-separated rejection sampling | foundation architecture already uses recipe/domain-separation concepts; no roulette RNG exists | implement seed byte definition, candidate transcript, rejection sampling, and verifier reproduction |
| number-to-colour derivation | changing number-to-colour mapping | fixed published European colour table under rule version | no current roulette mapping support | embed fixed colour table and verify against it |
| settlement | miscalculating payouts | deterministic payout table and reproducible settlement function | no current payout/settlement support | implement payout table encoding, settlement logic, and round totals proof |
| payout handling | refusing or altering payouts | settlement record fixed first; any later payment action must match the fixed settlement record | foundation proves read-only verification only; no payout execution exists | record settlement outputs now; defer payment execution to later authorised work |
| audit trail publication | fake audit logs | publish machine-readable round proof plus attached foundation live verifier JSON | foundation already exposes machine-readable verifier JSON and readiness gate result | define round-proof JSON schema and artifact bundle |
| integration claims | claiming mainnet or wallet/signing support that does not exist | explicit capability flags in proof and docs | foundation JSON already proves `mainnet_supported=false`, `readonly=true`, `signing_used=false`, `transaction_created=false`, `broadcast_used=false`, `wallet_access_used=false` | propagate these flags into roulette proof presentation and reject contradictory product claims |

## 8. Mapping current Toccata/foundation features to roulette

### 8.1 What the current layer can support for roulette

The current foundation layer can support:
- proof that the TN10 covenant evidence exists live
- proof that ENV-064 is accepted
- proof that the ENV-063 input relationship is confirmed
- proof that the continuing output, continuing output value, and covenant id match expected values
- proof that the verifier is read-only and did not sign, broadcast, or create transactions
- machine-readable JSON for application consumption
- a readiness gate before roulette adapter use

For the roulette PoC, this means:
- the adapter can attach a current foundation trust snapshot to each published round proof
- the adapter can refuse operation unless the readiness gate passes
- the adapter can truthfully state that its trust anchor is TN10 read-only verification only

### 8.2 What the current layer cannot support yet

The current foundation layer does not yet provide:
- a bet ledger implementation
- roulette round state machine code
- player accounts or wallets
- payout execution
- house bankroll management
- mainnet support
- generic all-Toccata feature support
- ZK or vProg support
- a web UI

For the roulette PoC, this means:
- the adapter must implement its own deterministic round schema above the foundation layer
- payout execution remains out of scope
- any claim of mainnet, wallet, signing, broadcasting, or custody support is false
- the PoC remains a specification and adapter-boundary exercise, not a deployable casino product

## 9. First roulette PoC boundary

The first roulette PoC is an adapter on top of the foundation, not a replacement for it.

The first PoC consumes:
- ENV-074 readiness script result
- ENV-073 live verifier JSON
- deterministic round specification
- deterministic result derivation
- deterministic settlement calculation

The first PoC does not include:
- real-money mainnet
- wallet custody
- live signing or broadcasting
- production payouts
- vProgs or ZK
- a full casino UI

Operationally, the adapter flow is:
1. verify readiness gate passes
2. load foundation live verifier JSON
3. open a roulette round under fixed rules
4. accept bets until the hard close boundary
5. hash the final accepted ledger
6. fix or reveal seed material under the declared round mode
7. derive the roulette number and colour deterministically
8. calculate deterministic settlement
9. publish the round proof with the attached foundation verifier JSON

## 10. Acceptance checklist for future roulette implementation

Future roulette code is fair only if every item below passes:

- [ ] fixed rules before bets
- [ ] fixed payout table before bets
- [ ] bets closed before result finalisation
- [ ] final bet ledger hash recorded
- [ ] deterministic seed material recorded
- [ ] deterministic rejection sampling used
- [ ] number and colour reproducible
- [ ] settlement reproducible
- [ ] foundation live verifier JSON attached
- [ ] readiness gate passes
- [ ] no mainnet unless explicitly authorised later

## 11. Minimum round-proof payload for the first PoC

A first-round proof payload must include at least:
- round id
- rule version
- payout table version
- round open timestamp
- no-more-bets timestamp
- accepted-bet ledger hash
- accepted-bet count
- seed-material mode
- seed material or verified reveal material
- result derivation transcript sufficient for independent reproduction
- final number
- final colour
- settlement summary
- attached foundation live verifier JSON
- readiness result snapshot
- capability flags proving read-only TN10 mode only

## 12. ENV-075 conclusion

ENV-075 defines the roulette adapter boundary cleanly:
- the foundation remains app-agnostic and read-only
- roulette fairness lives in a deterministic adapter specification above that layer
- the operator cannot accept bets and then choose an outcome if the round follows the fixed close-first and seed-fixation controls in this document
- published proof plus foundation verifier JSON enables independent round verification without claiming wallet, payout, signing, or mainnet support

## 13. ENV-076 dry-run adapter skeleton

ENV-076 adds the first non-web, no-wallet, dry-run roulette PoC adapter skeleton.

Primary dry-run command:
- `cargo run -p kaspa-fair-cli -- roulette-poc-dry-run --json`

Persistent wrapper command:
- `scripts/env076-roulette-poc-dry-run.sh`

The adapter demonstrates the intended app boundary for future roulette work:
- it consumes the live TN10 foundation verifier JSON contract
- it requires `verifier_result = PASS`
- it rejects unsafe verifier flags such as wallet access, signing, transaction creation, broadcasting, or mainnet support
- it builds deterministic seed material from the round id, verifier covenant id, verifier ENV-064 spend txid, verifier accepting block hash, and final mock bet-ledger hash
- it derives the roulette result with the ENV-075 BLAKE3 domain-separated rejection-sampling algorithm
- it derives colour from the fixed European table
- it calculates deterministic settlement over mock bets only
- it emits stable machine-readable round JSON

ENV-076 remains dry-run adapter work only.

It does not implement:
- real betting
- player accounts
- wallet custody
- real payouts
- signing
- transaction creation
- submitting or broadcasting
- mainnet
- a web app
- production casino behavior

The ENV-076 adapter proves that future roulette code can sit above the foundation verifier contract instead of replacing it.

## 14. ENV-077 deterministic roulette round engine

ENV-077 turns the ENV-076 adapter into a reusable deterministic round engine on the app/CLI side.

Primary engine command:
- `cargo run -p kaspa-fair-cli -- roulette-engine-dry-run --json`

Persistent readiness command:
- `scripts/env077-roulette-engine-check.sh`

The engine models the full deterministic sequence:
- `Created`
- `BetsOpen`
- `SpinVisualStarted`
- `NoMoreBets`
- `ResultFinalised`
- `Settled`
- `ProofPublished`

Required behavior now demonstrated by code and tests:
- wheel/spin animation may start before bet close
- spin animation is not result finalisation
- bets are accepted only in `BetsOpen` and `SpinVisualStarted`
- bets are rejected after `NoMoreBets`
- result finalisation is rejected before `NoMoreBets`
- settlement is rejected before `ResultFinalised`
- proof publication is rejected before settlement
- the final published round JSON carries `round_state = ProofPublished`

The deterministic engine keeps the foundation boundary intact:
- it consumes the live TN10 foundation verifier JSON contract
- it requires `verifier_result = PASS`
- it enforces read-only/no-wallet/no-signing/no-broadcast/no-mainnet safety flags
- it uses mock bets only
- it derives the result with the same domain-separated BLAKE3 rejection-sampling algorithm
- it uses the fixed European colour table
- it calculates deterministic settlement with fixed payout multipliers
- it emits the stable `kaspa-fair-roulette-engine-round-v1` JSON contract

ENV-077 still does not implement:
- a web app
- real betting
- real payouts
- wallets or custody
- signing
- transaction creation
- submitting or broadcasting
- mainnet
- ZK
- vProgs

## 15. ENV-078 simple roulette UI prototype

ENV-078 adds a simple static UI prototype that displays the existing ENV-077 deterministic engine JSON without changing the underlying result.

Primary readiness command:
- `scripts/env078-roulette-ui-smoke.sh`

Static UI files:
- `examples/roulette-poc/ui/index.html`
- `examples/roulette-poc/ui/styles.css`
- `examples/roulette-poc/ui/app.js`
- `examples/roulette-poc/ui/sample-round.json`

UI behavior now demonstrated:
- it displays the foundation verifier trust/safety status
- it displays the full round sequence through `ProofPublished`
- it explicitly states `spin animation != result finalisation`
- it renders a simple European `0..36` table and highlights the deterministic result number
- it displays mock bets, win/loss state, payout units, and net units from the engine JSON
- it displays proof fields including covenant id, ENV-064 txid, accepting block hash, and bet ledger hash
- it shows a clear failure/unsafe state if the JSON is not `PASS` or if safety flags are unsafe

ENV-078 is intentionally minimal:
- visual polish is intentionally minimal
- the UI displays existing deterministic engine JSON
- the UI does not decide or randomise the result
- the wheel/spin is visual only
- there is no real betting, real payouts, wallet integration, signing, broadcasting, custody, mainnet, or production casino functionality

## 16. ENV-079 interactive roulette UI flow prototype

ENV-079 upgrades the simple display into an interactive static UI flow prototype while preserving the deterministic round result from engine JSON.

Primary readiness command:
- `scripts/env079-roulette-ui-flow-smoke.sh`

Primary UI files:
- `examples/roulette-poc/ui/index.html`
- `examples/roulette-poc/ui/styles.css`
- `examples/roulette-poc/ui/app.js`
- `examples/roulette-poc/ui/sample-round.json`

UI behavior now demonstrated:
- `BetsOpen` is the initial visible state
- `Start Wheel` moves the UI to `SpinVisualStarted`
- bets may remain visually open while the wheel is spinning
- `No More Bets` moves the UI to `NoMoreBets`
- adding or changing bets is blocked after `NoMoreBets`
- result finalisation occurs only after `NoMoreBets`
- `Reveal Result` displays the deterministic result from `sample-round.json`
- `Show Settlement` displays deterministic settlement from `sample-round.json`
- `Publish Proof` displays proof fields and final `PASS` status
- the fairness statement `spin animation != result finalisation` is shown explicitly
- the roulette table highlights the deterministic result number only after reveal
- the UI displays foundation verifier status and safety flags: `testnet-10`, `mainnet_supported: false`, `readonly: true`, `signing_used: false`, `transaction_created: false`, `broadcast_used: false`, `wallet_access_used: false`

ENV-079 remains intentionally minimal and offline-safe:
- visual polish is intentionally minimal
- the UI demonstrates state sequence and proof display
- the UI does not decide the result
- the UI consumes deterministic engine JSON
- there is no real betting, no real payouts, no wallet, no signing, no broadcasting, no backend custody, and no mainnet

## 17. ENV-080 UI mock bet placement and reset flow

ENV-080 adds mock UI bet placement and reset/new-round flow on top of the interactive static roulette UI.

Primary readiness command:
- `scripts/env080-roulette-ui-bet-flow-smoke.sh`

Primary UI files:
- `examples/roulette-poc/ui/index.html`
- `examples/roulette-poc/ui/styles.css`
- `examples/roulette-poc/ui/app.js`
- `examples/roulette-poc/ui/sample-round.json`

UI behavior now demonstrated:
- visible `Place Mock Bet` control is available in the UI
- mock choices include straight number, red, black, odd, even, high, and low
- bets may be placed before wheel start in `BetsOpen`
- bets may also be placed during `SpinVisualStarted`
- bet placement is blocked only after `NoMoreBets`
- after `No More Bets`, the UI shows `BETS_CLOSED_NO_MORE_BETS` and `No more bets — ledger locked.`
- `Reset Round` returns the UI to `BetsOpen` without page refresh
- reset clears UI-added mock bets and hides result, settlement, and proof again
- result still loads from deterministic engine JSON in `sample-round.json` only
- deterministic settlement still comes from the engine sample round only
- UI-added bets are mock-only prototype display bets

ENV-080 remains intentionally minimal and offline-safe:
- visual polish is intentionally minimal
- no real betting
- no real payouts
- no wallet
- no backend custody
- no signing
- no broadcasting
- no transaction creation
- no mainnet
- no production casino functionality

## 18. ENV-080B UI bet placement cleanup before table-zone model

ENV-080B cleans up the temporary mock bet UI so the current roulette prototype no longer implies full roulette table bet placement support.

Primary readiness command:
- `scripts/env080b-roulette-ui-bet-cleanup-smoke.sh`

Primary UI files:
- `examples/roulette-poc/ui/index.html`
- `examples/roulette-poc/ui/styles.css`
- `examples/roulette-poc/ui/app.js`
- `examples/roulette-poc/ui/sample-round.json`

UI behavior now demonstrated:
- the current bet control is explicitly labeled as a temporary simple/prototype straight-number mock bet path only
- the UI visibly states that full roulette table bet zones are not implemented yet
- the UI visibly lists the future table-driven bet zones deferred to ENV-081: straight, split, street, corner, six-line, dozens, columns, red/black, odd/even, high/low
- simple mock bets remain allowed in `BetsOpen`
- simple mock bets remain allowed in `SpinVisualStarted`
- simple mock bets remain blocked after `NoMoreBets`
- `Reset Round` still clears UI-added mock bets and returns the UI flow to `BetsOpen`
- deterministic result, settlement, and proof still come from `sample-round.json` only

ENV-080B intentionally does not implement full roulette table bet zones.

ENV-081A now defines the proper table-zone model as declarative schema data for the standard European layout.

## 19. ENV-081A European roulette table layout schema

ENV-081A defines the table layout schema only. It does not rebuild the UI betting surface yet.

Primary readiness command:
- `scripts/env081a-roulette-table-schema-smoke.sh`

Primary schema files:
- `examples/roulette-poc/ui/roulette-table-schema.json`
- `examples/roulette-poc/ui/roulette-table-schema.js`

Schema behavior now defined:
- standard European roulette layout
- dedicated green `0` region on the left of the main number grid
- 12 x 3 number grid with visual rows `3,6,9,...,36`, `2,5,8,...,35`, `1,4,7,...,34`
- explicit dozen regions for `1st 12`, `2nd 12`, and `3rd 12`
- explicit outside bet regions for `1 to 18`, `EVEN`, `RED`, `BLACK`, `ODD`, and `19 to 36`
- explicit clickable column selector rectangles for the three standard European columns
- future hotspot geometry for split, street, corner, and six-line bets
- coordinates on every clickable region so the schema can drive future SVG/hotspot rendering

ENV-081A intentionally does not rebuild the roulette UI.

ENV-081B is the deferred UI rebuild step that should consume this schema for SVG or table-hotspot rendering.

ENV-081A explicitly avoids:
- giant inside-zone lists
- dropdown-based inside-zone betting
- real betting
- real payouts
- wallet access
- backend custody
- signing
- broadcasting
- transaction creation
- mainnet
- production casino functionality
