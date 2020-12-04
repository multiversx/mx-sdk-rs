extern crate elrond_codec_derive;
use elrond_codec_derive::*;

use elrond_codec::*;

// to test, run the following command in elrond-codec-derive folder:
// cargo expand --test enum_derive_test > expanded.rs

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

#[derive(NestedEncode)]
enum Message {
    Quit,
    Today(DayOfWeek),
    Write(Vec<u8>),
}

/* not supported yet - complex enums
#[derive(TopEncode, TopDecode)]
enum ComplexEnum {
    ComplexField(u8)
}
*/
