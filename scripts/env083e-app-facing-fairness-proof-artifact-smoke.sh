#!/usr/bin/env bash
set -euo pipefail

UI="examples/roulette-poc/ui/index.html"
APP_JS="examples/roulette-poc/ui/app.js"
RENDERER_JS="examples/roulette-poc/ui/roulette-table-renderer.js"
PROOF="examples/roulette-poc/ui/toccata-fairness-proof.json"
SAMPLE="examples/roulette-poc/ui/sample-round.json"

require_text() {
  local file="$1"
  local pattern="$2"
  if ! grep -Eiq -- "$pattern" "$file"; then
    printf 'missing required text in %s: %s
' "$file" "$pattern" >&2
    exit 1
  fi
}

reject_text() {
  local file="$1"
  local pattern="$2"
  if grep -Eiq -- "$pattern" "$file"; then
    printf 'forbidden text in %s: %s
' "$file" "$pattern" >&2
    exit 1
  fi
}

[[ -f "$PROOF" ]] || { printf 'missing proof artifact: %s
' "$PROOF" >&2; exit 1; }
python3 -m json.tool "$PROOF" >/dev/null

require_text "$APP_JS" 'fetchJson\("sample-round\.json"\)'
require_text "$APP_JS" 'fetchJson\("toccata-fairness-proof\.json"\)'
require_text "$APP_JS" 'validateProofArtifact'
require_text "$APP_JS" 'renderProofSnapshot'
require_text "$UI" '<h2>Verifier proof snapshot</h2>'
require_text "$UI" 'toccata-fairness-proof\.json'
require_text "$UI" 'UI still does not choose the result'
require_text "$UI" 'proof artifact is checked by Rust verifier logic'
require_text "$UI" 'ENV-087 TN10-only live round commitment/reveal anchor evidence'
require_text "$UI" 'ENV-087 adds authorised TN10-only live round-specific commitment/reveal transaction evidence'

require_text "$PROOF" '"verifier_result"[[:space:]]*:[[:space:]]*"PASS"'
require_text "$PROOF" '"evidence_mode"[[:space:]]*:[[:space:]]*"live_readonly_tn10"'
require_text "$PROOF" '"covenant_id_confirmed"[[:space:]]*:[[:space:]]*true'
require_text "$PROOF" '"future_live_round_transaction_evidence"[[:space:]]*:[[:space:]]*"replaced_by_env087_live_bare_tn10_anchor_evidence"'
require_text "$PROOF" '"live_round_commitment_evidence"'
require_text "$PROOF" '"live_round_reveal_evidence"'
require_text "$PROOF" '"commitment_reveal_check_status"[[:space:]]*:[[:space:]]*"PASS"'
require_text "$PROOF" '"deterministic_derivation_check_status"[[:space:]]*:[[:space:]]*"PASS"'
require_text "$PROOF" '"mock_display_only"[[:space:]]*:[[:space:]]*true'
require_text "$PROOF" '"real_betting"[[:space:]]*:[[:space:]]*false'
require_text "$PROOF" '"real_payouts"[[:space:]]*:[[:space:]]*false'
require_text "$PROOF" '"backend_custody"[[:space:]]*:[[:space:]]*false'
require_text "$PROOF" '"wallet_access_used"[[:space:]]*:[[:space:]]*true'
require_text "$PROOF" '"private_key_access_used"[[:space:]]*:[[:space:]]*false'
require_text "$PROOF" '"signing_used"[[:space:]]*:[[:space:]]*true'
require_text "$PROOF" '"transaction_created"[[:space:]]*:[[:space:]]*true'
require_text "$PROOF" '"broadcast_used"[[:space:]]*:[[:space:]]*true'
require_text "$PROOF" '"mainnet_supported"[[:space:]]*:[[:space:]]*false'

require_text "$APP_JS" 'verifier result'
require_text "$APP_JS" 'evidence mode'
require_text "$APP_JS" 'covenant_id_confirmed'
require_text "$APP_JS" 'result algorithm'
require_text "$APP_JS" 'commitment/reveal check status'
require_text "$APP_JS" 'deterministic derivation check status'
require_text "$APP_JS" 'future live round transaction evidence'
require_text "$APP_JS" 'safety flags summary'
require_text "$APP_JS" 'replaced_by_env087_live_bare_tn10_anchor_evidence'

python3 - <<'PY'
import json
from pathlib import Path
sample = json.loads(Path('examples/roulette-poc/ui/sample-round.json').read_text())
proof = json.loads(Path('examples/roulette-poc/ui/toccata-fairness-proof.json').read_text())
reveal = proof.get('application_round_transcript', {}).get('reveal', {})
assert sample.get('round_id') == proof.get('round_id') == reveal.get('round_id')
assert sample.get('result_number') == proof.get('result_number') == reveal.get('result_number')
assert sample.get('result_colour') == proof.get('result_colour') == reveal.get('result_colour')
assert sample.get('result_algorithm') == proof.get('result_algorithm') == reveal.get('result_algorithm')
assert sample.get('final_result') == 'PASS'
assert proof.get('verifier_result') == 'PASS'
assert proof.get('evidence_mode') == 'live_readonly_tn10'
assert proof.get('future_live_round_transaction_evidence') == 'replaced_by_env087_live_bare_tn10_anchor_evidence'
assert proof.get('live_round_commitment_evidence', {}).get('status') == 'present'
assert proof.get('live_round_reveal_evidence', {}).get('status') == 'present'
for flag in ['mainnet_supported', 'wallet_access_used', 'signing_used', 'transaction_created', 'broadcast_used']:
    assert sample.get(flag) is False
for flag in ['real_betting', 'real_payouts', 'backend_custody', 'private_key_access_used', 'mainnet_supported']:
    assert proof.get('safety_flags', {}).get(flag) is False
for flag in ['wallet_access_used', 'signing_used', 'transaction_created', 'broadcast_used']:
    assert proof.get('safety_flags', {}).get(flag) is True
PY

for file in "$UI" "$APP_JS" "$RENDERER_JS"; do
  reject_text "$file" 'Math\.random'
  reject_text "$file" 'crypto\.getRandomValues|randomUUID|window\.crypto|globalThis\.crypto|crypto\.subtle'
done

require_text "$UI" '>Start Wheel<'
require_text "$UI" '>Reset Round<'
reject_text "$UI" '>Reveal Result<'
reject_text "$UI" 'Wheel Visual'

printf 'APP_FACING_FAIRNESS_PROOF_ARTIFACT_READY=PASS
'
