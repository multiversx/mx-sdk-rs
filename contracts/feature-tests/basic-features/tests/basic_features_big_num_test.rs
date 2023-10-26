use multiversx_sc::types::{BigInt, BaseBigUint, ManagedBuffer};

use basic_features::{big_num_methods::BigIntMethods, big_num_operators::BigIntOperators};

#[test]
#[cfg_attr(not(feature = "static-api"), ignore)]
fn test_big_uint_zero() {
    let bf = basic_features::contract_obj();
    let result = bf.big_uint_zero();
    assert_eq!(BaseBigUint::zero(), result);
}

#[test]
#[cfg_attr(not(feature = "static-api"), ignore)]
fn test_big_uint_from() {
    let bf = basic_features::contract_obj();
    let result = bf.big_uint_from_u64_1(5);
    assert_eq!(BaseBigUint::from(5u32), result);
    let result = bf.big_uint_from_u64_2(5);
    assert_eq!(BaseBigUint::from(5u32), result);
    let result = bf.big_uint_from_managed_buffer(ManagedBuffer::from(&[5u8]));
    assert_eq!(BaseBigUint::from(5u32), result);
    let result = bf.big_uint_from_managed_buffer_ref(&ManagedBuffer::from(&[5u8]));
    assert_eq!(BaseBigUint::from(5u32), result);
}

#[test]
#[cfg_attr(not(feature = "static-api"), ignore)]
fn test_big_int_zero() {
    let bf = basic_features::contract_obj();
    let result = bf.big_int_zero();
    assert_eq!(BigInt::zero(), result);
}

#[test]
#[cfg_attr(not(feature = "static-api"), ignore)]
fn test_big_int_from() {
    let bf = basic_features::contract_obj();
    let result = bf.big_int_from_i64_1(5);
    assert_eq!(BigInt::from(5), result);
    let result = bf.big_int_from_i64_2(6);
    assert_eq!(BigInt::from(6), result);
}

fn check_big_uint_bitwise_and(a: u64, b: u64) {
    let bf = basic_features::contract_obj();
    let result = bf.bit_and_big_uint(BaseBigUint::from(a), BaseBigUint::from(b));
    assert_eq!(BaseBigUint::from(a & b), result);
}

#[test]
#[cfg_attr(not(feature = "static-api"), ignore)]
fn test_big_uint_bitwise_and() {
    check_big_uint_bitwise_and(1, 1);
    check_big_uint_bitwise_and(5, 7);
    check_big_uint_bitwise_and(0, 1023);
    check_big_uint_bitwise_and(0, 0);
}

fn check_big_uint_shift(a: u64, b: usize) {
    let bf = basic_features::contract_obj();
    let result = bf.shl_big_uint(BaseBigUint::from(a), b);
    assert_eq!(BaseBigUint::from(a << b), result);
    let result = bf.shr_big_uint(BaseBigUint::from(a), b);
    assert_eq!(BaseBigUint::from(a >> b), result);
}

#[test]
#[cfg_attr(not(feature = "static-api"), ignore)]
fn test_big_uint_bitwise_shift() {
    check_big_uint_shift(1, 3);
    check_big_uint_shift(256, 0);
    check_big_uint_shift(1023, 5);
    check_big_uint_shift(0, 10);
}
