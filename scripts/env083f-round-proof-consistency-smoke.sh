#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ROUND_JSON="$ROOT_DIR/examples/roulette-poc/ui/sample-round.json"
PROOF_JSON="$ROOT_DIR/examples/roulette-poc/ui/toccata-fairness-proof.json"
APP_JS="$ROOT_DIR/examples/roulette-poc/ui/app.js"
INDEX_HTML="$ROOT_DIR/examples/roulette-poc/ui/index.html"
DOC_PATH="$ROOT_DIR/docs/env083f-verifiable-demo-round-generation-path.md"

[[ -f "$ROUND_JSON" ]] || { echo "missing sample-round.json" >&2; exit 1; }
[[ -f "$PROOF_JSON" ]] || { echo "missing toccata-fairness-proof.json" >&2; exit 1; }
[[ -f "$APP_JS" ]] || { echo "missing app.js" >&2; exit 1; }
[[ -f "$INDEX_HTML" ]] || { echo "missing index.html" >&2; exit 1; }
[[ -f "$DOC_PATH" ]] || { echo "missing ENV-083F demo-generation path doc" >&2; exit 1; }

python3 - "$ROUND_JSON" "$PROOF_JSON" <<'PY'
import json
import sys

round_path, proof_path = sys.argv[1:3]
with open(round_path, "r", encoding="utf-8") as fh:
    round_data = json.load(fh)
with open(proof_path, "r", encoding="utf-8") as fh:
    proof = json.load(fh)

errors = []

def require(label, condition):
    if not condition:
        errors.append(label)

proof_reveal = proof.get("application_round_transcript", {}).get("reveal", {})
proof_anchor = proof.get("live_tn10_anchor", {})
proof_safety = proof.get("safety_flags", {})

require("result_number mismatch", round_data.get("result_number") == proof.get("result_number") == proof_reveal.get("result_number"))
require("result_colour mismatch", round_data.get("result_colour") == proof.get("result_colour") == proof_reveal.get("result_colour"))
require("result_algorithm mismatch", round_data.get("result_algorithm") == proof.get("result_algorithm") == proof_reveal.get("result_algorithm"))
require("round foundation verifier PASS missing", round_data.get("foundation_verifier_result") == "PASS")
require("proof verifier_result PASS missing", proof.get("verifier_result") == "PASS")
require("Rust verifier PASS missing", proof.get("rust_verifier_output", {}).get("verifier_result") == "PASS")
require("evidence_mode live_readonly_tn10 missing", proof.get("evidence_mode") == "live_readonly_tn10")
require("anchor evidence_mode live_readonly_tn10 missing", proof_anchor.get("evidence_mode") == "live_readonly_tn10")
require("covenant_id_confirmed missing", proof_anchor.get("covenant_id_confirmed") is True)
require(
    "ENV-087 live round transaction evidence present",
    proof.get("future_live_round_transaction_evidence") == "replaced_by_env087_live_bare_tn10_anchor_evidence"
    and proof.get("live_round_commitment_evidence", {}).get("status") == "present"
    and proof.get("live_round_reveal_evidence", {}).get("status") == "present",
)

for flag in ["real_betting", "real_payouts", "backend_custody", "private_key_access_used", "mainnet_supported"]:
    require(f"proof safety flag {flag} not false", proof_safety.get(flag) is False)
for flag in ["wallet_access_used", "signing_used", "broadcast_used", "transaction_created"]:
    require(f"proof safety flag {flag} not true", proof_safety.get(flag) is True)
require("proof mock_display_only not true", proof_safety.get("mock_display_only") is True)

round_false_flags = ["mainnet_supported", "signing_used", "transaction_created", "broadcast_used", "wallet_access_used"]
for flag in round_false_flags:
    require(f"round safety flag {flag} not false", round_data.get(flag) is False)
require("round foundation_readonly not true", round_data.get("foundation_readonly") is True)

if errors:
    for error in errors:
        print(f"FAIL: {error}", file=sys.stderr)
    sys.exit(1)
PY

grep -q 'fetchJson("sample-round.json")' "$APP_JS" || { echo "UI does not load sample-round.json" >&2; exit 1; }
grep -q 'fetchJson("toccata-fairness-proof.json")' "$APP_JS" || { echo "UI does not load toccata-fairness-proof.json" >&2; exit 1; }

grep -RIn 'Math\.random' "$APP_JS" "$INDEX_HTML" "$ROOT_DIR/examples/roulette-poc/ui/roulette-table-renderer.js" >/tmp/env083f-random-grep.txt && { cat /tmp/env083f-random-grep.txt >&2; exit 1; } || true
grep -RInE 'crypto\.getRandomValues|crypto\.randomUUID|window\.crypto|globalThis\.crypto' "$APP_JS" "$INDEX_HTML" "$ROOT_DIR/examples/roulette-poc/ui/roulette-table-renderer.js" >/tmp/env083f-crypto-grep.txt && { cat /tmp/env083f-crypto-grep.txt >&2; exit 1; } || true

grep -q 'Start Wheel' "$INDEX_HTML" || { echo "Start Wheel control missing" >&2; exit 1; }
grep -q 'Reset Round' "$INDEX_HTML" || { echo "Reset Round control missing" >&2; exit 1; }
! grep -RIn 'Reveal Result' "$APP_JS" "$INDEX_HTML" >/tmp/env083f-reveal-grep.txt || { cat /tmp/env083f-reveal-grep.txt >&2; exit 1; }
! grep -RIn 'Wheel Visual' "$APP_JS" "$INDEX_HTML" >/tmp/env083f-wheel-visual-grep.txt || { cat /tmp/env083f-wheel-visual-grep.txt >&2; exit 1; }

printf 'ROUND_PROOF_CONSISTENCY_READY=PASS\n'
