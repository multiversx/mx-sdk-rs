use crate::*;
use alloc::vec::Vec;
use core::fmt::Debug;

/// Calls `top_encode_or_exit` and panics if an encoding error occurs.
/// Do not use in smart contracts!
pub fn top_encode_to_vec_u8_or_panic<T: TopEncode>(obj: &T) -> Vec<u8> {
    let mut bytes = Vec::<u8>::new();
    obj.top_encode_or_exit(&mut bytes, (), encode_panic_exit);
    bytes
}

/// Calls `dep_encode_or_exit` and panics if an encoding error occurs.
/// Do not use in smart contracts!
pub fn dep_encode_to_vec_or_panic<T: NestedEncode>(obj: &T) -> Vec<u8> {
    let mut bytes = Vec::<u8>::new();
    obj.dep_encode_or_exit(&mut bytes, (), encode_panic_exit);
    bytes
}

fn encode_panic_exit(_: (), en_err: EncodeError) -> ! {
    panic!(
        "encode panicked: {}",
        core::str::from_utf8(en_err.message_bytes()).unwrap()
    )
}

/// Calls both the fast exit and the regular top-encode,
/// compares that the outputs are equal, then returns the result.
/// To be used in serialization tests.
pub fn check_top_encode<T: TopEncode>(obj: &T) -> Vec<u8> {
    let fast_exit_bytes = top_encode_to_vec_u8_or_panic(obj);
    let result_bytes = top_encode_to_vec_u8(obj).unwrap();
    assert_eq!(fast_exit_bytes, result_bytes);
    fast_exit_bytes
}

/// Calls both the fast exit and the regular dep-encode,
/// compares that the outputs are equal, then returns the result.
/// To be used in serialization tests.
pub fn check_dep_encode<T: NestedEncode>(obj: &T) -> Vec<u8> {
    let fast_exit_bytes = dep_encode_to_vec_or_panic(obj);
    let result_bytes = dep_encode_to_vec(obj).unwrap();
    assert_eq!(fast_exit_bytes, result_bytes);
    fast_exit_bytes
}

/// Calls `top_decode_or_exit` and panics if an encoding error occurs.
/// Do not use in smart contracts!
pub fn top_decode_from_byte_slice_or_panic<T: TopDecode>(input: &[u8]) -> T {
    T::top_decode_or_exit(input, (), decode_panic_exit)
}

/// Calls `dep_decode_or_exit` and panics if an encoding error occurs.
/// Do not use in smart contracts!
pub fn dep_decode_from_byte_slice_or_panic<T: NestedDecode>(input: &[u8]) -> T {
    dep_decode_from_byte_slice_or_exit(input, (), decode_panic_exit)
}

fn decode_panic_exit(_: (), de_err: DecodeError) -> ! {
    panic!(
        "decode panicked: {}",
        core::str::from_utf8(de_err.message_bytes()).unwrap()
    )
}

/// Calls both the fast exit and the regular top-decode,
/// compares that the outputs are equal, then returns the result.
/// To be used in serialization tests.
pub fn check_top_decode<T: TopDecode + PartialEq + Debug>(bytes: &[u8]) -> T {
    let fast_exit_obj = top_decode_from_byte_slice_or_panic(bytes);
    let result_obj = T::top_decode(bytes).unwrap();
    assert_eq!(fast_exit_obj, result_obj);
    fast_exit_obj
}

/// Calls both the fast exit and the regular dep-decode,
/// compares that the outputs are equal, then returns the result.
/// To be used in serialization tests.
pub fn check_dep_decode<T: NestedDecode + PartialEq + Debug>(bytes: &[u8]) -> T {
    let fast_exit_obj = dep_decode_from_byte_slice_or_panic(bytes);
    let result_obj = dep_decode_from_byte_slice::<T>(bytes).unwrap();
    assert_eq!(fast_exit_obj, result_obj);
    fast_exit_obj
}

/// backwards compatibility only, will remove in next major release
#[deprecated]
pub fn ser_deser_ok<V>(element: V, expected_bytes: &[u8])
where
    V: TopEncode + TopDecode + PartialEq + Debug,
{
    check_top_encode_decode(element, expected_bytes);
}

pub fn check_top_encode_decode<V>(element: V, expected_bytes: &[u8])
where
    V: TopEncode + TopDecode + PartialEq + Debug,
{
    // serialize
    let serialized_bytes = check_top_encode(&element);
    assert_eq!(serialized_bytes.as_slice(), expected_bytes);

    // deserialize
    let deserialized: V = check_top_decode::<V>(&serialized_bytes[..]);
    assert_eq!(deserialized, element);
}

pub fn check_dep_encode_decode<V>(element: V, expected_bytes: &[u8])
where
    V: NestedEncode + NestedDecode + PartialEq + Debug,
{
    // serialize
    let serialized_bytes = check_dep_encode(&element);
    assert_eq!(serialized_bytes.as_slice(), expected_bytes);

    // deserialize
    let deserialized: V = check_dep_decode::<V>(&serialized_bytes[..]);
    assert_eq!(deserialized, element);
}
