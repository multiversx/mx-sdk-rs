use core::fmt::Debug;
use elrond_wasm::elrond_codec::test_util::{check_top_decode, check_top_encode};
use elrond_wasm::elrond_codec::*;
// use elrond_wasm::types::BigInt;
use elrond_wasm_debug::api::RustBigUint;
// use elrond_wasm_debug::TxContext;

pub fn check_top_encode_decode<V>(element: V, expected_bytes: &[u8])
where
    V: TopEncode + TopDecode + PartialEq + Debug + 'static,
{
    // serialize
    let serialized_bytes = check_top_encode(&element);
    assert_eq!(serialized_bytes.as_slice(), expected_bytes);

    // deserialize
    let deserialized: V = check_top_decode::<V>(&serialized_bytes[..]);
    assert_eq!(deserialized, element);
}

// TODO: figure out a nice way to run this test
// A codec input/output object with context for testing is required.
// #[test]
// fn test_big_int_serialization() {
//     let api = TxContext::dummy();
//     check_top_encode_decode(BigInt::from_i64(5, api.clone()), &[5u8]);
//     check_top_encode_decode(BigInt::from_i64(-5, api), &[251u8]);
// }

#[test]
fn test_big_uint_serialization() {
    check_top_encode_decode(RustBigUint::from(5u32), &[5u8]);
}
