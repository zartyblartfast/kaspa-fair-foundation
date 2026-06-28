#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ARTIFACT_DIR="spikes/kaspa-foundation/artifacts/env-085-static-mock-verifiable-demo-milestone"
GENERATED_DIR="$ARTIFACT_DIR/generated-check"
RUNBOOK="docs/env085-static-mock-verifiable-demo-milestone.md"
SUMMARY="docs/roulette-poc-milestone-summary.md"
APP_JS="examples/roulette-poc/ui/app.js"
RENDERER_JS="examples/roulette-poc/ui/roulette-table-renderer.js"
INDEX_HTML="examples/roulette-poc/ui/index.html"
PROOF_JSON="examples/roulette-poc/ui/toccata-fairness-proof.json"
SAMPLE_JSON="examples/roulette-poc/ui/sample-round.json"

mkdir -p "$GENERATED_DIR"

cargo run -q -p kaspa-fair-cli -- env084-generate-verifiable-demo-round \
  --round-id env-085-demo-round-0001 \
  --demo-seed "env085-demo-seed-0001" \
  --out-dir "$GENERATED_DIR" >/tmp/env085-generator-output.txt

[[ -f "$GENERATED_DIR/sample-round.json" ]] || { echo "missing generated sample-round.json" >&2; exit 1; }
[[ -f "$GENERATED_DIR/toccata-fairness-proof.json" ]] || { echo "missing generated toccata-fairness-proof.json" >&2; exit 1; }
[[ -f "$GENERATED_DIR/verifier-output.json" ]] || { echo "missing generated verifier-output.json" >&2; exit 1; }
[[ -f "$RUNBOOK" ]] || { echo "missing milestone runbook" >&2; exit 1; }
[[ -f "$SUMMARY" ]] || { echo "missing non-developer summary" >&2; exit 1; }

python3 - "$GENERATED_DIR/sample-round.json" "$GENERATED_DIR/toccata-fairness-proof.json" "$GENERATED_DIR/verifier-output.json" <<'PY'
import json
import sys
sample_path, proof_path, verifier_path = sys.argv[1:4]
sample = json.load(open(sample_path, encoding='utf-8'))
proof = json.load(open(proof_path, encoding='utf-8'))
verifier = json.load(open(verifier_path, encoding='utf-8'))
reveal = proof.get('application_round_transcript', {}).get('reveal', {})
anchor = proof.get('live_tn10_anchor', {})
safety = proof.get('safety_flags', {})
errors = []

def require(label, condition):
    if not condition:
        errors.append(label)

require('sample/proof round_id agreement', sample.get('round_id') == proof.get('round_id') == reveal.get('round_id'))
require('sample/proof result_number agreement', sample.get('result_number') == proof.get('result_number') == reveal.get('result_number'))
require('sample/proof result_colour agreement', sample.get('result_colour') == proof.get('result_colour') == reveal.get('result_colour'))
require('sample/proof result_algorithm agreement', sample.get('result_algorithm') == proof.get('result_algorithm') == reveal.get('result_algorithm'))
require('sample final_result PASS', sample.get('final_result') == 'PASS')
require('proof verifier_result PASS', proof.get('verifier_result') == 'PASS')
require('verifier_output PASS', verifier.get('verifier_result') == 'PASS')
require('evidence_mode live_readonly_tn10', proof.get('evidence_mode') == 'live_readonly_tn10')
require('anchor evidence_mode live_readonly_tn10', anchor.get('evidence_mode') == 'live_readonly_tn10')
require('future live round transaction evidence not claimed', proof.get('future_live_round_transaction_evidence') == 'not_created_not_claimed_future_work')
require('mock display only', safety.get('mock_display_only') is True)
for flag in ['real_betting', 'real_payouts', 'backend_custody', 'wallet_access_used', 'private_key_access_used', 'signing_used', 'transaction_created', 'broadcast_used', 'mainnet_supported']:
    require(f'safety flag {flag} false', safety.get(flag) is False)
if errors:
    for error in errors:
        print(f'FAIL: {error}', file=sys.stderr)
    sys.exit(1)
PY

grep -q 'fetchJson("sample-round.json")' "$APP_JS" || { echo "UI does not reference sample-round.json" >&2; exit 1; }
grep -q 'fetchJson("toccata-fairness-proof.json")' "$APP_JS" || { echo "UI does not reference toccata-fairness-proof.json" >&2; exit 1; }

grep -q 'result_number\s*=' "$APP_JS" && { echo "UI result_number assignment found" >&2; exit 1; }
grep -q 'result_colour\s*=' "$APP_JS" && { echo "UI result_colour assignment found" >&2; exit 1; }
! grep -RInE 'Math\.random|crypto\.getRandomValues|crypto\.randomUUID|globalThis\.crypto|window\.crypto|self\.crypto' "$APP_JS" "$RENDERER_JS" "$INDEX_HTML" >/tmp/env085-random-api-grep.txt || { cat /tmp/env085-random-api-grep.txt >&2; exit 1; }
! grep -RInE 'wallet\.connect|connectWallet|privateKey\s*=|mnemonic\s*=|seedPhrase\s*=|createTransaction\(|signTransaction\(|broadcastTransaction\(|submitTransaction\(|mainnet_supported[[:space:]]*:[[:space:]]*true|mainnet_enabled[[:space:]]*:[[:space:]]*true' "$APP_JS" "$RENDERER_JS" "$INDEX_HTML" "$RUNBOOK" "$SUMMARY" >/tmp/env085-forbidden-live-scope-grep.txt || { cat /tmp/env085-forbidden-live-scope-grep.txt >&2; exit 1; }

grep -q 'env084-generate-verifiable-demo-round' "$RUNBOOK" || { echo "runbook missing ENV-084 generator command" >&2; exit 1; }
grep -qi 'explicit demo seed material' "$RUNBOOK" || { echo "runbook missing explicit demo seed boundary" >&2; exit 1; }
grep -q 'live_readonly_tn10' "$RUNBOOK" || { echo "runbook missing live_readonly_tn10" >&2; exit 1; }
grep -q 'future authorised work' "$RUNBOOK" || { echo "runbook missing future authorised work boundary" >&2; exit 1; }
grep -q 'The UI does not choose the result' "$SUMMARY" || { echo "summary missing UI non-generation statement" >&2; exit 1; }

scripts/env083f-round-proof-consistency-smoke.sh >/dev/null
scripts/env084-verifiable-demo-round-generator-smoke.sh >/dev/null

printf 'STATIC_MOCK_VERIFIABLE_DEMO_MILESTONE_READY=PASS\n'
