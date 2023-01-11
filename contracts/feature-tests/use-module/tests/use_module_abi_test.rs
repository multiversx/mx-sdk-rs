use multiversx_sc_meta::abi_json;
use multiversx_sc_scenario::*;

use std::{fs, fs::File, io::Write};

#[test]
fn use_module_abi_generated_ok() {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/use-module");

    // generate ABI
    let multi_contract_config = multiversx_sc_meta::multi_contract_config::<use_module::AbiProvider>(
        blockchain
            .current_dir()
            .join("multicontract.toml")
            .to_str()
            .unwrap(),
    );

    let main_contract = multi_contract_config.find_contract("use-module");
    assert!(!main_contract.settings.external_view);
    let view_contract = multi_contract_config.find_contract("use-module-view");
    assert!(view_contract.settings.external_view);
    assert_eq!(
        view_contract.endpoint_names(),
        vec!["external_view_mod_a", "external_view_mod_b"]
    );

    let main_contract_abi_json = abi_json::abi_to_json_dummy_environment(&main_contract.abi);
    let view_contract_abi_json = abi_json::abi_to_json_dummy_environment(&view_contract.abi);

    // save generated ABI to disk for easier comparison in case something is off
    let mut file = File::create("use_module_generated_main.abi.json").unwrap();
    file.write_all(main_contract_abi_json.as_bytes()).unwrap();
    let mut file = File::create("use_module_generated_view.abi.json").unwrap();
    file.write_all(view_contract_abi_json.as_bytes()).unwrap();

    // load expected from disk & check!
    assert_eq!(
        main_contract_abi_json,
        fs::read_to_string("./use_module_expected_main.abi.json").unwrap()
    );
    assert_eq!(
        view_contract_abi_json,
        fs::read_to_string("./use_module_expected_view.abi.json").unwrap()
    );
}
