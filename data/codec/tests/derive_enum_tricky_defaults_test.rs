use multiversx_sc_codec as codec;
use multiversx_sc_codec_derive::*;

use codec::test_util::{check_top_decode, check_top_encode, check_top_encode_decode};

// to test, run the following command in the crate folder:
// cargo expand --test derive_enum_tricky_defaults_test > enum_expanded.rs

/// Enum with default that is not the first variant.
/// Not fieldless, the version with fields.
/// NOT recommended!
#[derive(TopEncodeOrDefault, TopDecodeOrDefault, PartialEq, Eq, Clone, Debug)]
enum TrickyEnumWithDefault {
    FirstVariant,
    SecondVariant,
    VariantWithFields {
        int: u16,
        seq: Vec<u8>,
        another_byte: u8,
        uint_32: u32,
        uint_64: u64,
    },
}

impl codec::EncodeDefault for TrickyEnumWithDefault {
    fn is_default(&self) -> bool {
        matches!(self, TrickyEnumWithDefault::SecondVariant)
    }
}

impl codec::DecodeDefault for TrickyEnumWithDefault {
    fn default() -> Self {
        TrickyEnumWithDefault::SecondVariant
    }
}

#[test]
fn tricky_enum_defaults() {
    // the default
    check_top_encode_decode(TrickyEnumWithDefault::SecondVariant, &[]);

    // so this is the tricky bit, FirstVariant also serializes to `&[]`
    // being variant #0, and because we are serializing fieldless enums as top-level u8.
    // TODO: perhaps add an edge case to the code generation?
    // Not sure if worth it, since this is somewhat of an antipattern.
    assert_eq!(check_top_encode(&TrickyEnumWithDefault::FirstVariant), &[]);

    // we can deserialize it from [0], but this is not what gets serialized
    // unlike the fieldless enum, only precisely one "0" byte allowed
    assert_eq!(TrickyEnumWithDefault::FirstVariant, check_top_decode(&[0]));
}
