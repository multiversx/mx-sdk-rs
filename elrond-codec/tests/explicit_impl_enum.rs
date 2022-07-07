use elrond_codec::{
    test_util::{check_dep_encode_decode, check_top_encode_decode},
    top_decode_from_nested_or_handle_err, top_encode_from_nested, DecodeError, DecodeErrorHandler,
    EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeNoErr,
    NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

#[derive(PartialEq, Eq, Clone, Debug)]
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
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, _h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.dep_encode_no_err(dest);
        Ok(())
    }
}

impl TopEncode for E {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        top_encode_from_nested(self, output, h)
    }
}

impl NestedDecode for E {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        match u32::dep_decode_or_handle_err(input, h)? {
            0 => Ok(E::Unit),
            1 => Ok(E::Newtype(u32::dep_decode_or_handle_err(input, h)?)),
            2 => Ok(E::Tuple(
                u32::dep_decode_or_handle_err(input, h)?,
                u32::dep_decode_or_handle_err(input, h)?,
            )),
            3 => Ok(E::Struct {
                a: u32::dep_decode_or_handle_err(input, h)?,
            }),
            _ => Err(h.handle_error(DecodeError::INVALID_VALUE)),
        }
    }
}

impl TopDecode for E {
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
    let u = E::Unit;
    let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 0];
    check_top_encode_decode(u, expected);

    let n = E::Newtype(1);
    let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 1, /*data*/ 0, 0, 0, 1];
    check_top_encode_decode(n, expected);

    let t = E::Tuple(1, 2);
    let expected: &[u8] = &[
        /*variant index*/ 0, 0, 0, 2, /*(*/ 0, 0, 0, 1, /*,*/ 0, 0, 0, 2, /*)*/
    ];
    check_top_encode_decode(t, expected);

    let s = E::Struct { a: 1 };
    let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 3, /*data*/ 0, 0, 0, 1];
    check_top_encode_decode(s, expected);
}

#[test]
fn test_dep() {
    let u = E::Unit;
    let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 0];
    check_dep_encode_decode(u, expected);

    let n = E::Newtype(1);
    let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 1, /*data*/ 0, 0, 0, 1];
    check_dep_encode_decode(n, expected);

    let t = E::Tuple(1, 2);
    let expected: &[u8] = &[
        /*variant index*/ 0, 0, 0, 2, /*(*/ 0, 0, 0, 1, /*,*/ 0, 0, 0, 2, /*)*/
    ];
    check_dep_encode_decode(t, expected);

    let s = E::Struct { a: 1 };
    let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 3, /*data*/ 0, 0, 0, 1];
    check_dep_encode_decode(s, expected);
}
