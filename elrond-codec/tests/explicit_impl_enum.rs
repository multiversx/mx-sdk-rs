use elrond_codec::NestedDecode;
use elrond_codec::NestedDecodeInput;
use elrond_codec::NestedEncode;
use elrond_codec::NestedEncodeNoErr;
use elrond_codec::NestedEncodeOutput;
use elrond_codec::TopDecodeInput;
use elrond_codec::TopEncodeOutput;
use elrond_codec::{top_decode_from_nested, top_decode_from_nested_or_exit, TopDecode};
use elrond_codec::{top_encode_from_nested, top_encode_from_nested_or_exit, TopEncode};
use elrond_codec::{DecodeError, EncodeError};
use elrond_codec::test_util::{deser_ok, ser_ok};

#[derive(PartialEq, Clone, Debug)]
pub enum E {
    Unit,
    Newtype(u32),
    Tuple(u32, u32),
    Struct { a: u32 },
}

impl NestedEncodeNoErr for E {
    fn dep_encode_no_err<O: NestedEncodeOutput>(&self, dest: &mut O) {
        match self {
            E::Unit => {
                0u32.dep_encode_no_err(dest);
            },
            E::Newtype(arg1) => {
                1u32.dep_encode_no_err(dest);
                arg1.dep_encode_no_err(dest);
            },
            E::Tuple(arg1, arg2) => {
                2u32.dep_encode_no_err(dest);
                arg1.dep_encode_no_err(dest);
                arg2.dep_encode_no_err(dest);
            },
            E::Struct { a } => {
                3u32.dep_encode_no_err(dest);
                a.dep_encode_no_err(dest);
            },
        }
    }
}

impl NestedEncode for E {
    #[inline]
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.dep_encode_no_err(dest);
        Ok(())
    }

    #[inline]
    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        _: ExitCtx,
        _: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.dep_encode_no_err(dest);
    }
}

impl TopEncode for E {
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

impl NestedDecode for E {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        match u32::dep_decode(input)? {
            0 => Ok(E::Unit),
            1 => Ok(E::Newtype(u32::dep_decode(input)?)),
            2 => Ok(E::Tuple(u32::dep_decode(input)?, u32::dep_decode(input)?)),
            3 => Ok(E::Struct {
                a: u32::dep_decode(input)?,
            }),
            _ => Err(DecodeError::INVALID_VALUE),
        }
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        match u32::dep_decode_or_exit(input, c.clone(), exit) {
            0 => E::Unit,
            1 => E::Newtype(u32::dep_decode_or_exit(input, c.clone(), exit)),
            2 => E::Tuple(
                u32::dep_decode_or_exit(input, c.clone(), exit),
                u32::dep_decode_or_exit(input, c.clone(), exit),
            ),
            3 => E::Struct {
                a: u32::dep_decode_or_exit(input, c.clone(), exit),
            },
            _ => exit(c.clone(), DecodeError::INVALID_VALUE),
        }
    }
}

impl TopDecode for E {
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
fn test_encode() {
    let u = E::Unit;
    let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 0];
    ser_ok(u, expected);

    let n = E::Newtype(1);
    let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 1, /*data*/ 0, 0, 0, 1];
    ser_ok(n, expected);

    let t = E::Tuple(1, 2);
    let expected: &[u8] = &[
        /*variant index*/ 0, 0, 0, 2, /*(*/ 0, 0, 0, 1, /*,*/ 0, 0, 0, 2, /*)*/
    ];
    ser_ok(t, expected);

    let s = E::Struct { a: 1 };
    let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 3, /*data*/ 0, 0, 0, 1];
    ser_ok(s, expected);
}

#[test]
fn test_decode() {
    let u = E::Unit;
    let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 0];
    deser_ok(u, expected);

    let n = E::Newtype(1);
    let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 1, /*data*/ 0, 0, 0, 1];
    deser_ok(n, expected);

    let t = E::Tuple(1, 2);
    let expected: &[u8] = &[
        /*variant index*/ 0, 0, 0, 2, /*(*/ 0, 0, 0, 1, /*,*/ 0, 0, 0, 2, /*)*/
    ];
    deser_ok(t, expected);

    let s = E::Struct { a: 1 };
    let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 3, /*data*/ 0, 0, 0, 1];
    deser_ok(s, expected);
}
