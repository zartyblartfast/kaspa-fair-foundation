//! Constants for the proven corrected TN10 covenant path.
//!
//! These values are copied from the committed canonical fixtures under
//! `fixtures/tn10-canonical-covenant-path/`, not from live RPC, wallet files, or
//! the historical lab repository.

/// Canonical recipe id for the corrected ENV-063/064/065 TN10 path.
pub const CORRECTED_TN10_RECIPE_ID: &str = "toccata-v1-keyed-blake3-state-transition";

/// Historical, superseded ENV-060C recipe id. Kept only to label old evidence as
/// non-canonical.
pub const HISTORICAL_ENV060C_RECIPE_ID: &str = "env060c-v0-style-txid-proof-historical";

/// Name used by rusty-kaspa for Toccata transaction version support.
pub const TX_VERSION_TOCCATA_NAME: &str = "TX_VERSION_TOCCATA";

/// Observed Toccata transaction version in the corrected TN10 fixtures.
///
/// TODO: replace this app-owned constant with the official Kaspa crate constant
/// once `kaspa-consensus-core` is integrated as a clean dependency.
pub const TX_VERSION_TOCCATA: u16 = 1;

/// Opcode used by the corrected v1 txid-proof path.
pub const HASH_OPCODE_NAME: &str = "OpBlake3WithKey";

/// Domain key policy used by the corrected v1 path.
pub const DOMAIN_KEY_POLICY: &str =
    "32-byte padded domain keys for TransactionRest, PayloadDigest, and TransactionV1Id";

/// Canonical ENV-064 spend txid accepted on TN10 and later confirmed read-only.
pub const CANONICAL_ENV064_SPEND_TXID: &str =
    "4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c";

/// ENV-063 covenant outpoint spent by the canonical ENV-064 spend.
pub const CANONICAL_ENV063_INPUT_OUTPOINT: &str =
    "2c7802ff9a6eec2828a96168d8f62a9a276176441ed8cb6086cd5d5d0cb26849:0";

/// Continuing covenant output created by the canonical ENV-064 spend.
pub const CANONICAL_CONTINUING_OUTPUT: &str =
    "4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c:0";

/// Continuing output value observed in ENV-065.
pub const CANONICAL_CONTINUING_OUTPUT_VALUE_SOMPI: u64 = 99_700_000;

/// Covenant id preserved through the canonical corrected TN10 transition.
pub const CANONICAL_COVENANT_ID: &str =
    "e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7";
