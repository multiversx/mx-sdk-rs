use crate::{
    DecodeError, DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput,
    NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

/// Empty structure with an empty bytes representation. Equivalent to `false`, `0` or `[u8; 0]`, but more explicit.
///
/// Note: the unit type `()` would have naturally fit this role, but we decided to make the unit type multi-value only.
#[derive(Debug, PartialEq, Eq)]
pub struct Empty;

impl TopEncode for Empty {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, _h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        output.set_slice_u8(&[]);
        Ok(())
    }
}

impl TopDecode for Empty {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        if input.byte_len() == 0 {
            Ok(Empty)
        } else {
            Err(h.handle_error(DecodeError::INPUT_TOO_LONG))
        }
    }
}

impl NestedEncode for Empty {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, _dest: &mut O, _h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        Ok(())
    }
}

impl NestedDecode for Empty {
    #[inline]
    fn dep_decode_or_handle_err<I, H>(_input: &mut I, _h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Empty)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::test_util::{check_dep_encode_decode, check_top_encode_decode};
    use alloc::vec::Vec;

    #[test]
    fn test_top_empty() {
        check_top_encode_decode(super::Empty, &[]);
    }

    #[test]
    fn test_dep_empty() {
        check_dep_encode_decode(super::Empty, &[]);
    }

    #[test]
    fn test_dep_unit() {
        check_dep_encode_decode((), &[]);
    }

    #[test]
    fn test_empty_vec_compacted() {
        check_top_encode_decode(Vec::<u8>::new(), &[]);
    }
}
