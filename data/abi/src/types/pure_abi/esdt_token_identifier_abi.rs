use crate::{TypeAbi, TypeAbiFrom, TypeName};

/// Pure ABI counterpart of `EsdtTokenIdentifier<M>` (a.k.a. `TokenIdentifier<M>`).
///
/// Provides a stable, framework-agnostic type representation for an ESDT token identifier.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct EsdtTokenIdentifierAbi;

impl TypeAbiFrom<Self> for EsdtTokenIdentifierAbi {}

impl TypeAbi for EsdtTokenIdentifierAbi {
    type Unmanaged = Self;
    type Abi = Self;

    fn type_name() -> TypeName {
        // Kept as "TokenIdentifier" for backwards compatibility with existing tooling.
        TypeName::from("TokenIdentifier")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("EsdtTokenIdentifierAbi")
    }
}
