use multiversx_sc::types::{
    BigInt, BigUint, BoxedBytes, ManagedAddress, ManagedBuffer, ManagedVec,
};
use multiversx_sc_scenario::{api::StaticApi, managed_test_util::check_managed_top_encode_decode};

#[test]
fn test_big_uint_serialization() {
    check_managed_top_encode_decode(BigUint::<StaticApi>::from(5u32), &[5u8]);
}

#[test]
fn test_vec_of_big_uint_serialization() {
    let v = vec![
        BigUint::<StaticApi>::from(5u32),
        BigUint::<StaticApi>::from(6u32),
    ];

    check_managed_top_encode_decode(v, &[0, 0, 0, 1, 5, 0, 0, 0, 1, 6]);
}

#[test]
fn test_big_int_serialization() {
    check_managed_top_encode_decode(BigInt::<StaticApi>::from(5), &[5u8]);
    check_managed_top_encode_decode(BigInt::<StaticApi>::from(-5), &[251u8]);
}

#[test]
fn test_vec_of_big_int_serialization() {
    let v = vec![BigInt::<StaticApi>::from(5), BigInt::<StaticApi>::from(6)];

    check_managed_top_encode_decode(v, &[0, 0, 0, 1, 5, 0, 0, 0, 1, 6]);
}

#[test]
fn test_man_buf_serialization() {
    check_managed_top_encode_decode(
        ManagedBuffer::<StaticApi>::new_from_bytes(&b"abc"[..]),
        &b"abc"[..],
    );
}

#[test]
fn test_vec_of_man_buf_serialization() {
    let v = vec![
        ManagedBuffer::<StaticApi>::new_from_bytes(&b"abc"[..]),
        ManagedBuffer::<StaticApi>::new_from_bytes(&b"de"[..]),
    ];

    check_managed_top_encode_decode(v, &[0, 0, 0, 3, b'a', b'b', b'c', 0, 0, 0, 2, b'd', b'e']);
}

#[test]
fn test_man_address_serialization() {
    let v = ManagedAddress::<StaticApi>::new_from_bytes(&[7u8; 32]);

    check_managed_top_encode_decode(v, &[7u8; 32]);
}

#[test]
fn test_managed_vec_of_man_address_serialization() {
    let mut v = ManagedVec::<StaticApi, ManagedAddress<StaticApi>>::new();
    v.push(ManagedAddress::new_from_bytes(&[7u8; 32]));
    v.push(ManagedAddress::new_from_bytes(&[8u8; 32]));
    v.push(ManagedAddress::new_from_bytes(&[9u8; 32]));

    let expected_v = BoxedBytes::from_concat(&[&[7u8; 32], &[8u8; 32], &[9u8; 32]]);

    check_managed_top_encode_decode(v.clone(), expected_v.as_slice());

    let option = Some(v);
    let expected_opt_v = BoxedBytes::from_concat(&[&[1], &[0, 0, 0, 3], expected_v.as_slice()]);

    check_managed_top_encode_decode(option, expected_opt_v.as_slice());
}
