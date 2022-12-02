use crate::{
    DecodeError, DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput,
    NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

impl<T: NestedEncode> NestedEncode for Option<T> {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        match self {
            Some(v) => {
                dest.push_byte(1u8);
                v.dep_encode_or_handle_err(dest, h)
            },
            None => {
                dest.push_byte(0u8);
                Ok(())
            },
        }
    }
}

impl<T: NestedDecode> NestedDecode for Option<T> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        match input.read_byte(h)? {
            0 => Ok(None),
            1 => Ok(Some(T::dep_decode_or_handle_err(input, h)?)),
            _ => Err(h.handle_error(DecodeError::INVALID_VALUE)),
        }
    }
}

impl<T: NestedEncode> TopEncode for Option<T> {
    /// Allow None to be serialized to empty bytes, but leave the leading "1" for Some,
    /// to allow disambiguation between e.g. Some(0) and None.
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        match self {
            Some(v) => {
                let mut buffer = output.start_nested_encode();
                buffer.push_byte(1u8);
                v.dep_encode_or_handle_err(&mut buffer, h)?;
                output.finalize_nested_encode(buffer);
            },
            None => {
                output.set_slice_u8(&[]);
            },
        }
        Ok(())
    }
}

impl<T: NestedDecode> TopDecode for Option<T> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        let mut buffer = input.into_nested_buffer();
        if buffer.is_depleted() {
            Ok(None) // empty input is none
        } else {
            let first_byte = buffer.read_byte(h)?;
            let value = match first_byte {
                0 => None, // also allow "0x00" to be interpreted as None
                1 => Some(T::dep_decode_or_handle_err(&mut buffer, h)?),
                _ => return Err(h.handle_error(DecodeError::INVALID_VALUE)),
            };

            if buffer.is_depleted() {
                Ok(value)
            } else {
                Err(h.handle_error(DecodeError::INPUT_TOO_LONG))
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use alloc::vec::Vec;

    use crate::{
        test_util::{check_top_decode, check_top_encode_decode},
        DecodeError, TopDecode,
    };

    #[test]
    fn test_top() {
        let some_v = Some([1i32, 2i32, 3i32].to_vec());
        let expected: &[u8] = &[
            /*opt*/ 1, /*size*/ 0, 0, 0, 3, /*data*/ 0, 0, 0, 1, 0, 0, 0, 2, 0, 0,
            0, 3,
        ];
        check_top_encode_decode(some_v, expected);

        let none_v: Option<Vec<i32>> = None;
        check_top_encode_decode(none_v, &[]);
    }

    #[test]
    fn test_top_none() {
        // default representation
        assert_eq!(None, check_top_decode::<Option<u32>>(&[][..]));

        // single zero byte also allowed
        assert_eq!(None, check_top_decode::<Option<u32>>(&[0x00u8][..]));

        // invalid None bytes
        assert_eq!(
            Option::<u32>::top_decode(&[0x00u8, 0x00u8][..]),
            Err(DecodeError::INPUT_TOO_LONG)
        );

        // more invalid None bytes
        assert_eq!(
            Option::<u32>::top_decode(&[0x00u8, 0x00u8, 0x00u8][..]),
            Err(DecodeError::INPUT_TOO_LONG)
        );

        // just invalid byte
        assert_eq!(
            Option::<u32>::top_decode(&[0x02u8][..]),
            Err(DecodeError::INVALID_VALUE)
        );
    }
}
