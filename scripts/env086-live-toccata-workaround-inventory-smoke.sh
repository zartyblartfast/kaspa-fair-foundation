#!/usr/bin/env bash
set -Eeuo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

INVENTORY="docs/env086-live-toccata-workaround-inventory.md"
TARGET="docs/env086-first-live-toccata-replacement-target.md"
ARTIFACT_DIR="spikes/kaspa-foundation/artifacts/env-086-live-toccata-workaround-inventory"

[[ -f "$INVENTORY" ]] || { echo "missing inventory doc" >&2; exit 1; }
[[ -f "$TARGET" ]] || { echo "missing first replacement target brief" >&2; exit 1; }
[[ -f "$ARTIFACT_DIR/env-086-summary.md" ]] || { echo "missing summary artifact" >&2; exit 1; }
[[ -f "$ARTIFACT_DIR/env-086-workaround-search-evidence.txt" ]] || { echo "missing search evidence artifact" >&2; exit 1; }
[[ -f "$ARTIFACT_DIR/env-086-ranked-inventory.json" ]] || { echo "missing ranked inventory JSON" >&2; exit 1; }
[[ -f "$ARTIFACT_DIR/env-086-first-replacement-target.md" ]] || { echo "missing target artifact" >&2; exit 1; }

combined="$(tr '[:upper:]' '[:lower:]' < "$INVENTORY")"
for category in \
  "round commitment" \
  "reveal" \
  "explicit demo seed" \
  "static app-facing json" \
  "live tn10 anchor limitation" \
  "covenant enforcement gap" \
  "ui display limitation"; do
  [[ "$combined" == *"$category"* ]] || { echo "missing category: $category" >&2; exit 1; }
done

grep -q 'Toccata/TN10 replacement' "$INVENTORY" || { echo "inventory missing replacement column" >&2; exit 1; }
grep -qi 'requires tx/sign/broadcast' "$INVENTORY" || { echo "inventory missing authorisation column" >&2; exit 1; }
grep -qi 'explicit user authorisation' "$INVENTORY" || { echo "inventory missing explicit authorisation discussion" >&2; exit 1; }

python3 - "$ARTIFACT_DIR/env-086-ranked-inventory.json" "$TARGET" <<'PY'
import json, re, sys
inventory_path, target_path = sys.argv[1:3]
data = json.load(open(inventory_path, encoding='utf-8'))
target = open(target_path, encoding='utf-8').read()
rec = data.get('recommended_next_env', '')
if rec.count('ENV-087') != 1:
    raise SystemExit('ranked inventory must recommend exactly one ENV-087')
if target.count('ENV-087') < 1:
    raise SystemExit('target brief missing ENV-087')
# Exactly one recommended implementation ENV heading/line.
lines = [line.strip() for line in target.splitlines() if line.strip() == 'ENV-087 — Authorised TN10 round-specific commitment/reveal transaction spike']
if lines != ['ENV-087 — Authorised TN10 round-specific commitment/reveal transaction spike']:
    raise SystemExit(f'target brief must recommend exactly one next ENV, got {lines!r}')
for banned in ['planning ENV', 'package ENV', 'packaging ENV', 'UI-polish ENV', 'ui-polish ENV']:
    if re.search(r'recommended next .*' + re.escape(banned), target, re.I):
        raise SystemExit('target recommends a banned bridge/package/UI-polish ENV')
if 'another bridge/planning/package/UI-polish step' not in target:
    raise SystemExit('target brief missing not-bridge/planning/package/UI-polish boundary')
if 'Explicit user authorisation required' not in target or 'Yes.' not in target:
    raise SystemExit('target brief missing explicit authorisation requirement')
PY

grep -RqiE 'transaction creation|signing|broadcast|tx/sign/broadcast|explicit user authorisation' "$INVENTORY" "$TARGET" || { echo "docs do not mention tx/sign/broadcast authorisation" >&2; exit 1; }

status="$(git status --short --untracked-files=all)"
if grep -E '^[ MARC?][MDARC?] (crates/|examples/roulette-poc/ui/)' <<<"$status" >/dev/null; then
  echo "source/UI/Rust files modified" >&2
  grep -E '^[ MARC?][MDARC?] (crates/|examples/roulette-poc/ui/)' <<<"$status" >&2
  exit 1
fi

printf 'LIVE_TOCCATA_WORKAROUND_INVENTORY_READY=PASS\n'
