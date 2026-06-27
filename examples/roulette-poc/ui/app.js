const ROUND_SEQUENCE = [
  "BetsOpen",
  "SpinVisualStarted",
  "NoMoreBets",
  "ResultFinalised",
  "Settled",
  "ProofPublished",
];

const ROUND_DESCRIPTIONS = {
  BetsOpen: "Bets are open. Mock display bets may still be changed in the UI.",
  SpinVisualStarted: "Wheel visual started. Bets remain visually open while the wheel is spinning.",
  NoMoreBets: "No more bets. Adding or changing mock bets is now blocked in the UI.",
  ResultFinalised: "Deterministic result revealed from sample-round.json after NoMoreBets.",
  Settled: "Deterministic settlement from sample-round.json is now shown.",
  ProofPublished: "Proof fields and final PASS status are now displayed.",
};

const RED_NUMBERS = new Set([1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36]);
const BLACK_NUMBERS = new Set([2, 4, 6, 8, 10, 11, 13, 15, 17, 20, 22, 24, 26, 28, 29, 31, 33, 35]);

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
  rouletteTable: document.getElementById("roulette-table"),
  resultNumber: document.getElementById("result-number"),
  resultColour: document.getElementById("result-colour"),
  resultAlgorithm: document.getElementById("result-algorithm"),
  betsBody: document.getElementById("bets-body"),
  settlementSummary: document.getElementById("settlement-summary"),
  settlementList: document.getElementById("settlement-list"),
  proofStatus: document.getElementById("proof-status"),
  proofList: document.getElementById("proof-list"),
  betTypeSelect: document.getElementById("bet-type-select"),
  betSelectionInput: document.getElementById("bet-selection-input"),
  betStakeInput: document.getElementById("bet-stake-input"),
  applyBetButton: document.getElementById("apply-bet-button"),
  betStatus: document.getElementById("bet-status"),
  mockBetList: document.getElementById("mock-bet-list"),
  startWheelButton: document.getElementById("start-wheel-button"),
  noMoreBetsButton: document.getElementById("no-more-bets-button"),
  revealResultButton: document.getElementById("reveal-result-button"),
  showSettlementButton: document.getElementById("show-settlement-button"),
  publishProofButton: document.getElementById("publish-proof-button"),
};

const appState = {
  round: null,
  uiState: "BetsOpen",
  mockBets: [],
};

boot().catch((error) => {
  showFailure(`Failed to load round JSON: ${error.message}`);
});

async function boot() {
  const response = await fetch("sample-round.json", { cache: "no-store" });
  if (!response.ok) {
    throw new Error(`HTTP ${response.status}`);
  }

  const round = await response.json();
  validateRound(round);
  appState.round = round;
  appState.mockBets = createInitialMockBets(round);
  bindEvents();
  renderStaticPanels(round);
  renderFlow();
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

function bindEvents() {
  ui.startWheelButton.addEventListener("click", () => advanceState("SpinVisualStarted"));
  ui.noMoreBetsButton.addEventListener("click", () => advanceState("NoMoreBets"));
  ui.revealResultButton.addEventListener("click", () => advanceState("ResultFinalised"));
  ui.showSettlementButton.addEventListener("click", () => advanceState("Settled"));
  ui.publishProofButton.addEventListener("click", () => advanceState("ProofPublished"));
  ui.applyBetButton.addEventListener("click", applyMockBetChange);
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
  renderBets(round);
  renderRouletteTable();
  ui.resultAlgorithm.textContent = round.result_algorithm;
  setOverallStatus("PASS — deterministic sample JSON loaded; interactive flow ready", true);
}

function renderFlow() {
  const round = appState.round;
  const uiState = appState.uiState;
  const uiStateIndex = ROUND_SEQUENCE.indexOf(uiState);
  const betsLocked = uiStateIndex >= ROUND_SEQUENCE.indexOf("NoMoreBets");
  const resultVisible = uiStateIndex >= ROUND_SEQUENCE.indexOf("ResultFinalised");
  const settlementVisible = uiStateIndex >= ROUND_SEQUENCE.indexOf("Settled");
  const proofVisible = uiStateIndex >= ROUND_SEQUENCE.indexOf("ProofPublished");

  ui.currentRoundState.textContent = uiState;
  ui.stateDescription.textContent = ROUND_DESCRIPTIONS[uiState];
  ui.wheelVisual.textContent = uiStateIndex >= ROUND_SEQUENCE.indexOf("SpinVisualStarted") ? "SPINNING" : "READY";
  ui.wheelVisual.className = `wheel-visual ${uiStateIndex >= ROUND_SEQUENCE.indexOf("SpinVisualStarted") ? (betsLocked ? "wheel-stopped" : "wheel-spinning") : "wheel-ready"}`;

  ui.startWheelButton.disabled = uiState !== "BetsOpen";
  ui.noMoreBetsButton.disabled = uiState !== "SpinVisualStarted";
  ui.revealResultButton.disabled = uiState !== "NoMoreBets";
  ui.showSettlementButton.disabled = uiState !== "ResultFinalised";
  ui.publishProofButton.disabled = uiState !== "Settled";

  ui.betTypeSelect.disabled = betsLocked;
  ui.betSelectionInput.disabled = betsLocked;
  ui.betStakeInput.disabled = betsLocked;
  ui.applyBetButton.disabled = betsLocked;
  ui.betStatus.textContent = betsLocked
    ? "NoMoreBets: adding or changing mock bets is blocked in the UI."
    : uiState === "SpinVisualStarted"
      ? "SpinVisualStarted: wheel is visually spinning and bets are still visually allowed."
      : "BetsOpen: mock display bets can be changed.";
  ui.betStatus.className = `bet-status ${betsLocked ? "bet-closed" : "bet-open"}`;

  if (resultVisible) {
    ui.resultNumber.textContent = String(round.result_number);
    ui.resultNumber.className = `result-number ${round.result_colour}`;
    ui.resultColour.textContent = round.result_colour;
  } else {
    ui.resultNumber.textContent = "--";
    ui.resultNumber.className = "result-number hidden-result";
    ui.resultColour.textContent = "Hidden until reveal";
  }

  renderRouletteTable(resultVisible ? round.result_number : null);
  renderRoundSequence(uiState);
  renderMockBetList();
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
  ];

  ui.trustList.innerHTML = "";
  rows.forEach(([key, value]) => {
    const li = document.createElement("li");
    const isPass = ["PASS", "testnet-10", "sample-round.json"].includes(String(value));
    li.innerHTML = `<span class="kv-key">${escapeHtml(key)}</span><span class="kv-value ${isPass ? "pass" : ""}">${escapeHtml(String(value))}</span>`;
    ui.trustList.appendChild(li);
  });
}

function renderSafetyFlags(round) {
  const flags = [
    ["testnet-10", round.foundation_network === "testnet-10" ? "PASS" : "FAIL"],
    ["mainnet_supported: false", String(round.mainnet_supported)],
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

function renderRouletteTable(resultNumber = null) {
  ui.rouletteTable.innerHTML = "";

  const zero = document.createElement("div");
  zero.className = `zero-cell ${resultNumber === 0 ? "hit" : ""}`;
  zero.textContent = "0";
  ui.rouletteTable.appendChild(zero);

  for (let row = 0; row < 12; row += 1) {
    for (let offset = 0; offset < 3; offset += 1) {
      const number = 3 * row + (3 - offset);
      const colour = colourForNumber(number);
      const cell = document.createElement("div");
      cell.className = `table-cell ${colour} ${number === resultNumber ? "hit" : ""}`;
      cell.textContent = String(number);
      ui.rouletteTable.appendChild(cell);
    }
  }

  ["1–18", "even", "red", "black", "odd", "19–36"].forEach((label) => {
    const area = document.createElement("div");
    area.className = "bet-area";
    area.textContent = label;
    ui.rouletteTable.appendChild(area);
  });
}

function renderBets(round) {
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
  const rows = [
    ["winning bets", totals.wins],
    ["total stake units", totals.totalStake],
    ["total payout units", totals.totalPayout],
    ["total net units", totals.totalNet],
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

function createInitialMockBets(round) {
  const firstBet = round.bets?.[0] || { bet_type: "straight-number", selection_value: "17", stake_units: 10 };
  return [
    {
      label: "Editable mock display bet",
      betType: firstBet.bet_type,
      selection: firstBet.selection_value,
      stakeUnits: firstBet.stake_units,
    },
  ];
}

function applyMockBetChange() {
  const currentIndex = ROUND_SEQUENCE.indexOf(appState.uiState);
  const closeIndex = ROUND_SEQUENCE.indexOf("NoMoreBets");
  const canChangeBets = currentIndex < closeIndex;
  if (!canChangeBets) {
    ui.betStatus.textContent = "NoMoreBets: adding or changing mock bets is blocked in the UI.";
    ui.betStatus.className = "bet-status bet-closed";
    return;
  }

  const betType = ui.betTypeSelect.value.trim();
  const selection = ui.betSelectionInput.value.trim() || "17";
  const stakeUnits = Number.parseInt(ui.betStakeInput.value, 10);
  const safeStakeUnits = Number.isInteger(stakeUnits) && stakeUnits > 0 ? stakeUnits : 1;

  appState.mockBets[0] = {
    label: appState.uiState === "SpinVisualStarted" ? "Editable mock display bet (spin visual active)" : "Editable mock display bet",
    betType,
    selection,
    stakeUnits: safeStakeUnits,
  };
  renderMockBetList();
  ui.betStatus.textContent = appState.uiState === "SpinVisualStarted"
    ? "SpinVisualStarted: wheel is visually spinning and bets are still visually allowed."
    : "BetsOpen: mock display bets can be changed.";
  ui.betStatus.className = "bet-status bet-open";
}

function renderMockBetList() {
  ui.mockBetList.innerHTML = "";
  appState.mockBets.forEach((bet) => {
    const item = document.createElement("li");
    item.innerHTML = `
      <div class="label">${escapeHtml(bet.label)}</div>
      <strong>${escapeHtml(bet.betType)}</strong>
      <div>selection: <code>${escapeHtml(String(bet.selection))}</code></div>
      <div>stake_units: <code>${escapeHtml(String(bet.stakeUnits))}</code></div>
    `;
    ui.mockBetList.appendChild(item);
  });
}

function setOverallStatus(text, pass) {
  ui.overallStatus.textContent = text;
  ui.overallStatus.className = `status-pill ${pass ? "status-pass" : "status-fail"}`;
}

function showFailure(message) {
  ui.failurePanel.hidden = false;
  ui.failureMessage.textContent = message;
  setOverallStatus("FAIL — deterministic round could not be loaded safely", false);
}

function colourForNumber(number) {
  if (number === 0) return "green";
  if (RED_NUMBERS.has(number)) return "red";
  if (BLACK_NUMBERS.has(number)) return "black";
  return "green";
}

function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}
