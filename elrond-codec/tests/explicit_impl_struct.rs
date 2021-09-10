// Some structures with explicit encode/decode, for testing.
use core::fmt::Debug;
use elrond_codec::{
    test_util::{check_dep_encode_decode, check_top_encode_decode},
    top_decode_from_nested, top_decode_from_nested_or_exit, top_encode_from_nested,
    top_encode_from_nested_or_exit, DecodeError, EncodeError, NestedDecode, NestedDecodeInput,
    NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

#[derive(PartialEq, Debug, Clone)]
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

#[test]
fn test_top() {
    let test = S {
        int: 1,
        seq: [5, 6].to_vec(),
        another_byte: 7,
    };
    check_top_encode_decode(test, &[0, 1, 0, 0, 0, 2, 5, 6, 7]);
}

#[test]
fn test_dep() {
    let test = S {
        int: 1,
        seq: [5, 6].to_vec(),
        another_byte: 7,
    };
    check_dep_encode_decode(test, &[0, 1, 0, 0, 0, 2, 5, 6, 7]);
}
