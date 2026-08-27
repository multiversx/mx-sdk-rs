use crate::{AbiType, AbiTypeFrom, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName};

/// Pure ABI counterpart of `EsdtTokenIdentifier<M>` (a.k.a. `TokenIdentifier<M>`).
///
/// Provides a stable, framework-agnostic type representation for an ESDT token identifier.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct EsdtTokenIdentifierAbi;

impl AbiTypeFrom<Self> for EsdtTokenIdentifierAbi {}
impl AbiTypeFrom<alloc::vec::Vec<u8>> for EsdtTokenIdentifierAbi {}

impl AbiType for EsdtTokenIdentifierAbi {
    fn type_name() -> TypeName {
        // Kept as "TokenIdentifier" for backwards compatibility with existing tooling.
        TypeName::from("TokenIdentifier")
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbiFrom<Self> for EsdtTokenIdentifierAbi {}

impl TypeAbi for EsdtTokenIdentifierAbi {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        TypeName::from("EsdtTokenIdentifierAbi")
    }
}
