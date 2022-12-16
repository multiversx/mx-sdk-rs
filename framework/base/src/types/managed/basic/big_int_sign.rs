use crate::codec::{
    DecodeErrorHandler, EncodeErrorHandler, NestedDecode, NestedDecodeInput, NestedEncode,
    NestedEncodeOutput, TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

use crate::abi::TypeName;

// BigInt sign.
#[allow(clippy::enum_variant_names)]
#[derive(PartialEq, Eq, Debug)]
pub enum Sign {
    Minus,
    NoSign,
    Plus,
}

impl Sign {
    #[inline]
    pub fn is_minus(&self) -> bool {
        matches!(self, Sign::Minus)
    }

    fn to_int(&self) -> i8 {
        match self {
            Sign::Plus => 1,
            Sign::NoSign => 0,
            Sign::Minus => -1,
        }
    }

    fn from_int(value: i8) -> Self {
        match value.cmp(&0) {
            core::cmp::Ordering::Greater => Sign::Plus,
            core::cmp::Ordering::Equal => Sign::NoSign,
            core::cmp::Ordering::Less => Sign::Minus,
        }
    }
}

impl TopEncode for Sign {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.to_int().top_encode_or_handle_err(output, h)
    }
}

impl NestedEncode for Sign {
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        self.to_int().dep_encode_or_handle_err(dest, h)
    }
}

impl NestedDecode for Sign {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Sign::from_int(i8::dep_decode_or_handle_err(input, h)?))
    }
}

impl TopDecode for Sign {
    #[inline]
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Sign::from_int(i8::top_decode_or_handle_err(input, h)?))
    }
}

impl crate::abi::TypeAbi for Sign {
    fn type_name() -> TypeName {
        TypeName::from("Sign")
    }
}
