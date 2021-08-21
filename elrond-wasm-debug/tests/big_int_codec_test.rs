use core::fmt::Debug;
use elrond_wasm::{
    elrond_codec::{TopDecode, TopEncode},
    types::{BigInt, BigUint},
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
fn test_big_int_serialization() {
    let api = TxContext::dummy();

    check_managed_top_encode_decode(api.clone(), BigInt::from_i64(5, api.clone()), &[5u8]);
    check_managed_top_encode_decode(api.clone(), BigInt::from_i64(-5, api), &[251u8]);
}

#[test]
fn test_big_uint_serialization() {
    let api = TxContext::dummy();

    check_managed_top_encode_decode(api.clone(), BigUint::from_u32(5u32, api.clone()), &[5u8]);
}
