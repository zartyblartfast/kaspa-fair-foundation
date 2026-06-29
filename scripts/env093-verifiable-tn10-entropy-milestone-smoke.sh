#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ENV092_DIR="spikes/kaspa-foundation/artifacts/env-092-tn10-verifiable-entropy-round"
ENV093_DIR="spikes/kaspa-foundation/artifacts/env-093-verifiable-tn10-entropy-milestone"
SAMPLE="examples/roulette-poc/ui/sample-round.json"
PROOF="examples/roulette-poc/ui/toccata-fairness-proof.json"

require_file() {
  [[ -f "$1" ]] || { echo "missing required file: $1" >&2; exit 1; }
}

require_dir() {
  [[ -d "$1" ]] || { echo "missing required directory: $1" >&2; exit 1; }
}

require_grep() {
  local pattern="$1"
  local path="$2"
  local label="$3"
  grep -Rqs -- "$pattern" "$path" || { echo "missing ${label}: ${pattern} in ${path}" >&2; exit 1; }
}

require_dir "$ENV092_DIR"
require_dir "$ENV093_DIR"

for path in   "docs/env093-verifiable-tn10-entropy-milestone.md"   "docs/verifiable-tn10-entropy-milestone-summary.md"   "$ENV093_DIR/env-093-summary.md"   "$ENV093_DIR/env-093-claim-boundary.md"   "$ENV093_DIR/env-093-reviewer-checklist.json"   "$ENV093_DIR/env-093-command-results.txt"   "$ENV093_DIR/env-093-git-status.txt"   "$ENV092_DIR/env-092-summary.md"   "$ENV092_DIR/env-092-no-more-bets-evidence.json"   "$ENV092_DIR/env-092-entropy-target.json"   "$ENV092_DIR/env-092-tn10-entropy-readback.json"   "$ENV092_DIR/env-092-final-entropy-transcript.json"   "$ENV092_DIR/env-092-result-derivation.json"   "$ENV092_DIR/env-092-verifier-output.json"   "$SAMPLE"   "$PROOF"; do
  require_file "$path"
done

python3 - "$ENV092_DIR" "$ENV093_DIR" "$SAMPLE" "$PROOF" <<'PY'
import json, pathlib, sys

env092 = pathlib.Path(sys.argv[1])
env093 = pathlib.Path(sys.argv[2])
sample = json.loads(pathlib.Path(sys.argv[3]).read_text(encoding='utf-8'))
proof = json.loads(pathlib.Path(sys.argv[4]).read_text(encoding='utf-8'))
no_more = json.loads((env092 / 'env-092-no-more-bets-evidence.json').read_text(encoding='utf-8'))
target = json.loads((env092 / 'env-092-entropy-target.json').read_text(encoding='utf-8'))
entropy = json.loads((env092 / 'env-092-tn10-entropy-readback.json').read_text(encoding='utf-8'))
transcript = json.loads((env092 / 'env-092-final-entropy-transcript.json').read_text(encoding='utf-8'))
derivation = json.loads((env092 / 'env-092-result-derivation.json').read_text(encoding='utf-8'))
verifier = json.loads((env092 / 'env-092-verifier-output.json').read_text(encoding='utf-8'))
checklist = json.loads((env093 / 'env-093-reviewer-checklist.json').read_text(encoding='utf-8'))
errors = []

def require(label, condition):
    if not condition:
        errors.append(label)

require('sample source_env ENV-092', sample.get('source_env') == 'ENV-092')
require('proof source_env ENV-092', proof.get('source_env') == 'ENV-092')
require('result_number 34', sample.get('result_number') == proof.get('result_number') == derivation.get('result_number') == 34)
require('result_colour red', sample.get('result_colour') == proof.get('result_colour') == derivation.get('result_colour') == 'red')
require('verifier_result PASS', proof.get('verifier_result') == verifier.get('verifier_result') == 'PASS')
require('no_more_bets_txid present', no_more.get('no_more_bets_txid') == 'fd02e7d66ebe06aa50a106b02b3fad976a4e700323b40a4e48a9574108bf34c0')
require('entropy target blue score', target.get('entropy_target_blue_score') == no_more.get('entropy_target_blue_score') == proof.get('no_more_bets_evidence', {}).get('entropy_target_blue_score') == 492892499)
require('entropy source block hash present', entropy.get('entropy_source_block_hash') == proof.get('tn10_entropy_readback', {}).get('entropy_source_block_hash') == '76b09cd0f4eaaaaa668df0af324a920fd44b4d5f75a0ef327df9e5d41c24cbe3')
require('final entropy hash present', derivation.get('final_entropy_hash') == proof.get('final_entropy_hash') == 'f14b87fafbb6d5b8fd3bc1126bf78a87205c1e0e5830543cf13c091eb139df52')
require('sample production_randomness_claimed false', sample.get('production_randomness_claimed') is False)
require('proof production_randomness_claimed false', proof.get('production_randomness_claimed') is False)
require('proof safety production_randomness_claimed false', proof.get('safety_flags', {}).get('production_randomness_claimed') is False)
require('transcript includes future entropy value', transcript.get('tn10_future_entropy_value') == entropy.get('entropy_value_used_in_transcript'))
require('entropy source at target', entropy.get('entropy_source_blue_score') == 492892499)
require('checklist env', checklist.get('env') == 'ENV-093')
require('checklist no implementation', checklist.get('implementation_scope', {}).get('adds_new_implementation') is False)
require('checklist readiness line', checklist.get('required_smoke', {}).get('expected_final_line') == 'VERIFIABLE_TN10_ENTROPY_MILESTONE_READY=PASS')
if errors:
    for error in errors:
        print(f'FAIL: {error}', file=sys.stderr)
    raise SystemExit(1)
PY

require_grep 'fd02e7d66ebe06aa50a106b02b3fad976a4e700323b40a4e48a9574108bf34c0' "docs/env093-verifiable-tn10-entropy-milestone.md" "NoMoreBets txid in runbook"
require_grep '492892499' "docs/env093-verifiable-tn10-entropy-milestone.md" "entropy target blue score in runbook"
require_grep '76b09cd0f4eaaaaa668df0af324a920fd44b4d5f75a0ef327df9e5d41c24cbe3' "docs/env093-verifiable-tn10-entropy-milestone.md" "TN10 entropy block hash in runbook"
require_grep 'f14b87fafbb6d5b8fd3bc1126bf78a87205c1e0e5830543cf13c091eb139df52' "docs/env093-verifiable-tn10-entropy-milestone.md" "final entropy hash in runbook"
require_grep '34' "docs/env093-verifiable-tn10-entropy-milestone.md" "result number in runbook"
require_grep 'red' "docs/env093-verifiable-tn10-entropy-milestone.md" "result colour in runbook"
require_grep 'production casino randomness' "docs/env093-verifiable-tn10-entropy-milestone.md" "production randomness non-claim in runbook"
require_grep 'operator abort/griefing' "docs/env093-verifiable-tn10-entropy-milestone.md" "operator abort/griefing risk in runbook"
require_grep 'user/multi-party entropy' "docs/env093-verifiable-tn10-entropy-milestone.md" "user entropy future work in runbook"
require_grep 'KIP-21 sequencing/lane proof' "docs/env093-verifiable-tn10-entropy-milestone.md" "KIP-21 future work in runbook"
require_grep 'UI displays' "docs/verifiable-tn10-entropy-milestone-summary.md" "UI display boundary in summary"

scripts/env092-tn10-verifiable-entropy-round-smoke.sh >/tmp/env093-env092-smoke.out
if ! grep -qx 'TN10_VERIFIABLE_ENTROPY_ROUND_READY=PASS' /tmp/env093-env092-smoke.out; then
  cat /tmp/env093-env092-smoke.out >&2
  echo "ENV-092 smoke did not print required readiness line" >&2
  exit 1
fi

python3 - "$ENV092_DIR" "$ENV093_DIR" <<'PY'
import os, pathlib, re, sys
roots = [pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])]
patterns = [
    re.compile(r'-----BEGIN [A-Z ]*PRIVATE KEY-----'),
    re.compile(r'(kprv|xprv)[A-Za-z0-9]+'),
    re.compile(r"(?i)\b(api[_-]?key|auth[_-]?token|access[_-]?token|secret[_-]?key)\s*[:=]\s*[\"']?[A-Za-z0-9_\-]{16,}"),
    re.compile(r"(?i)\b(seed[_-]?phrase|mnemonic)\s*[:=]\s*[\"']?[a-z]+(?:\s+[a-z]+){11,23}[\"']?"),
]
for root in roots:
    for dirpath, _, filenames in os.walk(root):
        for filename in filenames:
            path = pathlib.Path(dirpath) / filename
            text = path.read_text(encoding='utf-8', errors='ignore')
            for pattern in patterns:
                if pattern.search(text):
                    print(f'secret-like material found in {path}: {pattern.pattern}', file=sys.stderr)
                    raise SystemExit(1)
PY

printf 'VERIFIABLE_TN10_ENTROPY_MILESTONE_READY=PASS
'
