use multiversx_sc_codec as codec;
use multiversx_sc_codec_derive::*;

use codec::test_util::{check_dep_encode_decode, check_top_encode_decode};

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone, Debug)]
pub struct EmptyStruct1;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone, Debug)]
pub struct EmptyStruct2 {}

#[test]
fn empty_struct_test() {
    check_top_encode_decode(EmptyStruct1, &[]);
    check_dep_encode_decode(EmptyStruct1, &[]);

    check_top_encode_decode(EmptyStruct2 {}, &[]);
    check_dep_encode_decode(EmptyStruct2 {}, &[]);
}
