//! Canonical fixture values from ENV-063/064/065.

use crate::covenant::constants::{
    CANONICAL_CONTINUING_OUTPUT, CANONICAL_CONTINUING_OUTPUT_VALUE_SOMPI, CANONICAL_COVENANT_ID,
    CANONICAL_ENV063_INPUT_OUTPOINT, CANONICAL_ENV064_SPEND_TXID,
};

/// Minimal set of canonical values needed by offline tests and proof transcript
/// scaffolding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanonicalFixtureSet {
    pub env064_spend_txid: &'static str,
    pub env063_input_outpoint: &'static str,
    pub continuing_output: &'static str,
    pub continuing_output_value_sompi: u64,
    pub covenant_id: &'static str,
}

/// Returns the canonical corrected TN10 ENV-063/064/065 fixture values.
pub const fn canonical_fixture_set() -> CanonicalFixtureSet {
    CanonicalFixtureSet {
        env064_spend_txid: CANONICAL_ENV064_SPEND_TXID,
        env063_input_outpoint: CANONICAL_ENV063_INPUT_OUTPOINT,
        continuing_output: CANONICAL_CONTINUING_OUTPUT,
        continuing_output_value_sompi: CANONICAL_CONTINUING_OUTPUT_VALUE_SOMPI,
        covenant_id: CANONICAL_COVENANT_ID,
    }
}

/// Performs cheap internal consistency checks over the canonical values.
pub fn fixture_values_are_internally_consistent() -> bool {
    let fixtures = canonical_fixture_set();
    fixtures.continuing_output == format!("{}:0", fixtures.env064_spend_txid)
        && fixtures.continuing_output_value_sompi == 99_700_000
        && fixtures.env064_spend_txid.len() == 64
        && fixtures.covenant_id.len() == 64
        && fixtures.env063_input_outpoint.ends_with(":0")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_values_are_consistent() {
        assert!(fixture_values_are_internally_consistent());
    }
}
