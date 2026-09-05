use std::{fs, fs::File, io::Write};

use multiversx_sc::abi::{ContractAbiProvider, imports::*};
use multiversx_sc_abi_derive::contract_abi;
use multiversx_sc_meta_lib::abi_json;

/// Mirrors `contracts/examples/adder/src/adder.rs`, but written directly against pure
/// ABI types instead of managed types, with no dependency on `multiversx-sc`.
#[contract_abi]
pub trait Adder {
    #[view(getSum)]
    fn sum(&self) -> BigUintAbi;

    #[init]
    fn init(&self, initial_value: BigUintAbi);

    #[endpoint]
    fn add(&self, value: BigUintAbi);
}

#[test]
fn contract_abi_generated_ok() {
    let abi = AbiProvider::abi();
    let abi_json = abi_json::abi_to_json_dummy_environment(&abi);

    // save generated ABI to disk for easier comparison in case something is off
    let mut file = File::create("contract_abi_generated.abi.json").unwrap();
    file.write_all(abi_json.as_bytes()).unwrap();

    // load expected from disk & check!
    assert_eq!(
        abi_json,
        fs::read_to_string("./contract_abi_expected.abi.json").unwrap()
    );
}
