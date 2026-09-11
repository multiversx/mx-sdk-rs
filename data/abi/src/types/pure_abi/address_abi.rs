use crate::{TypeAbi, TypeAbiFrom, TypeName};

/// Pure ABI counterpart of `ManagedAddress<M>`.
///
/// Provides a stable, framework-agnostic type representation for a 32-byte blockchain address.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct AddressAbi;

impl TypeAbiFrom<Self> for AddressAbi {}

impl TypeAbi for AddressAbi {
    type Unmanaged = Self;
    type Abi = Self;

    fn type_name() -> TypeName {
        TypeName::from("Address")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("AddressAbi")
    }
}
