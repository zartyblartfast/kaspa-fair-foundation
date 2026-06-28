const ROUND_SEQUENCE = [
  "BetsOpen",
  "SpinVisualStarted",
  "NoMoreBets",
  "ResultFinalised",
  "Settled",
  "ProofPublished",
];

const ROUND_DESCRIPTIONS = {
  BetsOpen: "Schema-driven mock bets may be placed before wheel start.",
  SpinVisualStarted: "Bets are still open while the wheel is visually spinning.",
  NoMoreBets: "No more bets — ledger locked.",
  ResultFinalised: "Deterministic result revealed from sample-round.json after NoMoreBets.",
  Settled: "Deterministic settlement from sample-round.json is shown. UI-added bets remain mock display bets only.",
  ProofPublished: "Proof fields and final PASS status are now displayed.",
};

const BETS_CLOSED_NO_MORE_BETS = "BETS_CLOSED_NO_MORE_BETS";
const PLACEABLE_STATES = new Set(["BetsOpen", "SpinVisualStarted"]);
const BET_PLACEMENT_DESCRIPTION = "Click any valid roulette table zone to add a chip: number cells, zero, split/street/corner/six-line selectors, dozens, columns, and outside rectangles all use the same additive bet placement behavior.";

const ui = {
  overallStatus: document.getElementById("overall-status"),
  failurePanel: document.getElementById("failure-panel"),
  failureMessage: document.getElementById("failure-message"),
  currentRoundState: document.getElementById("current-round-state"),
  stateDescription: document.getElementById("state-description"),
  wheelVisual: document.getElementById("wheel-visual"),
  trustList: document.getElementById("trust-status-list"),
  safetyFlagsList: document.getElementById("safety-flags-list"),
  sequenceList: document.getElementById("sequence-list"),
  rouletteTableSvgHost: document.getElementById("roulette-table-svg-host"),
  resultNumber: document.getElementById("result-number"),
  resultColour: document.getElementById("result-colour"),
  resultAlgorithm: document.getElementById("result-algorithm"),
  betsBody: document.getElementById("bets-body"),
  settlementSummary: document.getElementById("settlement-summary"),
  uiOnlyBetNote: document.getElementById("ui-only-bet-note"),
  settlementList: document.getElementById("settlement-list"),
  proofStatus: document.getElementById("proof-status"),
  proofList: document.getElementById("proof-list"),
  betStakeInput: document.getElementById("bet-stake-input"),
  betStatus: document.getElementById("bet-status"),
  betPlacementNote: document.getElementById("bet-placement-note"),
  mockBetList: document.getElementById("mock-bet-list"),
  startWheelButton: document.getElementById("start-wheel-button"),
  noMoreBetsButton: document.getElementById("no-more-bets-button"),
  revealResultButton: document.getElementById("reveal-result-button"),
  showSettlementButton: document.getElementById("show-settlement-button"),
  publishProofButton: document.getElementById("publish-proof-button"),
  resetRoundButton: document.getElementById("reset-round-button"),
};

const appState = {
  round: null,
  tableSchema: null,
  uiState: "BetsOpen",
  uiMockBets: [],
  nextMockBetId: 1,
};

boot().catch((error) => {
  showFailure(`Failed to load deterministic round or roulette table schema: ${error.message}`);
});

async function boot() {
  const [round, tableSchema] = await Promise.all([
    fetchJson("sample-round.json"),
    fetchJson("roulette-table-schema.json"),
  ]);

  validateRound(round);
  validateTableSchema(tableSchema);
  appState.round = round;
  appState.tableSchema = tableSchema;
  bindEvents();
  renderStaticPanels(round);
  resetRoundFlow();
}

async function fetchJson(path) {
  const response = await fetch(path, { cache: "no-store" });
  if (!response.ok) {
    throw new Error(`${path} HTTP ${response.status}`);
  }
  return response.json();
}

function validateRound(round) {
  const checks = [
    ["final_result == PASS", round.final_result === "PASS"],
    ["round_state == ProofPublished", round.round_state === "ProofPublished"],
    ["foundation_verifier_result == PASS", round.foundation_verifier_result === "PASS"],
    ["foundation_network == testnet-10", round.foundation_network === "testnet-10"],
    ["mainnet_supported == false", round.mainnet_supported === false],
    ["foundation_readonly == true", round.foundation_readonly === true],
    ["signing_used == false", round.signing_used === false],
    ["transaction_created == false", round.transaction_created === false],
    ["broadcast_used == false", round.broadcast_used === false],
    ["wallet_access_used == false", round.wallet_access_used === false],
    ["result_number in 0..36", Number.isInteger(round.result_number) && round.result_number >= 0 && round.result_number <= 36],
    ["result_colour in green/red/black", ["green", "red", "black"].includes(round.result_colour)],
  ];

  const failedChecks = checks.filter(([, passed]) => !passed).map(([label]) => label);
  if (failedChecks.length > 0) {
    throw new Error(`Unsafe or failed JSON: ${failedChecks.join(", ")}`);
  }
}

function validateTableSchema(tableSchema) {
  const topRow = tableSchema.layout?.main_grid?.top_row || [];
  const middleRow = tableSchema.layout?.main_grid?.middle_row || [];
  const bottomRow = tableSchema.layout?.main_grid?.bottom_row || [];
  const checks = [
    ["schema name", tableSchema.schema === "kaspa-fair-roulette-table-layout-v1"],
    ["roulette variant european", tableSchema.roulette_variant === "european"],
    ["mock only true", tableSchema.mock_only === true],
    ["mainnet_supported false", tableSchema.mainnet_supported === false],
    ["zero on left", tableSchema.layout?.zero_position === "left"],
    ["top row length", topRow.length === 12],
    ["middle row length", middleRow.length === 12],
    ["bottom row length", bottomRow.length === 12],
    ["number cell count", (tableSchema.regions?.number_cells || []).length === 37],
    ["dozen count", (tableSchema.regions?.dozens || []).length === 3],
    ["outside count", (tableSchema.regions?.outside_bets || []).length === 6],
    ["column count", (tableSchema.regions?.columns || []).length === 3],
    ["split hotspots", (tableSchema.regions?.hotspots?.split || []).length > 0],
    ["street hotspots", (tableSchema.regions?.hotspots?.street || []).length > 0],
    ["corner hotspots", (tableSchema.regions?.hotspots?.corner || []).length > 0],
    ["six_line hotspots", (tableSchema.regions?.hotspots?.six_line || []).length > 0],
  ];

  const failedChecks = checks.filter(([, passed]) => !passed).map(([label]) => label);
  if (failedChecks.length > 0) {
    throw new Error(`Unsafe or invalid roulette table schema: ${failedChecks.join(", ")}`);
  }
}

function bindEvents() {
  ui.startWheelButton.addEventListener("click", () => advanceState("SpinVisualStarted"));
  ui.noMoreBetsButton.addEventListener("click", () => advanceState("NoMoreBets"));
  ui.revealResultButton.addEventListener("click", () => advanceState("ResultFinalised"));
  ui.showSettlementButton.addEventListener("click", () => advanceState("Settled"));
  ui.publishProofButton.addEventListener("click", () => advanceState("ProofPublished"));
  ui.resetRoundButton.addEventListener("click", resetRoundFlow);
}

function advanceState(nextState) {
  const currentIndex = ROUND_SEQUENCE.indexOf(appState.uiState);
  const nextIndex = ROUND_SEQUENCE.indexOf(nextState);
  if (nextIndex !== currentIndex + 1) {
    return;
  }
  appState.uiState = nextState;
  renderFlow();
}

function renderStaticPanels(round) {
  renderTrustPanel(round);
  renderSafetyFlags(round);
  renderDeterministicSettlementInput(round);
  ui.resultAlgorithm.textContent = round.result_algorithm;
  setOverallStatus("PASS — deterministic sample JSON and schema-driven SVG roulette table loaded", true);
}

function resetRoundFlow() {
  appState.uiState = "BetsOpen";
  appState.uiMockBets = [];
  appState.nextMockBetId = 1;
  ui.betStakeInput.value = "5";
  renderFlow();
}

function renderFlow() {
  const round = appState.round;
  const uiState = appState.uiState;
  const resultVisible = hasReachedState("ResultFinalised");
  const settlementVisible = hasReachedState("Settled");
  const proofVisible = hasReachedState("ProofPublished");
  const canPlaceMockBets = canPlaceBetsForState(uiState);

  ui.currentRoundState.textContent = uiState;
  ui.stateDescription.textContent = ROUND_DESCRIPTIONS[uiState];
  ui.wheelVisual.textContent = hasReachedState("SpinVisualStarted") ? "SPINNING" : "READY";
  ui.wheelVisual.className = `wheel-visual ${hasReachedState("SpinVisualStarted") ? (hasReachedState("NoMoreBets") ? "wheel-stopped" : "wheel-spinning") : "wheel-ready"}`;

  ui.startWheelButton.disabled = uiState !== "BetsOpen";
  ui.noMoreBetsButton.disabled = uiState !== "SpinVisualStarted";
  ui.revealResultButton.disabled = uiState !== "NoMoreBets";
  ui.showSettlementButton.disabled = uiState !== "ResultFinalised";
  ui.publishProofButton.disabled = uiState !== "Settled";
  ui.betStakeInput.disabled = !canPlaceMockBets;

  ui.betStatus.textContent = buildBetStatusText(uiState);
  ui.betStatus.className = `bet-status ${canPlaceMockBets ? "bet-open" : "bet-closed"}`;
  ui.betPlacementNote.textContent = BET_PLACEMENT_DESCRIPTION;

  if (resultVisible) {
    ui.resultNumber.textContent = String(round.result_number);
    ui.resultNumber.className = `result-number ${round.result_colour}`;
    ui.resultColour.textContent = round.result_colour;
  } else {
    ui.resultNumber.textContent = "--";
    ui.resultNumber.className = "result-number hidden-result";
    ui.resultColour.textContent = "Hidden until reveal";
  }

  renderRouletteTable(resultVisible ? round.result_number : null, canPlaceMockBets);
  renderRoundSequence(uiState);
  renderUiMockBetLedger();
  renderSettlement(round, settlementVisible);
  renderProof(round, proofVisible);
}

function renderTrustPanel(round) {
  const rows = [
    ["foundation verifier", round.foundation_verifier_result],
    ["foundation network", round.foundation_network],
    ["round id", round.round_id],
    ["round source state", round.round_state],
    ["deterministic result source", "sample-round.json"],
    ["roulette table schema source", "roulette-table-schema.json"],
  ];

  ui.trustList.innerHTML = "";
  rows.forEach(([key, value]) => {
    const li = document.createElement("li");
    const isPass = ["PASS", "testnet-10", "sample-round.json", "roulette-table-schema.json"].includes(String(value));
    li.innerHTML = `<span class="kv-key">${escapeHtml(key)}</span><span class="kv-value ${isPass ? "pass" : ""}">${escapeHtml(String(value))}</span>`;
    ui.trustList.appendChild(li);
  });
}

function renderSafetyFlags(round) {
  const flags = [
    ["testnet-10", round.foundation_network === "testnet-10" ? "PASS" : "FAIL"],
    ["mock_only schema: true", String(appState.tableSchema.mock_only)],
    ["mainnet_supported: false", String(round.mainnet_supported || appState.tableSchema.mainnet_supported)],
    ["readonly: true", String(round.foundation_readonly)],
    ["signing_used: false", String(round.signing_used)],
    ["transaction_created: false", String(round.transaction_created)],
    ["broadcast_used: false", String(round.broadcast_used)],
    ["wallet_access_used: false", String(round.wallet_access_used)],
  ];

  ui.safetyFlagsList.innerHTML = "";
  flags.forEach(([key, value]) => {
    const li = document.createElement("li");
    const pass = value === "PASS" || value === "false" || value === "true";
    li.innerHTML = `<span class="kv-key">${escapeHtml(key)}</span><span class="kv-value ${pass ? "pass" : "fail"}">${escapeHtml(String(value))}</span>`;
    ui.safetyFlagsList.appendChild(li);
  });
}

function renderRoundSequence(currentState) {
  const currentIndex = ROUND_SEQUENCE.indexOf(currentState);
  ui.sequenceList.innerHTML = "";
  ROUND_SEQUENCE.forEach((state, index) => {
    const li = document.createElement("li");
    const stateClass = index < currentIndex ? "complete" : index === currentIndex ? "active" : "pending";
    li.innerHTML = `
      <div class="sequence-step ${stateClass}">
        <span class="sequence-badge">${index + 1}</span>
        <span>${state}</span>
      </div>
      <span class="${stateClass}">${index === currentIndex ? "current" : index < currentIndex ? "done" : "waiting"}</span>
    `;
    ui.sequenceList.appendChild(li);
  });
}

function renderRouletteTable(resultNumber = null, allowBetPlacement = false) {
  const chipStackCounts = new Map();
  const chips = appState.uiMockBets.map((bet) => {
    const stackKey = `${bet.anchor.x}:${bet.anchor.y}`;
    const stackIndex = chipStackCounts.get(stackKey) || 0;
    chipStackCounts.set(stackKey, stackIndex + 1);
    return {
      id: bet.betId,
      x: bet.anchor.x,
      y: bet.anchor.y,
      stakeUnits: bet.stakeUnits,
      stackIndex,
    };
  });

  window.RouletteTableRenderer.render(ui.rouletteTableSvgHost, appState.tableSchema, {
    allowBetPlacement,
    chips,
    highlightedNumber: resultNumber,
    onZoneClick: placeMockBetFromZone,
  });
}

function renderDeterministicSettlementInput(round) {
  const settlementById = new Map((round.settlement || []).map((item) => [item.bet_id, item]));
  ui.betsBody.innerHTML = "";

  (round.bets || []).forEach((bet) => {
    const settlement = settlementById.get(bet.bet_id) || {};
    const row = document.createElement("tr");
    const outcomeClass = settlement.won ? "badge-pass" : "badge-fail";
    const outcomeText = settlement.won ? "WIN" : "LOSS";
    row.innerHTML = `
      <td><code>${escapeHtml(bet.bet_id)}</code></td>
      <td>${escapeHtml(bet.bet_type)}</td>
      <td>${escapeHtml(bet.selection_value)}</td>
      <td>${escapeHtml(String(bet.stake_units))}</td>
      <td><span class="${outcomeClass}">${outcomeText}</span></td>
      <td>${escapeHtml(String(settlement.payout_units ?? 0))}</td>
      <td>${escapeHtml(String(settlement.net_units ?? 0))}</td>
    `;
    ui.betsBody.appendChild(row);
  });
}

function renderSettlement(round, visible) {
  ui.settlementList.innerHTML = "";
  if (!visible) {
    ui.settlementSummary.textContent = "Settlement hidden until Show Settlement.";
    ui.uiOnlyBetNote.textContent = "UI-added bets are mock display bets only; deterministic settlement is from the engine sample round.";
    return;
  }

  const totals = (round.settlement || []).reduce((acc, item) => {
    acc.totalStake += Number(item.stake_units || 0);
    acc.totalPayout += Number(item.payout_units || 0);
    acc.totalNet += Number(item.net_units || 0);
    acc.wins += item.won ? 1 : 0;
    return acc;
  }, { totalStake: 0, totalPayout: 0, totalNet: 0, wins: 0 });

  ui.settlementSummary.textContent = `Deterministic settlement shown from sample-round.json for ${round.settlement.length} bets.`;
  ui.uiOnlyBetNote.textContent = "UI-added bets are mock display bets only; deterministic settlement is from the engine sample round.";

  const rows = [
    ["winning bets", totals.wins],
    ["total stake units", totals.totalStake],
    ["total payout units", totals.totalPayout],
    ["total net units", totals.totalNet],
    ["ui-only mock bets in display ledger", appState.uiMockBets.length],
  ];

  rows.forEach(([key, value]) => {
    const li = document.createElement("li");
    li.innerHTML = `<span class="kv-key">${escapeHtml(String(key))}</span><span class="kv-value"><code>${escapeHtml(String(value))}</code></span>`;
    ui.settlementList.appendChild(li);
  });
}

function renderProof(round, visible) {
  ui.proofList.innerHTML = "";
  if (!visible) {
    ui.proofStatus.textContent = "Proof hidden until Publish Proof.";
    ui.proofStatus.className = "proof-status proof-hidden";
    return;
  }

  ui.proofStatus.textContent = `ProofPublished: foundation verifier ${round.foundation_verifier_result}; final_result ${round.final_result}.`;
  ui.proofStatus.className = "proof-status proof-pass";

  const proofRows = [
    ["foundation covenant id", round.foundation_covenant_id],
    ["ENV-064 txid", round.foundation_env064_spend_txid],
    ["accepting block hash", round.foundation_accepting_block_hash],
    ["bet ledger hash", round.bet_ledger_hash],
    ["result number", round.result_number],
    ["result colour", round.result_colour],
    ["final_result", round.final_result],
  ];

  proofRows.forEach(([key, value]) => {
    const li = document.createElement("li");
    li.innerHTML = `<span class="kv-key">${escapeHtml(String(key))}</span><span class="kv-value"><code>${escapeHtml(String(value))}</code></span>`;
    ui.proofList.appendChild(li);
  });
}

function placeMockBetFromZone(zone) {
  const currentState = appState.uiState;
  if (!canPlaceBetsForState(currentState)) {
    ui.betStatus.textContent = `${BETS_CLOSED_NO_MORE_BETS} — No more bets — ledger locked.`;
    ui.betStatus.className = "bet-status bet-closed";
    return;
  }

  const stakeUnits = normaliseStakeUnits(ui.betStakeInput.value);
  const anchor = window.RouletteTableRenderer.getZoneAnchor(zone);
  const mockBet = {
    betId: `ui-mock-${String(appState.nextMockBetId).padStart(3, "0")}`,
    betType: zone.bet_type,
    label: buildLedgerLabel(zone),
    coveredNumbers: [...(zone.covered_numbers || [])],
    payoutMultiplier: Number(zone.payout_multiplier ?? 0),
    stakeUnits,
    placedDuringState: currentState,
    anchor,
  };

  appState.nextMockBetId += 1;
  appState.uiMockBets.push(mockBet);
  ui.betStatus.textContent = currentState === "SpinVisualStarted"
    ? "Bets are still open while the wheel is visually spinning."
    : "BetsOpen: schema-driven mock bets may be placed before wheel start.";
  ui.betStatus.className = "bet-status bet-open";
  renderUiMockBetLedger();
  renderRouletteTable(hasReachedState("ResultFinalised") ? appState.round.result_number : null, true);
}

function renderUiMockBetLedger() {
  ui.mockBetList.innerHTML = "";
  if (appState.uiMockBets.length === 0) {
    const item = document.createElement("li");
    item.innerHTML = "<div class=\"label\">UI mock bet ledger</div><strong>No UI-added mock bets yet.</strong><div>Click the SVG roulette table to add a mock bet from the visible schema-driven zones.</div>";
    ui.mockBetList.appendChild(item);
    return;
  }

  appState.uiMockBets.forEach((bet) => {
    const item = document.createElement("li");
    item.innerHTML = `
      <div class="label">${escapeHtml(bet.betId)}</div>
      <strong>${escapeHtml(bet.betType)} — ${escapeHtml(bet.label)}</strong>
      <div>covered numbers: <code>${escapeHtml(bet.coveredNumbers.join(", "))}</code></div>
      <div>stake units: <code>${escapeHtml(String(bet.stakeUnits))}</code></div>
      <div>payout multiplier: <code>${escapeHtml(String(bet.payoutMultiplier))}</code></div>
      <div>placed during: <code>${escapeHtml(bet.placedDuringState)}</code></div>
    `;
    ui.mockBetList.appendChild(item);
  });
}

function normaliseStakeUnits(value) {
  const parsed = Number.parseInt(value, 10);
  return Number.isInteger(parsed) && parsed > 0 ? parsed : 1;
}

function buildLedgerLabel(zone) {
  if (zone.bet_type === "column") {
    return `${zone.label} — ${window.RouletteTableRenderer.visibleZoneLabel(zone)}`;
  }
  return window.RouletteTableRenderer.visibleZoneLabel(zone);
}

function buildBetStatusText(uiState) {
  if (uiState === "SpinVisualStarted") {
    return "Bets are still open while the wheel is visually spinning.";
  }
  if (uiState === "BetsOpen") {
    return "BetsOpen: schema-driven mock bets may be placed before wheel start.";
  }
  return `${BETS_CLOSED_NO_MORE_BETS} — No more bets — ledger locked.`;
}

function canPlaceBetsForState(uiState) {
  return PLACEABLE_STATES.has(uiState);
}

function hasReachedState(targetState) {
  return ROUND_SEQUENCE.indexOf(appState.uiState) >= ROUND_SEQUENCE.indexOf(targetState);
}

function setOverallStatus(text, pass) {
  ui.overallStatus.textContent = text;
  ui.overallStatus.className = `status-pill ${pass ? "status-pass" : "status-fail"}`;
}

function showFailure(message) {
  ui.failurePanel.hidden = false;
  ui.failureMessage.textContent = message;
  setOverallStatus("FAIL — deterministic round or SVG schema could not be loaded safely", false);
}

function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}
