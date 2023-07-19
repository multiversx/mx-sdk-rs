use multiversx_sc_codec as codec;
use multiversx_sc_codec_derive::*;

use codec::{test_util::check_top_encode_decode, *};

#[derive(TopEncodeOrDefault, TopDecodeOrDefault, PartialEq, Eq, Clone, Debug)]
pub struct StructOrDefault<T>
where
    T: NestedEncode + NestedDecode + Default + 'static,
{
    pub int: u16,
    pub seq: Vec<u8>,
    pub another_byte: u8,
    pub uint_32: T,
    pub uint_64: u64,
}

impl<T> codec::EncodeDefault for StructOrDefault<T>
where
    T: NestedEncode + NestedDecode + Default + 'static,
{
    fn is_default(&self) -> bool {
        self.int == 5
    }
}

impl<T> codec::DecodeDefault for StructOrDefault<T>
where
    T: NestedEncode + NestedDecode + Default + 'static,
{
    fn default() -> Self {
        StructOrDefault {
            int: 5,
            seq: vec![],
            another_byte: 0,
            uint_32: T::default(),
            uint_64: 0,
        }
    }
}

#[test]
fn struct_default() {
    let s = StructOrDefault {
        int: 5,
        seq: vec![],
        another_byte: 0,
        uint_32: 0,
        uint_64: 0,
    };

    check_top_encode_decode(s, &[]);
}

#[test]
fn struct_or_default_not_default() {
    let s = StructOrDefault {
        int: 0x42,
        seq: vec![0x1, 0x2, 0x3, 0x4, 0x5],
        another_byte: 0x6,
        uint_32: 0x12345,
        uint_64: 0x123456789,
    };

    #[rustfmt::skip]
	let bytes_1 = &[
		/* int */ 0, 0x42, 
		/* seq length */ 0, 0, 0, 5, 
		/* seq contents */ 1, 2, 3, 4, 5,
		/* another_byte */ 6,
		/* uint_32 */ 0x00, 0x01, 0x23, 0x45,
		/* uint_64 */ 0x00, 0x00, 0x00, 0x01, 0x23, 0x45, 0x67, 0x89,
	];

    check_top_encode_decode(s, bytes_1);
}
