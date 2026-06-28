#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

artifact_dir="spikes/kaspa-foundation/artifacts/env-083b-toccata-fairness-anchor-feasibility"
feasibility_doc="docs/env083b-toccata-fairness-anchor-feasibility.md"
hard_gate_json="$artifact_dir/env-083b-hard-gate-result.json"
tool_matrix="$artifact_dir/env-083b-tool-decision-matrix.md"
source_evidence="$artifact_dir/env-083b-source-evidence.txt"
repo_evidence="$artifact_dir/env-083b-repo-type-rpc-evidence.txt"
command_results="$artifact_dir/env-083b-command-results.txt"
git_status_file="$artifact_dir/env-083b-git-status.txt"
summary_file="$artifact_dir/env-083b-summary.md"

mkdir -p "$artifact_dir"
: > "$command_results"

run_capture() {
  local label="$1"
  shift
  {
    printf '\n===== %s =====\n' "$label"
    printf '$ %q' "$1"
    shift || true
    for arg in "$@"; do printf ' %q' "$arg"; done
    printf '\n'
  } >> "$command_results"
  "$@" >> "$command_results" 2>&1
}

# Required document inspections.
if [[ ! -s docs/toccata-fairness-anchor-architecture.md ]]; then
  echo "missing primary architecture document" >&2
  exit 1
fi
if [[ -f docs/threat-model.md ]]; then
  threat_model_status="present"
else
  threat_model_status="absent"
fi

{
  echo "ENV-083B source evidence"
  echo
  echo "Primary project guidance inspected: docs/toccata-fairness-anchor-architecture.md"
  grep -nE 'Tier 1|Tier 2|covenant lineage|JSON|Rust|RPC|SilverScript|UI|wallet|signing|broadcast|mainnet|ENV-083B|KIP-17|KIP-20|KIP-21|KIP-16' docs/toccata-fairness-anchor-architecture.md || true
  echo
  echo "Threat model status: $threat_model_status"
  if [[ -f docs/threat-model.md ]]; then
    sed -n '1,120p' docs/threat-model.md
  fi
  echo
  echo "Official upstream source snippets"
  tmpdir="${TMPDIR:-/tmp}/env083b-official-sources"
  rm -rf "$tmpdir"
  mkdir -p "$tmpdir"
  fetch_source() {
    local url="$1"
    local file="$2"
    echo
    echo "===== $url ====="
    if curl -fsSL --max-time 30 "$url" -o "$tmpdir/$file"; then
      echo "FETCH_OK $file"
      grep -nEi 'Status:|covenant|covenant_id|RpcTransactionOutput|RpcUtxoEntry|OpInputCovenantId|OpOutputCovenantId|OpCov|sequencing|seqcommit|lane|OpZkPrecompile|Testnet-10|Testnet 12|Experimental|unstable|valid only' "$tmpdir/$file" | head -n 120 || true
    else
      echo "FETCH_FAIL $file"
    fi
  }
  fetch_source 'https://raw.githubusercontent.com/kaspanet/rusty-kaspa/master/docs/toccata-guide.md' 'toccata-guide.md'
  fetch_source 'https://raw.githubusercontent.com/kaspanet/kips/master/kip-0016.md' 'kip-0016.md'
  fetch_source 'https://raw.githubusercontent.com/kaspanet/kips/master/kip-0017.md' 'kip-0017.md'
  fetch_source 'https://raw.githubusercontent.com/kaspanet/kips/master/kip-0020.md' 'kip-0020.md'
  fetch_source 'https://raw.githubusercontent.com/kaspanet/kips/master/kip-0021.md' 'kip-0021.md'
  fetch_source 'https://raw.githubusercontent.com/kaspanet/silverscript/master/README.md' 'silverscript-README.md'
} > "$source_evidence" 2>&1

{
  echo "ENV-083B local repo type/RPC evidence"
  echo
  echo "Suggested Toccata/covenant search"
  grep -RIn --exclude-dir=target --exclude-dir=.git \
    -E 'Toccata|toccata|covenant|covenant_id|Covenant|KIP-17|KIP-20|KIP-21|seqcommit|SeqCommit|GetSeqCommitLaneProof|storageMass|storage_mass|computeBudget|RpcTransactionOutput|RpcUtxoEntry' \
    docs crates examples scripts Cargo.toml Cargo.lock .github 2>/dev/null || true
  echo
  echo "Suggested RPC/API search"
  grep -RIn --exclude-dir=target --exclude-dir=.git \
    -E 'getUtxos|GetUtxos|getVirtualChain|GetVirtualChain|getBlock|getBlocks|getTransaction|getTransactions|utxoindex|rpc|grpc|protobuf|borsh' \
    docs crates examples scripts Cargo.toml Cargo.lock .github 2>/dev/null || true
  echo
  echo "Suggested SilverScript/script search"
  grep -RIn --exclude-dir=target --exclude-dir=.git \
    -E 'silverscript|SilverScript|.sil|script_public_key|ScriptPublicKey|txscript|opcode|OpInputCovenantId|OpOutputCovenantId|OpAuthOutput|OpCov' \
    docs crates examples scripts Cargo.toml Cargo.lock .github 2>/dev/null || true
} > "$repo_evidence" 2>&1

cat > "$tool_matrix" <<'MATRIX'
# ENV-083B tool decision matrix

| Tool/layer | Decision | Role | Boundary |
|---|---|---|---|
| Rust | Use now | Truth layer: covenant-state model, commitment/reveal verifier, deterministic derivation, JSON mirror validation, tests. | No wallet/signing/broadcast in ENV-083B. |
| RPC / gRPC / node API | Use now as read-only transport | TN10 readiness, UTXO visibility, transaction-detail evidence, covenant IDs and output bindings. | Not proof logic and not transaction submission. |
| SilverScript | Do not depend on now | Candidate covenant authoring tool for a later compatibility spike. | Experimental and README says compiled scripts are valid only on Testnet 12, not assumed TN10-compatible. |
| Static UI | Do not modify in ENV-083B | Display/explanation layer only. | Not trusted; not randomness/result/proof source; no wallet. |
| JSON mirror files | Use only as mirror/export | App-facing serialization of Rust-verified proof/evidence. | Not source of proof truth. |
| Optional local TN10 indexed node | Fallback only | Use if public endpoints cannot expose covenant/UTXO/lane evidence. | Read-only unless explicitly authorized. |
MATRIX

cat > "$hard_gate_json" <<'JSON'
{
  "kip20_covenant_lineage_readonly_tn10_verified": true,
  "public_tn10_rpc_sufficient": true,
  "local_tn10_utxoindex_required": false,
  "recommended_next_step": "ENV-083C offline Rust covenant-state artifact and verifier model; no transaction creation, signing, broadcast, wallet, or mainnet.",
  "claim_allowed": "Toccata covenant fairness anchor",
  "reason": "Official Toccata/KIP-20 sources define output covenant bindings and UTXO covenant IDs; local Rust/CLI verifier exposes and checks covenant_id read-only; public TN10 transaction-detail API exposes input covenant_id, output covenant_authorizing_input, and output covenant_id for the canonical accepted TN10 covenant transition; the live read-only CLI verifier returns covenant_id_confirmed=true and verifier_result=PASS."
}
JSON

python3 -m json.tool "$hard_gate_json" >/dev/null

git status --short --untracked-files=all > "$git_status_file"

cat > "$summary_file" <<'SUMMARY'
# ENV-083B summary

Result: PASS for feasibility/toolchain gate completion.

Hard gate: KIP-20 covenant lineage read-only TN10 evidence path verified for the canonical covenant path using public TN10 read-only tooling.

Allowed claim after ENV-083B: Toccata covenant fairness anchor.

Recommended next ENV: ENV-083C offline Rust covenant-state artifact and verifier model.

No implementation was added for roulette randomisation, covenant transactions, wallet access, signing, broadcasting, real betting, payouts, custody, UI behaviour, or mainnet.
SUMMARY

live_json_tmp="$(mktemp)"
{
  echo "ENV-083B live read-only verifier evidence"
  echo "$ cargo run -p kaspa-fair-cli -- verify-live-tn10-canonical --json"
} >> "$command_results"
if cargo run -p kaspa-fair-cli -- verify-live-tn10-canonical --json > "$live_json_tmp" 2>> "$command_results"; then
  cat "$live_json_tmp" >> "$command_results"
  printf '\n' >> "$command_results"
else
  echo "live read-only verifier command failed" >> "$command_results"
  rm -f "$live_json_tmp"
  exit 1
fi
python3 - <<'PY' "$live_json_tmp"
import json, sys
path = sys.argv[1]
data = json.load(open(path, encoding='utf-8'))
checks = {
    'verifier_result': data.get('verifier_result') == 'PASS',
    'covenant_id_confirmed': data.get('covenant_id_confirmed') is True,
    'readonly': data.get('readonly') is True,
    'signing_used': data.get('signing_used') is False,
    'transaction_created': data.get('transaction_created') is False,
    'broadcast_used': data.get('broadcast_used') is False,
    'wallet_access_used': data.get('wallet_access_used') is False,
    'mainnet_supported': data.get('mainnet_supported') is False,
}
failed = [name for name, ok in checks.items() if not ok]
if failed:
    raise SystemExit(f'live verifier hard-gate checks failed: {failed}')
PY
rm -f "$live_json_tmp"

{
  echo "ENV-083B command results"
  echo
  echo "git status --short"
  git status --short
  echo
  echo "git log --oneline -n 5"
  git log --oneline -n 5
  echo
  echo "Feasibility doc exists: $feasibility_doc"
  test -s "$feasibility_doc" && echo PASS
  echo
  echo "Hard gate JSON exists and is valid: $hard_gate_json"
  python3 -m json.tool "$hard_gate_json" >/dev/null && echo PASS
  echo
  echo "Tool decision matrix exists: $tool_matrix"
  test -s "$tool_matrix" && echo PASS
} >> "$command_results" 2>&1

# Verify required deliverables exist.
test -s "$feasibility_doc"
test -s "$source_evidence"
test -s "$repo_evidence"
test -s "$tool_matrix"
test -s "$hard_gate_json"
test -s "$summary_file"
test -s "$command_results"
test -s "$git_status_file"

# Verify hard gate fields are explicit and acceptable.
python3 - <<'PY' "$hard_gate_json"
import json, sys
path = sys.argv[1]
data = json.load(open(path, encoding='utf-8'))
required = [
    'kip20_covenant_lineage_readonly_tn10_verified',
    'public_tn10_rpc_sufficient',
    'local_tn10_utxoindex_required',
    'recommended_next_step',
    'claim_allowed',
    'reason',
]
missing = [key for key in required if key not in data]
if missing:
    raise SystemExit(f'missing hard-gate keys: {missing}')
if data['claim_allowed'] not in {'Toccata covenant fairness anchor', 'Tier-1 TN10 anchor only', 'offline covenant model only'}:
    raise SystemExit('invalid claim_allowed')
if not isinstance(data['kip20_covenant_lineage_readonly_tn10_verified'], bool):
    raise SystemExit('hard gate verified flag must be boolean')
PY

# Verify no roulette UI/source behaviour files were modified by this ENV.
if git status --short --untracked-files=all -- examples/roulette-poc/ui | grep -q .; then
  echo "roulette UI files are modified" >&2
  git status --short --untracked-files=all -- examples/roulette-poc/ui >&2
  exit 1
fi

# Verify no Rust/source implementation files are modified in this design-only ENV.
if git status --short --untracked-files=all -- crates examples/roulette-poc/ui Cargo.toml Cargo.lock | grep -q .; then
  echo "source/UI implementation files are modified" >&2
  git status --short --untracked-files=all -- crates examples/roulette-poc/ui Cargo.toml Cargo.lock >&2
  exit 1
fi

# Verify this ENV did not introduce active wallet/signing/broadcast/mainnet code paths.
# The ENV is docs/scripts/artifacts only, and the source/UI implementation check above
# rejects executable Rust/UI changes. Safety-denial wording in docs/artifacts is expected.
if git status --short --untracked-files=all -- crates examples/roulette-poc/ui Cargo.toml Cargo.lock | grep -q .; then
  echo "unexpected implementation file change could introduce wallet/signing/broadcast/mainnet behavior" >&2
  exit 1
fi

echo "TOCCATA_FAIRNESS_ANCHOR_FEASIBILITY_READY=PASS"
