use crate::{
    DecodeError, DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput,
    NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

const TOP_ENCODED_TRUE: &[u8] = &[1];
const TOP_ENCODED_FALSE: &[u8] = &[];

fn parse_byte<H: DecodeErrorHandler>(byte: u8, h: H) -> Result<bool, H::HandledErr> {
    match byte {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(h.handle_error(DecodeError::INVALID_VALUE)),
    }
}

impl TopEncode for bool {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, _h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        let bytes = if *self {
            TOP_ENCODED_TRUE
        } else {
            TOP_ENCODED_FALSE
        };
        output.set_slice_u8(bytes);
        Ok(())
    }
}

impl TopDecode for bool {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        let mut buffer = [0u8; 1];
        let length = input.into_max_size_buffer_align_right(&mut buffer, h)?;
        if length == 0 {
            Ok(false)
        } else {
            // Note: length can only be 1 at this point, because of how into_max_size_buffer_align_right works.
            // Not performing an additional check for length == 1, for optimization reasons.
            parse_byte(buffer[0], h)
        }
    }
}

impl NestedEncode for bool {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        // Note: u8 contains some additional optimizations (via specialization/monomorphization).
        // Do not change this implementation.
        (*self as u8).dep_encode_or_handle_err(dest, h)
    }
}

impl NestedDecode for bool {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        let byte = input.read_byte(h)?;
        parse_byte(byte, h)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        DecodeError, DefaultErrorHandler, TopDecode, dep_decode_from_byte_slice,
        test_util::{check_dep_encode_decode, check_top_decode, check_top_encode_decode},
    };

    #[test]
    fn test_top() {
        check_top_encode_decode(true, &[1]);
        check_top_encode_decode(false, &[]);
    }

    #[test]
    fn test_dep() {
        check_dep_encode_decode(true, &[1]);
        check_dep_encode_decode(false, &[0]);
    }

    #[test]
    fn test_top_decode_zero_byte_is_false() {
        assert_eq!(false, check_top_decode::<bool>(&[0]));
    }

    #[test]
    fn test_top_decode_invalid_value() {
        assert_eq!(
            bool::top_decode(&[2u8][..]),
            Err(DecodeError::INVALID_VALUE),
        );
        assert_eq!(
            bool::top_decode(&[255u8][..]),
            Err(DecodeError::INVALID_VALUE),
        );
    }

    #[test]
    fn test_dep_decode_invalid_value() {
        let result: Result<bool, _> = dep_decode_from_byte_slice(&[2u8], DefaultErrorHandler);
        assert_eq!(result, Err(DecodeError::INVALID_VALUE));

        let result: Result<bool, _> = dep_decode_from_byte_slice(&[255u8], DefaultErrorHandler);
        assert_eq!(result, Err(DecodeError::INVALID_VALUE));
    }
}
