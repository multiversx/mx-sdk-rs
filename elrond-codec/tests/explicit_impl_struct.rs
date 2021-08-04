// Some structures with explicit encode/decode, for testing.

use alloc::vec::Vec;
use crate::codec_err::{DecodeError, EncodeError};
use crate::nested_de::NestedDecode;
use crate::nested_de_input::NestedDecodeInput;
use crate::nested_ser::NestedEncode;
use crate::nested_ser_output::NestedEncodeOutput;
use crate::top_de::{top_decode_from_nested, top_decode_from_nested_or_exit, TopDecode};
use crate::top_de_input::TopDecodeInput;
use crate::top_ser::{top_encode_from_nested, top_encode_from_nested_or_exit, TopEncode};
use crate::top_ser_output::TopEncodeOutput;

#[derive(PartialEq, Debug)]
pub struct S {
    pub int: u16,
    pub seq: Vec<u8>,
    pub another_byte: u8,
}

impl NestedEncode for S {
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.int.dep_encode(dest)?;
        self.seq.dep_encode(dest)?;
        self.another_byte.dep_encode(dest)?;
        Ok(())
    }

    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.int.dep_encode_or_exit(dest, c.clone(), exit);
        self.seq.dep_encode_or_exit(dest, c.clone(), exit);
        self.another_byte.dep_encode_or_exit(dest, c.clone(), exit);
    }
}

impl TopEncode for S {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        top_encode_from_nested(self, output)
    }

    #[inline]
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        top_encode_from_nested_or_exit(self, output, c, exit);
    }
}

impl NestedDecode for S {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(S {
            int: u16::dep_decode(input)?,
            seq: Vec::<u8>::dep_decode(input)?,
            another_byte: u8::dep_decode(input)?,
        })
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        S {
            int: u16::dep_decode_or_exit(input, c.clone(), exit),
            seq: Vec::<u8>::dep_decode_or_exit(input, c.clone(), exit),
            another_byte: u8::dep_decode_or_exit(input, c.clone(), exit),
        }
    }
}

impl TopDecode for S {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        top_decode_from_nested(input)
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        top_decode_from_nested_or_exit(input, c, exit)
    }
}

use super::test_struct::*;
use crate::{
    test_util::{check_top_decode, check_top_encode, deser_ok, ser_ok},
    TopDecode, TopEncode,
};
use core::fmt::Debug;

pub fn the_same<V>(element: V)
where
    V: TopEncode + TopDecode + PartialEq + Debug + 'static,
{
    let serialized_bytes = check_top_encode(&element);
    let deserialized: V = check_top_decode::<V>(&serialized_bytes[..]);
    assert_eq!(deserialized, element);
}
#[test]
fn test_encode() {
    let test = S {
        int: 1,
        seq: [5, 6].to_vec(),
        another_byte: 7,
    };

    ser_ok(test, &[0, 1, 0, 0, 0, 2, 5, 6, 7]);
}

#[test]
fn test_decode() {
    let test = S {
        int: 1,
        seq: [5, 6].to_vec(),
        another_byte: 7,
    };
    deser_ok(test, &[0, 1, 0, 0, 0, 2, 5, 6, 7]);
}

#[test]
fn test_encode_decode() {
    let test = S {
        int: 1,
        seq: [5, 6].to_vec(),
        another_byte: 7,
    };
    the_same(test);
}
