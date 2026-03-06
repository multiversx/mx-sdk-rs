use std::fs;
use std::process::Command;

use multiversx_sc_meta::cmd::scen_blackbox::{ContractScenarios, TestGenerator};
use multiversx_sc_meta_lib::tools::find_current_workspace;

const PAYABLE_FEATURES_REL_PATH: &str = "contracts/feature-tests/payable-features";
const EXPECTED_FILE_NAME: &str = "payable_features_blackbox_from_scenarios.rs";
const TEMP_FILE_NAME: &str = "payable_features_blackbox_from_scenarios.temp.rs";

const PAYABLE_FEATURES_WORLD_FN: &str = "\
fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());

    blockchain.set_current_dir_from_workspace(\"contracts/feature-tests/payable-features\");
    blockchain.register_contract(
        PAYABLE_FEATURES_CODE_PATH,
        payable_features::ContractBuilder,
    );
    blockchain
}
";

fn load_payable_features_abi(workspace: &std::path::Path) -> multiversx_sc::abi::ContractAbi {
    let abi_path = workspace
        .join(PAYABLE_FEATURES_REL_PATH)
        .join("output")
        .join("payable-features.abi.json");
    let abi_json_str = fs::read_to_string(&abi_path)
        .unwrap_or_else(|_| panic!("Failed to read ABI file: {}", abi_path.display()));
    let abi_json: multiversx_sc_meta::abi_json::ContractAbiJson =
        serde_json::from_str(&abi_json_str).expect("Failed to parse ABI JSON");
    abi_json.into()
}

/// Generates the blackbox test for payable-features and compares the result
/// with the checked-in reference file.
///
/// Run `./run-scen-blackbox.sh` from `framework/meta` to update the reference.
#[test]
fn payable_features_blackbox_from_scenarios_up_to_date() {
    let workspace = find_current_workspace().expect("workspace not found");
    let contract_path = workspace.join(PAYABLE_FEATURES_REL_PATH);
    let meta_path = contract_path.join("meta");

    // Regenerate the ABI so the test doesn't depend on a stale output file.
    let status = Command::new("cargo")
        .args(["run", "abi"])
        .current_dir(&meta_path)
        .status()
        .expect("Failed to run `cargo run abi` in payable-features/meta");
    assert!(
        status.success(),
        "`cargo run abi` in payable-features/meta failed"
    );

    let abi = load_payable_features_abi(&workspace);

    // Generate the raw content as a string.
    let input = ContractScenarios::load(&contract_path)
        .expect("No scenario files found for payable-features");
    let mut generator = TestGenerator::new(input.crate_name, abi.clone());
    generator.override_world_fn(PAYABLE_FEATURES_WORLD_FN);
    // Some(generator.generate_combined_test_content(&input.scenario_files))
    let raw = generator.generate_combined_test_content(&input.scenario_files);

    // Add `#[rustfmt::skip] mod generated { ... }`.
    let mut generated = String::from("#[rustfmt::skip]\nmod generated {\n");
    generated.push_str(&raw);
    generated.push('}');
    generated.push('\n');

    // Write the raw content to a temp file and run rustfmt on it so that the
    // formatting matches the checked-in reference (which was rustfmt-formatted).
    let tmp_path = contract_path.join(TEMP_FILE_NAME);
    fs::write(&tmp_path, &generated).expect("Failed to write temp file");

    // Read the checked-in reference file.
    let reference_path = contract_path.join("tests").join(EXPECTED_FILE_NAME);
    let reference = fs::read_to_string(&reference_path).expect("Reference file not found");

    assert_eq!(
        generated, reference,
        "Generated blackbox test file does not match the checked-in reference."
    );
}
