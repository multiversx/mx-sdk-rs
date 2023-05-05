use core::marker::PhantomData;

use crate::{
    DecodeError, DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput,
    NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

/// Empty structure with an empty bytes representation. Equivalent to `false`, `0` or `[u8; 0]`, but more explicit.
///
/// Note: the unit type `()` would have naturally fit this role, but we decided to make the unit type multi-value only.

impl<T> TopEncode for PhantomData<T> {
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

impl<T> TopDecode for PhantomData<T> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        if input.byte_len() == 0 {
            Ok(PhantomData)
        } else {
            Err(h.handle_error(DecodeError::INPUT_TOO_LONG))
        }
    }
}

impl<T> NestedEncode for PhantomData<T> {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, _dest: &mut O, _h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        Ok(())
    }
}

impl<T> NestedDecode for PhantomData<T> {
    #[inline]
    fn dep_decode_or_handle_err<I, H>(_input: &mut I, _h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(PhantomData)
    }
}

#[cfg(test)]
pub mod tests {
    use crate as codec;
    use crate::test_util::{check_dep_encode_decode, check_top_encode_decode};
    use core::marker::PhantomData;
    use multiversx_sc_codec_derive::{NestedDecode, NestedEncode, TopDecode, TopEncode};

    #[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, Clone, Debug)]
    pub struct TestStructWithPhantom<M> {
        x: u32,
        y: u64,
        _phantom: PhantomData<M>,
    }

    #[test]
    fn test_dep_unit() {
        check_dep_encode_decode(PhantomData::<u32>, &[]);
    }

    #[test]
    fn test_top_unit() {
        check_top_encode_decode(PhantomData::<u32>, &[]);
    }

    #[test]
    fn test_dep_struc() {
        check_dep_encode_decode(
            TestStructWithPhantom::<u64> {
                x: 42,
                y: 42,
                _phantom: PhantomData::<u64>,
            },
            &[0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 42],
        );
    }

    #[test]
    fn test_top_struc() {
        check_top_encode_decode(
            TestStructWithPhantom::<u64> {
                x: 42,
                y: 42,
                _phantom: PhantomData::<u64>,
            },
            &[0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 42],
        );
    }
}
