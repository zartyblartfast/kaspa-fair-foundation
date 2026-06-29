#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ARTIFACT_DIR="spikes/kaspa-foundation/artifacts/env-092-tn10-verifiable-entropy-round"
SAMPLE="examples/roulette-poc/ui/sample-round.json"
PROOF="examples/roulette-poc/ui/toccata-fairness-proof.json"

require_file() {
  [[ -f "$1" ]] || { echo "missing required file: $1" >&2; exit 1; }
}

require_dir() {
  [[ -d "$1" ]] || { echo "missing required directory: $1" >&2; exit 1; }
}

require_dir "$ARTIFACT_DIR"
for path in \
  "$ARTIFACT_DIR/env-092-summary.md" \
  "$ARTIFACT_DIR/env-092-preflight.json" \
  "$ARTIFACT_DIR/env-092-commitment-payload.json" \
  "$ARTIFACT_DIR/env-092-no-more-bets-evidence.json" \
  "$ARTIFACT_DIR/env-092-entropy-target.json" \
  "$ARTIFACT_DIR/env-092-tn10-entropy-readback.json" \
  "$ARTIFACT_DIR/env-092-reveal-payload.json" \
  "$ARTIFACT_DIR/env-092-final-entropy-transcript.json" \
  "$ARTIFACT_DIR/env-092-result-derivation.json" \
  "$ARTIFACT_DIR/env-092-verifier-output.json" \
  "$ARTIFACT_DIR/env-092-negative-checks.txt" \
  "$ARTIFACT_DIR/env-092-safety-flags.json" \
  "$ARTIFACT_DIR/env-092-secret-scan.txt" \
  "$ARTIFACT_DIR/env-092-command-results.txt" \
  "$ARTIFACT_DIR/env-092-git-status.txt" \
  "$SAMPLE" \
  "$PROOF"; do
  require_file "$path"
done

python3 - "$ARTIFACT_DIR" "$SAMPLE" "$PROOF" <<'PY'
import json, pathlib, re, sys
root = pathlib.Path(sys.argv[1])
sample = json.loads(pathlib.Path(sys.argv[2]).read_text())
proof = json.loads(pathlib.Path(sys.argv[3]).read_text())
preflight = json.loads((root / 'env-092-preflight.json').read_text())
commitment = json.loads((root / 'env-092-commitment-payload.json').read_text())
no_more = json.loads((root / 'env-092-no-more-bets-evidence.json').read_text())
target = json.loads((root / 'env-092-entropy-target.json').read_text())
entropy = json.loads((root / 'env-092-tn10-entropy-readback.json').read_text())
transcript = json.loads((root / 'env-092-final-entropy-transcript.json').read_text())
derivation = json.loads((root / 'env-092-result-derivation.json').read_text())
verifier = json.loads((root / 'env-092-verifier-output.json').read_text())
safety = json.loads((root / 'env-092-safety-flags.json').read_text())
negative = (root / 'env-092-negative-checks.txt').read_text()
secret_scan = (root / 'env-092-secret-scan.txt').read_text()
errors = []
def require(label, condition):
    if not condition:
        errors.append(label)
require('source_env sample ENV-092', sample.get('source_env') == 'ENV-092')
require('source_env proof ENV-092', proof.get('source_env') == 'ENV-092')
require('claim level live TN10 entropy', 'live_tn10_entropy' in proof.get('claim_level',''))
require('sample production_randomness_claimed false', sample.get('production_randomness_claimed') is False)
require('proof production_randomness_claimed false', proof.get('production_randomness_claimed') is False)
require('proof safety production_randomness_claimed false', proof.get('safety_flags', {}).get('production_randomness_claimed') is False)
require('preflight TN10 only', preflight.get('tn10_only') is True and preflight.get('mainnet_supported') is False)
require('KIP-17 path available', preflight.get('kip17_lifecycle_command_path_available') is True)
require('no UI randomness preflight', preflight.get('no_ui_randomness') is True)
require('commitment unrevealed', commitment.get('operator_seed_revealed') is False and isinstance(commitment.get('commitment_hash'), str))
require('target recorded', target.get('entropy_target_blue_score') == no_more.get('entropy_target_blue_score'))
require('entropy delay target formula', no_more.get('entropy_target_blue_score') == no_more.get('no_more_bets_accepting_blue_score') + no_more.get('entropy_delay_blue_score'))
require('live TN10 entropy recorded', isinstance(entropy.get('entropy_value_used_in_transcript'), str) and len(entropy.get('entropy_value_used_in_transcript')) > 0)
require('entropy after target', entropy.get('entropy_source_blue_score') >= no_more.get('entropy_target_blue_score'))
require('transcript hash exists', isinstance(derivation.get('final_entropy_hash'), str) and len(derivation.get('final_entropy_hash')) == 64)
require('transcript includes entropy', transcript.get('tn10_future_entropy_value') == entropy.get('entropy_value_used_in_transcript'))
require('result derives from transcript marker', derivation.get('result_algorithm') == 'blake3-domain-separated-rejection-sampling-v1')
require('sample proof result number agreement', sample.get('result_number') == proof.get('result_number') == derivation.get('result_number'))
require('sample proof result colour agreement', sample.get('result_colour') == proof.get('result_colour') == derivation.get('result_colour'))
require('verifier PASS', verifier.get('verifier_result') == 'PASS' and proof.get('verifier_result') == 'PASS')
for flag in ['changed_operator_seed_rejected','changed_commitment_hash_rejected','changed_tn10_entropy_value_rejected','entropy_value_before_target_rejected','missing_no_more_bets_target_evidence_rejected','result_number_tampering_rejected','result_colour_tampering_rejected','ui_generated_randomness_rejected','production_randomness_claimed_true_rejected']:
    require(f'negative check {flag}', f'{flag}=true' in negative)
for flag in ['production_randomness_claimed','real_betting','real_payouts','backend_custody','mainnet_supported','wallet_access_used','private_key_access_used','signing_used']:
    require(f'safety false {flag}', safety.get(flag) is False)
require('secret scan PASS', 'PASS' in secret_scan and 'PRIVATE KEY-----' not in secret_scan)
if errors:
    for error in errors:
        print(f'FAIL: {error}', file=sys.stderr)
    raise SystemExit(1)
PY

if grep -Rqs 'Math\.random' examples/roulette-poc/ui; then
  echo 'Math.random found in UI' >&2
  exit 1
fi
if grep -Rqs 'crypto\.getRandomValues\|crypto\.randomUUID' examples/roulette-poc/ui; then
  echo 'browser crypto random API found in UI' >&2
  exit 1
fi
python3 - "$ARTIFACT_DIR" <<'PY'
import os, pathlib, re, sys
root = pathlib.Path(sys.argv[1])
patterns = [
    re.compile(r'-----BEGIN [A-Z ]*PRIVATE KEY-----'),
    re.compile(r'\b(kprv|xprv)[A-Za-z0-9]+\b'),
    re.compile(r'(?i)\b(api[_-]?key|auth[_-]?token|access[_-]?token)\s*[:=]\s*["\']?[A-Za-z0-9_-]{16,}'),
]
for dirpath, _, filenames in os.walk(root):
    for filename in filenames:
        path = pathlib.Path(dirpath) / filename
        text = path.read_text(encoding='utf-8', errors='ignore')
        for pattern in patterns:
            if pattern.search(text):
                print(f'secret-like material found in {path}: {pattern.pattern}', file=sys.stderr)
                raise SystemExit(1)
PY

printf 'TN10_VERIFIABLE_ENTROPY_ROUND_READY=PASS\n'
