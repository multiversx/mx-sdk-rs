use crate::{AbiType, AbiTypeFrom, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName};

/// Pure ABI counterpart of `NonZeroBigUint<M>`.
///
/// Provides a stable, framework-agnostic type representation for a guaranteed-non-zero
/// unsigned big integer. Using this type ensures ABI compatibility across multiple versions
/// of the framework or across different framework implementations entirely.
pub struct NonZeroBigUintAbi;

impl AbiTypeFrom<Self> for NonZeroBigUintAbi {}

impl AbiType for NonZeroBigUintAbi {
    fn type_name() -> TypeName {
        TypeName::from("NonZeroBigUint")
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbiFrom<Self> for NonZeroBigUintAbi {}

impl TypeAbi for NonZeroBigUintAbi {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        TypeName::from("NonZeroBigUintAbi")
    }
}
