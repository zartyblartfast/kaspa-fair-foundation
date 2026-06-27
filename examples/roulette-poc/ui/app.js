const ROUND_SEQUENCE = [
  "Created",
  "BetsOpen",
  "SpinVisualStarted",
  "NoMoreBets",
  "ResultFinalised",
  "Settled",
  "ProofPublished",
];

const RED_NUMBERS = new Set([1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36]);
const BLACK_NUMBERS = new Set([2, 4, 6, 8, 10, 11, 13, 15, 17, 20, 22, 24, 26, 28, 29, 31, 33, 35]);

const ui = {
  overallStatus: document.getElementById("overall-status"),
  failurePanel: document.getElementById("failure-panel"),
  failureMessage: document.getElementById("failure-message"),
  trustList: document.getElementById("trust-status-list"),
  sequenceList: document.getElementById("sequence-list"),
  rouletteTable: document.getElementById("roulette-table"),
  resultNumber: document.getElementById("result-number"),
  resultColour: document.getElementById("result-colour"),
  resultAlgorithm: document.getElementById("result-algorithm"),
  betsBody: document.getElementById("bets-body"),
  proofList: document.getElementById("proof-list"),
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
  render(round);
}

function render(round) {
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
  ];

  const failedChecks = checks.filter(([, passed]) => !passed).map(([label]) => label);
  const safe = failedChecks.length === 0;

  setOverallStatus(safe ? "PASS — deterministic proof and safety checks hold" : "FAIL — unsafe or failed round JSON", safe);
  if (!safe) {
    showFailure(`Unsafe or failed JSON: ${failedChecks.join(", ")}`);
  }

  renderTrustPanel(round, checks);
  renderRoundSequence(round.round_state);
  renderRouletteTable(round.result_number);
  renderResult(round);
  renderBets(round);
  renderProof(round);
}

function setOverallStatus(text, pass) {
  ui.overallStatus.textContent = text;
  ui.overallStatus.className = `status-pill ${pass ? "status-pass" : "status-fail"}`;
}

function showFailure(message) {
  ui.failurePanel.hidden = false;
  ui.failureMessage.textContent = message;
}

function renderTrustPanel(round, checks) {
  const rows = [
    ["TOCCATA_LAYER_READY / foundation verifier PASS", round.foundation_verifier_result],
    ["network", round.foundation_network],
    ["mainnet_supported", String(round.mainnet_supported)],
    ["readonly", String(round.foundation_readonly)],
    ["signing_used", String(round.signing_used)],
    ["transaction_created", String(round.transaction_created)],
    ["broadcast_used", String(round.broadcast_used)],
    ["wallet_access_used", String(round.wallet_access_used)],
  ];

  ui.trustList.innerHTML = "";
  rows.forEach(([key, value], index) => {
    const li = document.createElement("li");
    const isPass = checks[index]?.[1] ?? true;
    li.innerHTML = `<span class="kv-key">${escapeHtml(key)}</span><span class="kv-value ${isPass ? "pass" : "fail"}">${escapeHtml(String(value))}</span>`;
    ui.trustList.appendChild(li);
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

function renderRouletteTable(resultNumber) {
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

function renderResult(round) {
  ui.resultNumber.textContent = String(round.result_number);
  ui.resultNumber.className = `result-number ${round.result_colour}`;
  ui.resultColour.textContent = round.result_colour;
  ui.resultAlgorithm.textContent = round.result_algorithm;
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

function renderProof(round) {
  const proofRows = [
    ["foundation covenant id", round.foundation_covenant_id],
    ["ENV-064 txid", round.foundation_env064_spend_txid],
    ["accepting block hash", round.foundation_accepting_block_hash],
    ["bet ledger hash", round.bet_ledger_hash],
    ["final_result", round.final_result],
  ];

  ui.proofList.innerHTML = "";
  proofRows.forEach(([key, value]) => {
    const li = document.createElement("li");
    li.innerHTML = `<span class="kv-key">${escapeHtml(key)}</span><span class="kv-value"><code>${escapeHtml(String(value))}</code></span>`;
    ui.proofList.appendChild(li);
  });
}

function colourForNumber(number) {
  if (number === 0) return "green";
  if (RED_NUMBERS.has(number)) return "red";
  if (BLACK_NUMBERS.has(number)) return "black";
  return "green";
}

function escapeHtml(value) {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}
