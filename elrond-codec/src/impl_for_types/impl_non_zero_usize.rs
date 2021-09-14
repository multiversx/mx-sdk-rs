use core::num::NonZeroUsize;

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

impl TopEncode for NonZeroUsize {
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.get().top_encode(output)
    }

    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.get().top_encode_or_exit(output, c, exit);
    }
}

impl TopDecode for NonZeroUsize {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        if let Some(nz) = NonZeroUsize::new(usize::top_decode(input)?) {
            Ok(nz)
        } else {
            Err(DecodeError::INVALID_VALUE)
        }
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        if let Some(nz) = NonZeroUsize::new(usize::top_decode_or_exit(input, c.clone(), exit)) {
            nz
        } else {
            exit(c, DecodeError::INVALID_VALUE)
        }
    }
}

impl NestedEncode for NonZeroUsize {
    #[inline]
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.get().dep_encode(dest)
    }

    #[inline]
    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.get().dep_encode_or_exit(dest, c, exit);
    }
}

impl NestedDecode for NonZeroUsize {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        if let Some(nz) = NonZeroUsize::new(usize::dep_decode(input)?) {
            Ok(nz)
        } else {
            Err(DecodeError::INVALID_VALUE)
        }
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        if let Some(nz) = NonZeroUsize::new(usize::dep_decode_or_exit(input, c.clone(), exit)) {
            nz
        } else {
            exit(c, DecodeError::INVALID_VALUE)
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
