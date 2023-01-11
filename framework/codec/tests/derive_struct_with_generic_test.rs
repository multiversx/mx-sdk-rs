use multiversx_sc_codec as codec;
use multiversx_sc_codec_derive::*;

use codec::{
    test_util::{check_dep_encode_decode, check_top_encode_decode},
    *,
};

// to test, run the following command in the crate folder:
// cargo expand --test struct_with_generic_derive_test > expanded.rs

trait SimpleTrait {
    fn simple_function(&self);
}

impl SimpleTrait for u8 {
    fn simple_function(&self) {}
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone, Debug)]
struct StructWithNamedFieldsWithGeneric<ST: SimpleTrait>
where
    ST: NestedEncode + NestedDecode + TopEncode + TopDecode,
{
    data: u64,
    trait_stuff: ST,
}

#[test]
fn struct_with_generic_test() {
    let s_with_gen = StructWithNamedFieldsWithGeneric {
        data: 0xfedcab9876543210,
        trait_stuff: 5u8,
    };

    let mut bytes_2 = [0xfe, 0xdc, 0xab, 0x98, 0x76, 0x54, 0x32, 0x10].to_vec();
    bytes_2.push(5u8);

    check_top_encode_decode(s_with_gen.clone(), bytes_2.as_slice());
    check_dep_encode_decode(s_with_gen, bytes_2.as_slice());
}
