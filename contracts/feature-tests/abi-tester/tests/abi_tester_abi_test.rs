use std::{fs, fs::File, io::Write};

use elrond_wasm_debug::{abi_json, BlockchainMock};

#[test]
fn abi_tester_abi_generated_ok() {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/abi-tester");

    // generate ABI
    let multi_contract_config =
        elrond_wasm_debug::meta::multi_contract_config::<abi_tester::AbiProvider>(
            blockchain
                .current_dir
                .join("multicontract.toml")
                .to_str()
                .unwrap(),
        );

    let main_contract = multi_contract_config.find_contract("abi-tester");
    assert!(!main_contract.external_view);
    let view_contract = multi_contract_config.find_contract("abi-tester-ev");
    assert!(view_contract.external_view);
    assert_eq!(view_contract.endpoint_names(), vec!["external_view",]);

    let main_contract_abi_json = abi_json::abi_to_json_dummy_environment(&main_contract.abi);
    let view_contract_abi_json = abi_json::abi_to_json_dummy_environment(&view_contract.abi);

    // save generated ABI to disk for easier comparison in case something is off
    let mut file = File::create("abi_tester_generated_main.abi.json").unwrap();
    file.write_all(main_contract_abi_json.as_bytes()).unwrap();
    let mut file = File::create("abi_tester_generated_view.abi.json").unwrap();
    file.write_all(view_contract_abi_json.as_bytes()).unwrap();

    // load expected from disk & check!
    assert_eq!(
        main_contract_abi_json,
        fs::read_to_string("./abi_tester_expected_main.abi.json").unwrap()
    );
    assert_eq!(
        view_contract_abi_json,
        fs::read_to_string("./abi_tester_expected_view.abi.json").unwrap()
    );
}

#[test]
fn check_multi_contract_config() {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/abi-tester");

    let multi_contract_config =
        elrond_wasm_debug::meta::multi_contract_config::<abi_tester::AbiProvider>(
            blockchain
                .current_dir
                .join("multicontract.toml")
                .to_str()
                .unwrap(),
        );

    let ev_contract = multi_contract_config.find_contract("abi-tester-ev");
    assert!(ev_contract.external_view);
    assert_eq!(ev_contract.endpoint_names(), vec!["external_view",]);
}
