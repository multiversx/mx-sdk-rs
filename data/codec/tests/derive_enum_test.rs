use multiversx_sc_codec as codec;
use multiversx_sc_codec_derive::*;

use codec::test_util::{check_dep_encode_decode, check_top_decode, check_top_encode_decode};

// to test, run the following command in the crate folder:
// cargo expand --test derive_enum_test > enum_expanded.rs

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

#[test]
fn fieldless_enum() {
    check_top_encode_decode(DayOfWeek::Monday, &[]);
    check_top_encode_decode(DayOfWeek::Tuesday, &[1]);
    check_top_encode_decode(DayOfWeek::Wednesday, &[2]);
    check_top_encode_decode(DayOfWeek::Thursday, &[3]);
    check_top_encode_decode(DayOfWeek::Friday, &[4]);
    check_top_encode_decode(DayOfWeek::Saturday, &[5]);
    check_top_encode_decode(DayOfWeek::Sunday, &[6]);

    check_dep_encode_decode(DayOfWeek::Monday, &[0]);
    check_dep_encode_decode(DayOfWeek::Tuesday, &[1]);
    check_dep_encode_decode(DayOfWeek::Wednesday, &[2]);
    check_dep_encode_decode(DayOfWeek::Thursday, &[3]);
    check_dep_encode_decode(DayOfWeek::Friday, &[4]);
    check_dep_encode_decode(DayOfWeek::Saturday, &[5]);
    check_dep_encode_decode(DayOfWeek::Sunday, &[6]);

    // also allowed
    assert_eq!(DayOfWeek::Monday, check_top_decode(&[0]));
    assert_eq!(DayOfWeek::Monday, check_top_decode(&[0, 0]));
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

#[test]
fn field_enum_zero_value() {
    check_top_encode_decode(EnumWithEverything::Quit, &[]);
    check_dep_encode_decode(EnumWithEverything::Quit, &[0]);
    assert_eq!(EnumWithEverything::Quit, check_top_decode(&[0])); // also allowed
}

#[test]
fn field_enum_variant_with_value() {
    check_top_encode_decode(EnumWithEverything::Today(DayOfWeek::Friday), &[1, 4]);
    check_dep_encode_decode(EnumWithEverything::Today(DayOfWeek::Friday), &[1, 4]);

    let enum_tuple_0 = EnumWithEverything::Write(Vec::new(), 0);
    #[rustfmt::skip]
	let enum_tuple_0_bytes = &[
		/* discriminant */ 2,
		/* vec length */ 0, 0, 0, 0,
		/* u16 */ 0, 0,
	];
    check_top_encode_decode(enum_tuple_0.clone(), enum_tuple_0_bytes);
    check_dep_encode_decode(enum_tuple_0, enum_tuple_0_bytes);
}

#[test]
fn field_enum_variant_with_tuple() {
    let enum_tuple_1 = EnumWithEverything::Write([1, 2, 3].to_vec(), 4);
    #[rustfmt::skip]
	let enum_tuple_1_bytes = &[
		/* discriminant */ 2, 
		/* vec length */ 0, 0, 0, 3,
		/* vec contents */ 1, 2, 3,
		/* an extra 16 */ 0, 4,
	];

    check_top_encode_decode(enum_tuple_1.clone(), enum_tuple_1_bytes);
    check_dep_encode_decode(enum_tuple_1, enum_tuple_1_bytes);
}

#[test]
fn field_enum_struct_variant() {
    let enum_struct = EnumWithEverything::Struct {
        int: 0x42,
        seq: vec![0x1, 0x2, 0x3, 0x4, 0x5],
        another_byte: 0x6,
        uint_32: 0x12345,
        uint_64: 0x123456789,
    };

    #[rustfmt::skip]
	let enum_struct_bytes = &[
		/* discriminant */ 3,
		/* int */ 0, 0x42,
		/* seq length */ 0, 0, 0, 5,
		/* seq contents */ 1, 2, 3, 4, 5,
		/* another_byte */ 6,
		/* uint_32 */ 0x00, 0x01, 0x23, 0x45,
		/* uint_64 */ 0x00, 0x00, 0x00, 0x01, 0x23, 0x45, 0x67, 0x89,
	];

    check_top_encode_decode(enum_struct.clone(), enum_struct_bytes);
    check_dep_encode_decode(enum_struct, enum_struct_bytes);
}
