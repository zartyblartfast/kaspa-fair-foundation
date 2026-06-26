//! Small recipe abstraction for the corrected TN10 covenant path.

use super::constants::{
    CORRECTED_TN10_RECIPE_ID, DOMAIN_KEY_POLICY, HASH_OPCODE_NAME, HISTORICAL_ENV060C_RECIPE_ID,
    TX_VERSION_TOCCATA_NAME,
};

/// Describes a reusable covenant construction recipe without exposing live
/// signing, wallet, RPC, or submit behavior.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CovenantRecipe {
    pub id: &'static str,
    pub network: &'static str,
    pub tx_version_name: &'static str,
    pub hash_opcode_name: &'static str,
    pub domain_key_policy: &'static str,
    pub script_path: &'static str,
    pub canonical: bool,
    pub notes: &'static str,
}

/// Returns the canonical corrected TN10 recipe proven by ENV-063/064/065.
pub const fn corrected_tn10_recipe() -> CovenantRecipe {
    CovenantRecipe {
        id: CORRECTED_TN10_RECIPE_ID,
        network: "TN10/testnet-10",
        tx_version_name: TX_VERSION_TOCCATA_NAME,
        hash_opcode_name: HASH_OPCODE_NAME,
        domain_key_policy: DOMAIN_KEY_POLICY,
        script_path: "corrected v1 covenant script path from ENV-062B/ENV-063/ENV-064",
        canonical: true,
        notes: "Canonical corrected path: TX_VERSION_TOCCATA, OpBlake3WithKey, 32-byte padded domain keys, continuing output preserves the covenant id.",
    }
}

/// Labels the old ENV-060C path as historical evidence only.
pub const fn historical_env060c_recipe() -> CovenantRecipe {
    CovenantRecipe {
        id: HISTORICAL_ENV060C_RECIPE_ID,
        network: "TN10/testnet-10",
        tx_version_name: TX_VERSION_TOCCATA_NAME,
        hash_opcode_name: "OpBlake2bWithKey",
        domain_key_policy: "bare TransactionID blake2b domain string; historical only",
        script_path: "old ENV-060C v0-style txid-proof script path",
        canonical: false,
        notes: "historical, superseded, non-canonical: do not use as the canonical covenant implementation",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corrected_recipe_is_canonical() {
        assert!(corrected_tn10_recipe().canonical);
        assert!(!historical_env060c_recipe().canonical);
    }
}
