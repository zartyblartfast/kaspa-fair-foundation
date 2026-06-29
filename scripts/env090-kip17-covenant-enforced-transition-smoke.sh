#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

ARTIFACT_DIR="spikes/kaspa-foundation/artifacts/env-090-kip17-covenant-enforced-transition"
PREFLIGHT="$ARTIFACT_DIR/env-090-preflight.json"
CONSTRUCTION="$ARTIFACT_DIR/env-090-kip17-construction-evidence.md"
COMMIT_PAYLOAD="$ARTIFACT_DIR/env-090-commitment-payload.json"
REVEAL_PAYLOAD="$ARTIFACT_DIR/env-090-reveal-payload.json"
COMMIT_EVIDENCE="$ARTIFACT_DIR/env-090-commitment-tx-evidence.json"
REVEAL_EVIDENCE="$ARTIFACT_DIR/env-090-reveal-tx-evidence.json"
DIRECT_COMMIT="$ARTIFACT_DIR/env-090-direct-tn10-commitment-tx.json"
DIRECT_REVEAL="$ARTIFACT_DIR/env-090-direct-tn10-reveal-tx.json"
FIELD_VERIFY="$ARTIFACT_DIR/env-090-covenant-field-verification.json"
ENFORCEMENT="$ARTIFACT_DIR/env-090-kip17-enforcement-verification.json"
NEGATIVE="$ARTIFACT_DIR/env-090-negative-transition-checks.txt"
VERIFIER="$ARTIFACT_DIR/env-090-verifier-output.json"
SAFETY="$ARTIFACT_DIR/env-090-safety-flags.json"
PROOF="examples/roulette-poc/ui/toccata-fairness-proof.json"
SAMPLE="examples/roulette-poc/ui/sample-round.json"

[[ -d "$ARTIFACT_DIR" ]] || { echo "missing ENV-090 artifact directory" >&2; exit 1; }
for file in "$PREFLIGHT" "$CONSTRUCTION" "$COMMIT_PAYLOAD" "$REVEAL_PAYLOAD" "$COMMIT_EVIDENCE" "$REVEAL_EVIDENCE" "$DIRECT_COMMIT" "$DIRECT_REVEAL" "$FIELD_VERIFY" "$ENFORCEMENT" "$NEGATIVE" "$VERIFIER" "$SAFETY"; do
  [[ -f "$file" ]] || { echo "missing required ENV-090 artifact: $file" >&2; exit 1; }
done

python3 - "$PREFLIGHT" "$COMMIT_PAYLOAD" "$REVEAL_PAYLOAD" "$COMMIT_EVIDENCE" "$REVEAL_EVIDENCE" "$DIRECT_COMMIT" "$DIRECT_REVEAL" "$FIELD_VERIFY" "$ENFORCEMENT" "$VERIFIER" "$SAFETY" "$PROOF" "$SAMPLE" <<'PY'
import json, subprocess, sys
preflight, commit_payload, reveal_payload, commit_evidence, reveal_evidence, direct_commit, direct_reveal, field_verify, enforcement, verifier, safety, proof, sample = [json.load(open(p, encoding='utf-8')) for p in sys.argv[1:]]
errors=[]
def require(label, condition):
    if not condition:
        errors.append(label)

require('claim level full kip17', verifier.get('claim_level') == 'full_kip17_covenant_enforced_transition')
require('verifier pass', verifier.get('verifier_result') == 'PASS')
require('preflight network', preflight.get('network') == 'testnet-10')
require('preflight mainnet excluded', preflight.get('mainnet_excluded') is True and preflight.get('mainnet_supported') is False)
require('preflight kip17 tooling', preflight.get('kip17_construction_tooling_available') is True)
require('preflight negative check', preflight.get('negative_transition_check_defined') is True)
for label, evidence, direct in [('commitment', commit_evidence, direct_commit), ('reveal', reveal_evidence, direct_reveal)]:
    require(f'{label} network testnet-10', evidence.get('network') == 'testnet-10')
    require(f'{label} accepted true', evidence.get('is_accepted') is True)
    require(f'{label} direct accepted true', direct.get('is_accepted') is True)
    require(f'{label} txid matches direct', evidence.get('transaction_id') == direct.get('transaction_id'))
    require(f'{label} txid present', isinstance(evidence.get('transaction_id'), str) and len(evidence['transaction_id']) == 64)
    require(f'{label} not mainnet', evidence.get('mainnet_supported') is False)
require('reveal links commitment', reveal_evidence.get('commitment_txid') == commit_evidence.get('transaction_id'))
require('reveal spends commitment output', reveal_evidence.get('reveal_links_commitment_output') is True)
require('non-null covenant evidence', field_verify.get('any_required_covenant_evidence_non_null') is True)
require('direct/script evidence source', 'payload JSON' in field_verify.get('evidence_source','') and 'KIP-17' in field_verify.get('evidence_source',''))
require('kip17 enforcement exists', enforcement.get('kip17_rule_enforced_on_transition') is True)
require('invalid transition rejected', enforcement.get('invalid_no_increment_rejected') is True and verifier.get('invalid_transition_rejected') is True)
require('valid transition passed', enforcement.get('valid_increment_transition_passed') is True)
require('not bare anchor', verifier.get('claim_level') != 'bare_tn10_anchor')
require('not lineage only', verifier.get('claim_level') != 'covenant-linked lineage')
require('bare rejected', enforcement.get('bare_tn10_anchor_rejected_for_env090_pass') is True and verifier.get('bare_tn10_anchor_rejected_for_env090_pass') is True)
require('lineage-only rejected', enforcement.get('kip20_lineage_only_rejected_for_env090_pass') is True and verifier.get('kip20_lineage_only_rejected_for_env090_pass') is True)
require('proof source env090', proof.get('source_env') == 'ENV-090')
require('proof claim level', proof.get('claim_level') == 'full_kip17_covenant_enforced_transition')
require('proof commitment txid agrees', proof.get('live_round_commitment_evidence', {}).get('transaction_id') == commit_evidence.get('transaction_id'))
require('proof reveal txid agrees', proof.get('live_round_reveal_evidence', {}).get('transaction_id') == reveal_evidence.get('transaction_id'))
require('commitment hash matches reveal', verifier.get('commitment_hash_matches_reveal_material') is True)
require('result derives', verifier.get('result_derives_from_reveal_material') is True)
require('sample result number agrees', sample.get('result_number') == verifier.get('result_number'))
require('sample result colour agrees', sample.get('result_colour') == verifier.get('result_colour'))
for field in ['mainnet_supported','real_betting','real_payouts','backend_custody','production_randomness_claimed','private_key_material_written_to_artifacts']:
    require(f'safety {field} false', safety.get(field) is False)
# payload JSON alone must not be sufficient
require('commit payload has state but no claim_level', 'claim_level' not in commit_payload)
require('reveal payload has state but no claim_level', 'claim_level' not in reveal_payload)
expected_raw = subprocess.check_output(['cargo','run','-q','-p','kaspa-fair-cli','--','env084-generate-verifiable-demo-round','--round-id',commit_payload['round_id'],'--demo-seed',reveal_payload['revealed_seed_material'],'--json'], text=True)
expected = json.loads(expected_raw)
expected_proof = expected['proof_artifact']
require('commitment hash matches Rust transcript', commit_payload.get('commitment_hash') == expected_proof['application_round_transcript']['commitment'].get('commitment_hash'))
require('result number matches Rust transcript', reveal_payload.get('result_number') == expected_proof['application_round_transcript']['reveal'].get('result_number'))
require('result colour matches Rust transcript', reveal_payload.get('result_colour') == expected_proof['application_round_transcript']['reveal'].get('result_colour'))
if errors:
    for error in errors:
        print(f'FAIL: {error}', file=sys.stderr)
    sys.exit(1)
PY

grep -q 'invalid_no_increment_transition_rejected=true' "$NEGATIVE" || { echo "negative transition check did not fail as expected" >&2; exit 1; }

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
  examples/roulette-poc/ui/app.js examples/roulette-poc/ui/index.html examples/roulette-poc/ui/roulette-table-renderer.js >/tmp/env090-ui-random-grep.txt || { cat /tmp/env090-ui-random-grep.txt >&2; exit 1; }

git status --short > "$ARTIFACT_DIR/env-090-git-status.txt"
printf 'KIP17_COVENANT_ENFORCED_TRANSITION_READY=PASS\n'
