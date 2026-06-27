#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

artifact_dir="spikes/kaspa-foundation/artifacts/env-076-roulette-poc-adapter-skeleton"
foundation_json_artifact="$artifact_dir/env-076-live-foundation-verifier.json"
roulette_json_artifact="$artifact_dir/env-076-roulette-round-output.json"
dry_run_output_artifact="$artifact_dir/env-076-dry-run-output.txt"

mkdir -p "$artifact_dir"
: > "$dry_run_output_artifact"

cargo run -p kaspa-fair-cli -- verify-live-tn10-canonical --json > "$foundation_json_artifact"

python3 -m json.tool "$foundation_json_artifact" > /dev/null
python3 - "$foundation_json_artifact" <<'PY' | tee -a "$dry_run_output_artifact"
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

echo "TOCCATA_LAYER_READY=PASS" | tee -a "$dry_run_output_artifact"

cargo run -p kaspa-fair-cli -- roulette-poc-dry-run --json > "$roulette_json_artifact"

python3 -m json.tool "$roulette_json_artifact" > /dev/null
python3 - "$roulette_json_artifact" <<'PY' | tee -a "$dry_run_output_artifact"
import json
import sys
from pathlib import Path

round_data = json.loads(Path(sys.argv[1]).read_text())
checks = [
    ("schema == kaspa-fair-roulette-poc-round-v1", round_data.get("schema") == "kaspa-fair-roulette-poc-round-v1"),
    ("foundation_verifier_result == PASS", round_data.get("foundation_verifier_result") == "PASS"),
    ("foundation_network == testnet-10", round_data.get("foundation_network") == "testnet-10"),
    ("mainnet_supported == false", round_data.get("mainnet_supported") is False),
    ("wallet_access_used == false", round_data.get("wallet_access_used") is False),
    ("signing_used == false", round_data.get("signing_used") is False),
    ("transaction_created == false", round_data.get("transaction_created") is False),
    ("broadcast_used == false", round_data.get("broadcast_used") is False),
    ("final_result == PASS", round_data.get("final_result") == "PASS"),
]

failed = False
for name, passed in checks:
    print(f"{'PASS' if passed else 'FAIL'}: {name}")
    failed = failed or not passed
if failed:
    sys.exit(1)
PY

echo "ROULETTE_POC_DRY_RUN=PASS" | tee -a "$dry_run_output_artifact"
