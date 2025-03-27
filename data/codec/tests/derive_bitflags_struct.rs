use bitflags::bitflags;
use multiversx_sc_codec::{
    self as codec,
    test_util::{check_dep_encode_decode, check_top_encode_decode},
};
use multiversx_sc_codec_derive::*;

bitflags! {
    #[derive(NestedEncode, NestedDecode, TopDecode, TopEncode, PartialEq, Eq, Clone, Debug)]
    struct BitFlagsTest: u16 {
        const FLAG_1 = 0b00000001;
        const FLAG_2 = 0b00000010;
        const FLAG_3 = 0b00000100;
        const FLAG_4 = 0b00001000;
        const FLAG_5 = 0b00010000;
        const FLAG_6 = 0b00100000;
    }

}

// to test, run the following command in the crate folder:
// cargo expand --test derive_bitflags_struct > expanded.rs

#[test]
fn bitflags_struct_derive_test() {
    let s = BitFlagsTest::FLAG_1 | BitFlagsTest::FLAG_3 | BitFlagsTest::FLAG_5;

    #[rustfmt::skip]
    let bytes: &[u8] = &[0, 21];

    check_top_encode_decode(s.clone(), bytes);
    check_dep_encode_decode(s, bytes);
}
