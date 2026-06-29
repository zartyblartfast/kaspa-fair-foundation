#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

APP_JS="examples/roulette-poc/ui/app.js"
RENDERER_JS="examples/roulette-poc/ui/roulette-table-renderer.js"
INDEX_HTML="examples/roulette-poc/ui/index.html"
PROOF="examples/roulette-poc/ui/toccata-fairness-proof.json"
SAMPLE="examples/roulette-poc/ui/sample-round.json"
ARTIFACT_DIR="spikes/kaspa-foundation/artifacts/env-090-ui-accepts-kip17-proof"
EVIDENCE="$ARTIFACT_DIR/env-090-ui-proof-validation-evidence.txt"

mkdir -p "$ARTIFACT_DIR"

for file in "$APP_JS" "$RENDERER_JS" "$INDEX_HTML" "$PROOF" "$SAMPLE"; do
  [[ -f "$file" ]] || { echo "missing required UI file: $file" >&2; exit 1; }
done

node --check "$APP_JS" >/dev/null
python3 -m json.tool "$PROOF" >/dev/null
python3 -m json.tool "$SAMPLE" >/dev/null

node <<'NODE' > "$EVIDENCE"
const fs = require('fs');
const vm = require('vm');
const appJs = fs.readFileSync('examples/roulette-poc/ui/app.js', 'utf8');
const start = appJs.indexOf('function validateProofArtifact');
const end = appJs.indexOf('function bindEvents');
if (start < 0 || end < 0 || end <= start) {
  throw new Error('validateProofArtifact block not found in app.js');
}
const sandbox = {};
vm.runInNewContext(`${appJs.slice(start, end)}\nglobalThis.__validateProofArtifact = validateProofArtifact;`, sandbox);
const validateProofArtifact = sandbox.__validateProofArtifact;
const proof = JSON.parse(fs.readFileSync('examples/roulette-poc/ui/toccata-fairness-proof.json', 'utf8'));
const sample = JSON.parse(fs.readFileSync('examples/roulette-poc/ui/sample-round.json', 'utf8'));
function assert(label, condition) {
  if (!condition) throw new Error(label);
  console.log(`PASS: ${label}`);
}
function assertRejects(label, mutate) {
  const unsafeProof = structuredClone(proof);
  mutate(unsafeProof);
  let rejected = false;
  try {
    validateProofArtifact(unsafeProof, sample);
  } catch (_error) {
    rejected = true;
  }
  assert(label, rejected);
}
validateProofArtifact(proof, sample);
assert('app.js accepts current authorised ENV-090 proof', true);
assert('source_env ENV-090', proof.source_env === 'ENV-090');
assert('claim_level full_kip17_covenant_enforced_transition', proof.claim_level === 'full_kip17_covenant_enforced_transition');
assert('verifier_result PASS', proof.verifier_result === 'PASS');
assert('evidence_mode live_readonly_tn10', proof.evidence_mode === 'live_readonly_tn10');
assert('network testnet-10', proof.network === 'testnet-10');
assert('commitment/reveal evidence present', proof.live_round_commitment_evidence?.status === 'present' && proof.live_round_reveal_evidence?.status === 'present');
assert('commitment txid linked to reveal', proof.live_round_reveal_evidence?.commitment_txid === proof.live_round_commitment_evidence?.transaction_id);
assert('KIP-17 enforcement represented true', proof.kip17_enforcement?.kip17_rule_enforced_on_transition === true && proof.live_round_reveal_evidence?.kip17_rule_enforced_on_transition === true);
assert('invalid transition rejection represented true', proof.kip17_enforcement?.invalid_no_increment_rejected === true);
assert('sample/proof result_number agree', sample.result_number === proof.result_number && proof.result_number === proof.application_round_transcript?.reveal?.result_number);
assert('sample/proof result_colour agree', sample.result_colour === proof.result_colour && proof.result_colour === proof.application_round_transcript?.reveal?.result_colour);
assert('sample/proof result_algorithm agree', sample.result_algorithm === proof.result_algorithm && proof.result_algorithm === proof.application_round_transcript?.reveal?.result_algorithm);
assert('safety flags reject real betting/payouts/backend/mainnet/production randomness', proof.safety_flags?.real_betting === false && proof.safety_flags?.real_payouts === false && proof.safety_flags?.backend_custody === false && proof.safety_flags?.mainnet_supported === false && proof.production_randomness_claimed === false);
assert('private key access not exposed', proof.safety_flags?.private_key_access_used === false);
assertRejects('unsafe mainnet proof rejected', p => { p.network = 'mainnet'; p.safety_flags.mainnet_supported = true; });
assertRejects('unsafe real betting proof rejected', p => { p.safety_flags.real_betting = true; });
assertRejects('unsafe real payouts proof rejected', p => { p.safety_flags.real_payouts = true; });
assertRejects('unsafe backend custody proof rejected', p => { p.safety_flags.backend_custody = true; });
assertRejects('unsafe production randomness proof rejected', p => { p.production_randomness_claimed = true; });
assertRejects('verifier_result not PASS rejected', p => { p.verifier_result = 'FAIL'; });
assertRejects('unknown source_env rejected', p => { p.source_env = 'ENV-999'; });
assertRejects('unsupported live claim_level rejected', p => { p.claim_level = 'bare TN10 anchor'; });
assertRejects('missing live commitment evidence rejected', p => { p.live_round_commitment_evidence.status = 'missing'; });
assertRejects('mismatched sample/proof result rejected', p => { p.result_number = (p.result_number + 1) % 37; });
assertRejects('secret-like UI material rejected', p => { p.ui_secret_like_material = 'PRIVATE' + '_KEY=abc123abc123abc123'; });
NODE

grep -q 'source_env' "$APP_JS" || { echo "app.js does not validate source_env" >&2; exit 1; }
grep -q 'full_kip17_covenant_enforced_transition' "$APP_JS" || { echo "app.js does not represent ENV-090 claim level" >&2; exit 1; }
grep -q 'kip17_rule_enforced_on_transition' "$APP_JS" || { echo "app.js does not validate KIP-17 enforcement" >&2; exit 1; }
grep -q 'invalid_no_increment_rejected' "$APP_JS" || { echo "app.js does not validate invalid transition rejection" >&2; exit 1; }
grep -q 'containsSecretLikeUiMaterial' "$APP_JS" || { echo "app.js does not scan for secret-like UI material" >&2; exit 1; }

grep -RInE 'Math\.random|crypto\.getRandomValues|crypto\.randomUUID|window\.crypto|globalThis\.crypto|self\.crypto' \
  "$APP_JS" "$RENDERER_JS" "$INDEX_HTML" >/tmp/env090-ui-random-grep.txt && { cat /tmp/env090-ui-random-grep.txt >&2; exit 1; } || true

python3 - "$PROOF" "$SAMPLE" <<'PY'
import pathlib, sys
needles = [
    '-----' + 'BEGIN ',
    'PRIVATE' + '_KEY=',
    'SECRET' + '_KEY=',
    'MNEM' + 'ONIC=',
    'SEED' + '_PHRASE=',
    'KASPA' + '_PRIVATE',
    'API' + '_KEY=',
    'ACCESS' + '_TOKEN=',
]
for raw_path in sys.argv[1:]:
    text = pathlib.Path(raw_path).read_text(encoding='utf-8', errors='ignore').upper()
    for needle in needles:
        if needle in text:
            raise SystemExit(f'secret-like UI JSON material found in {raw_path}: {needle}')
PY

git status --short > "$ARTIFACT_DIR/env-090-ui-git-status.txt"
printf 'UI_ACCEPTS_KIP17_PROOF_READY=PASS\n'
