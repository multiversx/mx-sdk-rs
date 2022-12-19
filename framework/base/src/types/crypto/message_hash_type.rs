use crate::{
    abi::{TypeAbi, TypeName},
    codec,
    codec::derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
};

/// Message hash type for the `verifyCustomSecp256k1` CryptoApi function
#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, Clone, PartialEq, Eq, Debug)]
pub enum MessageHashType {
    ECDSAPlainMsg,
    ECDSASha256,
    ECDSADoubleSha256,
    ECDSAKeccak256,
    ECDSARipemd160,
}

impl MessageHashType {
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::ECDSAPlainMsg => 0,
            Self::ECDSASha256 => 1,
            Self::ECDSADoubleSha256 => 2,
            Self::ECDSAKeccak256 => 3,
            Self::ECDSARipemd160 => 4,
        }
    }
}

impl From<u8> for MessageHashType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::ECDSASha256,
            2 => Self::ECDSADoubleSha256,
            3 => Self::ECDSAKeccak256,
            4 => Self::ECDSARipemd160,
            _ => Self::ECDSAPlainMsg,
        }
    }
}

impl TypeAbi for MessageHashType {
    fn type_name() -> TypeName {
        "MessageHashType".into()
    }
}
