use multiversx_sc_codec as codec;
use multiversx_sc_codec_derive::*;

use codec::test_util::{check_top_decode, check_top_encode_decode};

// to test, run the following command in the crate folder:
// cargo expand --test enum_or_default_derive_test > expanded.rs

/// This is a good example of an enum with a useful default.
/// Because the first variant has fields, a default cannot be auto-generated.
/// So we need to provide an explicit default.
#[derive(TopEncodeOrDefault, TopDecodeOrDefault, PartialEq, Eq, Clone, Debug)]
enum EnumWithDefault {
    Basic(i8),
    SomethingElse,
    Struct {
        int: u16,
        seq: Vec<u8>,
        another_byte: u8,
        uint_32: u32,
        uint_64: u64,
    },
}

impl codec::EncodeDefault for EnumWithDefault {
    fn is_default(&self) -> bool {
        *self == EnumWithDefault::Basic(0)
    }
}

impl codec::DecodeDefault for EnumWithDefault {
    fn default() -> Self {
        EnumWithDefault::Basic(0)
    }
}

#[test]
fn enum_defaults() {
    check_top_encode_decode(EnumWithDefault::Basic(0), &[]);
    check_top_encode_decode(EnumWithDefault::Basic(1), &[0, 1]);
    assert_eq!(EnumWithDefault::Basic(0), check_top_decode(&[0, 0])); // also allowed
    check_top_encode_decode(EnumWithDefault::SomethingElse, &[1]);
}

#[test]
fn enum_not_defaults() {
    let enum_struct = EnumWithDefault::Struct {
        int: 0x42,
        seq: vec![0x1, 0x2, 0x3, 0x4, 0x5],
        another_byte: 0x6,
        uint_32: 0x12345,
        uint_64: 0x123456789,
    };

    #[rustfmt::skip]
	let enum_struct_bytes = &[
		/* discriminant */ 2,
		/* int */ 0, 0x42,
		/* seq length */ 0, 0, 0, 5,
		/* seq contents */ 1, 2, 3, 4, 5,
		/* another_byte */ 6,
		/* uint_32 */ 0x00, 0x01, 0x23, 0x45,
		/* uint_64 */ 0x00, 0x00, 0x00, 0x01, 0x23, 0x45, 0x67, 0x89,
	];

    check_top_encode_decode(enum_struct, enum_struct_bytes);
}
