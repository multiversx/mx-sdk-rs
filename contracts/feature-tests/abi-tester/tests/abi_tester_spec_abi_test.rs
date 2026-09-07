use std::{fs, fs::File, io::Write};

use abi_tester::{
    abi_enum::AbiEnum, abi_test_type::AbiTestType, only_nested::OnlyShowsUpInConstructor,
};
use multiversx_sc::abi::ContractAbiProvider;
use multiversx_sc_abi_derive::contract_abi;
use multiversx_sc_meta_lib::abi_json::{ContractAbiJson, serialize_abi_to_json};

/// Framework-agnostic ABI description of a subset of the `abi-tester` contract's interface
/// (`contracts/feature-tests/abi-tester/src/abi_tester.rs`'s `init`/`upgrade`/
/// `echo_abi_test_type`/`echo_enum`), written directly against the real `AbiTestType`/`AbiEnum`/
/// `OnlyShowsUpInConstructor` types (all already framework-agnostic — no managed types), with no
/// dependency on `multiversx-sc`/`VMApi`.
///
/// Exercises `#[contract_abi]`'s ABI generation over a richer type surface than the kitty
/// examples: recursive structs (`AbiTestType`), tuples, and a multi-variant enum with both
/// tuple and struct payloads (`AbiEnum`).
#[contract_abi]
pub trait AbiTesterSpec {
    #[init]
    #[payable("EGLD")]
    fn init(&self, constructor_arg_1: i32, constructor_arg_2: OnlyShowsUpInConstructor);

    #[upgrade]
    fn upgrade(&self, constructor_arg_1: i32, constructor_arg_2: OnlyShowsUpInConstructor);

    #[endpoint]
    fn echo_abi_test_type(&self, att: AbiTestType) -> AbiTestType;

    #[endpoint]
    #[only_owner]
    fn echo_enum(&self, e: AbiEnum) -> AbiEnum;
}

#[test]
fn abi_tester_spec_abi_generated_ok() {
    let abi = AbiProvider::abi();
    let abi_json = ContractAbiJson::from(&abi);
    let abi_string = serialize_abi_to_json(&abi_json);

    // save generated ABI to disk for easier comparison in case something is off
    let mut file = File::create("abi_tester_spec_generated.abi.json").unwrap();
    file.write_all(abi_string.as_bytes()).unwrap();

    // load expected from disk & check!
    assert_eq!(
        abi_string,
        fs::read_to_string("./abi_tester_spec_expected.abi.json").unwrap()
    );
}
