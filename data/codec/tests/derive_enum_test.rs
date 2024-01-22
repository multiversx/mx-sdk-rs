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

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone, Debug)]
pub enum Enum256Variants {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Eleven = 11,
    Twelve = 12,
    Thirteen = 13,
    Fourteen = 14,
    Fifteen = 15,
    Sixteen = 16,
    Seventeen = 17,
    Eighteen = 18,
    Nineteen = 19,
    Twenty = 20,
    TwentyOne = 21,
    TwentyTwo = 22,
    TwentyThree = 23,
    TwentyFour = 24,
    TwentyFive = 25,
    TwentySix = 26,
    TwentySeven = 27,
    TwentyEight = 28,
    TwentyNine = 29,
    Thirty = 30,
    ThirtyOne = 31,
    ThirtyTwo = 32,
    ThirtyThree = 33,
    ThirtyFour = 34,
    ThirtyFive = 35,
    ThirtySix = 36,
    ThirtySeven = 37,
    ThirtyEight = 38,
    ThirtyNine = 39,
    Forty = 40,
    FortyOne = 41,
    FortyTwo = 42,
    FortyThree = 43,
    FortyFour = 44,
    FortyFive = 45,
    FortySix = 46,
    FortySeven = 47,
    FortyEight = 48,
    FortyNine = 49,
    Fifty = 50,
    FiftyOne = 51,
    FiftyTwo = 52,
    FiftyThree = 53,
    FiftyFour = 54,
    FiftyFive = 55,
    FiftySix = 56,
    FiftySeven = 57,
    FiftyEight = 58,
    FiftyNine = 59,
    Sixty = 60,
    SixtyOne = 61,
    SixtyTwo = 62,
    SixtyThree = 63,
    SixtyFour = 64,
    SixtyFive = 65,
    SixtySix = 66,
    SixtySeven = 67,
    SixtyEight = 68,
    SixtyNine = 69,
    Seventy = 70,
    SeventyOne = 71,
    SeventyTwo = 72,
    SeventyThree = 73,
    SeventyFour = 74,
    SeventyFive = 75,
    SeventySix = 76,
    SeventySeven = 77,
    SeventyEight = 78,
    SeventyNine = 79,
    Eighty = 80,
    EightyOne = 81,
    EightyTwo = 82,
    EightyThree = 83,
    EightyFour = 84,
    EightyFive = 85,
    EightySix = 86,
    EightySeven = 87,
    EightyEight = 88,
    EightyNine = 89,
    Ninety = 90,
    NinetyOne = 91,
    NinetyTwo = 92,
    NinetyThree = 93,
    NinetyFour = 94,
    NinetyFive = 95,
    NinetySix = 96,
    NinetySeven = 97,
    NinetyEight = 98,
    NinetyNine = 99,
    OneHundred = 100,
    OneHundredOne = 101,
    OneHundredTwo = 102,
    OneHundredThree = 103,
    OneHundredFour = 104,
    OneHundredFive = 105,
    OneHundredSix = 106,
    OneHundredSeven = 107,
    OneHundredEight = 108,
    OneHundredNine = 109,
    OneHundredTen = 110,
    OneHundredEleven = 111,
    OneHundredTwelve = 112,
    OneHundredThirteen = 113,
    OneHundredFourteen = 114,
    OneHundredFifteen = 115,
    OneHundredSixteen = 116,
    OneHundredSeventeen = 117,
    OneHundredEighteen = 118,
    OneHundredNineteen = 119,
    OneHundredTwenty = 120,
    OneHundredTwentyOne = 121,
    OneHundredTwentyTwo = 122,
    OneHundredTwentyThree = 123,
    OneHundredTwentyFour = 124,
    OneHundredTwentyFive = 125,
    OneHundredTwentySix = 126,
    OneHundredTwentySeven = 127,
    OneHundredTwentyEight = 128,
    OneHundredTwentyNine = 129,
    OneHundredThirty = 130,
    OneHundredThirtyOne = 131,
    OneHundredThirtyTwo = 132,
    OneHundredThirtyThree = 133,
    OneHundredThirtyFour = 134,
    OneHundredThirtyFive = 135,
    OneHundredThirtySix = 136,
    OneHundredThirtySeven = 137,
    OneHundredThirtyEight = 138,
    OneHundredThirtyNine = 139,
    OneHundredForty = 140,
    OneHundredFortyOne = 141,
    OneHundredFortyTwo = 142,
    OneHundredFortyThree = 143,
    OneHundredFortyFour = 144,
    OneHundredFortyFive = 145,
    OneHundredFortySix = 146,
    OneHundredFortySeven = 147,
    OneHundredFortyEight = 148,
    OneHundredFortyNine = 149,
    OneHundredFifty = 150,
    OneHundredFiftyOne = 151,
    OneHundredFiftyTwo = 152,
    OneHundredFiftyThree = 153,
    OneHundredFiftyFour = 154,
    OneHundredFiftyFive = 155,
    OneHundredFiftySix = 156,
    OneHundredFiftySeven = 157,
    OneHundredFiftyEight = 158,
    OneHundredFiftyNine = 159,
    OneHundredSixty = 160,
    OneHundredSixtyOne = 161,
    OneHundredSixtyTwo = 162,
    OneHundredSixtyThree = 163,
    OneHundredSixtyFour = 164,
    OneHundredSixtyFive = 165,
    OneHundredSixtySix = 166,
    OneHundredSixtySeven = 167,
    OneHundredSixtyEight = 168,
    OneHundredSixtyNine = 169,
    OneHundredSeventy = 170,
    OneHundredSeventyOne = 171,
    OneHundredSeventyTwo = 172,
    OneHundredSeventyThree = 173,
    OneHundredSeventyFour = 174,
    OneHundredSeventyFive = 175,
    OneHundredSeventySix = 176,
    OneHundredSeventySeven = 177,
    OneHundredSeventyEight = 178,
    OneHundredSeventyNine = 179,
    OneHundredEighty = 180,
    OneHundredEightyOne = 181,
    OneHundredEightyTwo = 182,
    OneHundredEightyThree = 183,
    OneHundredEightyFour = 184,
    OneHundredEightyFive = 185,
    OneHundredEightySix = 186,
    OneHundredEightySeven = 187,
    OneHundredEightyEight = 188,
    OneHundredEightyNine = 189,
    OneHundredNinety = 190,
    OneHundredNinetyOne = 191,
    OneHundredNinetyTwo = 192,
    OneHundredNinetyThree = 193,
    OneHundredNinetyFour = 194,
    OneHundredNinetyFive = 195,
    OneHundredNinetySix = 196,
    OneHundredNinetySeven = 197,
    OneHundredNinetyEight = 198,
    OneHundredNinetyNine = 199,
    TwoHundred = 200,
    TwoHundredOne = 201,
    TwoHundredTwo = 202,
    TwoHundredThree = 203,
    TwoHundredFour = 204,
    TwoHundredFive = 205,
    TwoHundredSix = 206,
    TwoHundredSeven = 207,
    TwoHundredEight = 208,
    TwoHundredNine = 209,
    TwoTen = 210,
    TwoEleven = 211,
    TwoTwelve = 212,
    TwoThirteen = 213,
    TwoFourteen = 214,
    TwoFifteen = 215,
    TwoSixteen = 216,
    TwoSeventeen = 217,
    TwoEighteen = 218,
    TwoNineteen = 219,
    TwoTwenty = 220,
    TwoTwentyOne = 221,
    TwoTwentyTwo = 222,
    TwoTwentyThree = 223,
    TwoTwentyFour = 224,
    TwoTwentyFive = 225,
    TwoTwentySix = 226,
    TwoTwentySeven = 227,
    TwoTwentyEight = 228,
    TwoTwentyNine = 229,
    TwoThirty = 230,
    TwoThirtyOne = 231,
    TwoThirtyTwo = 232,
    TwoThirtyThree = 233,
    TwoThirtyFour = 234,
    TwoThirtyFive = 235,
    TwoThirtySix = 236,
    TwoThirtySeven = 237,
    TwoThirtyEight = 238,
    TwoThirtyNine = 239,
    TwoForty = 240,
    TwoFortyOne = 241,
    TwoFortyTwo = 242,
    TwoFortyThree = 243,
    TwoFortyFour = 244,
    TwoFortyFive = 245,
    TwoFortySix = 246,
    TwoFortySeven = 247,
    TwoFortyEight = 248,
    TwoFortyNine = 249,
    TwoFifty = 250,
    TwoFiftyOne = 251,
    TwoFiftyTwo = 252,
    TwoFiftyThree = 253,
    TwoFiftyFour = 254,
    TwoFiftyFive = 255,
}

#[test]
fn enum_256_variants() {
    check_top_encode_decode(Enum256Variants::Zero, &[][..]);
    check_top_encode_decode(Enum256Variants::One, &[1u8][..]);
    check_dep_encode_decode(Enum256Variants::TwoFiftyFive, &[255u8][..]);
}
