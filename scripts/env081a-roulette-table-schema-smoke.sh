#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

schema_file="examples/roulette-poc/ui/roulette-table-schema.json"

if [[ ! -f "$schema_file" ]]; then
  printf 'FAIL: missing schema file %s\n' "$schema_file" >&2
  exit 1
fi

node - "$schema_file" <<'NODE'
const fs = require("fs");

const schemaPath = process.argv[2];
const RED_NUMBERS = [1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36];
const BLACK_NUMBERS = [2, 4, 6, 8, 10, 11, 13, 15, 17, 20, 22, 24, 26, 28, 29, 31, 33, 35];
const EXPECTED_DOZENS = {
  dozen_1: Array.from({ length: 12 }, (_, i) => i + 1),
  dozen_2: Array.from({ length: 12 }, (_, i) => i + 13),
  dozen_3: Array.from({ length: 12 }, (_, i) => i + 25),
};
const EXPECTED_COLUMNS = {
  column_1: [1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34],
  column_2: [2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32, 35],
  column_3: [3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36],
};
const EXPECTED_OUTSIDE = {
  low: Array.from({ length: 18 }, (_, i) => i + 1),
  even: Array.from({ length: 18 }, (_, i) => (i + 1) * 2),
  red: RED_NUMBERS,
  black: BLACK_NUMBERS,
  odd: Array.from({ length: 18 }, (_, i) => i * 2 + 1),
  high: Array.from({ length: 18 }, (_, i) => i + 19),
};
const EXPECTED_COUNTS = {
  straight: 37,
  split: 57,
  street: 12,
  corner: 22,
  six_line: 11,
  dozen: 3,
  column: 3,
  outside: 6,
};
const EXPECTED_PAYOUTS = {
  straight: 35,
  split: 17,
  street: 11,
  corner: 8,
  six_line: 5,
  dozen: 2,
  column: 2,
  outside: 1,
};

function fail(message) {
  console.error(`FAIL: ${message}`);
  process.exit(1);
}

function sorted(numbers) {
  return [...numbers].sort((a, b) => a - b);
}

function sameNumbers(actual, expected) {
  return JSON.stringify(sorted(actual)) === JSON.stringify(sorted(expected));
}

function assert(condition, message) {
  if (!condition) {
    fail(message);
  }
}

let schema;
try {
  schema = JSON.parse(fs.readFileSync(schemaPath, "utf8"));
} catch (error) {
  fail(`unparseable JSON in ${schemaPath}: ${error.message}`);
}

assert(schema.schema === "kaspa-fair-roulette-table-layout-v1", "schema id mismatch");
assert(schema.roulette_variant === "european", "roulette_variant mismatch");
assert(schema.mainnet_supported === false, "mainnet_supported must be false");
assert(schema.mock_only === true, "mock_only must be true");
assert(schema.coordinate_system && schema.coordinate_system.origin === "top-left", "coordinate system origin must be top-left");
assert(schema.coordinate_system && schema.coordinate_system.units === "arbitrary table units", "coordinate system units mismatch");

const numberCells = schema?.regions?.number_cells;
const dozens = schema?.regions?.dozens;
const outsideBets = schema?.regions?.outside_bets;
const columns = schema?.regions?.columns;
const split = schema?.regions?.hotspots?.split;
const street = schema?.regions?.hotspots?.street;
const corner = schema?.regions?.hotspots?.corner;
const sixLine = schema?.regions?.hotspots?.six_line;

assert(Array.isArray(numberCells), "regions.number_cells must be an array");
assert(Array.isArray(dozens), "regions.dozens must be an array");
assert(Array.isArray(outsideBets), "regions.outside_bets must be an array");
assert(Array.isArray(columns), "regions.columns must be an array");
assert(Array.isArray(split), "regions.hotspots.split must be an array");
assert(Array.isArray(street), "regions.hotspots.street must be an array");
assert(Array.isArray(corner), "regions.hotspots.corner must be an array");
assert(Array.isArray(sixLine), "regions.hotspots.six_line must be an array");

const allRegions = [...numberCells, ...dozens, ...outsideBets, ...columns, ...split, ...street, ...corner, ...sixLine];

function hasRect(region) {
  return region && region.rect && ["x", "y", "width", "height"].every((key) => Number.isFinite(region.rect[key]));
}

allRegions.forEach((region) => {
  assert(typeof region.id === "string" && region.id.length > 0, `region missing id: ${JSON.stringify(region)}`);
  assert(hasRect(region), `region ${region.id} missing rect coordinates`);
  const covered = region.covered_numbers;
  assert(Array.isArray(covered) && covered.length > 0, `region ${region.id} missing covered_numbers`);
  covered.forEach((value) => {
    assert(Number.isInteger(value) && value >= 0 && value <= 36, `region ${region.id} covers invalid number ${value}`);
  });
});

numberCells.forEach((cell) => {
  assert(cell.bet_type === "straight", `number cell ${cell.id} bet_type must be straight`);
  assert(Number.isInteger(cell.number), `number cell ${cell.id} missing integer number`);
  assert(Array.isArray(cell.covered_numbers) && cell.covered_numbers.length === 1 && cell.covered_numbers[0] === cell.number, `number cell ${cell.id} covered_numbers mismatch`);
  assert(cell.payout_multiplier === EXPECTED_PAYOUTS.straight, `number cell ${cell.id} payout mismatch`);
  assert(cell.chip_anchor && Number.isFinite(cell.chip_anchor.x) && Number.isFinite(cell.chip_anchor.y), `number cell ${cell.id} missing chip_anchor`);
});

const numbers = numberCells.map((cell) => cell.number).sort((a, b) => a - b);
assert(numbers.length === 37, "straight number cell count must be 37");
assert(JSON.stringify(numbers) === JSON.stringify(Array.from({ length: 37 }, (_, i) => i)), "numbers 0..36 must exist exactly once as straight cells");

const numberIdSet = new Set(numberCells.map((cell) => cell.id));
assert(numberIdSet.size === numberCells.length, "duplicate number cell ids found");

const redSet = new Set(RED_NUMBERS);
const blackSet = new Set(BLACK_NUMBERS);
numberCells.forEach((cell) => {
  if (cell.number === 0) {
    assert(cell.colour === "green", "zero must be green");
    assert(cell.rect.x === 0, "zero must be the dedicated region on the left");
    return;
  }
  if (redSet.has(cell.number)) {
    assert(cell.colour === "red", `number ${cell.number} must be red`);
  } else if (blackSet.has(cell.number)) {
    assert(cell.colour === "black", `number ${cell.number} must be black`);
  } else {
    fail(`unexpected colour mapping for ${cell.number}`);
  }
});

Object.entries(EXPECTED_DOZENS).forEach(([id, expected]) => {
  const region = dozens.find((entry) => entry.id === id);
  assert(region, `missing dozen region ${id}`);
  assert(region.bet_type === "dozen", `${id} bet_type must be dozen`);
  assert(region.payout_multiplier === EXPECTED_PAYOUTS.dozen, `${id} payout mismatch`);
  assert(sameNumbers(region.covered_numbers, expected), `${id} covered_numbers mismatch`);
});

Object.entries(EXPECTED_COLUMNS).forEach(([id, expected]) => {
  const region = columns.find((entry) => entry.id === id);
  assert(region, `missing column region ${id}`);
  assert(region.bet_type === "column", `${id} bet_type must be column`);
  assert(region.payout_multiplier === EXPECTED_PAYOUTS.column, `${id} payout mismatch`);
  assert(sameNumbers(region.covered_numbers, expected), `${id} covered_numbers mismatch`);
});

Object.entries(EXPECTED_OUTSIDE).forEach(([id, expected]) => {
  const region = outsideBets.find((entry) => entry.id === id);
  assert(region, `missing outside region ${id}`);
  assert(region.bet_type === "outside", `${id} bet_type must be outside`);
  assert(region.payout_multiplier === EXPECTED_PAYOUTS.outside, `${id} payout mismatch`);
  assert(sameNumbers(region.covered_numbers, expected), `${id} covered_numbers mismatch`);
});

const counts = {
  straight: numberCells.length,
  split: split.length,
  street: street.length,
  corner: corner.length,
  six_line: sixLine.length,
  dozen: dozens.length,
  column: columns.length,
  outside: outsideBets.length,
};

Object.entries(EXPECTED_COUNTS).forEach(([key, expected]) => {
  assert(counts[key] === expected, `${key} count mismatch: expected ${expected}, got ${counts[key]}`);
});

[...split, ...street, ...corner, ...sixLine].forEach((hotspot) => {
  assert(hotspot.anchor && Number.isFinite(hotspot.anchor.x) && Number.isFinite(hotspot.anchor.y), `hotspot ${hotspot.id} missing anchor`);
  assert(typeof hotspot.hotspot_kind === "string" && hotspot.hotspot_kind.length > 0, `hotspot ${hotspot.id} missing hotspot_kind`);
  assert(typeof hotspot.ui_note === "string" && hotspot.ui_note.length > 0, `hotspot ${hotspot.id} missing ui_note`);
  assert(hotspot.payout_multiplier === EXPECTED_PAYOUTS[hotspot.bet_type], `hotspot ${hotspot.id} payout mismatch`);
});

dozens.forEach((region) => assert(region.payout_multiplier === EXPECTED_PAYOUTS.dozen, `${region.id} payout mismatch`));
columns.forEach((region) => assert(region.payout_multiplier === EXPECTED_PAYOUTS.column, `${region.id} payout mismatch`));
outsideBets.forEach((region) => assert(region.payout_multiplier === EXPECTED_PAYOUTS.outside, `${region.id} payout mismatch`));

const allIds = allRegions.map((region) => region.id);
assert(new Set(allIds).size === allIds.length, "duplicate ids found");

const duplicateKeyMap = new Map();
allRegions.forEach((region) => {
  const betType = region.bet_type;
  const setKey = `${betType}:${sorted(region.covered_numbers).join(",")}`;
  if (!duplicateKeyMap.has(setKey)) {
    duplicateKeyMap.set(setKey, []);
  }
  duplicateKeyMap.get(setKey).push(region.id);
});
for (const [setKey, ids] of duplicateKeyMap.entries()) {
  assert(ids.length === 1, `duplicate covered-number set within bet type ${setKey}: ${ids.join(", ")}`);
}

const layout = schema?.layout?.main_grid;
assert(layout && layout.columns === 12 && layout.rows === 3, "main grid shape must be 12 x 3");
assert(JSON.stringify(layout.top_row) === JSON.stringify([3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36]), "top row layout mismatch");
assert(JSON.stringify(layout.middle_row) === JSON.stringify([2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32, 35]), "middle row layout mismatch");
assert(JSON.stringify(layout.bottom_row) === JSON.stringify([1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34]), "bottom row layout mismatch");

console.log("ROULETTE_TABLE_SCHEMA_READY=PASS");
NODE
