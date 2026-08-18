use crate::{
    AbiType, AbiTypeFrom, HasUnmanaged, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName,
};

/// Pure ABI counterpart of `ManagedAddress<M>`.
///
/// Provides a stable, framework-agnostic type representation for a 32-byte blockchain address.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct AddressAbi;

impl AbiTypeFrom<Self> for AddressAbi {}

impl AbiType for AddressAbi {
    fn type_name() -> TypeName {
        TypeName::from("Address")
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbiFrom<Self> for AddressAbi {}

impl TypeAbi for AddressAbi {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        TypeName::from("AddressAbi")
    }
}

impl HasUnmanaged for AddressAbi {
    type Unmanaged = multiversx_chain_core::types::Address;
}
