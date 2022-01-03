use crate::{
    codec_err::{DecodeError, EncodeError},
    nested_de::NestedDecode,
    nested_de_input::NestedDecodeInput,
    nested_ser::NestedEncode,
    nested_ser_output::NestedEncodeOutput,
    top_de::TopDecode,
    top_de_input::TopDecodeInput,
    top_ser::TopEncode,
    top_ser_output::TopEncodeOutput,
};

impl<T: NestedEncode> NestedEncode for Option<T> {
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        match self {
            Some(v) => {
                dest.push_byte(1u8);
                v.dep_encode(dest)
            },
            None => {
                dest.push_byte(0u8);
                Ok(())
            },
        }
    }

    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        match self {
            Some(v) => {
                dest.push_byte(1u8);
                v.dep_encode_or_exit(dest, c, exit);
            },
            None => {
                dest.push_byte(0u8);
            },
        }
    }
}

impl<T: NestedDecode> NestedDecode for Option<T> {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        match input.read_byte()? {
            0 => Ok(None),
            1 => Ok(Some(T::dep_decode(input)?)),
            _ => Err(DecodeError::INVALID_VALUE),
        }
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        match input.read_byte_or_exit(c.clone(), exit) {
            0 => None,
            1 => Some(T::dep_decode_or_exit(input, c, exit)),
            _ => exit(c, DecodeError::INVALID_VALUE),
        }
    }
}

impl<T: NestedEncode> TopEncode for Option<T> {
    /// Allow None to be serialized to empty bytes, but leave the leading "1" for Some,
    /// to allow disambiguation between e.g. Some(0) and None.
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        match self {
            Some(v) => {
                let mut buffer = output.start_nested_encode();
                buffer.push_byte(1u8);
                v.dep_encode(&mut buffer)?;
                output.finalize_nested_encode(buffer);
            },
            None => {
                output.set_slice_u8(&[]);
            },
        }
        Ok(())
    }

    /// Allow None to be serialized to empty bytes, but leave the leading "1" for Some,
    /// to allow disambiguation between e.g. Some(0) and None.
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        match self {
            Some(v) => {
                let mut buffer = output.start_nested_encode();
                buffer.push_byte(1u8);
                v.dep_encode_or_exit(&mut buffer, c, exit);
                output.finalize_nested_encode(buffer);
            },
            None => {
                output.set_slice_u8(&[]);
            },
        }
    }
}

impl<T: NestedDecode> TopDecode for Option<T> {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        let mut buffer = input.into_nested_buffer();
        if buffer.is_depleted() {
            Ok(None)
        } else {
            let first_byte = buffer.read_byte()?;
            if first_byte == 1 {
                let item = T::dep_decode(&mut buffer)?;
                if buffer.is_depleted() {
                    Ok(Some(item))
                } else {
                    Err(DecodeError::INPUT_TOO_LONG)
                }
            } else {
                Err(DecodeError::INVALID_VALUE)
            }
        }
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        let mut buffer = input.into_nested_buffer();
        if buffer.is_depleted() {
            None
        } else {
            let first_byte = buffer.read_byte_or_exit(c.clone(), exit);
            if first_byte == 1 {
                let item = T::dep_decode_or_exit(&mut buffer, c.clone(), exit);
                if buffer.is_depleted() {
                    Some(item)
                } else {
                    exit(c, DecodeError::INPUT_TOO_LONG)
                }
            } else {
                exit(c, DecodeError::INVALID_VALUE)
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use alloc::vec::Vec;

    use crate::test_util::check_top_encode_decode;

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
}
