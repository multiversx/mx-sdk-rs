extern crate elrond_codec_derive;
use elrond_codec_derive::*;

use elrond_codec::*;

// to test, run the following command in elrond-codec-derive folder:
// cargo expand --test enum_derive_test > expanded.rs

#[derive(TopEncode, TopDecode)]
enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}

/* not supported yet - complex enums
#[derive(TopEncode, TopDecode)]
enum ComplexEnum {
    ComplexField(u8)
}
*/
