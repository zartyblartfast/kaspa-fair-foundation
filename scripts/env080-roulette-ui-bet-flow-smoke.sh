#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

env077_artifact_dir="spikes/kaspa-foundation/artifacts/env-077-deterministic-roulette-engine"
env080_artifact_dir="spikes/kaspa-foundation/artifacts/env-080-roulette-ui-bet-flow"
source_engine_json="$env077_artifact_dir/env-077-roulette-engine-output.json"
ui_dir="examples/roulette-poc/ui"
sample_round_json="$ui_dir/sample-round.json"
artifact_sample_round="$env080_artifact_dir/env-080-sample-round.json"
smoke_output="$env080_artifact_dir/env-080-ui-bet-flow-smoke-output.txt"

mkdir -p "$env080_artifact_dir"

if [[ ! -f "$source_engine_json" ]]; then
  printf 'FAIL: missing prerequisite artifact %s\n' "$source_engine_json" >&2
  exit 1
fi

cp "$source_engine_json" "$sample_round_json"
cp "$source_engine_json" "$artifact_sample_round"

python3 - "$repo_root" "$smoke_output" <<'PY'
import json
import sys
from pathlib import Path

repo_root = Path(sys.argv[1])
smoke_output = Path(sys.argv[2])
ui_dir = repo_root / "examples/roulette-poc/ui"
index_html = (ui_dir / "index.html").read_text()
styles_css = (ui_dir / "styles.css").read_text()
app_js = (ui_dir / "app.js").read_text()
sample = json.loads((ui_dir / "sample-round.json").read_text())
checks = []

def add_check(name, passed):
    checks.append((name, bool(passed)))

add_check("index.html references app.js", 'src="app.js"' in index_html)
add_check("index.html references styles.css", 'href="styles.css"' in index_html)
add_check("UI contains Place Mock Bet", "Place Mock Bet" in index_html)
add_check("UI contains Reset Round or New Mock Round", "Reset Round" in index_html or "New Mock Round" in index_html)
add_check("UI contains Bets are still open while the wheel is visually spinning", "Bets are still open while the wheel is visually spinning" in index_html or "Bets are still open while the wheel is visually spinning" in app_js)
add_check("app.js has state handling for BetsOpen", "BetsOpen" in app_js)
add_check("app.js has state handling for SpinVisualStarted", "SpinVisualStarted" in app_js)
add_check("app.js has state handling for NoMoreBets", "NoMoreBets" in app_js)
add_check("app.js allows bet placement in BetsOpen", 'const PLACEABLE_STATES = new Set(["BetsOpen", "SpinVisualStarted"])' in app_js)
add_check("app.js allows bet placement in SpinVisualStarted", 'const PLACEABLE_STATES = new Set(["BetsOpen", "SpinVisualStarted"])' in app_js)
add_check("app.js blocks bet placement after NoMoreBets", 'return `${BETS_CLOSED_NO_MORE_BETS} — No more bets — ledger locked.`;' in app_js and 'ui.placeMockBetButton.disabled = !canPlaceMockBets;' in app_js)
add_check("app.js contains blocked message BETS_CLOSED_NO_MORE_BETS", 'BETS_CLOSED_NO_MORE_BETS' in app_js)
add_check("app.js implements reset/new-round behaviour", 'function resetRoundFlow()' in app_js and 'ui.resetRoundButton.addEventListener("click", resetRoundFlow);' in app_js)
add_check("app.js does not call Math.random", "Math.random" not in app_js)
add_check("app.js does not use crypto random APIs", all(token not in app_js for token in ["crypto.getRandomValues", "crypto.randomBytes", "globalThis.crypto", "window.crypto", "self.crypto"]))
add_check("app.js does not generate a roulette result", all(token not in app_js for token in ["% 37", "mod 37", "candidate_", "random roulette", "generateRoulette"]))
add_check("sample-round.json is valid JSON", True)
add_check("sample-round.json has final_result == PASS", sample.get("final_result") == "PASS")
add_check("sample-round.json has result_number in 0..36", isinstance(sample.get("result_number"), int) and 0 <= sample.get("result_number") <= 36)
add_check("sample-round.json has result_colour in green/red/black", sample.get("result_colour") in {"green", "red", "black"})
add_check("sample-round.json has foundation_verifier_result == PASS", sample.get("foundation_verifier_result") == "PASS")
add_check("sample-round.json has mainnet_supported == false", sample.get("mainnet_supported") is False)
add_check("sample-round.json has signing_used == false", sample.get("signing_used") is False)
add_check("sample-round.json has transaction_created == false", sample.get("transaction_created") is False)
add_check("sample-round.json has broadcast_used == false", sample.get("broadcast_used") is False)
add_check("sample-round.json has wallet_access_used == false", sample.get("wallet_access_used") is False)

lines = [f"{'PASS' if passed else 'FAIL'}: {name}" for name, passed in checks]
all_passed = all(passed for _, passed in checks)
if all_passed:
    lines.append("ROULETTE_UI_BET_FLOW_READY=PASS")
smoke_output.write_text("\n".join(lines) + "\n")
if not all_passed:
    failed = [name for name, passed in checks if not passed]
    print("FAIL: " + "; ".join(failed))
    sys.exit(1)
print("ROULETTE_UI_BET_FLOW_READY=PASS")
PY
