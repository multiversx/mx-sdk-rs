use multiversx_sc::types::{BigInt, BigUint};
use multiversx_sc_scenario::DebugApi;
use multiversx_sdk::data::types::native::NativeConvertible;

#[test]
fn test_biguint_to_native() {
    let _ = DebugApi::dummy();
    let biguint: BigUint<DebugApi> = BigUint::from(10u64).pow(18);
    let native = biguint.to_native();

    let expected = num_bigint::BigUint::from(10u64).pow(18);

    assert_eq!(
        native,
        expected
    )
}

#[test]
fn test_bigint_to_native() {
    let _ = DebugApi::dummy();
    let bigint: BigInt<DebugApi> = BigInt::from(10i64).pow(18);
    let native = bigint.to_native();

    let expected = num_bigint::BigInt::from(10i64).pow(18);

    assert_eq!(
        native,
        expected
    )
}

#[test]
fn test_negative_bigint_to_native() {
    let _ = DebugApi::dummy();
    let bigint: BigInt<DebugApi> = BigInt::from(-10i64).pow(18);
    let native = bigint.to_native();

    let expected = num_bigint::BigInt::from(-10i64).pow(18);

    assert_eq!(
        native,
        expected
    )
}