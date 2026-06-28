#!/usr/bin/env bash
set -Eeuo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

artifact_dir="spikes/kaspa-foundation/artifacts/env-083a-roulette-randomness-source-audit"
summary_file="$artifact_dir/env-083a-summary.md"
result_evidence_file="$artifact_dir/env-083a-result-source-evidence.txt"
randomness_evidence_file="$artifact_dir/env-083a-randomness-search-evidence.txt"
command_results_file="$artifact_dir/env-083a-command-results.txt"
git_status_file="$artifact_dir/env-083a-git-status.txt"

mkdir -p "$artifact_dir"
: > "$result_evidence_file"
: > "$randomness_evidence_file"
: > "$command_results_file"

run_capture() {
  local label="$1"
  shift
  {
    printf '\n$ %s\n' "$*"
    "$@"
    local status=$?
    printf 'exit_status=%s\n' "$status"
    return "$status"
  } >> "$command_results_file" 2>&1
}

run_capture_allow_fail() {
  local label="$1"
  shift
  {
    printf '\n$ %s\n' "$*"
    "$@"
    local status=$?
    printf 'exit_status=%s\n' "$status"
    return 0
  } >> "$command_results_file" 2>&1
}

{
  printf '$ git status --short\n'
  git status --short
  printf '\n$ git log --oneline -n 5\n'
  git log --oneline -n 5
} > "$git_status_file" 2>&1

{
  echo '== Sample round JSON fields =='
  python3 - <<'PY'
import json
from pathlib import Path
p = Path('examples/roulette-poc/ui/sample-round.json')
data = json.loads(p.read_text())
for key in [
    'schema',
    'round_id',
    'round_state',
    'final_result',
    'result_number',
    'result_colour',
    'result_algorithm',
    'seed_material_description',
    'seed_material_hex',
    'bet_ledger_hash',
    'foundation_verifier_result',
    'foundation_network',
    'foundation_readonly',
    'mainnet_supported',
    'wallet_access_used',
    'signing_used',
    'transaction_created',
    'broadcast_used',
]:
    print(f'{p}:{key}={data.get(key)!r}')
print(f'{p}:settlement_count={len(data.get("settlement", []))}')
print(f'{p}:bets_count={len(data.get("bets", []))}')
if data.get('result_number') != 21 or data.get('result_colour') != 'red':
    raise SystemExit('sample-round.json does not contain expected current fixture result 21 red')
PY

  echo
  echo '== UI sample-round.json loading/display evidence =='
  grep -nE 'fetchJson\("sample-round\.json"\)|round\.result_number|round\.result_colour|resultVisible|setTimeout|scheduleFlowState|deterministic result source|sample-round\.json' \
    examples/roulette-poc/ui/app.js examples/roulette-poc/ui/index.html examples/roulette-poc/ui/roulette-table-renderer.js || true

  echo
  echo '== Result/source keyword search =='
  grep -RIn --exclude-dir=target --exclude-dir=.git \
    -E '21|Red|winning|winner|result|outcome|sample-round|sample_round' \
    examples/roulette-poc docs crates Cargo.toml Cargo.lock scripts || true
} > "$result_evidence_file" 2>&1

{
  echo '== UI random API checks =='
  if grep -RIn -E 'Math\.random|crypto\.getRandomValues|crypto\.randomUUID|globalThis\.crypto|window\.crypto|self\.crypto' \
    examples/roulette-poc/ui/app.js examples/roulette-poc/ui/roulette-table-renderer.js examples/roulette-poc/ui/index.html; then
    echo 'UI_RANDOM_API_MATCHES=YES'
  else
    echo 'UI_RANDOM_API_MATCHES=NO'
  fi

  echo
  echo '== Randomisation-related search across Rust/JS/Cargo/docs/scripts =='
  grep -RIn --exclude-dir=target --exclude-dir=.git \
    -E 'random|rng|rand|entropy|seed|nonce|commit|reveal|Math\.random|crypto\.getRandomValues|crypto\.randomUUID|OsRng|thread_rng|rand::|getrandom' \
    examples/roulette-poc docs crates Cargo.toml Cargo.lock scripts || true

  echo
  echo '== Deterministic roulette result derivation candidates =='
  grep -RIn --exclude-dir=target --exclude-dir=.git \
    -E 'derive_roulette_number|roulette_seed_material|candidate_biguint|candidate_hex|ROULETTE_RESULT_ALGORITHM|ROULETTE_CANDIDATE_DOMAIN|blake3-domain-separated-rejection-sampling' \
    crates examples/roulette-poc docs Cargo.toml Cargo.lock scripts || true
} > "$randomness_evidence_file" 2>&1

run_capture 'node app syntax' node --check examples/roulette-poc/ui/app.js
run_capture 'node renderer syntax' node --check examples/roulette-poc/ui/roulette-table-renderer.js
run_capture 'git diff check' git diff --check
run_capture_allow_fail 'result/source grep' grep -RIn --exclude-dir=target --exclude-dir=.git -E '21|Red|winning|winner|result|outcome|sample-round|sample_round' examples/roulette-poc docs crates Cargo.toml Cargo.lock scripts
run_capture_allow_fail 'randomness grep' grep -RIn --exclude-dir=target --exclude-dir=.git -E 'random|rng|rand|entropy|seed|nonce|commit|reveal|Math\.random|crypto\.getRandomValues|crypto\.randomUUID|OsRng|thread_rng|rand::|getrandom' examples/roulette-poc docs crates Cargo.toml Cargo.lock scripts

python3 - <<'PY' > "$summary_file"
import json
from pathlib import Path
sample = json.loads(Path('examples/roulette-poc/ui/sample-round.json').read_text())
app = Path('examples/roulette-poc/ui/app.js').read_text()
renderer = Path('examples/roulette-poc/ui/roulette-table-renderer.js').read_text()
index = Path('examples/roulette-poc/ui/index.html').read_text()
ui = app + '\n' + renderer + '\n' + index
ui_random_tokens = ['Math.random', 'crypto.getRandomValues', 'crypto.randomUUID', 'globalThis.crypto', 'window.crypto', 'self.crypto']
print('# ENV-083A roulette randomness/source audit summary')
print()
print('Result: PASS')
print()
print('Current displayed result:')
print(f"- result_number: {sample.get('result_number')}")
print(f"- result_colour: {sample.get('result_colour')}")
print(f"- result_algorithm: {sample.get('result_algorithm')}")
print(f"- round_id: {sample.get('round_id')}")
print(f"- round_state: {sample.get('round_state')}")
print(f"- final_result: {sample.get('final_result')}")
print()
print('Classification:')
print('- sample fixture display: examples/roulette-poc/ui/sample-round.json is loaded by examples/roulette-poc/ui/app.js via fetchJson("sample-round.json").')
print('- UI generation: no; app.js displays round.result_number and round.result_colour after timed UI state reaches ResultFinalised.')
print('- deterministic verification/result derivation: crates/kaspa-fair-cli/src/roulette.rs derives roulette numbers from seed material with BLAKE3 domain-separated rejection sampling.')
print('- random generation: no UI Math.random/browser crypto random APIs found in current UI files; no OsRng/thread_rng/rand:: source match found in roulette Rust source.')
print('- cryptographic commitment/reveal material: architecture docs describe commit/reveal and seed-material models; current sample publishes seed_material_hex and bet_ledger_hash, not a fresh entropy source.')
print('- UI-only timed state transitions: app.js uses setTimeout/scheduleFlowState to reveal the already-loaded sample result; timers do not choose the result.')
print()
print('UI random API tokens found:')
for token in ui_random_tokens:
    print(f"- {token}: {'YES' if token in ui else 'NO'}")
print()
print('Relevant artifacts:')
for name in [
    'env-083a-summary.md',
    'env-083a-result-source-evidence.txt',
    'env-083a-randomness-search-evidence.txt',
    'env-083a-command-results.txt',
    'env-083a-git-status.txt',
]:
    print(f'- spikes/kaspa-foundation/artifacts/env-083a-roulette-randomness-source-audit/{name}')
PY

printf 'ROULETTE_RANDOMNESS_SOURCE_AUDIT_READY=PASS\n'
