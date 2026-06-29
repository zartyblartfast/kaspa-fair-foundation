#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ENV088_DIR="spikes/kaspa-foundation/artifacts/env-088-tn10-covenant-lineage-commit-reveal"
ENV089_DIR="spikes/kaspa-foundation/artifacts/env-089-live-toccata-covenant-lineage-milestone"
MILESTONE_DOC="docs/env089-live-toccata-covenant-lineage-milestone.md"
SUMMARY_DOC="docs/live-toccata-covenant-lineage-summary.md"
COMMIT_TXID="ebb28c6b34532cb97ae3a0a135fda74a0566b336df4dbf248283c5cad8c9ff65"
REVEAL_TXID="f8fe14932071ac49cdac9e4f3df1177b9655dffbd0ad66b0e7491d6f78e5654b"
COVENANT_ID="9931b78d93e1019ed132d52ccc8dc0b812b7fb5fa41cb561342c184afd11735c"

[[ -d "$ENV088_DIR" ]] || { echo "missing ENV-088 artifact directory" >&2; exit 1; }
[[ -d "$ENV089_DIR" ]] || { echo "missing ENV-089 artifact directory" >&2; exit 1; }

for file in \
  "$ENV088_DIR/env-088-summary.md" \
  "$ENV088_DIR/env-088-commitment-tx-evidence.json" \
  "$ENV088_DIR/env-088-reveal-tx-evidence.json" \
  "$ENV088_DIR/env-088-direct-tn10-commitment-tx.json" \
  "$ENV088_DIR/env-088-direct-tn10-reveal-tx.json" \
  "$ENV088_DIR/env-088-covenant-field-verification.json" \
  "$ENV088_DIR/env-088-verifier-output.json" \
  "$ENV088_DIR/env-088-safety-flags.json" \
  "$MILESTONE_DOC" \
  "$SUMMARY_DOC" \
  "$ENV089_DIR/env-089-summary.md" \
  "$ENV089_DIR/env-089-claim-boundary.md" \
  "$ENV089_DIR/env-089-reviewer-checklist.json" \
  "$ENV089_DIR/env-089-command-results.txt" \
  "$ENV089_DIR/env-089-git-status.txt"; do
  [[ -f "$file" ]] || { echo "missing required artifact: $file" >&2; exit 1; }
done

python3 - "$COMMIT_TXID" "$REVEAL_TXID" "$COVENANT_ID" "$ENV088_DIR" "$ENV089_DIR" "$MILESTONE_DOC" "$SUMMARY_DOC" <<'PY'
import json, os, re, sys
commit_txid, reveal_txid, covenant_id, env088_dir, env089_dir, milestone_doc, summary_doc = sys.argv[1:]
errors=[]
def require(label, condition):
    if not condition:
        errors.append(label)

def load(path):
    with open(path, encoding='utf-8') as fh:
        return json.load(fh)

def text(path):
    with open(path, encoding='utf-8', errors='ignore') as fh:
        return fh.read()

commit = load(os.path.join(env088_dir, 'env-088-commitment-tx-evidence.json'))
reveal = load(os.path.join(env088_dir, 'env-088-reveal-tx-evidence.json'))
field = load(os.path.join(env088_dir, 'env-088-covenant-field-verification.json'))
verifier = load(os.path.join(env088_dir, 'env-088-verifier-output.json'))
checklist = load(os.path.join(env089_dir, 'env-089-reviewer-checklist.json'))
all_text = '\n'.join(text(p) for p in [
    os.path.join(env088_dir, 'env-088-summary.md'),
    os.path.join(env089_dir, 'env-089-summary.md'),
    os.path.join(env089_dir, 'env-089-claim-boundary.md'),
    milestone_doc,
    summary_doc,
])

require('commitment txid present', commit.get('transaction_id') == commit_txid and commit_txid in all_text)
require('reveal txid present', reveal.get('transaction_id') == reveal_txid and reveal_txid in all_text)
require('covenant id present', covenant_id in all_text)
require('claim_level covenant-linked lineage', verifier.get('claim_level') == 'covenant-linked lineage' and 'covenant-linked lineage' in all_text)
require('commit covenant field non-null', commit.get('covenant_evidence', {}).get('any_non_null') is True)
require('reveal covenant field non-null', reveal.get('covenant_evidence', {}).get('any_non_null') is True)
require('reveal spends commitment output', reveal.get('commitment_txid') == commit_txid and reveal.get('reveal_links_commitment_output') is True)
require('covenant id continues', commit.get('covenant_evidence', {}).get('output_covenant_id') == covenant_id and reveal.get('covenant_evidence', {}).get('input_covenant_id') == covenant_id and reveal.get('covenant_evidence', {}).get('output_covenant_id') == covenant_id)
require('direct TN10 field evidence source', field.get('evidence_source') == 'direct TN10 transaction readback fields, not payload JSON')
require('payload only rejected', field.get('payload_only_covenant_id_rejected') is True and verifier.get('payload_only_covenant_id_rejected') is True)
require('full KIP-17 not claimed', re.search(r'not claim[s]?:?\s*(?:\n|.){0,200}full KIP-17', all_text, re.I) or 'full KIP-17 covenant-enforced state transition logic' in all_text)
require('reviewer checklist schema', checklist.get('schema') == 'kaspa-fair-env089-reviewer-checklist-v1')
require('reviewer checklist readiness line', checklist.get('expected_readiness_line') == 'LIVE_TOCCATA_COVENANT_LINEAGE_MILESTONE_READY=PASS')
require('not payload-only JSON wording', 'not payload JSON' in all_text or 'not payload-only JSON' in all_text)
require('stronger than bare anchor wording', 'stronger than ENV-087 bare TN10 anchoring' in all_text or 'stronger than a plain TN10 anchor' in all_text)
require('safety boundary wording', all(term in all_text for term in ['no new transaction', 'no signing', 'no broadcast', 'no mainnet']))

secret_patterns = [
    re.compile(r'-----BEGIN [A-Z ]*PRIVATE KEY-----'),
    re.compile(r'\b(kprv|xprv)[A-Za-z0-9]+\b'),
    re.compile(r'(?i)(api[_-]?key|auth[_-]?token|secret|private[_-]?key)\s*[:=]\s*["\']?[A-Za-z0-9_./+\-]{16,}'),
]
for root in [env088_dir, env089_dir]:
    for dirpath, _, filenames in os.walk(root):
        for name in filenames:
            path = os.path.join(dirpath, name)
            body = text(path)
            for pattern in secret_patterns:
                if pattern.search(body):
                    errors.append(f'secret-like material found: {path}: {pattern.pattern}')

if errors:
    for error in errors:
        print(f'FAIL: {error}', file=sys.stderr)
    sys.exit(1)
PY

scripts/env088-tn10-covenant-lineage-commit-reveal-smoke.sh >/tmp/env089-env088-smoke.out
if ! grep -qx 'TN10_COVENANT_LINEAGE_COMMIT_REVEAL_READY=PASS' /tmp/env089-env088-smoke.out; then
  cat /tmp/env089-env088-smoke.out >&2
  echo "ENV-088 smoke did not print expected readiness line" >&2
  exit 1
fi

printf 'LIVE_TOCCATA_COVENANT_LINEAGE_MILESTONE_READY=PASS\n'
