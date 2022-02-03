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
    DecodeErrorHandler,
};
use alloc::{boxed::Box, string::String, vec::Vec};

impl TopEncode for String {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.as_bytes().top_encode(output)
    }

    #[inline]
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_bytes().top_encode_or_exit(output, c, exit);
    }
}

impl TopEncode for &str {
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        output.set_slice_u8(self.as_bytes());
        Ok(())
    }

    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        _: ExitCtx,
        _: fn(ExitCtx, EncodeError) -> !,
    ) {
        output.set_slice_u8(self.as_bytes());
    }
}

impl TopEncode for Box<str> {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.as_ref().as_bytes().top_encode(output)
    }

    #[inline]
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_ref().as_bytes().top_encode_or_exit(output, c, exit);
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
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.as_bytes().dep_encode(dest)
    }

    #[inline]
    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_bytes().dep_encode_or_exit(dest, c, exit);
    }
}

impl NestedEncode for &str {
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.as_bytes().dep_encode(dest)
    }

    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_bytes().dep_encode_or_exit(dest, c, exit);
    }
}

impl NestedEncode for Box<str> {
    #[inline]
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.as_ref().as_bytes().dep_encode(dest)
    }

    #[inline]
    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_ref().as_bytes().dep_encode_or_exit(dest, c, exit);
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
    use crate::test_util::{check_dep_encode_decode, check_top_encode_decode};
    use alloc::string::String;

    #[test]
    fn test_top() {
        let s = "abc";
        check_top_encode_decode(String::from(s), &[b'a', b'b', b'c']);
        check_top_encode_decode(String::from(s).into_boxed_str(), &[b'a', b'b', b'c']);
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
}
