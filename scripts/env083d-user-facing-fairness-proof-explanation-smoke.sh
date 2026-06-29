#!/usr/bin/env bash
set -euo pipefail

UI="examples/roulette-poc/ui/index.html"
APP_JS="examples/roulette-poc/ui/app.js"
RENDERER_JS="examples/roulette-poc/ui/roulette-table-renderer.js"
SAMPLE="examples/roulette-poc/ui/sample-round.json"

require_text() {
  local file="$1"
  local pattern="$2"
  if ! grep -Eiq -- "$pattern" "$file"; then
    printf 'missing required text in %s: %s\n' "$file" "$pattern" >&2
    exit 1
  fi
}

reject_text() {
  local file="$1"
  local pattern="$2"
  if grep -Eiq -- "$pattern" "$file"; then
    printf 'forbidden text in %s: %s\n' "$file" "$pattern" >&2
    exit 1
  fi
}

require_text "$UI" '<h2>How this result is verified</h2>'
require_text "$UI" 'UI does not choose the result'
require_text "$UI" 'Rust verifier logic'
require_text "$UI" 'live TN10 Toccata covenant evidence'
require_text "$UI" 'commitment/reveal'
require_text "$UI" 'deterministic BLAKE3 result derivation'
require_text "$UI" 'Safety boundary'
require_text "$UI" 'not real betting'
require_text "$UI" 'no real payouts'
require_text "$UI" 'do not trust the UI alone'
require_text "$UI" 'do not trust the operator alone'
require_text "$UI" 'Verify the proof'
require_text "$UI" 'production-grade unbiased randomness'
require_text "$UI" 'seed and entropy hardening remain future work'
require_text "$UI" 'Kaspa beats a private operator database'
require_text "$UI" 'public PoW DAG'
require_text "$UI" 'Toccata adds more than a plain anchored hash'
require_text "$UI" 'covenant lineage and state-transition evidence'
require_text "$UI" 'ENV-087 adds authorised TN10-only live round-specific commitment/reveal transaction evidence'

for file in "$UI" "$APP_JS" "$RENDERER_JS"; do
  reject_text "$file" 'Math\.random'
  reject_text "$file" 'crypto\.getRandomValues|randomUUID|window\.crypto|globalThis\.crypto|crypto\.subtle'
  reject_text "$file" 'create(Transaction| transaction)|sign(Transaction| transaction)|broadcast(Transaction| transaction)|submit(Transaction| transaction)'
  reject_text "$file" 'wallet\.connect|connectWallet|privateKey|mnemonic|seedPhrase|faucet'
done

python3 - <<'PY'
import json
from pathlib import Path
sample = json.loads(Path('examples/roulette-poc/ui/sample-round.json').read_text())
assert sample.get('final_result') == 'PASS'
assert sample.get('round_state') == 'ProofPublished'
assert sample.get('result_algorithm') == 'blake3-domain-separated-rejection-sampling-v1'
assert isinstance(sample.get('result_number'), int) and 0 <= sample['result_number'] <= 36
assert sample.get('result_colour') in {'green', 'red', 'black'}
assert sample.get('foundation_verifier_result') == 'PASS'
assert sample.get('foundation_network') == 'testnet-10'
assert sample.get('foundation_readonly') is (False if sample.get('source_env') == 'ENV-092' else True)
for flag in ['mainnet_supported', 'wallet_access_used', 'signing_used']:
    assert sample.get(flag) is False
for flag in ['transaction_created', 'broadcast_used']:
    assert sample.get(flag) is (True if sample.get('source_env') == 'ENV-092' else False)
PY

require_text "$UI" '>Start Wheel<'
require_text "$UI" '>Reset Round<'
reject_text "$UI" '>Reveal Result<'
reject_text "$UI" 'Wheel Visual'

require_text "$APP_JS" 'BETS_CLOSED_NO_MORE_BETS'
require_text "$APP_JS" 'No more bets accepted this round\.'
require_text "$APP_JS" 'sample-round\.json'
require_text "$APP_JS" 'setTimeout'
reject_text "$APP_JS" 'result_number\s*='
reject_text "$APP_JS" 'result_colour\s*='

printf 'USER_FACING_FAIRNESS_PROOF_EXPLANATION_READY=PASS\n'
