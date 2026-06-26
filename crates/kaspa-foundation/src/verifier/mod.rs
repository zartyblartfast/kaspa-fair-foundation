//! Lightweight offline verifier placeholders.
//!
//! ENV-068 does not integrate the official Kaspa VM crates yet. This module is
//! intentionally limited to deterministic metadata/fixture checks.

use crate::covenant::recipe::corrected_tn10_recipe;
use crate::evidence::canonical_fixtures::fixture_values_are_internally_consistent;
use crate::safety::default_capabilities;

/// Returns true when the canonical recipe and fixture constants describe the
/// safe offline corrected TN10 path extracted in ENV-068.
pub fn canonical_recipe_sanity_passes() -> bool {
    let recipe = corrected_tn10_recipe();
    let capabilities = default_capabilities();
    recipe.canonical
        && recipe.network == "TN10/testnet-10"
        && fixture_values_are_internally_consistent()
        && !capabilities.signing_api_exposed
        && !capabilities.submit_api_exposed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_sanity_passes() {
        assert!(canonical_recipe_sanity_passes());
    }
}
