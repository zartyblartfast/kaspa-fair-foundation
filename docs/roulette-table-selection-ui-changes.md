# Roulette Table Selection UI Changes

This note summarizes the roulette table selection UI changes made in this phase, in order of importance.

## 1. Unified bet placement behavior

All valid roulette table bet areas now use one consistent interaction model: clicking a valid zone adds a bet chip.

This applies to:

- straight number cells, including zero
- split selectors
- street selectors
- corner selectors
- six-line selectors
- dozen bets
- column bets
- outside bets

The previous separate bet-type mode buttons were removed because they created inconsistent behavior and made split/street/corner/six-line bets feel like UI modes instead of normal roulette bets.

## 2. Multiple bets are supported

Bet placement is now additive. New selections no longer replace or deselect previous selections.

Multiple chips can be placed across different bet types, and repeated bets on the same selector/tile are preserved. Same-location chips are slightly offset so repeated bets remain visible instead of appearing as a single chip.

## 3. Chips are the only selection indicator

The large temporary selector/focus highlight was removed. A placed chip is now the selection indicator for every bet type.

Circle selectors remain compact clickable targets, but clicking them no longer leaves a large doughnut/outline selection marker behind. Focus styling is suppressed after placement so the table does not show an extra selection artifact before or after the chip appears.

## 4. Roulette table position remains stable

The UI mock bet ledger above the table now uses a fixed-height scroll area. Adding bets no longer expands that section and pushes the roulette table downward.

This prevents vertical jumps in the table position while bets are being placed.

## 5. Top selector clipping fixed

The SVG viewBox was given additional top padding so the upper row of street selector circles is not clipped at the top of the table area.

## 6. Regression checks updated

The roulette SVG smoke script now checks for the key UI contracts from this phase:

- no bet-type selector buttons
- no active/passive hotspot mode state
- no large focus/selection highlight styling
- chip stacking for repeated same-location bets
- chips do not intercept further table clicks
- fixed-height ledger scroll area
- additional SVG top padding for street selectors
