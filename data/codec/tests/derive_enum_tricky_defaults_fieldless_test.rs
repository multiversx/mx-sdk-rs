use multiversx_sc_codec as codec;
use multiversx_sc_codec_derive::*;

use codec::test_util::{check_top_decode, check_top_encode, check_top_encode_decode};

// to test, run the following command in the crate folder:
// cargo expand --test enum_tricky_defaults_derive_test > enum_expanded.rs

/// Fieldless enum with default that is not the first variant.
/// NOT recommended!
#[derive(TopEncodeOrDefault, TopDecodeOrDefault, PartialEq, Eq, Clone, Debug)]
enum TrickyDefaultDayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl codec::EncodeDefault for TrickyDefaultDayOfWeek {
    fn is_default(&self) -> bool {
        matches!(self, TrickyDefaultDayOfWeek::Friday)
    }
}

impl codec::DecodeDefault for TrickyDefaultDayOfWeek {
    fn default() -> Self {
        TrickyDefaultDayOfWeek::Friday
    }
}

#[test]
fn fieldless_tricky_defaults() {
    // default
    check_top_encode_decode(TrickyDefaultDayOfWeek::Friday, &[]);

    // so this is the tricky bit, Monday also serializes to `&[]`
    // being variant #0, and because we are serializing fieldless enums as top-level u8.
    // TODO: perhaps add an edge case to the code generation?
    // Not sure if worth it, since this is somewhat of an antipattern.
    assert_eq!(check_top_encode(&TrickyDefaultDayOfWeek::Monday), &[]);

    // we can deserialize it from [0], but this is not what gets serialized
    assert_eq!(TrickyDefaultDayOfWeek::Monday, check_top_decode(&[0]));

    // we can even deserialize it from longer int representations
    // but still not what gets serialized
    assert_eq!(TrickyDefaultDayOfWeek::Monday, check_top_decode(&[0; 13]));

    // unaffected
    check_top_encode_decode(TrickyDefaultDayOfWeek::Tuesday, &[1]);
    check_top_encode_decode(TrickyDefaultDayOfWeek::Wednesday, &[2]);
    check_top_encode_decode(TrickyDefaultDayOfWeek::Thursday, &[3]);
    check_top_encode_decode(TrickyDefaultDayOfWeek::Saturday, &[5]);
    check_top_encode_decode(TrickyDefaultDayOfWeek::Sunday, &[6]);
}
