use crate::{
    AbiType, AbiTypeFrom, HasUnmanaged, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName,
};

/// Pure ABI counterpart of `BigFloat<M>`.
///
/// Provides a stable, framework-agnostic type representation for the floating-point big number type.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct BigFloatAbi;

impl AbiTypeFrom<Self> for BigFloatAbi {}

impl AbiType for BigFloatAbi {
    fn type_name() -> TypeName {
        TypeName::from("BigFloat")
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbiFrom<Self> for BigFloatAbi {}

impl TypeAbi for BigFloatAbi {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        TypeName::from("BigFloatAbi")
    }
}

impl HasUnmanaged for BigFloatAbi {
    type Unmanaged = Self;
}
