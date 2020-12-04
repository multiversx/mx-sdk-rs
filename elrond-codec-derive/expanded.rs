#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
extern crate elrond_codec_derive;
use elrond_codec_derive::*;
use elrond_codec::*;
enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
impl TopEncode for DayOfWeek {
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        match self {
            DayOfWeek::Monday => 0u8.top_encode(output),
            DayOfWeek::Tuesday => 1u8.top_encode(output),
            DayOfWeek::Wednesday => 2u8.top_encode(output),
            DayOfWeek::Thursday => 3u8.top_encode(output),
            DayOfWeek::Friday => 4u8.top_encode(output),
            DayOfWeek::Saturday => 5u8.top_encode(output),
            DayOfWeek::Sunday => 6u8.top_encode(output),
        }
    }
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        match self {
            DayOfWeek::Monday => 0u8.top_encode_or_exit(output, c, exit),
            DayOfWeek::Tuesday => 1u8.top_encode_or_exit(output, c, exit),
            DayOfWeek::Wednesday => 2u8.top_encode_or_exit(output, c, exit),
            DayOfWeek::Thursday => 3u8.top_encode_or_exit(output, c, exit),
            DayOfWeek::Friday => 4u8.top_encode_or_exit(output, c, exit),
            DayOfWeek::Saturday => 5u8.top_encode_or_exit(output, c, exit),
            DayOfWeek::Sunday => 6u8.top_encode_or_exit(output, c, exit),
        }
    }
}
impl TopDecode for DayOfWeek {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        match u8::top_decode(input)? {
            0u8 => core::result::Result::Ok(DayOfWeek::Monday),
            1u8 => core::result::Result::Ok(DayOfWeek::Tuesday),
            2u8 => core::result::Result::Ok(DayOfWeek::Wednesday),
            3u8 => core::result::Result::Ok(DayOfWeek::Thursday),
            4u8 => core::result::Result::Ok(DayOfWeek::Friday),
            5u8 => core::result::Result::Ok(DayOfWeek::Saturday),
            6u8 => core::result::Result::Ok(DayOfWeek::Sunday),
            _ => core::result::Result::Err(DecodeError::INVALID_VALUE),
        }
    }
    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        match u8::top_decode_or_exit(input, c.clone(), exit) {
            0u8 => DayOfWeek::Monday,
            1u8 => DayOfWeek::Tuesday,
            2u8 => DayOfWeek::Wednesday,
            3u8 => DayOfWeek::Thursday,
            4u8 => DayOfWeek::Friday,
            5u8 => DayOfWeek::Saturday,
            6u8 => DayOfWeek::Sunday,
            _ => exit(c, DecodeError::INVALID_VALUE),
        }
    }
}
impl NestedEncode for DayOfWeek {
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        match self {
            DayOfWeek::Monday => 0u8.dep_encode(dest)?,
            DayOfWeek::Tuesday => 1u8.dep_encode(dest)?,
            DayOfWeek::Wednesday => 2u8.dep_encode(dest)?,
            DayOfWeek::Thursday => 3u8.dep_encode(dest)?,
            DayOfWeek::Friday => 4u8.dep_encode(dest)?,
            DayOfWeek::Saturday => 5u8.dep_encode(dest)?,
            DayOfWeek::Sunday => 6u8.dep_encode(dest)?,
        };
        Ok(())
    }
    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        match self {
            DayOfWeek::Monday => 0u8.dep_encode_or_exit(dest, c.clone(), exit),
            DayOfWeek::Tuesday => 1u8.dep_encode_or_exit(dest, c.clone(), exit),
            DayOfWeek::Wednesday => 2u8.dep_encode_or_exit(dest, c.clone(), exit),
            DayOfWeek::Thursday => 3u8.dep_encode_or_exit(dest, c.clone(), exit),
            DayOfWeek::Friday => 4u8.dep_encode_or_exit(dest, c.clone(), exit),
            DayOfWeek::Saturday => 5u8.dep_encode_or_exit(dest, c.clone(), exit),
            DayOfWeek::Sunday => 6u8.dep_encode_or_exit(dest, c.clone(), exit),
        };
    }
}
impl NestedDecode for DayOfWeek {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(match u8::dep_decode(input)? {
            0u8 => DayOfWeek::Monday,
            1u8 => DayOfWeek::Tuesday,
            2u8 => DayOfWeek::Wednesday,
            3u8 => DayOfWeek::Thursday,
            4u8 => DayOfWeek::Friday,
            5u8 => DayOfWeek::Saturday,
            6u8 => DayOfWeek::Sunday,
        })
    }
    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        match u8::dep_decode_or_exit(input, c.clone(), exit) {
            0u8 => DayOfWeek::Monday,
            1u8 => DayOfWeek::Tuesday,
            2u8 => DayOfWeek::Wednesday,
            3u8 => DayOfWeek::Thursday,
            4u8 => DayOfWeek::Friday,
            5u8 => DayOfWeek::Saturday,
            6u8 => DayOfWeek::Sunday,
        };
    }
}
enum Message {
    Quit,
    Today(DayOfWeek),
    Write(Vec<u8>),
}
impl NestedEncode for Message {
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        match self {
            Message::Quit => 0u8.dep_encode(dest)?,
            Message::Today(_var_enum_local_ident) => {
                1u8.dep_encode(dest)?;
                _var_enum_local_ident.dep_encode(dest)?;
            }
            Message::Write(_var_enum_local_ident) => {
                2u8.dep_encode(dest)?;
                _var_enum_local_ident.dep_encode(dest)?;
            }
        };
        Ok(())
    }
    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        match self {
            Message::Quit => 0u8.dep_encode_or_exit(dest, c.clone(), exit),
            Message::Today(_var_enum_local_ident) => {
                1u8.dep_encode_or_exit(dest, c.clone(), exit);
                _var_enum_local_ident.dep_encode_or_exit(dest, c.clone(), exit);
            }
            Message::Write(_var_enum_local_ident) => {
                2u8.dep_encode_or_exit(dest, c.clone(), exit);
                _var_enum_local_ident.dep_encode_or_exit(dest, c.clone(), exit);
            }
        };
    }
}
#[main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
