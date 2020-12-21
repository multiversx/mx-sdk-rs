extern crate abi_tester;
use abi_tester::*;
use elrond_wasm_debug::*;

use std::fs;
use std::fs::File;
use std::io::Write;

#[test]
fn test_abi_generated_ok() {
	// load expected from disk
	let expected_abi_json = fs::read_to_string("./abi_test_expected.abi.json").unwrap();

	// generate ABI
	let contract = AbiTesterImpl::new(TxContext::dummy());
	let contract_abi_json = abi_json::contract_abi(&contract);

	// save generated ABI to disk for easier comparison in case something is off
	let mut file = File::create("abi_test_generated.abi.json").unwrap();
	file.write_all(contract_abi_json.as_bytes()).unwrap();

	// check!
	assert_eq!(contract_abi_json, expected_abi_json);
}
