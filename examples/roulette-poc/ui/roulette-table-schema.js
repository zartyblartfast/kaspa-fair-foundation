const fs = require("fs");
const path = require("path");

const RED_NUMBERS = [1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36];
const BLACK_NUMBERS = [2, 4, 6, 8, 10, 11, 13, 15, 17, 20, 22, 24, 26, 28, 29, 31, 33, 35];
const RED_SET = new Set(RED_NUMBERS);
const BLACK_SET = new Set(BLACK_NUMBERS);

const CELL_WIDTH = 10;
const CELL_HEIGHT = 10;
const ZERO_WIDTH = 10;
const GRID_X = ZERO_WIDTH;
const GRID_Y = 0;
const GRID_COLUMNS = 12;
const GRID_ROWS = 3;
const GRID_WIDTH = GRID_COLUMNS * CELL_WIDTH;
const GRID_HEIGHT = GRID_ROWS * CELL_HEIGHT;
const COLUMN_SELECTOR_WIDTH = 8;
const DOZEN_HEIGHT = 6;
const OUTSIDE_HEIGHT = 6;
const STREET_HOTSPOT_WIDTH = 2;
const SIX_LINE_HOTSPOT_WIDTH = 2;
const HOTSPOT_THICKNESS = 2;
const CORNER_SIZE = 3;

function rect(x, y, width, height) {
  return { x, y, width, height };
}

function chipAnchorForRect(regionRect) {
  return {
    x: regionRect.x + regionRect.width / 2,
    y: regionRect.y + regionRect.height / 2,
  };
}

function sortNumbers(numbers) {
  return [...numbers].sort((a, b) => a - b);
}

function numberAt(columnIndex, rowIndex) {
  return columnIndex * 3 + (3 - rowIndex);
}

function colourForNumber(number) {
  if (number === 0) {
    return "green";
  }
  if (RED_SET.has(number)) {
    return "red";
  }
  if (BLACK_SET.has(number)) {
    return "black";
  }
  throw new Error(`Unexpected roulette number: ${number}`);
}

function cellRect(columnIndex, rowIndex) {
  return rect(
    GRID_X + columnIndex * CELL_WIDTH,
    GRID_Y + rowIndex * CELL_HEIGHT,
    CELL_WIDTH,
    CELL_HEIGHT,
  );
}

function buildStraightNumberCells() {
  const cells = [];
  const zeroRect = rect(0, 0, ZERO_WIDTH, GRID_HEIGHT);
  cells.push({
    id: "straight_0",
    label: "0",
    bet_type: "straight",
    number: 0,
    covered_numbers: [0],
    colour: "green",
    payout_multiplier: 35,
    rect: zeroRect,
    chip_anchor: chipAnchorForRect(zeroRect),
  });

  for (let columnIndex = 0; columnIndex < GRID_COLUMNS; columnIndex += 1) {
    for (let rowIndex = 0; rowIndex < GRID_ROWS; rowIndex += 1) {
      const number = numberAt(columnIndex, rowIndex);
      const regionRect = cellRect(columnIndex, rowIndex);
      cells.push({
        id: `straight_${number}`,
        label: String(number),
        bet_type: "straight",
        number,
        covered_numbers: [number],
        colour: colourForNumber(number),
        payout_multiplier: 35,
        rect: regionRect,
        chip_anchor: chipAnchorForRect(regionRect),
      });
    }
  }

  return cells;
}

function buildDozens() {
  const dozenY = GRID_Y + GRID_HEIGHT;
  const labels = ["1st 12", "2nd 12", "3rd 12"];
  return [0, 1, 2].map((dozenIndex) => {
    const startNumber = dozenIndex * 12 + 1;
    const covered_numbers = Array.from({ length: 12 }, (_, offset) => startNumber + offset);
    const regionRect = rect(GRID_X + dozenIndex * 4 * CELL_WIDTH, dozenY, 4 * CELL_WIDTH, DOZEN_HEIGHT);
    return {
      id: `dozen_${dozenIndex + 1}`,
      label: labels[dozenIndex],
      bet_type: "dozen",
      covered_numbers,
      payout_multiplier: 2,
      rect: regionRect,
      chip_anchor: chipAnchorForRect(regionRect),
    };
  });
}

function buildOutsideBets() {
  const outsideY = GRID_Y + GRID_HEIGHT + DOZEN_HEIGHT;
  const segmentWidth = GRID_WIDTH / 6;
  const definitions = [
    { id: "low", label: "1 to 18", covered_numbers: Array.from({ length: 18 }, (_, i) => i + 1) },
    { id: "even", label: "EVEN", covered_numbers: Array.from({ length: 18 }, (_, i) => (i + 1) * 2) },
    { id: "red", label: "RED", covered_numbers: RED_NUMBERS },
    { id: "black", label: "BLACK", covered_numbers: BLACK_NUMBERS },
    { id: "odd", label: "ODD", covered_numbers: Array.from({ length: 18 }, (_, i) => i * 2 + 1) },
    { id: "high", label: "19 to 36", covered_numbers: Array.from({ length: 18 }, (_, i) => i + 19) },
  ];

  return definitions.map((definition, index) => {
    const regionRect = rect(GRID_X + index * segmentWidth, outsideY, segmentWidth, OUTSIDE_HEIGHT);
    return {
      id: definition.id,
      label: definition.label,
      bet_type: "outside",
      covered_numbers: definition.covered_numbers,
      payout_multiplier: 1,
      rect: regionRect,
      chip_anchor: chipAnchorForRect(regionRect),
    };
  });
}

function buildColumnSelectors() {
  const columnX = GRID_X + GRID_WIDTH;
  const definitions = [
    { id: "column_3", label: "Column 3", rowIndex: 0 },
    { id: "column_2", label: "Column 2", rowIndex: 1 },
    { id: "column_1", label: "Column 1", rowIndex: 2 },
  ];

  return definitions.map(({ id, label, rowIndex }) => {
    const covered_numbers = Array.from({ length: GRID_COLUMNS }, (_, columnIndex) => numberAt(columnIndex, rowIndex));
    const regionRect = rect(columnX, GRID_Y + rowIndex * CELL_HEIGHT, COLUMN_SELECTOR_WIDTH, CELL_HEIGHT);
    return {
      id,
      label,
      bet_type: "column",
      covered_numbers,
      payout_multiplier: 2,
      rect: regionRect,
      chip_anchor: chipAnchorForRect(regionRect),
    };
  });
}

function buildSplitHotspots() {
  const hotspots = [];

  for (let columnIndex = 0; columnIndex < GRID_COLUMNS; columnIndex += 1) {
    const bottom = numberAt(columnIndex, 2);
    const middle = numberAt(columnIndex, 1);
    const top = numberAt(columnIndex, 0);

    const bottomMiddleRect = rect(
      GRID_X + columnIndex * CELL_WIDTH + 1,
      GRID_Y + 2 * CELL_HEIGHT - HOTSPOT_THICKNESS / 2,
      CELL_WIDTH - 2,
      HOTSPOT_THICKNESS,
    );
    hotspots.push({
      id: `split_${bottom}_${middle}`,
      bet_type: "split",
      label: `${bottom}/${middle}`,
      covered_numbers: [bottom, middle],
      payout_multiplier: 17,
      hotspot_kind: "shared_edge",
      anchor: chipAnchorForRect(bottomMiddleRect),
      rect: bottomMiddleRect,
      ui_note: "Thin horizontal hotspot on the shared edge between vertically adjacent cells.",
    });

    const middleTopRect = rect(
      GRID_X + columnIndex * CELL_WIDTH + 1,
      GRID_Y + CELL_HEIGHT - HOTSPOT_THICKNESS / 2,
      CELL_WIDTH - 2,
      HOTSPOT_THICKNESS,
    );
    hotspots.push({
      id: `split_${middle}_${top}`,
      bet_type: "split",
      label: `${middle}/${top}`,
      covered_numbers: [middle, top],
      payout_multiplier: 17,
      hotspot_kind: "shared_edge",
      anchor: chipAnchorForRect(middleTopRect),
      rect: middleTopRect,
      ui_note: "Thin horizontal hotspot on the shared edge between vertically adjacent cells.",
    });
  }

  for (let columnIndex = 0; columnIndex < GRID_COLUMNS - 1; columnIndex += 1) {
    for (let rowIndex = 0; rowIndex < GRID_ROWS; rowIndex += 1) {
      const leftNumber = numberAt(columnIndex, rowIndex);
      const rightNumber = numberAt(columnIndex + 1, rowIndex);
      const regionRect = rect(
        GRID_X + (columnIndex + 1) * CELL_WIDTH - HOTSPOT_THICKNESS / 2,
        GRID_Y + rowIndex * CELL_HEIGHT + 1,
        HOTSPOT_THICKNESS,
        CELL_HEIGHT - 2,
      );
      hotspots.push({
        id: `split_${sortNumbers([leftNumber, rightNumber]).join("_")}`,
        bet_type: "split",
        label: `${leftNumber}/${rightNumber}`,
        covered_numbers: [leftNumber, rightNumber],
        payout_multiplier: 17,
        hotspot_kind: "shared_edge",
        anchor: chipAnchorForRect(regionRect),
        rect: regionRect,
        ui_note: "Thin vertical hotspot on the shared edge between horizontally adjacent cells.",
      });
    }
  }

  return hotspots;
}

function buildStreetHotspots() {
  return Array.from({ length: GRID_COLUMNS }, (_, columnIndex) => {
    const covered_numbers = [numberAt(columnIndex, 2), numberAt(columnIndex, 1), numberAt(columnIndex, 0)];
    const regionRect = rect(
      GRID_X + (columnIndex + 1) * CELL_WIDTH - STREET_HOTSPOT_WIDTH / 2,
      GRID_Y + 1,
      STREET_HOTSPOT_WIDTH,
      GRID_HEIGHT - 2,
    );
    return {
      id: `street_${covered_numbers[0]}_${covered_numbers[1]}_${covered_numbers[2]}`,
      bet_type: "street",
      label: covered_numbers.join("/"),
      covered_numbers,
      payout_multiplier: 11,
      hotspot_kind: "column_end",
      anchor: chipAnchorForRect(regionRect),
      rect: regionRect,
      ui_note: "Future street hotspot aligned to the outside edge of a three-number column.",
    };
  });
}

function buildCornerHotspots() {
  const hotspots = [];
  for (let columnIndex = 0; columnIndex < GRID_COLUMNS - 1; columnIndex += 1) {
    for (let rowBoundary = 1; rowBoundary < GRID_ROWS; rowBoundary += 1) {
      const leftUpper = numberAt(columnIndex, rowBoundary - 1);
      const leftLower = numberAt(columnIndex, rowBoundary);
      const rightUpper = numberAt(columnIndex + 1, rowBoundary - 1);
      const rightLower = numberAt(columnIndex + 1, rowBoundary);
      const covered_numbers = sortNumbers([leftUpper, leftLower, rightUpper, rightLower]);
      const regionRect = rect(
        GRID_X + (columnIndex + 1) * CELL_WIDTH - CORNER_SIZE / 2,
        GRID_Y + rowBoundary * CELL_HEIGHT - CORNER_SIZE / 2,
        CORNER_SIZE,
        CORNER_SIZE,
      );
      hotspots.push({
        id: `corner_${covered_numbers.join("_")}`,
        bet_type: "corner",
        label: covered_numbers.join("/"),
        covered_numbers,
        payout_multiplier: 8,
        hotspot_kind: "intersection",
        anchor: chipAnchorForRect(regionRect),
        rect: regionRect,
        ui_note: "Future corner hotspot centered on the intersection of four adjacent number cells.",
      });
    }
  }
  return hotspots;
}

function buildSixLineHotspots() {
  const hotspots = [];
  for (let columnIndex = 0; columnIndex < GRID_COLUMNS - 1; columnIndex += 1) {
    const covered_numbers = sortNumbers([
      numberAt(columnIndex, 0),
      numberAt(columnIndex, 1),
      numberAt(columnIndex, 2),
      numberAt(columnIndex + 1, 0),
      numberAt(columnIndex + 1, 1),
      numberAt(columnIndex + 1, 2),
    ]);
    const regionRect = rect(
      GRID_X + (columnIndex + 1) * CELL_WIDTH - SIX_LINE_HOTSPOT_WIDTH / 2,
      GRID_Y + 1,
      SIX_LINE_HOTSPOT_WIDTH,
      GRID_HEIGHT - 2,
    );
    hotspots.push({
      id: `six_line_${covered_numbers.join("_")}`,
      bet_type: "six_line",
      label: covered_numbers.join("/"),
      covered_numbers,
      payout_multiplier: 5,
      hotspot_kind: "double_column_end",
      anchor: chipAnchorForRect(regionRect),
      rect: regionRect,
      ui_note: "Future six-line hotspot aligned to the outside edge shared by two adjacent three-number columns.",
    });
  }
  return hotspots;
}

function buildSchema() {
  const number_cells = buildStraightNumberCells();
  const dozens = buildDozens();
  const outside_bets = buildOutsideBets();
  const columns = buildColumnSelectors();
  const split = buildSplitHotspots();
  const street = buildStreetHotspots();
  const corner = buildCornerHotspots();
  const six_line = buildSixLineHotspots();

  return {
    schema: "kaspa-fair-roulette-table-layout-v1",
    roulette_variant: "european",
    coordinate_system: {
      units: "arbitrary table units",
      origin: "top-left",
      clickable_region_contract: "all clickable regions have x, y, width, height",
    },
    mainnet_supported: false,
    mock_only: true,
    colour_sets: {
      green: [0],
      red: RED_NUMBERS,
      black: BLACK_NUMBERS,
    },
    layout: {
      zero_position: "left",
      main_grid: {
        columns: GRID_COLUMNS,
        rows: GRID_ROWS,
        origin: { x: GRID_X, y: GRID_Y },
        cell_size: { width: CELL_WIDTH, height: CELL_HEIGHT },
        top_row: Array.from({ length: GRID_COLUMNS }, (_, columnIndex) => numberAt(columnIndex, 0)),
        middle_row: Array.from({ length: GRID_COLUMNS }, (_, columnIndex) => numberAt(columnIndex, 1)),
        bottom_row: Array.from({ length: GRID_COLUMNS }, (_, columnIndex) => numberAt(columnIndex, 2)),
      },
      column_selector_position: "right_of_main_grid",
      dozens_position: "below_main_grid",
      outside_bets_position: "below_dozens",
    },
    regions: {
      number_cells,
      dozens,
      outside_bets,
      columns,
      hotspots: {
        split,
        street,
        corner,
        six_line,
      },
    },
    required_counts: {
      straight: number_cells.length,
      split: split.length,
      street: street.length,
      corner: corner.length,
      six_line: six_line.length,
      dozen: dozens.length,
      column: columns.length,
      outside_even_money: outside_bets.length,
    },
    ui_constraints: [
      "ENV-081A defines the table layout schema only.",
      "UI rebuild is deferred to ENV-081B.",
      "No giant inside-zone lists.",
      "No dropdown-based inside-zone betting.",
      "No real betting, real payouts, wallet, backend custody, signing, broadcasting, mainnet, or production casino functionality.",
    ],
  };
}

function writeSchemaFile(outputPath = path.join(__dirname, "roulette-table-schema.json")) {
  const schema = buildSchema();
  fs.writeFileSync(outputPath, `${JSON.stringify(schema, null, 2)}\n`);
  return outputPath;
}

if (require.main === module) {
  writeSchemaFile();
}

module.exports = {
  BLACK_NUMBERS,
  RED_NUMBERS,
  buildSchema,
  writeSchemaFile,
};
