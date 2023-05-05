use multiversx_sc_codec as codec;

// Some structures with explicit encode/decode, for testing.
use codec::{
    test_util::{check_dep_encode_decode, check_top_encode_decode},
    top_decode_from_nested_or_handle_err, top_encode_from_nested, DecodeErrorHandler,
    EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};
use core::fmt::Debug;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct S {
    pub int: u16,
    pub seq: Vec<u8>,
    pub another_byte: u8,
}

impl NestedEncode for S {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.int.dep_encode_or_handle_err(dest, h)?;
        self.seq.dep_encode_or_handle_err(dest, h)?;
        self.another_byte.dep_encode_or_handle_err(dest, h)?;
        Ok(())
    }
}

impl TopEncode for S {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        top_encode_from_nested(self, output, h)
    }
}

impl NestedDecode for S {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(S {
            int: u16::dep_decode_or_handle_err(input, h)?,
            seq: Vec::<u8>::dep_decode_or_handle_err(input, h)?,
            another_byte: u8::dep_decode_or_handle_err(input, h)?,
        })
    }
}

impl TopDecode for S {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        top_decode_from_nested_or_handle_err(input, h)
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
