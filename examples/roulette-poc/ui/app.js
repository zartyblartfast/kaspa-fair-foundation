const ROUND_SEQUENCE = [
  "BetsOpen",
  "SpinVisualStarted",
  "NoMoreBets",
  "ResultFinalised",
  "Settled",
  "ProofPublished",
];

const ROUND_DESCRIPTIONS = {
  BetsOpen: "Bets open. Place mock chips on the table, then start the wheel.",
  SpinVisualStarted: "Wheel spinning — bets still open.",
  NoMoreBets: "No more bets.",
  ResultFinalised: "Result revealed from sample-round.json.",
  Settled: "Settlement shown from sample-round.json.",
  ProofPublished: "Proof snapshot published from toccata-fairness-proof.json.",
};

const ROUND_STATUS_LABELS = {
  BetsOpen: "Bets open",
  SpinVisualStarted: "Wheel spinning — bets still open",
  NoMoreBets: "No more bets",
  ResultFinalised: "Result revealed",
  Settled: "Settlement shown",
  ProofPublished: "Proof published",
};

const BETS_CLOSED_NO_MORE_BETS = "BETS_CLOSED_NO_MORE_BETS";
const NO_MORE_BETS_MESSAGE = "No more bets accepted this round.";
const PLACEABLE_STATES = new Set(["BetsOpen", "SpinVisualStarted"]);
const BET_PLACEMENT_DESCRIPTION = "Click any valid roulette table zone to add a chip: number cells, zero, split/street/corner/six-line selectors, dozens, columns, and outside rectangles all use the same additive bet placement behavior.";
const SPIN_TO_NO_MORE_BETS_MS = 900;
const NO_MORE_BETS_TO_RESULT_MS = 450;
const RESULT_TO_SETTLEMENT_MS = 250;
const SETTLEMENT_TO_PROOF_MS = 250;

const ui = {
  overallStatus: document.getElementById("overall-status"),
  failurePanel: document.getElementById("failure-panel"),
  failureMessage: document.getElementById("failure-message"),
  roundStatusLabel: document.getElementById("round-status-label"),
  stateDescription: document.getElementById("state-description"),
  trustList: document.getElementById("trust-status-list"),
  safetyFlagsList: document.getElementById("safety-flags-list"),
  proofSnapshotList: document.getElementById("proof-snapshot-list"),

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
  resetRoundButton: document.getElementById("reset-round-button"),
};

const appState = {
  round: null,
  tableSchema: null,
  proofArtifact: null,
  uiState: "BetsOpen",
  uiMockBets: [],
  nextMockBetId: 1,
  flowTimers: [],
};

boot().catch((error) => {
  showFailure(`Failed to load deterministic round, roulette table schema, or Toccata proof artifact: ${error.message}`);
});

async function boot() {
  const [round, tableSchema, proofArtifact] = await Promise.all([
    fetchJson("sample-round.json"),
    fetchJson("roulette-table-schema.json"),
    fetchJson("toccata-fairness-proof.json"),
  ]);

  validateRound(round);
  validateTableSchema(tableSchema);
  validateProofArtifact(proofArtifact, round);
  appState.round = round;
  appState.tableSchema = tableSchema;
  appState.proofArtifact = proofArtifact;
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


function validateProofArtifact(proofArtifact, round) {
  const anchor = proofArtifact.live_tn10_anchor || {};
  const safety = proofArtifact.safety_flags || {};
  const transcript = proofArtifact.application_round_transcript || {};
  const reveal = transcript.reveal || {};
  const rustOutput = proofArtifact.rust_verifier_output || {};
  const sourceEnv = proofArtifact.source_env;
  const contract = authorisedProofContracts[sourceEnv];
  const checks = [
    ["proof schema", proofArtifact.schema === "kaspa-fair-roulette-ui-toccata-fairness-proof-v1"],
    ["source_env authorised", Boolean(contract)],
    ["verifier_result == PASS", proofArtifact.verifier_result === "PASS"],
    ["network == testnet-10", isTn10Network(proofArtifact.network)],
    ["evidence_mode == live_readonly_tn10", proofArtifact.evidence_mode === "live_readonly_tn10"],
    ["anchor evidence_mode == live_readonly_tn10", anchor.evidence_mode === "live_readonly_tn10"],
    ["anchor verifier_result == PASS", anchor.verifier_result === "PASS"],
    ["anchor network testnet-10", anchor.network === undefined || isTn10Network(anchor.network)],
    ["covenant_id_confirmed == true", anchor.covenant_id_confirmed === true],
    ["covenant id present", typeof proofArtifact.covenant_id === "string" && proofArtifact.covenant_id.length > 0],
    ["covenant lineage present", typeof proofArtifact.covenant_lineage_reference === "string" && proofArtifact.covenant_lineage_reference.length > 0],
    ["result algorithm present", proofArtifact.result_algorithm === "blake3-domain-separated-rejection-sampling-v1"],
    ["commitment/reveal check PASS", proofArtifact.commitment_reveal_check_status === "PASS"],
    ["deterministic derivation check PASS", proofArtifact.deterministic_derivation_check_status === "PASS"],
    ["result number matches reveal", proofArtifact["result_number"] === reveal["result_number"]],
    ["result colour matches reveal", proofArtifact["result_colour"] === reveal["result_colour"]],
    ["result algorithm matches reveal", proofArtifact["result_algorithm"] === reveal["result_algorithm"]],
    ["sample result number agrees", round["result_number"] === proofArtifact["result_number"]],
    ["sample result colour agrees", round["result_colour"] === proofArtifact["result_colour"]],
    ["sample result algorithm agrees", round["result_algorithm"] === proofArtifact["result_algorithm"]],
    ["Rust verifier checks passed", rustOutput.all_checks_passed === true || rustOutput.verifier_result === "PASS"],
    ["mock_display_only true", safety.mock_display_only === true],
    ["real_betting false", safety.real_betting === false],
    ["real_payouts false", safety.real_payouts === false],
    ["backend_custody false", safety.backend_custody === false],
    ["private_key_access_used false", safety.private_key_access_used === false],
    ["mainnet_supported false", safety.mainnet_supported === false && anchor.mainnet_supported === false],
    ["production_randomness_claimed false", proofArtifact.production_randomness_claimed === false],
    ["no secret-like UI material", !containsSecretLikeUiMaterial(proofArtifact)],
    ["JSON mirror/export only", proofArtifact.json_mirror_export_only === true],
    ["authorised source_env proof contract", contract ? contract(proofArtifact) : false],
  ];

  const failedChecks = checks.filter(([, passed]) => !passed).map(([label]) => label);
  if (failedChecks.length > 0) {
    throw new Error(`Unsafe or failed Toccata proof artifact: ${failedChecks.join(", ")}`);
  }
}

const authorisedProofContracts = {
  "ENV-083E": acceptsStaticFutureProof,
  "ENV-083F": acceptsStaticFutureProof,
  "ENV-087": (proofArtifact) => acceptsLiveProof(proofArtifact, {
    claimLevel: "bare TN10 anchor",
    futureEvidence: "replaced_by_env087_live_bare_tn10_anchor_evidence",
  }),
  "ENV-088": (proofArtifact) => acceptsLiveProof(proofArtifact, {
    claimLevel: "covenant-linked lineage",
    futureEvidence: "replaced_by_env088_covenant_linked_lineage_evidence",
  }),
  "ENV-090": acceptsEnv090Kip17Proof,
};

function acceptsStaticFutureProof(proofArtifact) {
  const liveCommitment = proofArtifact.live_round_commitment_evidence || {};
  const liveReveal = proofArtifact.live_round_reveal_evidence || {};
  const safety = proofArtifact.safety_flags || {};
  return proofArtifact.future_live_round_transaction_evidence === "not_created_not_claimed_future_work" &&
    liveCommitment.status !== "present" &&
    liveReveal.status !== "present" &&
    safety.wallet_access_used === false &&
    safety.signing_used === false &&
    safety.transaction_created === false &&
    safety.broadcast_used === false;
}

function acceptsLiveProof(proofArtifact, { claimLevel, futureEvidence }) {
  const liveCommitment = proofArtifact.live_round_commitment_evidence || {};
  const liveReveal = proofArtifact.live_round_reveal_evidence || {};
  const safety = proofArtifact.safety_flags || {};
  return proofArtifact.claim_level === claimLevel &&
    proofArtifact.future_live_round_transaction_evidence === futureEvidence &&
    liveCommitment.status === "present" &&
    liveReveal.status === "present" &&
    liveCommitment.claim_level === claimLevel &&
    liveReveal.claim_level === claimLevel &&
    isTxid(liveCommitment.transaction_id) &&
    isTxid(liveReveal.transaction_id) &&
    liveReveal.commitment_txid === liveCommitment.transaction_id &&
    safety.wallet_access_used === true &&
    safety.signing_used === true &&
    safety.transaction_created === true &&
    safety.broadcast_used === true;
}

function acceptsEnv090Kip17Proof(proofArtifact) {
  const liveReveal = proofArtifact.live_round_reveal_evidence || {};
  const enforcement = proofArtifact.kip17_enforcement || {};
  return proofArtifact.claim_level === "full_kip17_covenant_enforced_transition" &&
    proofArtifact.env090_superseding_live_round_transaction_evidence === "replaced_by_env090_kip17_covenant_enforced_transition_evidence" &&
    acceptsLiveProof(proofArtifact, {
      claimLevel: "full_kip17_covenant_enforced_transition",
      futureEvidence: "replaced_by_env088_covenant_linked_lineage_evidence",
    }) &&
    liveReveal.kip17_rule_enforced_on_transition === true &&
    enforcement.kip17_rule_enforced_on_transition === true &&
    enforcement.invalid_no_increment_rejected === true &&
    enforcement.kip20_lineage_only_rejected_for_env090_pass === true &&
    enforcement.bare_tn10_anchor_rejected_for_env090_pass === true;
}

function isTn10Network(network) {
  return network === "testnet-10" || network === "TN10";
}

function isTxid(value) {
  return typeof value === "string" && /^[0-9a-f]{64}$/i.test(value);
}

function containsSecretLikeUiMaterial(value) {
  const text = JSON.stringify(value).toUpperCase();
  const secretIndicators = [
    "-----BEGIN ",
    "PRIVATE" + "_KEY=",
    "SECRET" + "_KEY=",
    "MNEM" + "ONIC=",
    "SEED" + "_PHRASE=",
    "KASPA" + "_PRIVATE",
    "API" + "_KEY=",
    "ACCESS" + "_TOKEN=",
  ];
  return secretIndicators.some((indicator) => text.includes(indicator));
}

function bindEvents() {
  ui.startWheelButton.addEventListener("click", startWheelFlow);
  ui.resetRoundButton.addEventListener("click", resetRoundFlow);
}

function startWheelFlow() {
  if (appState.uiState !== "BetsOpen") {
    return;
  }
  clearFlowTimers();
  advanceState("SpinVisualStarted");
  scheduleFlowState("NoMoreBets", SPIN_TO_NO_MORE_BETS_MS);
  scheduleFlowState("ResultFinalised", SPIN_TO_NO_MORE_BETS_MS + NO_MORE_BETS_TO_RESULT_MS);
  scheduleFlowState("Settled", SPIN_TO_NO_MORE_BETS_MS + NO_MORE_BETS_TO_RESULT_MS + RESULT_TO_SETTLEMENT_MS);
  scheduleFlowState("ProofPublished", SPIN_TO_NO_MORE_BETS_MS + NO_MORE_BETS_TO_RESULT_MS + RESULT_TO_SETTLEMENT_MS + SETTLEMENT_TO_PROOF_MS);
}

function scheduleFlowState(nextState, delayMs) {
  const timerId = window.setTimeout(() => advanceState(nextState), delayMs);
  appState.flowTimers.push(timerId);
}

function clearFlowTimers() {
  appState.flowTimers.forEach((timerId) => window.clearTimeout(timerId));
  appState.flowTimers = [];
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
  renderProofSnapshot(appState.proofArtifact);
  renderDeterministicSettlementInput(round);
  ui.resultAlgorithm.textContent = round.result_algorithm;
  setOverallStatus("PASS — sample round, schema-driven SVG roulette table, and static Toccata proof artifact loaded", true);
}

function resetRoundFlow() {
  clearFlowTimers();
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

  ui.roundStatusLabel.textContent = ROUND_STATUS_LABELS[uiState];
  ui.roundStatusLabel.className = `round-status-label ${canPlaceMockBets ? "round-status-open" : "round-status-closed"}`;
  ui.stateDescription.textContent = ROUND_DESCRIPTIONS[uiState];

  ui.startWheelButton.disabled = uiState !== "BetsOpen";
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
    ui.resultColour.textContent = "Hidden until automatic reveal";
  }

  renderRouletteTable(resultVisible ? round.result_number : null, true);
  renderUiMockBetLedger();
  renderSettlement(round, settlementVisible);
  renderProof(round, proofVisible);
}


function renderProofSnapshot(proofArtifact) {
  const safety = proofArtifact.safety_flags;
  const safetySummary = [
    `mock_display_only: ${safety.mock_display_only}`,
    `real_betting: ${safety.real_betting}`,
    `real_payouts: ${safety.real_payouts}`,
    `backend_custody: ${safety.backend_custody}`,
    `wallet_access_used: ${safety.wallet_access_used}`,
    `private_key_access_used: ${safety.private_key_access_used}`,
    `signing_used: ${safety.signing_used}`,
    `transaction_created: ${safety.transaction_created}`,
    `broadcast_used: ${safety.broadcast_used}`,
    `mainnet_supported: ${safety.mainnet_supported}`,
  ].join("; ");

  const rows = [
    ["verifier result", proofArtifact.verifier_result],
    ["source ENV", proofArtifact.source_env],
    ["claim level", proofArtifact.claim_level],
    ["evidence mode", proofArtifact.evidence_mode],
    ["live TN10 anchor evidence mode", proofArtifact.live_tn10_anchor.evidence_mode],
    ["live TN10 anchor verifier result", proofArtifact.live_tn10_anchor.verifier_result],
    ["covenant_id_confirmed", proofArtifact.live_tn10_anchor.covenant_id_confirmed ? "yes" : "no"],
    ["covenant ID", proofArtifact.covenant_id],
    ["covenant lineage reference", proofArtifact.covenant_lineage_reference],
    ["result algorithm", proofArtifact.result_algorithm],
    ["commitment/reveal check status", proofArtifact.commitment_reveal_check_status],
    ["deterministic derivation check status", proofArtifact.deterministic_derivation_check_status],
    ["proof artifact result number", proofArtifact.result_number],
    ["proof artifact result colour", proofArtifact.result_colour],
    ["future live round transaction evidence", proofArtifact.future_live_round_transaction_evidence],
    ["Rust verifier role", "checks this JSON mirror/export; UI is not proof authority"],
    ["safety flags summary", safetySummary],
  ];

  ui.proofSnapshotList.innerHTML = "";
  rows.forEach(([key, value]) => {
    const li = document.createElement("li");
    const valueText = String(value);
    const pass = ["PASS", "live_readonly_tn10", "yes", "ENV-090", "replaced_by_env087_live_bare_tn10_anchor_evidence", "replaced_by_env088_covenant_linked_lineage_evidence", "replaced_by_env090_kip17_covenant_enforced_transition_evidence", "bare TN10 anchor", "covenant-linked lineage", "full covenant transition", "full_kip17_covenant_enforced_transition"].includes(valueText) || valueText.includes("false") || valueText.includes("mock_display_only: true") || valueText.includes("transaction_created: true") || valueText.includes("signing_used: true") || valueText.includes("broadcast_used: true") || valueText.includes("wallet_access_used: true");
    li.innerHTML = `<span class="kv-key">${escapeHtml(String(key))}</span><span class="kv-value ${pass ? "pass" : ""}"><code>${escapeHtml(valueText)}</code></span>`;
    ui.proofSnapshotList.appendChild(li);
  });
}

function renderTrustPanel(round) {
  const rows = [
    ["foundation verifier", round.foundation_verifier_result],
    ["foundation network", round.foundation_network],
    ["round id", round.round_id],
    ["round source state", round.round_state],
    ["roulette mock round source", "sample-round.json"],
    ["app-facing proof artifact source", "toccata-fairness-proof.json"],
    ["roulette table schema source", "roulette-table-schema.json"],
  ];

  ui.trustList.innerHTML = "";
  rows.forEach(([key, value]) => {
    const li = document.createElement("li");
    const isPass = ["PASS", "testnet-10", "sample-round.json", "toccata-fairness-proof.json", "roulette-table-schema.json"].includes(String(value));
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
    ui.settlementSummary.textContent = "Settlement shown automatically after result reveal.";
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
  const proofArtifact = appState.proofArtifact;
  ui.proofList.innerHTML = "";
  if (!visible) {
    ui.proofStatus.textContent = "Proof published automatically after settlement display.";
    ui.proofStatus.className = "proof-status proof-hidden";
    return;
  }

  ui.proofStatus.textContent = `ProofPublished: foundation verifier ${round.foundation_verifier_result}; app-facing verifier ${proofArtifact.verifier_result}; final_result ${round.final_result}.`;
  ui.proofStatus.className = "proof-status proof-pass";

  const proofRows = [
    ["foundation covenant id", round.foundation_covenant_id],
    ["ENV-064 txid", round.foundation_env064_spend_txid],
    ["accepting block hash", round.foundation_accepting_block_hash],
    ["bet ledger hash", round.bet_ledger_hash],
    ["result number", round.result_number],
    ["result colour", round.result_colour],
    ["final_result", round.final_result],
    ["app-facing proof artifact", "toccata-fairness-proof.json"],
    ["app-facing verifier_result", proofArtifact.verifier_result],
    ["app-facing evidence_mode", proofArtifact.evidence_mode],
    ["future live round transaction evidence", proofArtifact.future_live_round_transaction_evidence],
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
    ui.betStatus.textContent = NO_MORE_BETS_MESSAGE;
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
    ? ROUND_STATUS_LABELS.SpinVisualStarted
    : ROUND_STATUS_LABELS.BetsOpen;
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
    return ROUND_STATUS_LABELS.SpinVisualStarted;
  }
  if (uiState === "BetsOpen") {
    return ROUND_STATUS_LABELS.BetsOpen;
  }
  return ROUND_STATUS_LABELS[uiState];
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
  setOverallStatus("FAIL — deterministic round, SVG schema, or Toccata proof artifact could not be loaded safely", false);
}

function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}
