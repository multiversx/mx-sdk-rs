use std::{fs, fs::File, io::Write};

use elrond_wasm::{abi::EndpointLocationAbi, contract_base::ContractAbiProvider};
use elrond_wasm_debug::abi_json;

#[test]
fn abi_tester_abi_generated_ok() {
    // generate ABI
    let original_contract_abi = <abi_tester::AbiProvider>::abi();
    let main_contract_abi = original_contract_abi.main_contract();
    let view_contract_abi =
        original_contract_abi.secondary_contract(EndpointLocationAbi::ViewContract);

    let main_contract_abi_json = abi_json::abi_to_json_dummy_environment(&main_contract_abi);
    let view_contract_abi_json = abi_json::abi_to_json_dummy_environment(&view_contract_abi);

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
