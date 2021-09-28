use std::{fs, fs::File, io::Write};

use elrond_wasm_debug::abi_json;

#[test]
fn abi_tester_abi_generated_ok() {
    // load expected from disk
    let expected_abi_json = fs::read_to_string("./abi_tester_expected.abi.json").unwrap();

    // generate ABI
    let contract_abi_json = abi_json::contract_abi_dummy_environment::<abi_tester::AbiProvider>();

    // save generated ABI to disk for easier comparison in case something is off
    let mut file = File::create("abi_tester_generated.abi.json").unwrap();
    file.write_all(contract_abi_json.as_bytes()).unwrap();

    // check!
    assert_eq!(contract_abi_json, expected_abi_json);
}
