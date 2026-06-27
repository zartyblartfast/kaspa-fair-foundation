#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

env077_artifact_dir="spikes/kaspa-foundation/artifacts/env-077-deterministic-roulette-engine"
env078_artifact_dir="spikes/kaspa-foundation/artifacts/env-078-roulette-ui-prototype"
source_engine_json="$env077_artifact_dir/env-077-roulette-engine-output.json"
source_foundation_json="$env077_artifact_dir/env-077-live-foundation-verifier.json"
ui_dir="examples/roulette-poc/ui"
sample_round_json="$ui_dir/sample-round.json"
artifact_sample_round="$env078_artifact_dir/env-078-sample-round.json"
smoke_output="$env078_artifact_dir/env-078-ui-smoke-output.txt"

mkdir -p "$env078_artifact_dir"
: > "$smoke_output"

for required in "$source_engine_json" "$source_foundation_json"; do
  if [[ ! -f "$required" ]]; then
    echo "FAIL: missing prerequisite artifact $required" | tee -a "$smoke_output"
    exit 1
  fi
done

echo "PASS: ENV-077 prerequisite artifacts exist" | tee -a "$smoke_output"
cp "$source_engine_json" "$sample_round_json"
cp "$source_engine_json" "$artifact_sample_round"
echo "PASS: sample-round.json refreshed from ENV-077 engine output" | tee -a "$smoke_output"

python3 -m json.tool "$sample_round_json" > /dev/null
python3 - "$sample_round_json" <<'PY' | tee -a "$smoke_output"
import json
import sys
from pathlib import Path

sample = json.loads(Path(sys.argv[1]).read_text())
checks = [
    ("final_result == PASS", sample.get("final_result") == "PASS"),
    ("round_state == ProofPublished", sample.get("round_state") == "ProofPublished"),
    ("foundation_verifier_result == PASS", sample.get("foundation_verifier_result") == "PASS"),
    ("foundation_network == testnet-10", sample.get("foundation_network") == "testnet-10"),
    ("mainnet_supported == false", sample.get("mainnet_supported") is False),
    ("wallet_access_used == false", sample.get("wallet_access_used") is False),
    ("signing_used == false", sample.get("signing_used") is False),
    ("transaction_created == false", sample.get("transaction_created") is False),
    ("broadcast_used == false", sample.get("broadcast_used") is False),
]
failed = False
for name, passed in checks:
    print(f"{'PASS' if passed else 'FAIL'}: {name}")
    failed = failed or not passed
number = sample.get("result_number")
colour = sample.get("result_colour")
number_ok = isinstance(number, int) and 0 <= number <= 36
colour_ok = colour in {"green", "red", "black"}
print(f"{'PASS' if number_ok else 'FAIL'}: result_number in 0..36")
print(f"{'PASS' if colour_ok else 'FAIL'}: result_colour in green/red/black")
if failed or not number_ok or not colour_ok:
    sys.exit(1)
PY

for required in "$ui_dir/index.html" "$ui_dir/styles.css" "$ui_dir/app.js" "$sample_round_json"; do
  if [[ ! -f "$required" ]]; then
    echo "FAIL: missing UI file $required" | tee -a "$smoke_output"
    exit 1
  fi
done

echo "PASS: required UI files exist" | tee -a "$smoke_output"

grep -Fq 'src="app.js"' "$ui_dir/index.html"
echo "PASS: index.html references app.js" | tee -a "$smoke_output"
grep -Fq 'href="styles.css"' "$ui_dir/index.html"
echo "PASS: index.html references styles.css" | tee -a "$smoke_output"

grep -Fq 'Kaspa Fair Roulette PoC' "$ui_dir/index.html"
echo "PASS: UI contains Kaspa Fair Roulette PoC" | tee -a "$smoke_output"
grep -Fq 'spin animation != result finalisation' "$ui_dir/index.html"
echo "PASS: UI contains spin animation != result finalisation" | tee -a "$smoke_output"
grep -Fq 'ProofPublished' "$ui_dir/app.js"
echo "PASS: UI contains ProofPublished" | tee -a "$smoke_output"

echo 'ROULETTE_UI_PROTOTYPE_READY=PASS' | tee -a "$smoke_output"
