use elrond_wasm::types::{BigInt, BigUint, ManagedFrom};
use elrond_wasm_debug::*;

use basic_features::big_num_methods::BigIntMethods;

#[test]
fn test_big_uint_zero() {
    let context = DebugApi::dummy();
    let bf = basic_features::contract_obj(context.clone());
    let result = bf.big_uint_zero();
    assert_eq!(BigUint::zero(context), result);
}

#[test]
fn test_big_uint_from() {
    let context = DebugApi::dummy();
    let bf = basic_features::contract_obj(context.clone());
    let result = bf.big_uint_from_u64_1(5);
    assert_eq!(BigUint::managed_from(context.clone(), 5u32), result);
    let result = bf.big_uint_from_u64_2(5);
    assert_eq!(BigUint::managed_from(context, 5u32), result);
}

#[test]
fn test_big_int_zero() {
    let context = DebugApi::dummy();
    let bf = basic_features::contract_obj(context.clone());
    let result = bf.big_int_zero();
    assert_eq!(BigInt::zero(context), result);
}

#[test]
fn test_big_int_from() {
    let context = DebugApi::dummy();
    let bf = basic_features::contract_obj(context.clone());
    let result = bf.big_int_from_i64_1(5);
    assert_eq!(BigInt::managed_from(context.clone(), 5), result);
    let result = bf.big_int_from_i64_2(6);
    assert_eq!(BigInt::managed_from(context, 6), result);
}
