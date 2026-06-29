#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ARTIFACT_DIR="spikes/kaspa-foundation/artifacts/env-088-tn10-covenant-lineage-commit-reveal"
PREFLIGHT="$ARTIFACT_DIR/env-088-preflight.json"
CONSTRUCTION="$ARTIFACT_DIR/env-088-covenant-construction-evidence.md"
COMMIT_PAYLOAD="$ARTIFACT_DIR/env-088-commitment-payload.json"
REVEAL_PAYLOAD="$ARTIFACT_DIR/env-088-reveal-payload.json"
COMMIT_EVIDENCE="$ARTIFACT_DIR/env-088-commitment-tx-evidence.json"
REVEAL_EVIDENCE="$ARTIFACT_DIR/env-088-reveal-tx-evidence.json"
DIRECT_COMMIT="$ARTIFACT_DIR/env-088-direct-tn10-commitment-tx.json"
DIRECT_REVEAL="$ARTIFACT_DIR/env-088-direct-tn10-reveal-tx.json"
FIELD_VERIFY="$ARTIFACT_DIR/env-088-covenant-field-verification.json"
VERIFIER="$ARTIFACT_DIR/env-088-verifier-output.json"
SAFETY="$ARTIFACT_DIR/env-088-safety-flags.json"
PROOF="examples/roulette-poc/ui/toccata-fairness-proof.json"
SAMPLE="examples/roulette-poc/ui/sample-round.json"

[[ -d "$ARTIFACT_DIR" ]] || { echo "missing ENV-088 artifact directory" >&2; exit 1; }
for file in "$PREFLIGHT" "$CONSTRUCTION" "$COMMIT_PAYLOAD" "$REVEAL_PAYLOAD" "$COMMIT_EVIDENCE" "$REVEAL_EVIDENCE" "$DIRECT_COMMIT" "$DIRECT_REVEAL" "$FIELD_VERIFY" "$VERIFIER" "$SAFETY"; do
  [[ -f "$file" ]] || { echo "missing required ENV-088 artifact: $file" >&2; exit 1; }
done

python3 - "$PREFLIGHT" "$COMMIT_PAYLOAD" "$REVEAL_PAYLOAD" "$COMMIT_EVIDENCE" "$REVEAL_EVIDENCE" "$DIRECT_COMMIT" "$DIRECT_REVEAL" "$FIELD_VERIFY" "$VERIFIER" "$SAFETY" "$PROOF" "$SAMPLE" <<'PY'
import json, subprocess, sys
preflight, commit_payload, reveal_payload, commit_evidence, reveal_evidence, direct_commit, direct_reveal, field_verify, verifier, safety, proof, sample = [json.load(open(p, encoding='utf-8')) for p in sys.argv[1:]]
errors=[]
def require(label, condition):
    if not condition:
        errors.append(label)

require('preflight network testnet-10', preflight.get('network') == 'testnet-10')
require('preflight mainnet excluded', preflight.get('mainnet_excluded') is True and preflight.get('mainnet_supported') is False)
require('preflight covenant types identified', preflight.get('covenant_construction_types_identified') is True)
require('preflight output covenant binding implemented', preflight.get('output_covenant_binding_construction_implemented') is True)
require('commit payload round id', commit_payload.get('round_id') == 'env-088-covenant-round-0001')
require('reveal payload round id', reveal_payload.get('round_id') == commit_payload.get('round_id'))
for label, evidence, direct in [('commitment', commit_evidence, direct_commit), ('reveal', reveal_evidence, direct_reveal)]:
    require(f'{label} network testnet-10', evidence.get('network') == 'testnet-10')
    require(f'{label} accepted true', evidence.get('is_accepted') is True)
    require(f'{label} direct accepted true', direct.get('is_accepted') is True)
    require(f'{label} txid matches direct', evidence.get('transaction_id') == direct.get('transaction_id'))
    require(f'{label} broadcast true', evidence.get('broadcast_used') is True)
    require(f'{label} signing true', evidence.get('signing_used') is True)
    require(f'{label} mainnet false', evidence.get('mainnet_supported') is False)
    require(f'{label} txid present', isinstance(evidence.get('transaction_id'), str) and len(evidence['transaction_id']) == 64)
require('reveal links commitment evidence', reveal_evidence.get('commitment_txid') == commit_evidence.get('transaction_id'))
require('reveal spends commitment output', reveal_evidence.get('reveal_links_commitment_output') is True)
require('field verify non-null covenant evidence', field_verify.get('any_required_covenant_evidence_non_null') is True)
require('field verify direct source', field_verify.get('evidence_source') == 'direct TN10 transaction readback fields, not payload JSON')
require('payload only covenant rejected', field_verify.get('payload_only_covenant_id_rejected') is True and verifier.get('payload_only_covenant_id_rejected') is True)
require('bare anchor rejected', field_verify.get('bare_tn10_anchor_rejected_for_env088_pass') is True and verifier.get('bare_tn10_anchor_rejected_for_env088_pass') is True)
require('claim level covenant linked or full', verifier.get('claim_level') in ('covenant-linked lineage', 'full covenant transition'))
require('claim level not bare', verifier.get('claim_level') != 'bare_tn10_anchor')
require('verifier pass', verifier.get('verifier_result') == 'PASS')
require('commitment hash matches reveal', verifier.get('commitment_hash_matches_reveal_material') is True)
require('result derives', verifier.get('result_derives_from_reveal_material') is True)
if proof.get('source_env') == 'ENV-090':
    # ENV-090 is an authorised later app-facing proof source. Keep ENV-088
    # artifact validation strict, but do not require current proof txids/results
    # to equal the historical ENV-088 transaction pair.
    require('app-facing artifacts advanced to ENV-090', proof.get('claim_level') == 'full_kip17_covenant_enforced_transition')
else:
    require('sample result number agrees', sample.get('result_number') == verifier.get('result_number'))
    require('sample result colour agrees', sample.get('result_colour') == verifier.get('result_colour'))
    require('proof env088 source', proof.get('source_env') == 'ENV-088')
    require('proof claim level not bare', proof.get('claim_level') in ('covenant-linked lineage', 'full covenant transition'))
    require('proof commitment txid agrees', proof.get('live_round_commitment_evidence', {}).get('transaction_id') == commit_evidence.get('transaction_id'))
    require('proof reveal txid agrees', proof.get('live_round_reveal_evidence', {}).get('transaction_id') == reveal_evidence.get('transaction_id'))
require('safety network', safety.get('network') == 'testnet-10')
for field in ['mainnet_supported','real_betting','real_payouts','backend_custody','production_randomness_claimed','private_key_material_written_to_artifacts']:
    require(f'safety {field} false', safety.get(field) is False)

# Ensure covenant evidence is from top-level tx/input/output fields, not a payload-only covenant_id string.
commit_cov = commit_evidence.get('covenant_evidence', {})
reveal_cov = reveal_evidence.get('covenant_evidence', {})
require('transaction covenant evidence field non-null', commit_cov.get('any_non_null') is True or reveal_cov.get('any_non_null') is True)
for payload in [commit_payload, reveal_payload]:
    require('payload has no masquerading covenant_id field', 'covenant_id' not in payload)

expected_raw = subprocess.check_output(['cargo','run','-q','-p','kaspa-fair-cli','--','env084-generate-verifiable-demo-round','--round-id','env-088-covenant-round-0001','--demo-seed','env088-demo-seed-0001','--json'], text=True)
expected = json.loads(expected_raw)
expected_proof = expected['proof_artifact']
expected_commit = expected_proof['application_round_transcript']['commitment']
expected_reveal = expected_proof['application_round_transcript']['reveal']
require('commitment hash matches Rust transcript', commit_payload.get('commitment_hash') == expected_commit.get('commitment_hash'))
require('reveal hash matches Rust transcript', reveal_payload.get('reveal_payload_hash') == expected_reveal.get('reveal_payload_hash'))
require('result number matches Rust transcript', reveal_payload.get('result_number') == expected_reveal.get('result_number'))
require('result colour matches Rust transcript', reveal_payload.get('result_colour') == expected_reveal.get('result_colour'))

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
  examples/roulette-poc/ui/app.js examples/roulette-poc/ui/index.html examples/roulette-poc/ui/roulette-table-renderer.js >/tmp/env088-ui-random-grep.txt || { cat /tmp/env088-ui-random-grep.txt >&2; exit 1; }

git status --short > "$ARTIFACT_DIR/env-088-git-status.txt"
printf 'TN10_COVENANT_LINEAGE_COMMIT_REVEAL_READY=PASS\n'
