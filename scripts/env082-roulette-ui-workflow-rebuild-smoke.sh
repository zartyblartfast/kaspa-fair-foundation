#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT_DIR"

required_files=(
  "examples/roulette-poc/ui/index.html"
  "examples/roulette-poc/ui/styles.css"
  "examples/roulette-poc/ui/app.js"
  "examples/roulette-poc/ui/roulette-table-schema.json"
  "examples/roulette-poc/ui/roulette-table-renderer.js"
  "examples/roulette-poc/ui/sample-round.json"
)

for file in "${required_files[@]}"; do
  if [[ ! -f "$file" ]]; then
    echo "missing required file: $file" >&2
    exit 1
  fi
done

node --check examples/roulette-poc/ui/app.js >/dev/null
node --check examples/roulette-poc/ui/roulette-table-renderer.js >/dev/null

python3 <<'PY'
import json
import pathlib
import re
import sys

root = pathlib.Path('.')
index_html = (root / 'examples/roulette-poc/ui/index.html').read_text()
styles_css = (root / 'examples/roulette-poc/ui/styles.css').read_text()
app_js = (root / 'examples/roulette-poc/ui/app.js').read_text()
renderer_js = (root / 'examples/roulette-poc/ui/roulette-table-renderer.js').read_text()
schema = json.loads((root / 'examples/roulette-poc/ui/roulette-table-schema.json').read_text())
round_json = json.loads((root / 'examples/roulette-poc/ui/sample-round.json').read_text())
combined_source = '\n'.join([index_html, styles_css, app_js, renderer_js])

def check(condition, message):
    if not condition:
        print(message, file=sys.stderr)
        raise SystemExit(1)

def pos(token):
    i = index_html.find(token)
    check(i >= 0, f'missing token in index.html: {token}')
    return i

table_pos = pos('id="table-panel"')
controls_pos = pos('id="controls-panel"')
bet_editor_pos = pos('id="bet-editor-panel"')
ledger_pos = pos('id="mock-bet-list"')
settlement_pos = pos('id="settlement-panel"')
proof_pos = pos('id="proof-panel"')
trust_pos = pos('id="trust-panel"')
safety_pos = pos('id="safety-panel"')
check(table_pos < controls_pos < bet_editor_pos, 'roulette table is not before control/explanation sections')
for later_name, later_pos in [('ledger', ledger_pos), ('settlement', settlement_pos), ('proof', proof_pos), ('verifier', trust_pos), ('safety', safety_pos)]:
    check(table_pos < later_pos, f'roulette table is not before {later_name} section')

check('id="start-wheel-button"' in index_html and '>Start Wheel<' in index_html, 'Start Wheel button missing')
check('id="reset-round-button"' in index_html and '>Reset Round<' in index_html, 'Reset Round button missing')
check('reveal-result-button' not in combined_source and 'Reveal Result' not in combined_source, 'Reveal Result control/text remains')
check('wheel-visual' not in combined_source and 'Wheel visual' not in combined_source, 'Wheel Visual section remains')
check('Interactive round controls' not in index_html, 'old Interactive Round Controls section remains')
for forbidden in ['no-more-bets-button', 'show-settlement-button', 'publish-proof-button', 'No More Bets</button>', 'Show Settlement</button>', 'Publish Proof</button>']:
    check(forbidden not in combined_source, f'old status-like workflow button remains: {forbidden}')

check('id="round-status-label"' in index_html, 'status label element missing')
for status_text in ['Bets open', 'Wheel spinning — bets still open', 'No more bets', 'Result revealed', 'Settlement shown', 'Proof published']:
    check(status_text in app_js or status_text in index_html, f'status label text missing: {status_text}')
check('ROUND_STATUS_LABELS' in app_js and 'roundStatusLabel.textContent = ROUND_STATUS_LABELS[uiState]' in app_js, 'status label is not driven from UI state')

check('ui.startWheelButton.addEventListener("click", startWheelFlow)' in app_js, 'Start Wheel is not wired to flow start')
check('advanceState("SpinVisualStarted")' in app_js, 'Start Wheel does not move to spinning state')
check('PLACEABLE_STATES = new Set(["BetsOpen", "SpinVisualStarted"])' in app_js, 'betting is not enabled for BetsOpen and SpinVisualStarted')
check('renderRouletteTable(resultVisible ? round.result_number : null, true)' in app_js, 'table clicks are not available to show blocked-bet message after NoMoreBets')
check('if (!canPlaceBetsForState(currentState))' in app_js and 'ui.betStatus.textContent = NO_MORE_BETS_MESSAGE' in app_js and 'appState.uiMockBets.push(mockBet)' in app_js, 'NoMoreBets state does not block table bets before ledger append')
check(app_js.find('if (!canPlaceBetsForState(currentState))') < app_js.find('appState.uiMockBets.push(mockBet)'), 'blocked bet guard does not precede ledger append')
check('NO_MORE_BETS_MESSAGE = "No more bets accepted this round."' in app_js, 'blocked-bet message constant missing or wrong')
check('ui.betStatus.textContent = NO_MORE_BETS_MESSAGE' in app_js, 'blocked-bet message is not shown on blocked table bet')

check('scheduleFlowState("NoMoreBets", SPIN_TO_NO_MORE_BETS_MS)' in app_js, 'NoMoreBets is not scheduled automatically')
check('scheduleFlowState("ResultFinalised", SPIN_TO_NO_MORE_BETS_MS + NO_MORE_BETS_TO_RESULT_MS)' in app_js, 'result is not scheduled to reveal automatically after NoMoreBets')
check('scheduleFlowState("Settled"' in app_js and 'scheduleFlowState("ProofPublished"' in app_js, 'settlement/proof automatic flow missing')
check('window.setTimeout(() => advanceState(nextState), delayMs)' in app_js, 'deterministic timed flow scheduling missing')
check('clearFlowTimers();' in app_js and 'ui.resetRoundButton.addEventListener("click", resetRoundFlow)' in app_js, 'reset does not clear timers and return flow without page refresh')
check('appState.uiState = "BetsOpen"' in app_js and 'appState.uiMockBets = []' in app_js and 'appState.nextMockBetId = 1' in app_js, 'reset does not restore initial state and clear mock bets')

check('fetchJson("sample-round.json")' in app_js, 'sample-round.json is not loaded')
check('fetchJson("roulette-table-schema.json")' in app_js, 'roulette-table-schema.json is not loaded')
check('round.result_number' in app_js and 'round.result_colour' in app_js, 'result is not displayed from loaded round JSON')
check(round_json['round_state'] == 'ProofPublished' and round_json['final_result'] == 'PASS', 'sample-round.json is not the deterministic PASS proof source')
check('Math.random' not in combined_source, 'Math.random forbidden')
check('crypto.getRandomValues' not in combined_source and 'randomUUID' not in combined_source, 'crypto random APIs forbidden')
for forbidden in ['privateKey', 'signTransaction', 'broadcastTransaction', 'createTransaction(', 'wallet.connect', 'mainnet:true']:
    check(forbidden not in combined_source, f'forbidden unsafe token present: {forbidden}')

check('<select' not in index_html.lower(), 'dropdown-only workflow remains')
check('giant inside-zone list' not in combined_source.lower(), 'giant inside-zone list text remains')
check('data-bet-mode' not in combined_source and 'betModeButtons' not in combined_source, 'bet type buttons above table remain')
check('stroke-dasharray' not in styles_css and 'stroke-dasharray' not in renderer_js, 'large dashed guideline overlay remains')
check('fill: rgba(245, 197, 66, 0.72);' in styles_css, 'small subtle hotspot marker styling missing')

schema_types = {zone['bet_type'] for zone in schema['regions']['number_cells']}
schema_types.update(zone['bet_type'] for zone in schema['regions']['dozens'])
schema_types.update(zone['bet_type'] for zone in schema['regions']['columns'])
schema_types.update(zone['bet_type'] for zone in schema['regions']['outside_bets'])
for hotspot_group in ['split', 'street', 'corner', 'six_line']:
    zones = schema['regions']['hotspots'][hotspot_group]
    check(len(zones) > 0, f'{hotspot_group} hotspots missing from schema')
    schema_types.update(zone['bet_type'] for zone in zones)
for required_type in ['straight', 'split', 'street', 'corner', 'six_line', 'outside', 'dozen', 'column']:
    check(required_type in schema_types, f'{required_type} bet zones missing')
check('return "2 to 1";' in renderer_js, 'column visible label 2 to 1 missing')
check('RouletteTableRenderer.render(ui.rouletteTableSvgHost, appState.tableSchema' in app_js, 'table is not rendered from loaded schema state')
check('flattenHotspotZones(schema).forEach((zone) => appendZone(svg, zone, options));' in renderer_js, 'hotspots are not rendered from schema')

allows_live_env092 = round_json.get('source_env') == 'ENV-092'
check(round_json['mainnet_supported'] is False, 'sample round mainnet_supported must be false')
check(round_json['signing_used'] is False, 'sample round signing_used must be false')
check(round_json['transaction_created'] is (True if allows_live_env092 else False), 'sample round transaction_created safety flag mismatch')
check(round_json['broadcast_used'] is (True if allows_live_env092 else False), 'sample round broadcast_used safety flag mismatch')
check(round_json['wallet_access_used'] is False, 'sample round wallet_access_used must be false')
check(schema['mock_only'] is True and schema['mainnet_supported'] is False, 'table schema safety flags not preserved')
PY

printf '%s\n' 'ROULETTE_UI_WORKFLOW_REBUILD_READY=PASS'
