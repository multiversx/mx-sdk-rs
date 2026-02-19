use crate::{
    DecodeError, DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput,
    NestedEncode, NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};
use alloc::{boxed::Box, string::String, vec::Vec};

impl TopEncode for String {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_bytes().top_encode_or_handle_err(output, h)
    }
}

impl TopEncode for &str {
    fn top_encode_or_handle_err<O, H>(&self, output: O, _h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        output.set_slice_u8(self.as_bytes());
        Ok(())
    }
}

impl TopEncode for Box<str> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_ref().as_bytes().top_encode_or_handle_err(output, h)
    }
}

impl TopDecode for String {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        let raw = Vec::<u8>::top_decode_or_handle_err(input, h)?;
        match String::from_utf8(raw) {
            Ok(s) => Ok(s),
            Err(_) => Err(h.handle_error(DecodeError::UTF8_DECODE_ERROR)),
        }
    }
}

impl TopDecode for Box<str> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(String::top_decode_or_handle_err(input, h)?.into_boxed_str())
    }
}

impl NestedEncode for String {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_bytes().dep_encode_or_handle_err(dest, h)
    }
}

impl NestedEncode for &str {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_bytes().dep_encode_or_handle_err(dest, h)
    }
}

impl NestedEncode for Box<str> {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.as_ref().as_bytes().dep_encode_or_handle_err(dest, h)
    }
}

impl NestedDecode for String {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        let raw = Vec::<u8>::dep_decode_or_handle_err(input, h)?;
        match String::from_utf8(raw) {
            Ok(s) => Ok(s),
            Err(_) => Err(h.handle_error(DecodeError::UTF8_DECODE_ERROR)),
        }
    }
}

impl NestedDecode for Box<str> {
    #[inline]
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(String::dep_decode_or_handle_err(input, h)?.into_boxed_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        DecodeError, TopDecode,
        test_util::{
            check_dep_encode, check_dep_encode_decode, check_top_encode, check_top_encode_decode,
        },
    };
    use alloc::string::String;

    #[test]
    fn test_top() {
        let s = "abc";
        check_top_encode_decode(String::from(s), b"abc");
        check_top_encode_decode(String::from(s).into_boxed_str(), b"abc");
    }

    #[test]
    fn test_dep() {
        let s = "abc";
        check_dep_encode_decode(String::from(s), &[0, 0, 0, 3, b'a', b'b', b'c']);
        check_dep_encode_decode(
            String::from(s).into_boxed_str(),
            &[0, 0, 0, 3, b'a', b'b', b'c'],
        );
    }

    #[test]
    fn test_top_empty() {
        check_top_encode_decode(String::new(), &[]);
        check_top_encode_decode(String::new().into_boxed_str(), &[]);
    }

    #[test]
    fn test_dep_empty() {
        check_dep_encode_decode(String::new(), &[0, 0, 0, 0]);
        check_dep_encode_decode(String::new().into_boxed_str(), &[0, 0, 0, 0]);
    }

    #[test]
    fn test_top_encode_str_ref() {
        let bytes = check_top_encode(&"hello");
        assert_eq!(bytes, b"hello");
    }

    #[test]
    fn test_dep_encode_str_ref() {
        let bytes = check_dep_encode(&"hello");
        assert_eq!(bytes, &[0, 0, 0, 5, b'h', b'e', b'l', b'l', b'o']);
    }

    #[test]
    fn test_top_decode_invalid_utf8() {
        // 0xFF is not valid UTF-8
        assert_eq!(
            String::top_decode(&[0xFFu8][..]),
            Err(DecodeError::UTF8_DECODE_ERROR),
        );
    }
}
