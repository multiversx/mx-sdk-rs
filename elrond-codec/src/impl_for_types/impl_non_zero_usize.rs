use core::num::NonZeroUsize;

use crate::{
    DecodeError, DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput,
    NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

impl TopEncode for NonZeroUsize {
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.get().top_encode_or_handle_err(output, h)
    }
}

impl TopDecode for NonZeroUsize {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        if let Some(nz) = NonZeroUsize::new(usize::top_decode_or_handle_err(input, h)?) {
            Ok(nz)
        } else {
            Err(h.handle_error(DecodeError::INVALID_VALUE))
        }
    }
}

impl NestedEncode for NonZeroUsize {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.get().dep_encode_or_handle_err(dest, h)
    }
}

impl NestedDecode for NonZeroUsize {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        if let Some(nz) = NonZeroUsize::new(usize::dep_decode_or_handle_err(input, h)?) {
            Ok(nz)
        } else {
            Err(h.handle_error(DecodeError::INVALID_VALUE))
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::test_util::{check_dep_encode_decode, check_top_encode_decode};
    use core::num::NonZeroUsize;

    #[test]
    fn test_top() {
        check_top_encode_decode(NonZeroUsize::new(5).unwrap(), &[5]);
    }

    #[test]
    fn test_dep() {
        check_dep_encode_decode(NonZeroUsize::new(5).unwrap(), &[0, 0, 0, 5]);
    }
}
