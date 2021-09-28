use elrond_wasm_debug::*;

use std::{fs, fs::File, io::Write};

#[test]
fn use_module_abi_generated_ok() {
    // load expected from disk
    let expected_abi_json = fs::read_to_string("./use_module_expected.abi.json").unwrap();

    // generate ABI
    let contract_abi_json = abi_json::contract_abi_dummy_environment::<use_module::AbiProvider>();

    // save generated ABI to disk for easier comparison in case something is off
    let mut file = File::create("use_module_generated.abi.json").unwrap();
    file.write_all(contract_abi_json.as_bytes()).unwrap();

    // check!
    assert_eq!(contract_abi_json, expected_abi_json);
}
