#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

artifact_dir="spikes/kaspa-foundation/artifacts/env-077-deterministic-roulette-engine"
foundation_json_artifact="$artifact_dir/env-077-live-foundation-verifier.json"
engine_json_artifact="$artifact_dir/env-077-roulette-engine-output.json"
engine_check_output_artifact="$artifact_dir/env-077-engine-check-output.txt"

mkdir -p "$artifact_dir"
: > "$engine_check_output_artifact"

cargo run -p kaspa-fair-cli -- verify-live-tn10-canonical --json > "$foundation_json_artifact"
python3 -m json.tool "$foundation_json_artifact" > /dev/null
python3 - "$foundation_json_artifact" <<'PY' | tee -a "$engine_check_output_artifact"
import json
import sys
from pathlib import Path

foundation = json.loads(Path(sys.argv[1]).read_text())
checks = [
    ("schema == kaspa-fair-live-verification-result-v1", foundation.get("schema") == "kaspa-fair-live-verification-result-v1"),
    ("network == testnet-10", foundation.get("network") == "testnet-10"),
    ("mainnet_supported == false", foundation.get("mainnet_supported") is False),
    ("verifier_result == PASS", foundation.get("verifier_result") == "PASS"),
    ("accepted == true", foundation.get("accepted") is True),
    ("input_relationship_confirmed == true", foundation.get("input_relationship_confirmed") is True),
    ("continuing_output_confirmed == true", foundation.get("continuing_output_confirmed") is True),
    ("continuing_output_value_sompi == 99700000", foundation.get("continuing_output_value_sompi") == 99_700_000),
    ("continuing_output_value_confirmed == true", foundation.get("continuing_output_value_confirmed") is True),
    ("covenant_id_confirmed == true", foundation.get("covenant_id_confirmed") is True),
    ("readonly == true", foundation.get("readonly") is True),
    ("signing_used == false", foundation.get("signing_used") is False),
    ("transaction_created == false", foundation.get("transaction_created") is False),
    ("broadcast_used == false", foundation.get("broadcast_used") is False),
    ("wallet_access_used == false", foundation.get("wallet_access_used") is False),
]
failed = False
for name, passed in checks:
    print(f"{'PASS' if passed else 'FAIL'}: {name}")
    failed = failed or not passed
if failed:
    sys.exit(1)
PY

cargo run -p kaspa-fair-cli -- roulette-engine-dry-run --json > "$engine_json_artifact"
python3 -m json.tool "$engine_json_artifact" > /dev/null
python3 - "$engine_json_artifact" <<'PY' | tee -a "$engine_check_output_artifact"
import json
import sys
from pathlib import Path

engine = json.loads(Path(sys.argv[1]).read_text())
checks = [
    ("schema == kaspa-fair-roulette-engine-round-v1", engine.get("schema") == "kaspa-fair-roulette-engine-round-v1"),
    ("round_state == ProofPublished", engine.get("round_state") == "ProofPublished"),
    ("foundation_verifier_result == PASS", engine.get("foundation_verifier_result") == "PASS"),
    ("foundation_network == testnet-10", engine.get("foundation_network") == "testnet-10"),
    ("mainnet_supported == false", engine.get("mainnet_supported") is False),
    ("wallet_access_used == false", engine.get("wallet_access_used") is False),
    ("signing_used == false", engine.get("signing_used") is False),
    ("transaction_created == false", engine.get("transaction_created") is False),
    ("broadcast_used == false", engine.get("broadcast_used") is False),
    ("final_result == PASS", engine.get("final_result") == "PASS"),
]
failed = False
for name, passed in checks:
    print(f"{'PASS' if passed else 'FAIL'}: {name}")
    failed = failed or not passed
number = engine.get("result_number")
colour = engine.get("result_colour")
number_ok = isinstance(number, int) and 0 <= number <= 36
colour_ok = colour in {"green", "red", "black"}
print(f"{'PASS' if number_ok else 'FAIL'}: result_number in 0..36")
print(f"{'PASS' if colour_ok else 'FAIL'}: result_colour in green/red/black")
if failed or not number_ok or not colour_ok:
    sys.exit(1)
PY

echo "ROULETTE_ENGINE_READY=PASS" | tee -a "$engine_check_output_artifact"
