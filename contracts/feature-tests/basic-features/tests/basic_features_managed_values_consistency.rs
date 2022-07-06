use basic_features::{big_num_methods::BigIntMethods, BasicFeatures};
use elrond_wasm::types::{BigUint, TokenIdentifier};
use elrond_wasm_debug::{
    managed_biguint, rust_biguint, testing_framework::BlockchainStateWrapper, DebugApi,
};

const WASM_PATH: &'static str = "output/basic-features.wasm";

#[test]
fn test_managed_values_standalone_consistency() {
    let _ = DebugApi::dummy();

    let mut blockchain_wrapper = BlockchainStateWrapper::new();

    let owner_address = blockchain_wrapper.create_user_account(&rust_biguint!(0u64));
    let basic_features_wrapper = blockchain_wrapper.create_sc_account(
        &rust_biguint!(0u64),
        Some(&owner_address),
        basic_features::contract_obj,
        WASM_PATH,
    );

    let foo = TokenIdentifier::<DebugApi>::from_esdt_bytes(b"FOO-a1a1a1");
    let _ = blockchain_wrapper
        .execute_query(&basic_features_wrapper, |_sc| {
            let _bar = TokenIdentifier::<DebugApi>::from_esdt_bytes(b"BAR-a1a1a1");
            assert_eq!(
                foo,
                TokenIdentifier::<DebugApi>::from_esdt_bytes(b"FOO-a1a1a1")
            );
        })
        .assert_ok();
}

#[test]
fn test_managed_values_argument_and_return_value_consistency() {
    let _ = DebugApi::dummy();

    let mut blockchain_wrapper = BlockchainStateWrapper::new();

    let owner_address = blockchain_wrapper.create_user_account(&rust_biguint!(0u64));
    let basic_features_wrapper = blockchain_wrapper.create_sc_account(
        &rust_biguint!(0u64),
        Some(&owner_address),
        basic_features::contract_obj,
        WASM_PATH,
    );

    let argument = managed_biguint!(42u64);
    let mut result = managed_biguint!(0u64);

    let _ = blockchain_wrapper
        .execute_tx(
            &owner_address,
            &basic_features_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                let dummy: BigUint<DebugApi> = managed_biguint!(100u64);
                assert_eq!(dummy.to_u64().unwrap(), 100);

                assert_eq!(argument.to_u64().unwrap(), 42);
                result = sc.endpoint_with_mutable_arg(argument, 3, 4);
                assert_eq!(result.to_u64().unwrap(), 49);
            },
        )
        .assert_ok();
    assert_eq!(result.to_u64().unwrap(), 49);
}
