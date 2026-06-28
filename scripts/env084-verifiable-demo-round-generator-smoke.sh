#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT
OUT_DIR="$TMP_DIR/generated"
COMMAND_LOG="$TMP_DIR/generator.log"

cargo run -q -p kaspa-fair-cli -- env084-generate-verifiable-demo-round \
  --round-id env-084-smoke-round-0001 \
  --demo-seed "env084-smoke-demo-seed-0001" \
  --out-dir "$OUT_DIR" \
  > "$COMMAND_LOG"

[[ -f "$OUT_DIR/sample-round.json" ]] || { echo "missing generated sample-round.json" >&2; exit 1; }
[[ -f "$OUT_DIR/toccata-fairness-proof.json" ]] || { echo "missing generated toccata-fairness-proof.json" >&2; exit 1; }
[[ -f "$OUT_DIR/verifier-output.json" ]] || { echo "missing generated verifier-output.json" >&2; exit 1; }

python3 - "$OUT_DIR/sample-round.json" "$OUT_DIR/toccata-fairness-proof.json" "$OUT_DIR/verifier-output.json" <<'PY'
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

require('result_number agreement', sample.get('result_number') == proof.get('result_number') == reveal.get('result_number'))
require('result_colour agreement', sample.get('result_colour') == proof.get('result_colour') == reveal.get('result_colour'))
require('result_algorithm agreement', sample.get('result_algorithm') == proof.get('result_algorithm') == reveal.get('result_algorithm'))
require('verifier_result PASS', verifier.get('verifier_result') == 'PASS')
require('proof verifier_result PASS', proof.get('verifier_result') == 'PASS')
require('evidence_mode live_readonly_tn10', proof.get('evidence_mode') == 'live_readonly_tn10')
require('anchor evidence_mode live_readonly_tn10', anchor.get('evidence_mode') == 'live_readonly_tn10')
require('future live round transaction evidence safe', proof.get('future_live_round_transaction_evidence') == 'not_created_not_claimed_future_work')
require('mock display only', safety.get('mock_display_only') is True)
for flag in ['real_betting', 'real_payouts', 'backend_custody', 'wallet_access_used', 'private_key_access_used', 'signing_used', 'transaction_created', 'broadcast_used', 'mainnet_supported']:
    require(f'safety flag {flag} false', safety.get(flag) is False)
for flag in ['mainnet_supported', 'wallet_access_used', 'private_key_access_used', 'signing_used', 'transaction_created', 'broadcast_used']:
    if flag in sample:
        require(f'sample flag {flag} false', sample.get(flag) is False)
require('sample foundation_readonly true', sample.get('foundation_readonly') is True)
if errors:
    for error in errors:
        print(f'FAIL: {error}', file=sys.stderr)
    sys.exit(1)
PY

! grep -RInE 'Math\.random|crypto\.getRandomValues|crypto\.randomUUID|window\.crypto|globalThis\.crypto|self\.crypto' \
  examples/roulette-poc/ui/app.js examples/roulette-poc/ui/index.html examples/roulette-poc/ui/roulette-table-renderer.js >/tmp/env084-ui-random-grep.txt || { cat /tmp/env084-ui-random-grep.txt >&2; exit 1; }
! grep -RInE 'rand::thread_rng|thread_rng\(|OsRng|getrandom\(' \
  crates/kaspa-foundation/src crates/kaspa-fair-cli/src >/tmp/env084-rust-random-grep.txt || { cat /tmp/env084-rust-random-grep.txt >&2; exit 1; }

grep -q 'fetchJson("sample-round.json")' examples/roulette-poc/ui/app.js || { echo "UI does not load sample-round.json" >&2; exit 1; }
grep -q 'fetchJson("toccata-fairness-proof.json")' examples/roulette-poc/ui/app.js || { echo "UI does not load toccata-fairness-proof.json" >&2; exit 1; }
grep -q 'Start Wheel' examples/roulette-poc/ui/index.html || { echo "Start Wheel control missing" >&2; exit 1; }
grep -q 'Reset Round' examples/roulette-poc/ui/index.html || { echo "Reset Round control missing" >&2; exit 1; }
! grep -RIn 'Reveal Result' examples/roulette-poc/ui/app.js examples/roulette-poc/ui/index.html >/tmp/env084-reveal-grep.txt || { cat /tmp/env084-reveal-grep.txt >&2; exit 1; }
! grep -RIn 'Wheel Visual' examples/roulette-poc/ui/app.js examples/roulette-poc/ui/index.html >/tmp/env084-wheel-visual-grep.txt || { cat /tmp/env084-wheel-visual-grep.txt >&2; exit 1; }

scripts/env083f-round-proof-consistency-smoke.sh >/dev/null

printf 'VERIFIABLE_DEMO_ROUND_GENERATOR_READY=PASS\n'
