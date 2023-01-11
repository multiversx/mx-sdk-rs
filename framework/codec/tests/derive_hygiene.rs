#![allow(dead_code)]

use multiversx_sc_codec as codec;
use multiversx_sc_codec_derive::*;

// This test doesn't run any code, the fact that it compiles is the actual testing.
// It checks that the derive macro generation is immune to type shadowing,
// i.e. any types that are in scope and happen to have the same name as multiversx-sc-codec types do not break the build.
// The derive macro must generate fully qualified type names everywhere to avoid this hurdle.

// All exported traits:
struct TopEncode;
struct TopDecode;
struct NestedEncode;
struct NestedDecode;
struct EncodeError;
struct DecodeError;
struct TopDecodeInput;
struct TopEncodeOutput;
struct NestedDecodeInput;
struct NestedEncodeOutput;

// Making sure derive explicitly only works with core::result::Result
// and doesn't get tricked by other enums with the same name.
enum Result {
    Ok,
    Err,
}

// This one will interfere with any improperly generated `Ok` and `Err` expressions.
#[allow(unused_imports)]
use crate::Result::{Err, Ok};

// Also adding all public functions exposed by multiversx-sc-codec.
// They are not used in the derive, but just to make sure:
fn top_encode_number() {}
fn universal_decode_number() {}
fn dep_decode_from_byte_slice() {}
fn dep_encode_to_vec() {}
fn top_decode_from_nested_or_handle_err() {}
fn top_decode_from_nested() {}
fn top_encode_from_nested() {}
fn top_encode_to_vec_u8() {}
fn boxed_slice_into_vec() {}
fn vec_into_boxed_slice() {}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone, Debug)]
pub struct Struct {
    pub int: u16,
    pub seq: Vec<u8>,
    pub another_byte: u8,
    pub uint_32: u32,
    pub uint_64: u64,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone, Debug)]
struct TupleStruct(u8, u16, u32);

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Eq, Clone, Debug)]
enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone, Debug)]
enum EnumWithEverything {
    Quit,
    Today(DayOfWeek),
    Write(Vec<u8>, u16),
    Struct {
        int: u16,
        seq: Vec<u8>,
        another_byte: u8,
        uint_32: u32,
        uint_64: u64,
    },
}

trait SimpleTrait {
    fn simple_function(&self);
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone, Debug)]
struct StructWithNamedFieldsWithGeneric<ST: SimpleTrait>
where
    ST: codec::NestedEncode + codec::NestedDecode + codec::TopEncode + codec::TopDecode,
{
    data: u64,
    trait_stuff: ST,
}
