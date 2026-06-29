#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ENV090_DIR="spikes/kaspa-foundation/artifacts/env-090-kip17-covenant-enforced-transition"
ENV090_UI_DIR="spikes/kaspa-foundation/artifacts/env-090-ui-accepts-kip17-proof"
ENV091_DIR="spikes/kaspa-foundation/artifacts/env-091-full-kip17-toccata-milestone"

COMMITMENT_TXID="050bbe398ff7e8f7511697c65b511ab23bf1548bcba1ed0fb29380d1e582ec26"
REVEAL_TXID="269abfe10635d666d0c5b7624550a4abee5a47a8bd08d6a0e0b1a09dc2cf0620"
CLAIM_LEVEL="full_kip17_covenant_enforced_transition"

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

require_dir "$ENV090_DIR"
require_dir "$ENV090_UI_DIR"
require_dir "$ENV091_DIR"

require_file "docs/env091-full-kip17-toccata-milestone.md"
require_file "docs/full-kip17-toccata-milestone-summary.md"
require_file "$ENV091_DIR/env-091-summary.md"
require_file "$ENV091_DIR/env-091-claim-boundary.md"
require_file "$ENV091_DIR/env-091-reviewer-checklist.json"
require_file "$ENV091_DIR/env-091-command-results.txt"
require_file "$ENV091_DIR/env-091-git-status.txt"

require_grep "$COMMITMENT_TXID" "$ENV090_DIR" "ENV-090 commitment txid"
require_grep "$REVEAL_TXID" "$ENV090_DIR" "ENV-090 reveal/continuation txid"
require_grep "$COMMITMENT_TXID" "docs/env091-full-kip17-toccata-milestone.md" "documented commitment txid"
require_grep "$REVEAL_TXID" "docs/env091-full-kip17-toccata-milestone.md" "documented reveal/continuation txid"
require_grep "$CLAIM_LEVEL" "$ENV090_DIR" "ENV-090 claim level"
require_grep "$CLAIM_LEVEL" "docs/env091-full-kip17-toccata-milestone.md" "documented claim level"
require_grep '"verifier_result"[[:space:]]*:[[:space:]]*"PASS"' "$ENV090_DIR/env-090-verifier-output.json" "verifier_result PASS"
require_grep '"kip17_rule_enforced_on_transition"[[:space:]]*:[[:space:]]*true' "$ENV090_DIR" "KIP-17 transition enforcement true"
require_grep '"invalid_transition_rejected"[[:space:]]*:[[:space:]]*true' "$ENV090_DIR/env-090-verifier-output.json" "invalid transition rejected true"

python3 - "$ENV090_DIR" "$ENV090_UI_DIR" <<'PY'
import json, pathlib, sys
root = pathlib.Path(sys.argv[1])
ui_root = pathlib.Path(sys.argv[2])
verifier = json.loads((root / 'env-090-verifier-output.json').read_text(encoding='utf-8'))
enforcement = json.loads((root / 'env-090-kip17-enforcement-verification.json').read_text(encoding='utf-8'))
commit_direct = json.loads((root / 'env-090-direct-tn10-commitment-tx.json').read_text(encoding='utf-8'))
reveal_direct = json.loads((root / 'env-090-direct-tn10-reveal-tx.json').read_text(encoding='utf-8'))
ui_evidence = (ui_root / 'env-090-ui-proof-validation-evidence.txt').read_text(encoding='utf-8')
errors = []
def require(label, condition):
    if not condition:
        errors.append(label)
require('verifier_result PASS', verifier.get('verifier_result') == 'PASS')
require('claim_level full KIP-17', verifier.get('claim_level') == 'full_kip17_covenant_enforced_transition')
require('kip17_rule_enforced_on_transition true', verifier.get('kip17_rule_enforced_on_transition') is True and enforcement.get('kip17_rule_enforced_on_transition') is True)
require('invalid_transition_rejected true', verifier.get('invalid_transition_rejected') is True)
require('valid increment transition passed', enforcement.get('valid_increment_transition_passed') is True)
require('invalid no increment rejected', enforcement.get('invalid_no_increment_rejected') is True)
require('commitment direct TN10 accepted', commit_direct.get('is_accepted') is True and commit_direct.get('transaction_id') == '050bbe398ff7e8f7511697c65b511ab23bf1548bcba1ed0fb29380d1e582ec26')
require('reveal direct TN10 accepted', reveal_direct.get('is_accepted') is True and reveal_direct.get('transaction_id') == '269abfe10635d666d0c5b7624550a4abee5a47a8bd08d6a0e0b1a09dc2cf0620')
require('reveal readback links commitment', reveal_direct['inputs'][0].get('previous_outpoint_hash') == '050bbe398ff7e8f7511697c65b511ab23bf1548bcba1ed0fb29380d1e582ec26')
require('UI accepts authorised proof evidence', 'PASS: app.js accepts current authorised ENV-090 proof' in ui_evidence)
for label in [
    'unsafe mainnet proof rejected',
    'unsafe real betting proof rejected',
    'unsafe real payouts proof rejected',
    'unsafe backend custody proof rejected',
    'unsafe production randomness proof rejected',
    'verifier_result not PASS rejected',
    'unknown source_env rejected',
    'unsupported live claim_level rejected',
    'missing live commitment evidence rejected',
    'mismatched sample/proof result rejected',
    'secret-like UI material rejected',
]:
    require(f'UI {label}', f'PASS: {label}' in ui_evidence)
if errors:
    for error in errors:
        print(f'FAIL: {error}', file=sys.stderr)
    raise SystemExit(1)
PY

scripts/env090-kip17-covenant-enforced-transition-smoke.sh >/tmp/env091-env090-smoke.out
# The ENV-090 smoke refreshes its own tracked git-status artifact. ENV-091
# verifies the older smoke but must not package or leave older ENV artifacts
# modified as part of this documentation/checkpoint step.
if git ls-files --error-unmatch "$ENV090_DIR/env-090-git-status.txt" >/dev/null 2>&1; then
  git restore -- "$ENV090_DIR/env-090-git-status.txt"
fi
if ! grep -qx 'KIP17_COVENANT_ENFORCED_TRANSITION_READY=PASS' /tmp/env091-env090-smoke.out; then
  cat /tmp/env091-env090-smoke.out >&2
  echo "ENV-090 smoke did not print required readiness line" >&2
  exit 1
fi

scripts/env090-ui-accepts-kip17-proof-smoke.sh >/tmp/env091-env090-ui-smoke.out
# Same preservation rule for the ENV-090 UI-fix smoke.
if git ls-files --error-unmatch "$ENV090_UI_DIR/env-090-ui-git-status.txt" >/dev/null 2>&1; then
  git restore -- "$ENV090_UI_DIR/env-090-ui-git-status.txt"
fi
if ! grep -qx 'UI_ACCEPTS_KIP17_PROOF_READY=PASS' /tmp/env091-env090-ui-smoke.out; then
  cat /tmp/env091-env090-ui-smoke.out >&2
  echo "ENV-090 UI smoke did not print required readiness line" >&2
  exit 1
fi

python3 - "$ENV090_DIR" "$ENV090_UI_DIR" "$ENV091_DIR" <<'PY'
import os, pathlib, re, sys
roots = [pathlib.Path(p) for p in sys.argv[1:]]
patterns = [
    re.compile(r'-----BEGIN [A-Z ]*PRIVATE KEY-----'),
    re.compile(r'\b(kprv|xprv)[A-Za-z0-9]+\b'),
    re.compile(r'(?i)\b(api[_-]?key|auth[_-]?token|access[_-]?token|secret[_-]?key)\s*[:=]\s*["\']?[A-Za-z0-9_\-]{16,}'),
    re.compile(r'(?i)\b(seed[_-]?phrase|mnemonic)\s*[:=]\s*["\']?[a-z]+(?:\s+[a-z]+){11,23}["\']?'),
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

printf 'FULL_KIP17_TOCCATA_MILESTONE_READY=PASS\n'
