use crate::{AbiType, AbiTypeFrom, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName};

/// Pure ABI counterpart of `EgldOrEsdtTokenIdentifier<M>`.
///
/// Provides a stable, framework-agnostic type representation for a token identifier
/// that can be either the native EGLD token or an ESDT token.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct EgldOrEsdtTokenIdentifierAbi;

impl AbiTypeFrom<Self> for EgldOrEsdtTokenIdentifierAbi {}

impl AbiType for EgldOrEsdtTokenIdentifierAbi {
    fn type_name() -> TypeName {
        TypeName::from("EgldOrEsdtTokenIdentifier")
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbiFrom<Self> for EgldOrEsdtTokenIdentifierAbi {}

impl TypeAbi for EgldOrEsdtTokenIdentifierAbi {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        TypeName::from("EgldOrEsdtTokenIdentifierAbi")
    }
}
