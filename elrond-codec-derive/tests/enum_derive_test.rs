extern crate elrond_codec_derive;
use elrond_codec_derive::*;

use elrond_codec::test_util::{check_dep_encode_decode, check_top_decode, check_top_encode_decode};
use elrond_codec::*;

// to test, run the following command in elrond-codec-derive folder:
// cargo expand --test enum_derive_test > enum_expanded.rs

#[derive(PartialEq, Debug, TopEncode, TopDecode, NestedEncode, NestedDecode)]
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
fn fieldless_enum_test() {
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

#[derive(PartialEq, Debug, NestedEncode, NestedDecode, TopEncode)]
enum Message {
	Quit,
	Today(DayOfWeek),
	Write(Vec<u8>),
}

#[test]
fn field_enum_test() {
	check_dep_encode_decode(Message::Quit, &[0]);
	check_dep_encode_decode(Message::Today(DayOfWeek::Friday), &[1, 4]);
	check_dep_encode_decode(Message::Write(Vec::new()), &[2, /*vec length */ 0, 0, 0, 0]);
	#[rustfmt::skip]
	check_dep_encode_decode(
		Message::Write([1, 2, 3].to_vec()),
		&[
			/* discriminant */ 2, 
			/* vec length */ 0, 0, 0, 3,
			/* vec contents */ 1, 2, 3,
		],
	);
}

/* not supported yet - complex enums
#[derive(TopEncode, TopDecode)]
enum ComplexEnum {
	ComplexField(u8)
}
*/
