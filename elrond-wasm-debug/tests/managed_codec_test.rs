use core::fmt::Debug;
use elrond_wasm::{
    elrond_codec::{TopDecode, TopEncode},
    types::{BigInt, BigUint, ManagedBuffer},
};
use elrond_wasm_debug::{check_managed_top_decode, check_managed_top_encode, TxContext};

pub fn check_managed_top_encode_decode<V>(api: TxContext, element: V, expected_bytes: &[u8])
where
    V: TopEncode + TopDecode + PartialEq + Debug + 'static,
{
    // serialize
    let serialized_bytes = check_managed_top_encode(api.clone(), &element);
    assert_eq!(serialized_bytes.as_slice(), expected_bytes);

    // deserialize
    let deserialized: V = check_managed_top_decode::<V>(api, serialized_bytes.as_slice());
    assert_eq!(deserialized, element);
}

#[test]
fn test_big_uint_serialization() {
    let api = TxContext::dummy();

    check_managed_top_encode_decode(api.clone(), BigUint::from_u32(api.clone(), 5u32), &[5u8]);
}

#[test]
fn test_big_uint_vec_serialization() {
    let api = TxContext::dummy();
    let v = vec![
        BigUint::from_u32(api.clone(), 5u32),
        BigUint::from_u32(api.clone(), 6u32),
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
fn test_big_int_vec_serialization() {
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
fn test_man_buf_vec_serialization() {
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
