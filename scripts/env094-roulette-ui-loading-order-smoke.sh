#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

APP="examples/roulette-poc/ui/app.js"
INDEX="examples/roulette-poc/ui/index.html"
STYLES="examples/roulette-poc/ui/styles.css"
RENDERER="examples/roulette-poc/ui/roulette-table-renderer.js"
SCHEMA="examples/roulette-poc/ui/roulette-table-schema.json"
SAMPLE="examples/roulette-poc/ui/sample-round.json"
PROOF="examples/roulette-poc/ui/toccata-fairness-proof.json"

require_file() {
  [[ -f "$1" ]] || { echo "missing required file: $1" >&2; exit 1; }
}

for file in "$APP" "$INDEX" "$STYLES" "$RENDERER" "$SCHEMA" "$SAMPLE" "$PROOF"; do
  require_file "$file"
done

node --check "$APP" >/dev/null
node --check "$RENDERER" >/dev/null

python3 - "$APP" "$INDEX" "$STYLES" "$RENDERER" "$SCHEMA" "$SAMPLE" "$PROOF" <<'PY'
import json
import pathlib
import re
import sys

app_path, index_path, styles_path, renderer_path, schema_path, sample_path, proof_path = [pathlib.Path(arg) for arg in sys.argv[1:]]
app = app_path.read_text(encoding='utf-8')
index = index_path.read_text(encoding='utf-8')
styles = styles_path.read_text(encoding='utf-8')
renderer = renderer_path.read_text(encoding='utf-8')
schema = json.loads(schema_path.read_text(encoding='utf-8'))
sample = json.loads(sample_path.read_text(encoding='utf-8'))
proof = json.loads(proof_path.read_text(encoding='utf-8'))
combined_ui = '\n'.join([app, index, styles, renderer])
combined_json_app = '\n'.join([app, sample_path.read_text(encoding='utf-8'), proof_path.read_text(encoding='utf-8')])
errors = []

def require(label, condition):
    if not condition:
        errors.append(label)

def pos(text, token):
    idx = text.find(token)
    require(f'missing token: {token}', idx >= 0)
    return idx

# Loading-order contract: table schema is fetched/validated/rendered before proof artifact validation can complete.
schema_fetch = pos(app, 'const tableSchema = await fetchJson("roulette-table-schema.json")')
validate_schema = pos(app, 'validateTableSchema(tableSchema)')
assign_schema = pos(app, 'appState.tableSchema = tableSchema')
initialise = pos(app, 'initialiseTableFirstLayout();')
async_load = pos(app, 'void loadRoundAndProofArtifacts();')
proof_fetch = pos(app, 'fetchJson("toccata-fairness-proof.json")')
proof_validate = pos(app, 'validateProofArtifact(proofArtifact, round)')
render_table = pos(app, 'renderRouletteTable(resultVisible ? round.result_number : null, true)')
require('schema fetch before proof fetch', schema_fetch < proof_fetch)
require('schema validated before table-first init', schema_fetch < validate_schema < assign_schema < initialise < async_load)
require('proof validation separated into async loader', async_load < proof_fetch < proof_validate)
require('table render path exists outside proof validator', render_table < proof_validate or 'function renderFlow()' in app and 'initialiseTableFirstLayout()' in app)
require('boot does not Promise.all schema with proof', 'fetchJson("roulette-table-schema.json"),\n    fetchJson("toccata-fairness-proof.json")' not in app)
require('table-first status implemented', 'Table ready — loading Toccata proof…' in app)

# Proof loading/failure contract.
require('proof loading state text', 'Loading Toccata proof…' in app or 'Loading Toccata proof…' in index)
require('proof load state model', 'proofLoadState' in app and '"loading"' in app and '"validated"' in app and '"failed"' in app)
require('proof failure renderer exists', 'function renderProofFailure(error)' in app)
require('proof failure shown in proof status', 'Toccata proof validation failed:' in app and 'ui.proofStatus.textContent = message' in app)
require('proof failure does not call showFailure', 'renderProofFailure(error);\n    renderFlow();' in app)
require('proof failure keeps table render path available', app.find('renderProofFailure(error);') < app.find('renderFlow();', app.find('renderProofFailure(error);')) and 'renderRouletteTable(resultVisible ? round.result_number : null, true)' in app)
require('global schema failure only', 'Failed to load roulette table schema safely' in app)

# Artifact compatibility and proof strictness.
require('sample source_env ENV-092 accepted', sample.get('source_env') == 'ENV-092')
require('proof source_env ENV-092 accepted', proof.get('source_env') == 'ENV-092')
require('app accepts ENV-092 contract', '"ENV-092": acceptsEnv092Tn10EntropyProof' in app)
require('live TN10 entropy claim accepted', 'full_kip17_covenant_enforced_transition_with_live_tn10_entropy' in proof.get('claim_level', '') and 'live_tn10_entropy' in app)
require('sample/proof result number agreement required', sample.get('result_number') == proof.get('result_number') and 'sample result number agrees' in app)
require('sample/proof result colour agreement required', sample.get('result_colour') == proof.get('result_colour') and 'sample result colour agrees' in app)
require('sample/proof result algorithm agreement required', sample.get('result_algorithm') == proof.get('result_algorithm') and 'sample result algorithm agrees' in app)
require('sample production_randomness_claimed false', sample.get('production_randomness_claimed') is False)
require('proof production_randomness_claimed false', proof.get('production_randomness_claimed') is False)
require('proof safety production_randomness_claimed false', proof.get('safety_flags', {}).get('production_randomness_claimed') is False)
require('production randomness false required in app', 'production_randomness_claimed false' in app and 'proofArtifact.production_randomness_claimed === false' in app and 'safety.production_randomness_claimed === false' in app)
require('unsafe proof states still rejected', 'throw new Error(`Unsafe or failed Toccata proof artifact:' in app)

# UI controls and forbidden affordances.
require('Start Wheel exists', 'id="start-wheel-button"' in index and '>Start Wheel<' in index)
require('Reset Round exists', 'id="reset-round-button"' in index and '>Reset Round<' in index)
require('Start Wheel gated until validated artifacts', 'ui.startWheelButton.disabled = uiState !== "BetsOpen" || !hasValidatedArtifacts;' in app)
require('result not shown before validation', 'const resultVisible = hasValidatedArtifacts && hasReachedState("ResultFinalised")' in app)
require('settlement not shown before validation', 'const settlementVisible = hasValidatedArtifacts && hasReachedState("Settled")' in app)
require('proof not claimed before validation', 'const proofVisible = hasValidatedArtifacts && hasReachedState("ProofPublished")' in app)
require('Reveal Result absent', 'Reveal Result' not in combined_ui and 'reveal-result' not in combined_ui.lower())
require('Wheel Visual absent', 'Wheel Visual' not in combined_ui and 'wheel-visual' not in combined_ui.lower())
require('NoMoreBets blocked-bet guard precedes ledger append', app.find('if (!canPlaceBetsForState(currentState))') >= 0 and app.find('if (!canPlaceBetsForState(currentState))') < app.find('appState.uiMockBets.push(mockBet)'))
require('blocked-bet message preserved', 'NO_MORE_BETS_MESSAGE = "No more bets accepted this round."' in app and 'ui.betStatus.textContent = NO_MORE_BETS_MESSAGE' in app)

# Safety/static source checks.
require('no Math.random', 'Math.random' not in combined_ui)
require('no browser crypto random APIs', not re.search(r'\bcrypto\.(getRandomValues|randomUUID)\b|\bwindow\.crypto\.(getRandomValues|randomUUID)\b', combined_ui))
require('no UI result generation function added', 'generateResult' not in combined_ui and 'deriveResultInUi' not in combined_ui and 'ui_generated_randomness' not in combined_ui)
require('mainnet rejected', sample.get('mainnet_supported') is False and proof.get('mainnet_supported') is False and proof.get('safety_flags', {}).get('mainnet_supported') is False and 'mainnet_supported false' in app)
require('real betting rejected', proof.get('real_betting') is False and proof.get('safety_flags', {}).get('real_betting') is False and 'real_betting false' in app)
require('real payouts rejected', proof.get('real_payouts') is False and proof.get('safety_flags', {}).get('real_payouts') is False and 'real_payouts false' in app)
require('wallet/private-key flags rejected', proof.get('safety_flags', {}).get('private_key_access_used') is False and proof.get('safety_flags', {}).get('wallet_access_used') is False and 'private_key_access_used false' in app)
require('schema remains mock only', schema.get('mock_only') is True and schema.get('mainnet_supported') is False)

secret_like_patterns = [
    re.compile(r'-----BEGIN [A-Z ]*PRIVATE KEY-----'),
    re.compile(r'\b(kprv|xprv)[A-Za-z0-9]+\b'),
    re.compile(r'(?i)\b(api[_-]?key|auth[_-]?token|access[_-]?token|secret[_-]?key)\s*[:=]\s*["\']?[A-Za-z0-9_-]{16,}'),
    re.compile(r'(?i)\b(seed[_-]?phrase|mnemonic)\s*[:=]\s*["\']?[a-z]+(?:\s+[a-z]+){11,23}["\']?'),
]
for pattern in secret_like_patterns:
    require(f'no secret-like UI material: {pattern.pattern}', pattern.search(combined_json_app) is None)

if errors:
    for error in errors:
        print(f'FAIL: {error}', file=sys.stderr)
    raise SystemExit(1)
PY

printf 'ROULETTE_UI_LOADING_ORDER_READY=PASS\n'
