use elrond_wasm::types::{ManagedBuffer, TokenIdentifier};
use elrond_wasm_debug::*;

use basic_features::token_identifier_features::TokenIdentifierFeatures;

#[test]
fn test_token_identifier_egld() {
    let context = DebugApi::dummy();
    let bf = basic_features::contract_obj(context.clone());
    let result = bf.token_identifier_egld();
    assert_eq!(TokenIdentifier::egld(), result);
}

/// This just tests the contract syntax.
/// For a complete suite of test cases, see `elrond-wasm-debug/tests/managed_token_identifier_test.rs`.
#[test]
fn test_token_identifier_is_valid() {
    let context = DebugApi::dummy();
    let bf = basic_features::contract_obj(context.clone());
    let result = bf.token_identifier_is_valid_1(&b"ALC-6258d2"[..]);
    assert!(result);
    let result = bf.token_identifier_is_valid_1(&b"AL-C6258d2"[..]);
    assert!(!result);
    let result = bf.token_identifier_is_valid_2(ManagedBuffer::from(&b"12345-6258d2"[..]));
    assert!(result);
    let result = bf.token_identifier_is_valid_2(ManagedBuffer::from(&b"ALCCCCCCCCC-6258d2"[..]));
    assert!(!result);
}
