use crate::{TypeAbi, TypeAbiFrom, TypeName};

/// Pure ABI counterpart of `EgldOrEsdtTokenIdentifier<M>`.
///
/// Provides a stable, framework-agnostic type representation for a token identifier
/// that can be either the native EGLD token or an ESDT token.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct EgldOrEsdtTokenIdentifierAbi;

impl TypeAbiFrom<Self> for EgldOrEsdtTokenIdentifierAbi {}

impl TypeAbi for EgldOrEsdtTokenIdentifierAbi {
    type Unmanaged = Self;
    type Abi = Self;

    fn type_name() -> TypeName {
        TypeName::from("EgldOrEsdtTokenIdentifier")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("EgldOrEsdtTokenIdentifierAbi")
    }
}
