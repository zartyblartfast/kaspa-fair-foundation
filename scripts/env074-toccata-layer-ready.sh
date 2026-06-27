#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

artifact_dir="spikes/kaspa-foundation/artifacts/env-074-toccata-layer-readiness"
json_artifact="$artifact_dir/env-074-live-verification-result.json"
summary_artifact="$artifact_dir/env-074-summary.md"
commands_artifact="$artifact_dir/env-074-commands.txt"
test_output_artifact="$artifact_dir/env-074-test-output.txt"
readiness_output_artifact="$artifact_dir/env-074-readiness-output.txt"
git_status_artifact="$artifact_dir/env-074-git-status.txt"

mkdir -p "$artifact_dir"
: > "$test_output_artifact"

cat > "$commands_artifact" <<'COMMANDS'
ENV-074 readiness gate commands

Precheck:
git status --short --untracked-files=all
git log --oneline -5

Readiness command:
scripts/env074-toccata-layer-ready.sh

Live JSON verifier command run by readiness gate:
cargo run -p kaspa-fair-cli -- verify-live-tn10-canonical --json

Required verification suite:
cargo fmt --check
cargo test -p kaspa-fair-cli
cargo test -p kaspa-foundation
cargo check -p kaspa-fair-cli
cargo check -p kaspa-foundation
git diff --check
scripts/env074-toccata-layer-ready.sh
COMMANDS

{
  echo "ENV-074 git status capture"
  echo
  echo '$ git status --short --untracked-files=all'
  git status --short --untracked-files=all
  echo
  echo '$ git log --oneline -5'
  git log --oneline -5
} > "$git_status_artifact"

{
  echo '$ cargo run -p kaspa-fair-cli -- verify-live-tn10-canonical --json'
  cargo run -p kaspa-fair-cli -- verify-live-tn10-canonical --json > "$json_artifact"
} >> "$test_output_artifact" 2>&1

{
  echo
  echo '$ python3 JSON readiness assertions'
  python3 - "$json_artifact" <<'PY'
import json
import sys
from pathlib import Path

json_path = Path(sys.argv[1])
data = json.loads(json_path.read_text())

checks = [
    ("schema == kaspa-fair-live-verification-result-v1", data.get("schema") == "kaspa-fair-live-verification-result-v1"),
    ("network == testnet-10", data.get("network") == "testnet-10"),
    ("mainnet_supported == false", data.get("mainnet_supported") is False),
    ("verifier_result == PASS", data.get("verifier_result") == "PASS"),
    ("accepted == true", data.get("accepted") is True),
    ("input_relationship_confirmed == true", data.get("input_relationship_confirmed") is True),
    ("continuing_output_confirmed == true", data.get("continuing_output_confirmed") is True),
    ("continuing_output_value_sompi == 99700000", data.get("continuing_output_value_sompi") == 99_700_000),
    ("continuing_output_value_confirmed == true", data.get("continuing_output_value_confirmed") is True),
    ("covenant_id_confirmed == true", data.get("covenant_id_confirmed") is True),
    ("readonly == true", data.get("readonly") is True),
    ("signing_used == false", data.get("signing_used") is False),
    ("transaction_created == false", data.get("transaction_created") is False),
    ("broadcast_used == false", data.get("broadcast_used") is False),
    ("wallet_access_used == false", data.get("wallet_access_used") is False),
]

failed = False
for name, passed in checks:
    status = "PASS" if passed else "FAIL"
    print(f"{status}: {name}")
    failed = failed or not passed

if failed:
    sys.exit(1)
PY
} | tee -a "$test_output_artifact" > "$readiness_output_artifact"

cat >> "$readiness_output_artifact" <<'READY'
TOCCATA_LAYER_READY=PASS
READY
cat >> "$test_output_artifact" <<'READY'
TOCCATA_LAYER_READY=PASS
READY

cat > "$summary_artifact" <<SUMMARY
# ENV-074 — Limited Toccata layer readiness gate

Result: PASS

## Readiness command

\`\`\`bash
scripts/env074-toccata-layer-ready.sh
\`\`\`

Final readiness line:

\`\`\`text
TOCCATA_LAYER_READY=PASS
\`\`\`

## Live JSON verifier

The readiness command ran:

\`\`\`bash
cargo run -p kaspa-fair-cli -- verify-live-tn10-canonical --json
\`\`\`

and wrote the live app-facing JSON contract to:

\`\`\`text
$json_artifact
\`\`\`

## Contract assertions

All required ENV-074 JSON assertions passed:

- schema == kaspa-fair-live-verification-result-v1
- network == testnet-10
- mainnet_supported == false
- verifier_result == PASS
- accepted == true
- input_relationship_confirmed == true
- continuing_output_confirmed == true
- continuing_output_value_sompi == 99700000
- continuing_output_value_confirmed == true
- covenant_id_confirmed == true
- readonly == true
- signing_used == false
- transaction_created == false
- broadcast_used == false
- wallet_access_used == false

## Safety boundary

ENV-074 performed read-only TN10 verification only.

- no signing
- no transaction creation
- no submitting or broadcasting
- no wallet or private-key access
- no mainnet
- no roulette implementation
- no secrets added
SUMMARY

cat "$readiness_output_artifact"
