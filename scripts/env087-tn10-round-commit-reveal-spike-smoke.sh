#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ARTIFACT_DIR="spikes/kaspa-foundation/artifacts/env-087-tn10-round-commit-reveal-spike"
PREFLIGHT="$ARTIFACT_DIR/env-087-preflight.json"
COMMIT_PAYLOAD="$ARTIFACT_DIR/env-087-commitment-payload.json"
REVEAL_PAYLOAD="$ARTIFACT_DIR/env-087-reveal-payload.json"
COMMIT_EVIDENCE="$ARTIFACT_DIR/env-087-commitment-tx-evidence.json"
REVEAL_EVIDENCE="$ARTIFACT_DIR/env-087-reveal-tx-evidence.json"
READBACK="$ARTIFACT_DIR/env-087-live-readback-evidence.json"
VERIFIER="$ARTIFACT_DIR/env-087-verifier-output.json"
SAFETY="$ARTIFACT_DIR/env-087-safety-flags.json"
PROOF="examples/roulette-poc/ui/toccata-fairness-proof.json"
SAMPLE="examples/roulette-poc/ui/sample-round.json"

[[ -d "$ARTIFACT_DIR" ]] || { echo "missing ENV-087 artifact directory" >&2; exit 1; }
for file in "$PREFLIGHT" "$COMMIT_PAYLOAD" "$REVEAL_PAYLOAD" "$COMMIT_EVIDENCE" "$REVEAL_EVIDENCE" "$READBACK" "$VERIFIER" "$SAFETY"; do
  [[ -f "$file" ]] || { echo "missing required ENV-087 artifact: $file" >&2; exit 1; }
done

python3 - "$PREFLIGHT" "$COMMIT_PAYLOAD" "$REVEAL_PAYLOAD" "$COMMIT_EVIDENCE" "$REVEAL_EVIDENCE" "$READBACK" "$VERIFIER" "$SAFETY" "$PROOF" "$SAMPLE" <<'PY'
import json, subprocess, sys
paths = sys.argv[1:]
preflight, commit_payload, reveal_payload, commit_evidence, reveal_evidence, readback, verifier, safety, proof, sample = [json.load(open(p, encoding='utf-8')) for p in paths]
errors = []

def require(label, condition):
    if not condition:
        errors.append(label)

require('preflight network testnet-10', preflight.get('network') == 'testnet-10')
require('preflight mainnet_supported false', preflight.get('mainnet_supported') is False)
require('commitment payload round id', commit_payload.get('round_id') == 'env-087-live-round-0001')
require('reveal payload round id', reveal_payload.get('round_id') == commit_payload.get('round_id'))
for label, evidence in [('commitment', commit_evidence), ('reveal', reveal_evidence)]:
    require(f'{label} network testnet-10', evidence.get('network') == 'testnet-10')
    require(f'{label} accepted true', evidence.get('accepted') is True)
    require(f'{label} broadcast_used true', evidence.get('broadcast_used') is True)
    require(f'{label} signing_used true', evidence.get('signing_used') is True)
    require(f'{label} wallet_access_used true', evidence.get('wallet_access_used') is True)
    require(f'{label} mainnet_supported false', evidence.get('mainnet_supported') is False)
    require(f'{label} claim level bare anchor', evidence.get('claim_level') == 'bare TN10 anchor')
    require(f'{label} txid present', isinstance(evidence.get('transaction_id'), str) and len(evidence['transaction_id']) == 64)
    rb = evidence.get('readback', {})
    require(f'{label} readback accepted', rb.get('is_accepted') is True)
    require(f'{label} readback txid matches', rb.get('transaction_id') == evidence.get('transaction_id'))
require('reveal links to commitment evidence', reveal_evidence.get('commitment_txid') == commit_evidence.get('transaction_id'))
require('reveal payload links commitment txid', reveal_payload.get('commitment_txid') == commit_evidence.get('transaction_id'))
require('readback network testnet-10', readback.get('network') == 'testnet-10')
require('verifier PASS', verifier.get('verifier_result') == 'PASS')
require('verifier commitment present', verifier.get('env087_live_round_commitment_evidence') == 'present')
require('verifier reveal present', verifier.get('env087_live_round_reveal_evidence') == 'present')
require('safety mainnet false', safety.get('mainnet_supported') is False)
require('safety broadcast true', safety.get('broadcast_used') is True)
require('safety signing true', safety.get('signing_used') is True)
require('safety wallet access true', safety.get('wallet_access_used') is True)
require('no private key in artifacts', safety.get('private_key_material_written_to_artifacts') is False)

expected_raw = subprocess.check_output([
    'cargo','run','-q','-p','kaspa-fair-cli','--','env084-generate-verifiable-demo-round',
    '--round-id','env-087-live-round-0001','--demo-seed','env087-demo-seed-0001','--json'
], text=True)
expected = json.loads(expected_raw)
expected_proof = expected['proof_artifact']
expected_commit = expected_proof['application_round_transcript']['commitment']
expected_reveal = expected_proof['application_round_transcript']['reveal']
require('commitment hash matches reveal material', commit_payload.get('commitment_hash') == expected_commit.get('commitment_hash'))
require('reveal payload hash matches Rust transcript', reveal_payload.get('reveal_payload_hash') == expected_reveal.get('reveal_payload_hash'))
require('result number derives from reveal material', reveal_payload.get('result_number') == expected_reveal.get('result_number'))
require('result colour derives from reveal material', reveal_payload.get('result_colour') == expected_reveal.get('result_colour'))
if proof.get('source_env') == 'ENV-088':
    require('app-facing artifacts advanced to authorised ENV-088', proof.get('claim_level') in ('covenant-linked lineage', 'full covenant transition'))
else:
    require('sample result number agrees', sample.get('result_number') == reveal_payload.get('result_number'))
    require('sample result colour agrees', sample.get('result_colour') == reveal_payload.get('result_colour'))
    require('proof live commitment evidence present', proof.get('live_round_commitment_evidence', {}).get('status') == 'present')
    require('proof live reveal evidence present', proof.get('live_round_reveal_evidence', {}).get('status') == 'present')
    require('proof does not retain future not-created sentinel', proof.get('future_live_round_transaction_evidence') != 'not_created_not_claimed_future_work')
    require('proof commitment txid agrees', proof.get('live_round_commitment_evidence', {}).get('transaction_id') == commit_evidence.get('transaction_id'))
    require('proof reveal txid agrees', proof.get('live_round_reveal_evidence', {}).get('transaction_id') == reveal_evidence.get('transaction_id'))

if errors:
    for error in errors:
        print(f'FAIL: {error}', file=sys.stderr)
    sys.exit(1)
PY

python3 - "$ARTIFACT_DIR" <<'PY'
import os, re, sys
root = sys.argv[1]
patterns = [
    re.compile(r'-----BEGIN [A-Z ]*PRIVATE KEY-----'),
    re.compile(r'\b(kprv|xprv)[A-Za-z0-9]+\b'),
    re.compile(r'\b[A-Fa-f0-9]{64}\b.*(?:private|secret|token)', re.I),
    re.compile(r'(api[_-]?key|auth[_-]?token)\s*[:=]\s*["\']?[A-Za-z0-9_\-]{16,}', re.I),
]
for dirpath, _, filenames in os.walk(root):
    for name in filenames:
        path = os.path.join(dirpath, name)
        text = open(path, encoding='utf-8', errors='ignore').read()
        for pattern in patterns:
            if pattern.search(text):
                print(f'secret-like material found in {path}: {pattern.pattern}', file=sys.stderr)
                sys.exit(1)
PY

! grep -RInE 'Math\.random|crypto\.getRandomValues|crypto\.randomUUID|window\.crypto|globalThis\.crypto|self\.crypto' \
  examples/roulette-poc/ui/app.js examples/roulette-poc/ui/index.html examples/roulette-poc/ui/roulette-table-renderer.js >/tmp/env087-ui-random-grep.txt || { cat /tmp/env087-ui-random-grep.txt >&2; exit 1; }

git status --short > "$ARTIFACT_DIR/env-087-git-status.txt"

printf 'TN10_ROUND_COMMIT_REVEAL_SPIKE_READY=PASS\n'
