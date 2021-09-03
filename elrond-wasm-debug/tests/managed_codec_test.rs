use elrond_wasm::types::{BigInt, BigUint, BoxedBytes, ManagedAddress, ManagedBuffer, ManagedFrom};
use elrond_wasm_debug::{check_managed_top_encode_decode, TxContext};

#[test]
fn test_big_uint_serialization() {
    let api = TxContext::dummy();

    check_managed_top_encode_decode(
        api.clone(),
        BigUint::managed_from(api.clone(), 5u32),
        &[5u8],
    );
}

#[test]
fn test_vec_of_big_uint_serialization() {
    let api = TxContext::dummy();
    let v = vec![
        BigUint::managed_from(api.clone(), 5u32),
        BigUint::managed_from(api.clone(), 6u32),
    ];

    check_managed_top_encode_decode(api, v, &[0, 0, 0, 1, 5, 0, 0, 0, 1, 6]);
}

#[test]
fn test_big_int_serialization() {
    let api = TxContext::dummy();

    check_managed_top_encode_decode(api.clone(), BigInt::from_i64(api.clone(), 5), &[5u8]);
    check_managed_top_encode_decode(api.clone(), BigInt::from_i64(api, -5), &[251u8]);
}

#[test]
fn test_vec_of_big_int_serialization() {
    let api = TxContext::dummy();
    let v = vec![
        BigInt::from_i32(api.clone(), 5),
        BigInt::from_i32(api.clone(), 6),
    ];

    check_managed_top_encode_decode(api, v, &[0, 0, 0, 1, 5, 0, 0, 0, 1, 6]);
}

#[test]
fn test_man_buf_serialization() {
    let api = TxContext::dummy();

    check_managed_top_encode_decode(
        api.clone(),
        ManagedBuffer::new_from_bytes(api.clone(), &b"abc"[..]),
        &b"abc"[..],
    );
}

#[test]
fn test_vec_of_man_buf_serialization() {
    let api = TxContext::dummy();
    let v = vec![
        ManagedBuffer::new_from_bytes(api.clone(), &b"abc"[..]),
        ManagedBuffer::new_from_bytes(api.clone(), &b"de"[..]),
    ];

    check_managed_top_encode_decode(
        api,
        v,
        &[0, 0, 0, 3, b'a', b'b', b'c', 0, 0, 0, 2, b'd', b'e'],
    );
}

#[test]
fn test_man_address_serialization() {
    let api = TxContext::dummy();
    let v = ManagedAddress::new_from_bytes(api.clone(), &[7u8; 32]);

    check_managed_top_encode_decode(api, v, &[7u8; 32]);
}

#[test]
fn test_vec_of_man_address_serialization() {
    let api = TxContext::dummy();
    let v = vec![
        ManagedAddress::new_from_bytes(api.clone(), &[7u8; 32]),
        ManagedAddress::new_from_bytes(api.clone(), &[8u8; 32]),
        ManagedAddress::new_from_bytes(api.clone(), &[9u8; 32]),
    ];

    let expected = BoxedBytes::from_concat(&[&[7u8; 32], &[8u8; 32], &[9u8; 32]]);

    check_managed_top_encode_decode(api, v, expected.as_slice());
}
