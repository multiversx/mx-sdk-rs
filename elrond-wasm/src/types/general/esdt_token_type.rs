use crate::abi::TypeAbi;
use alloc::string::String;
use elrond_codec::*;

const ESDT_TYPE_INVALID: &'static [u8] = &[];
const ESDT_TYPE_FUNGIBLE: &'static [u8] = b"FungibleESDT";
const ESDT_TYPE_NON_FUNGIBLE: &'static [u8] = b"NonFungibleESDT";
const ESDT_TYPE_SEMI_FUNGIBLE: &'static [u8] = b"SemiFungibleESDT";

#[derive(Clone, PartialEq, Debug)]
pub enum EsdtTokenType {
    Invalid,
    Fungible,
    NonFungible,
    SemiFungible
}

impl EsdtTokenType {
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Invalid => 0,
            Self::Fungible => 1,
            Self::NonFungible => 2,
            Self::SemiFungible => 3
        }
    }

    pub fn as_type_name(&self) -> &'static [u8] {
        match self {
            Self::Invalid => ESDT_TYPE_INVALID,
            Self::Fungible => ESDT_TYPE_FUNGIBLE,
            Self::NonFungible => ESDT_TYPE_NON_FUNGIBLE,
            Self::SemiFungible => ESDT_TYPE_SEMI_FUNGIBLE
        }
    }
}

impl From<u8> for EsdtTokenType {
	#[inline]
	fn from(value: u8) -> Self {
		match value {
            1 => Self::Fungible,
            2 => Self::NonFungible,
            3 => Self::SemiFungible,
            _ => Self::Invalid
        }
	}
}

impl<'a> From<&'a [u8]> for EsdtTokenType {
	#[inline]
	fn from(byte_slice: &'a [u8]) -> Self {
		if byte_slice == ESDT_TYPE_FUNGIBLE {
            Self::Fungible
        }
        else if byte_slice == ESDT_TYPE_NON_FUNGIBLE {
            Self::NonFungible
        }
        else if byte_slice == ESDT_TYPE_SEMI_FUNGIBLE {
            Self::SemiFungible
        }
        else {
            Self::Invalid
        }
	}
}

impl NestedEncode for EsdtTokenType {
	#[inline]
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.as_u8().dep_encode(dest)
	}

	#[inline]
	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_u8().dep_encode_or_exit(dest, c, exit);
	}
}

impl TopEncode for EsdtTokenType {
	#[inline]
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		self.as_u8().top_encode(output)
	}

	#[inline]
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_u8().top_encode_or_exit(output, c, exit);
	}
}

impl NestedDecode for EsdtTokenType {
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		Ok(Self::from(u8::dep_decode(input)?))
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		Self::from(u8::dep_decode_or_exit(input, c, exit))
	}
}

impl TopDecode for EsdtTokenType {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		Ok(Self::from(u8::top_decode(input)?))
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		Self::from(u8::top_decode_or_exit(input, c, exit))
	}
}

impl TypeAbi for EsdtTokenType {
	fn type_name() -> String {
		"EsdtTokenType".into()
	}
}
