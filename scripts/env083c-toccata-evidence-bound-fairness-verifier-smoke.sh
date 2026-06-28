#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ARTIFACT_DIR="$ROOT/spikes/kaspa-foundation/artifacts/env-083c-toccata-evidence-bound-fairness-verifier"
COMBINED="$ARTIFACT_DIR/env-083c-combined-output.json"
PROOF="$ARTIFACT_DIR/env-083c-proof-artifact.json"
VERIFIER="$ARTIFACT_DIR/env-083c-verifier-output.json"
ANCHOR="$ARTIFACT_DIR/env-083c-live-tn10-anchor-evidence.json"
NEGATIVE="$ARTIFACT_DIR/env-083c-negative-checks.txt"
COMMANDS="$ARTIFACT_DIR/env-083c-command-results.txt"
GIT_STATUS="$ARTIFACT_DIR/env-083c-git-status.txt"

mkdir -p "$ARTIFACT_DIR"
: > "$COMMANDS"

cd "$ROOT"

echo '$ cargo test -p kaspa-foundation env083c --test env083c_fairness_verifier' >> "$COMMANDS"
cargo test -p kaspa-foundation env083c --test env083c_fairness_verifier >> "$COMMANDS" 2>&1

echo '$ cargo run -q -p kaspa-fair-cli -- env083c-toccata-evidence-bound-fairness-proof --json' >> "$COMMANDS"
cargo run -q -p kaspa-fair-cli -- env083c-toccata-evidence-bound-fairness-proof --json > "$COMBINED" 2>> "$COMMANDS"
python3 -m json.tool "$COMBINED" >/dev/null
jq '.proof_artifact' "$COMBINED" > "$PROOF"
jq '.verifier_output' "$COMBINED" > "$VERIFIER"
jq '{evidence_mode:"live_readonly_tn10", verifier_result:.live_tn10_anchor_evidence.verifier_result, covenant_id_confirmed:.live_tn10_anchor_evidence.covenant_id_confirmed, transaction_created:.live_tn10_anchor_evidence.transaction_created, signing_used:.live_tn10_anchor_evidence.signing_used, broadcast_used:.live_tn10_anchor_evidence.broadcast_used, wallet_access_used:.live_tn10_anchor_evidence.wallet_access_used, mainnet_supported:.live_tn10_anchor_evidence.mainnet_supported, live_tn10_anchor_evidence:.live_tn10_anchor_evidence, proof_anchor:.proof_artifact.live_tn10_anchor}' "$COMBINED" > "$ANCHOR"

[[ -s "$PROOF" ]]
[[ -s "$VERIFIER" ]]
[[ -s "$ANCHOR" ]]

test "$(jq -r '.verifier_result' "$VERIFIER")" = "PASS"
test "$(jq -r '.live_tn10_anchor.evidence_mode' "$PROOF")" = "live_readonly_tn10"
test "$(jq -r '.live_tn10_anchor.verifier_result' "$PROOF")" = "PASS"
test "$(jq -r '.live_tn10_anchor.covenant_id_confirmed' "$PROOF")" = "true"
test "$(jq -r '.application_round_transcript.commitment.round_id' "$PROOF")" = "$(jq -r '.round_id' "$PROOF")"
test "$(jq -r '.application_round_transcript.reveal.round_id' "$PROOF")" = "$(jq -r '.round_id' "$PROOF")"
test "$(jq -r '.future_live_round_transaction_evidence' "$PROOF")" = "not_created_not_claimed_future_work"
test "$(jq -r '.safety_flags.transaction_created' "$PROOF")" = "false"
test "$(jq -r '.safety_flags.signing_used' "$PROOF")" = "false"
test "$(jq -r '.safety_flags.broadcast_used' "$PROOF")" = "false"
test "$(jq -r '.safety_flags.wallet_access_used' "$PROOF")" = "false"
test "$(jq -r '.safety_flags.mainnet_supported' "$PROOF")" = "false"

grep -q 'env083c_negative_cases_reject_tampering_and_claim_upgrades ... ok' "$COMMANDS"
cat > "$NEGATIVE" <<'NEGATIVE_CHECKS'
tampered reveal rejected: tested in Rust
mismatched covenant_id rejected: tested in Rust
mismatched result rejected: tested in Rust
omitted TN10 anchor rejected for Toccata-bound claim: tested in Rust
application-only evidence rejected for live round transaction claim: tested in Rust
NEGATIVE_CHECKS

if git diff --name-only -- examples/roulette-poc/ui/index.html examples/roulette-poc/ui/app.js examples/roulette-poc/ui/styles.css examples/roulette-poc/ui/sample-round.json examples/roulette-poc/ui/roulette-table-renderer.js | grep -q .; then
  echo "UI files modified" >&2
  exit 1
fi
if git diff --name-only -- examples/roulette-poc/ui/sample-round.json | grep -q .; then
  echo "sample-round.json modified" >&2
  exit 1
fi
if git diff -U0 -- . ':!spikes/kaspa-foundation/artifacts/env-083c-toccata-evidence-bound-fairness-verifier/**' | grep -E '^\+.*(Math\.random|thread_rng|OsRng|getrandom|submit_transaction|broadcast_transaction|sign_transaction|NetworkType::Mainnet)' >/dev/null; then
  echo "Forbidden random/wallet/signing/broadcast/mainnet code introduced" >&2
  exit 1
fi

git status --short --untracked-files=all > "$GIT_STATUS"

echo 'TOCCATA_EVIDENCE_BOUND_FAIRNESS_VERIFIER_READY=PASS'
