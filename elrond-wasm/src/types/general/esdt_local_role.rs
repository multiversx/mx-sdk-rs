use crate::abi::TypeAbi;
use alloc::string::String;
use elrond_codec::*;

const ESDT_ROLE_NONE: &'static [u8] = &[];
const ESDT_ROLE_LOCAL_MINT: &'static [u8] = b"ESDTRoleLocalMint";
const ESDT_ROLE_LOCAL_BURN: &'static [u8] = b"ESDTRoleLocalBurn";
const ESDT_ROLE_NFT_CREATE: &'static [u8] = b"ESDTRoleNFTCreate";
const ESDT_ROLE_NFT_ADD_QUANTITY: &'static [u8] = b"ESDTRoleNFTAddQuantity";
const ESDT_ROLE_NFT_BURN: &'static [u8] = b"ESDTRoleNFTBurn";

#[derive(Clone, PartialEq, Debug)]
pub enum EsdtLocalRole {
    None,
    Mint,
    Burn,
    NftCreate,
    NftAddQuantity,
    NftBurn,
}

impl EsdtLocalRole {
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Mint => 1,
            Self::Burn => 2,
            Self::NftCreate => 3,
            Self::NftAddQuantity => 4,
            Self::NftBurn => 5
        }
    }

    pub fn as_role_name(&self) -> &'static [u8] {
        match self {
            Self::None => ESDT_ROLE_NONE,
            Self::Mint => ESDT_ROLE_LOCAL_MINT,
            Self::Burn => ESDT_ROLE_LOCAL_BURN,
            Self::NftCreate => ESDT_ROLE_NFT_CREATE,
            Self::NftAddQuantity => ESDT_ROLE_NFT_ADD_QUANTITY,
            Self::NftBurn => ESDT_ROLE_NFT_BURN
        }
    }
}

impl From<u8> for EsdtLocalRole {
	#[inline]
	fn from(value: u8) -> Self {
		match value {
            1 => Self::Mint,
            2 => Self::Burn,
            3 => Self::NftCreate,
            4 => Self::NftAddQuantity,
            5 => Self::NftBurn,
            _ => Self::None
        }
	}
}

impl<'a> From<&'a [u8]> for EsdtLocalRole {
	#[inline]
	fn from(byte_slice: &'a [u8]) -> Self {
        if byte_slice == ESDT_ROLE_LOCAL_MINT {
            Self::Mint
        }
        else if byte_slice == ESDT_ROLE_LOCAL_BURN {
            Self::Burn
        }
        else if byte_slice == ESDT_ROLE_NFT_CREATE {
            Self::NftCreate
        }
        else if byte_slice == ESDT_ROLE_NFT_ADD_QUANTITY {
            Self::NftAddQuantity
        }
        else if byte_slice == ESDT_ROLE_NFT_BURN {
            Self::NftBurn
        }
        else {
            Self::None
        }
    }
}

impl NestedEncode for EsdtLocalRole {
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

impl TopEncode for EsdtLocalRole {
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

impl NestedDecode for EsdtLocalRole {
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

impl TopDecode for EsdtLocalRole {
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

impl TypeAbi for EsdtLocalRole {
	fn type_name() -> String {
		"EsdtLocalRole".into()
	}
}
