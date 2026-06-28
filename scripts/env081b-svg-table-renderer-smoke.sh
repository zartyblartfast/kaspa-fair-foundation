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
combined_source = '\n'.join([index_html, styles_css, app_js, renderer_js])


def check(condition, message):
    if not condition:
        print(message, file=sys.stderr)
        raise SystemExit(1)

check('roulette-table-svg-host' in index_html, 'missing SVG roulette table container in index.html')
check('app.js' in index_html, 'index.html does not reference app.js')
check('styles.css' in index_html, 'index.html does not reference styles.css')
check('roulette-table-renderer.js' in index_html, 'index.html does not reference roulette-table-renderer.js')
check('fetchJson("roulette-table-schema.json")' in app_js, 'app.js does not load roulette-table-schema.json')
check('RouletteTableRenderer.render(ui.rouletteTableSvgHost, appState.tableSchema' in app_js, 'table is not rendered from loaded schema state')
check('flattenHotspotZones(schema).forEach((zone) => appendZone(svg, zone, options));' in renderer_js, 'compound hotspots are not rendered visibly at all times from schema')
check('visibleZoneLabel(zone)' in renderer_js, 'renderer visible label formatter missing')
check('return "2 to 1";' in renderer_js, 'column visible label 2 to 1 missing')
check('buildLedgerLabel(zone)' in app_js, 'ledger label builder missing')
check('appendHotspotMarker(group, zone);' in renderer_js, 'compound hotspot marker renderer missing')
check('hotspot-marker-shape' in renderer_js, 'compound hotspot marker shape missing')
check('hotspot-zone-passive' not in combined_source, 'passive hotspot mode styling/logic remains')
check('hotspot-zone-active' not in combined_source, 'active hotspot mode styling/logic remains')
check('selectedOverlayMode' not in combined_source and 'selectedMode' not in combined_source, 'bet type selector mode state remains')
check('data-bet-mode' not in combined_source and 'betModeButtons' not in combined_source, 'unwanted bet type selector buttons remain')
check('overlay-mode-caption' not in combined_source and 'Active selector family' not in combined_source, 'unwanted selector family caption remains')
check('fill: rgba(245, 197, 66, 0.72);' in styles_css and 'stroke: rgba(255, 247, 204, 0.92);' in styles_css, 'compact hotspot marker styling missing')
check('return { minX: -2, minY: -5, width: maxX + 4, height: maxY + 7 };' in renderer_js, 'SVG viewBox top padding does not protect top-row street selectors from clipping')
check('zone.bet_type === "street" && rect' in renderer_js and 'x: baseAnchor.x - 5' in renderer_js, 'street selector anchor is not moved above each 3-number column')
check('zone.bet_type === "six_line" && rect' in renderer_js and 'x: baseAnchor.x' in renderer_js, 'six-line selector anchor is not centered between adjacent streets')
check('r: geometry.r + 1.45' in renderer_js, 'compound selector circle hit area missing')
check('width: 3.4, height: 10.4' not in renderer_js and 'width: 5.1, height: 12' not in renderer_js, 'large street or six-line selector markers remain')
check('BET_PLACEMENT_DESCRIPTION' in app_js and 'number cells, zero, split/street/corner/six-line selectors' in app_js, 'single consistent table-click placement rule text missing')
check('<select' not in index_html.lower(), 'dropdown-only inside-zone workflow remains in index.html')
check('giant inside-zone list' not in combined_source.lower(), 'giant inside-zone list text remains in source')
check('bet-choice-select' not in combined_source, 'old dropdown bet-choice-select remains in source')
check('3 * row +' not in combined_source, 'hard-coded 12x3 layout formula remains in source')
check('[3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36]' not in app_js + renderer_js, 'hard-coded top row array remains in renderer source')
check('stroke-dasharray' not in styles_css and 'stroke-dasharray' not in renderer_js, 'dashed guideline overlay styling remains')

layout = schema['layout']['main_grid']
check(schema['layout']['zero_position'] == 'left', 'schema zero region is not on the left')
check(layout['top_row'] == [3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36], 'schema top row mismatch')
check(layout['middle_row'] == [2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32, 35], 'schema middle row mismatch')
check(layout['bottom_row'] == [1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34], 'schema bottom row mismatch')
check(len(schema['regions']['dozens']) == 3, 'schema dozens missing')
check(len(schema['regions']['outside_bets']) == 6, 'schema outside bets missing')
check(len(schema['regions']['columns']) == 3, 'schema column selectors missing')
check(any(zone.get('number') == 0 for zone in schema['regions']['number_cells']), 'schema zero region missing')

schema_types = {zone['bet_type'] for zone in schema['regions']['number_cells']}
schema_types.update(zone['bet_type'] for zone in schema['regions']['dozens'])
schema_types.update(zone['bet_type'] for zone in schema['regions']['columns'])
schema_types.update(zone['bet_type'] for zone in schema['regions']['outside_bets'])
for hotspot_group in ['split', 'street', 'corner', 'six_line']:
    zones = schema['regions']['hotspots'][hotspot_group]
    check(len(zones) > 0, f'{hotspot_group} hotspots missing from schema')
    schema_types.update(zone['bet_type'] for zone in zones)

check('straight' in schema_types, 'straight zones missing')
check('split' in schema_types, 'split zones missing')
check('street' in schema_types, 'street zones missing')
check('corner' in schema_types, 'corner zones missing')
check('six_line' in schema_types, 'six_line zones missing')
check('dozen' in schema_types, 'dozen zones missing')
check('column' in schema_types, 'column zones missing')

all_zones = {}
for zone in schema['regions']['number_cells']:
    all_zones[zone['id']] = zone
for zone in schema['regions']['dozens']:
    all_zones[zone['id']] = zone
for zone in schema['regions']['columns']:
    all_zones[zone['id']] = zone
for zone in schema['regions']['outside_bets']:
    all_zones[zone['id']] = zone
for zones in schema['regions']['hotspots'].values():
    for zone in zones:
        all_zones[zone['id']] = zone

outside_ids = {zone['id'] for zone in schema['regions']['outside_bets']}
for outside_id in ['red', 'black', 'odd', 'even', 'low', 'high']:
    check(outside_id in outside_ids, f'{outside_id} outside bet missing')

representative_bets = {
    'straight_1': ('straight', [1]),
    'straight_0': ('straight', [0]),
    'split_1_4': ('split', [1, 4]),
    'split_1_2': ('split', [1, 2]),
    'corner_1_2_4_5': ('corner', [1, 2, 4, 5]),
    'street_1_2_3': ('street', [1, 2, 3]),
    'six_line_1_2_3_4_5_6': ('six_line', [1, 2, 3, 4, 5, 6]),
    'red': ('outside', schema['colour_sets']['red']),
    'black': ('outside', schema['colour_sets']['black']),
    'odd': ('outside', list(range(1, 37, 2))),
    'even': ('outside', list(range(2, 37, 2))),
    'low': ('outside', list(range(1, 19))),
    'high': ('outside', list(range(19, 37))),
    'dozen_1': ('dozen', list(range(1, 13))),
    'dozen_2': ('dozen', list(range(13, 25))),
    'dozen_3': ('dozen', list(range(25, 37))),
    'column_1': ('column', [1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34]),
    'column_2': ('column', [2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32, 35]),
    'column_3': ('column', [3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36]),
}
for zone_id, (bet_type, covered_numbers) in representative_bets.items():
    zone = all_zones.get(zone_id)
    check(zone is not None, f'representative bet zone missing: {zone_id}')
    check(zone['bet_type'] == bet_type, f'{zone_id} bet_type mismatch')
    check(zone['covered_numbers'] == covered_numbers, f'{zone_id} covered numbers mismatch')
    check('rect' in zone and ('chip_anchor' in zone or 'anchor' in zone), f'{zone_id} missing clickable rect or chip anchor')

check('const isClickable = options.allowBetPlacement;' in renderer_js, 'all table zones are not uniformly clickable during open betting')
check('PLACEABLE_STATES = new Set(["BetsOpen", "SpinVisualStarted"])' in app_js, 'BetsOpen/SpinVisualStarted placement rule missing')
check('BETS_CLOSED_NO_MORE_BETS' in app_js, 'BETS_CLOSED_NO_MORE_BETS missing')
check('appState.uiMockBets = []' in app_js, 'Reset Round does not clear UI-added bets')
check('appState.nextMockBetId = 1' in app_js, 'Reset Round does not reset UI bet ids')
check('BET_PLACEMENT_DESCRIPTION' in app_js, 'single consistent bet placement description missing')
check('Math.random' not in combined_source, 'Math.random forbidden')
check('crypto.getRandomValues' not in combined_source, 'crypto.getRandomValues forbidden')
check('randomUUID' not in combined_source, 'randomUUID forbidden')
for forbidden in ['privateKey', 'signTransaction', 'broadcastTransaction', 'createTransaction(', 'wallet.connect', 'mainnet:true']:
    check(forbidden not in combined_source, f'forbidden live betting behavior token present: {forbidden}')

check('Compact inside-bet overlay mode' not in index_html, 'unwanted inside-bet overlay mode label remains')
check('if (options.onZoneClick && isClickable)' in renderer_js, 'zone click activation gating missing')
check('.roulette-zone:focus' in styles_css and '.roulette-zone:active' in styles_css and 'outline: none;' in styles_css, 'click/focus selection outline suppression missing')
check('.zone-clickable:focus' not in styles_css and '.hotspot-zone.zone-clickable:focus' not in styles_css, 'focus-based selector highlight styling remains')
check('group.blur();' in renderer_js, 'clicked bet zones must blur after chip placement')
check('chip-marker' in renderer_js, 'chip marker rendering missing')
check('.chip-marker-group' in styles_css and 'pointer-events: none;' in styles_css, 'placed chips must not intercept later table clicks')
check('stackIndex' in app_js and 'computeChipStackOffset' in renderer_js, 'same-anchor multi-bet chip stacking missing')
check('Highlighted overlay family' not in index_html + app_js, 'overlay control still describes a bet family as a selection highlight')
check('.mock-bet-list' in styles_css and 'height: 178px;' in styles_css and 'overflow-y: auto;' in styles_css and 'overscroll-behavior: contain;' in styles_css, 'fixed-height ledger scroll region missing; table can shift vertically when bets are added')
check('covered numbers:' in app_js, 'ledger covered numbers field missing')
check('payout multiplier:' in app_js, 'ledger payout multiplier field missing')
PY

printf '%s\n' 'ROULETTE_SVG_TABLE_RENDERER_READY=PASS'
