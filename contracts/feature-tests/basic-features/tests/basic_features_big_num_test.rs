use multiversx_sc::{
    api::BigIntApi,
    types::{BigInt, BigUint, ManagedBuffer, ManagedType},
};
use multiversx_sc_scenario::*;

use basic_features::big_num_methods::BigIntMethods;

#[test]
fn test_big_uint_zero() {
    let _ = DebugApi::dummy();
    let bf = basic_features::contract_obj::<DebugApi>();
    let result = bf.big_uint_zero();
    assert_eq!(BigUint::zero(), result);
}

#[test]
fn test_big_uint_from() {
    let _ = DebugApi::dummy();
    let bf = basic_features::contract_obj::<DebugApi>();
    let result = bf.big_uint_from_u64_1(5);
    assert_eq!(BigUint::from(5u32), result);
    let result = bf.big_uint_from_u64_2(5);
    assert_eq!(BigUint::from(5u32), result);
    let result = bf.big_uint_from_managed_buffer(ManagedBuffer::from(&[5u8]));
    assert_eq!(BigUint::from(5u32), result);
    let result = bf.big_uint_from_managed_buffer_ref(&ManagedBuffer::from(&[5u8]));
    assert_eq!(BigUint::from(5u32), result);
}

#[test]
fn test_big_int_zero() {
    let _ = DebugApi::dummy();
    let bf = basic_features::contract_obj::<DebugApi>();
    let result = bf.big_int_zero();
    assert_eq!(BigInt::zero(), result);
}

#[test]
fn test_big_int_from() {
    let _ = DebugApi::dummy();
    let bf = basic_features::contract_obj::<DebugApi>();
    let result = bf.big_int_from_i64_1(5);
    assert_eq!(BigInt::from(5), result);
    let result = bf.big_int_from_i64_2(6);
    assert_eq!(BigInt::from(6), result);
}

#[test]
fn test_big_int_shr() {
    let api = DebugApi::dummy();
    let bf = basic_features::contract_obj::<DebugApi>();
    // let debug_handle = DebugHandle::new(8i32);
    let big_int = bf.big_int_from_i64_1(128);

    api.bi_shr(big_int.get_handle(), big_int.get_handle(), 3);
    assert_eq!(BigInt::from(16i32), big_int);
}
