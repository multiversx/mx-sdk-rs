use crate::{AbiType, AbiTypeFrom, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName};

/// Pure ABI counterpart of `TokenId<M>`.
///
/// Provides a stable, framework-agnostic type representation for a universal token identifier
/// that can hold either the native EGLD token or an ESDT token.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct TokenIdAbi;

impl AbiTypeFrom<Self> for TokenIdAbi {}
impl AbiTypeFrom<crate::EsdtTokenIdentifierAbi> for TokenIdAbi {}

impl AbiType for TokenIdAbi {
    fn type_name() -> TypeName {
        TypeName::from("TokenId")
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbiFrom<Self> for TokenIdAbi {}

impl TypeAbi for TokenIdAbi {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        TypeName::from("TokenIdAbi")
    }
}
