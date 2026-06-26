use kaspa_foundation::covenant::constants::{
    CANONICAL_CONTINUING_OUTPUT, CANONICAL_CONTINUING_OUTPUT_VALUE_SOMPI, CANONICAL_COVENANT_ID,
    CANONICAL_ENV063_INPUT_OUTPOINT, CANONICAL_ENV064_SPEND_TXID, CORRECTED_TN10_RECIPE_ID,
    HASH_OPCODE_NAME, HISTORICAL_ENV060C_RECIPE_ID, TX_VERSION_TOCCATA, TX_VERSION_TOCCATA_NAME,
};
use kaspa_foundation::covenant::recipe::{corrected_tn10_recipe, historical_env060c_recipe};
use kaspa_foundation::evidence::canonical_fixtures::canonical_fixture_set;
use kaspa_foundation::safety::default_capabilities;

#[test]
fn corrected_recipe_identity_and_network_are_canonical() {
    let recipe = corrected_tn10_recipe();

    assert_eq!(recipe.id, "toccata-v1-keyed-blake3-state-transition");
    assert_eq!(recipe.id, CORRECTED_TN10_RECIPE_ID);
    assert_eq!(recipe.network, "TN10/testnet-10");
    assert!(recipe.canonical);
}

#[test]
fn corrected_recipe_records_toccata_keyed_blake3_and_domain_policy() {
    let recipe = corrected_tn10_recipe();

    assert_eq!(TX_VERSION_TOCCATA, 1);
    assert_eq!(recipe.tx_version_name, TX_VERSION_TOCCATA_NAME);
    assert_eq!(recipe.hash_opcode_name, HASH_OPCODE_NAME);
    assert!(recipe.hash_opcode_name.contains("OpBlake3WithKey"));
    assert!(recipe
        .domain_key_policy
        .contains("32-byte padded domain keys"));
    assert!(recipe.script_path.contains("corrected v1"));
}

#[test]
fn canonical_env064_fixture_values_are_represented() {
    let fixtures = canonical_fixture_set();

    assert_eq!(
        fixtures.env064_spend_txid,
        "4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c"
    );
    assert_eq!(fixtures.env064_spend_txid, CANONICAL_ENV064_SPEND_TXID);
    assert_eq!(
        fixtures.env063_input_outpoint,
        "2c7802ff9a6eec2828a96168d8f62a9a276176441ed8cb6086cd5d5d0cb26849:0"
    );
    assert_eq!(
        fixtures.env063_input_outpoint,
        CANONICAL_ENV063_INPUT_OUTPOINT
    );
    assert_eq!(
        fixtures.continuing_output,
        "4cb31dbad4465665b978ba3ec5eeecb21824a3ea686f5085b46a97066446466c:0"
    );
    assert_eq!(fixtures.continuing_output, CANONICAL_CONTINUING_OUTPUT);
    assert_eq!(fixtures.continuing_output_value_sompi, 99_700_000);
    assert_eq!(
        fixtures.continuing_output_value_sompi,
        CANONICAL_CONTINUING_OUTPUT_VALUE_SOMPI
    );
    assert_eq!(
        fixtures.covenant_id,
        "e2bdd874add81ebcdba4d0f9ef650967ddadf1085ce4ab15f5eb29fddbf79ff7"
    );
    assert_eq!(fixtures.covenant_id, CANONICAL_COVENANT_ID);
}

#[test]
fn historical_env060c_recipe_is_not_canonical() {
    let recipe = historical_env060c_recipe();

    assert_eq!(recipe.id, HISTORICAL_ENV060C_RECIPE_ID);
    assert!(!recipe.canonical);
    assert!(recipe.notes.contains("historical"));
    assert!(recipe.notes.contains("non-canonical"));
}

#[test]
fn default_foundation_capabilities_do_not_expose_sign_or_submit() {
    let capabilities = default_capabilities();

    assert!(!capabilities.signing_api_exposed);
    assert!(!capabilities.submit_api_exposed);
    assert!(!capabilities.mainnet_enabled);
    assert!(!capabilities.roulette_enabled);
}
