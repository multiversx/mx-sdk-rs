use alloc::string::String;
use elrond_codec::{
    DecodeError, EncodeError, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput,
};

// BigInt sign.
#[allow(clippy::enum_variant_names)]
#[derive(PartialEq, Debug)]
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
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.to_int().top_encode(output)
    }
}

impl NestedEncode for Sign {
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.to_int().dep_encode(dest)
    }
}

impl NestedDecode for Sign {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(Sign::from_int(i8::dep_decode(input)?))
    }
}

impl TopDecode for Sign {
    #[inline]
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        Ok(Sign::from_int(i8::top_decode(input)?))
    }
}

impl crate::abi::TypeAbi for Sign {
    fn type_name() -> String {
        String::from("Sign")
    }
}
