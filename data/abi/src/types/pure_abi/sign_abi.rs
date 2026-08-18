use crate::{
    AbiType, AbiTypeFrom, HasUnmanaged, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName,
};

/// Pure ABI counterpart of `Sign` (the sign of a `BigInt`).
///
/// Provides a stable, framework-agnostic type representation for the sign enum.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct SignAbi;

impl AbiTypeFrom<Self> for SignAbi {}

impl AbiType for SignAbi {
    fn type_name() -> TypeName {
        TypeName::from("Sign")
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbiFrom<Self> for SignAbi {}

impl TypeAbi for SignAbi {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        TypeName::from("SignAbi")
    }
}

impl HasUnmanaged for SignAbi {
    type Unmanaged = Self;
}
