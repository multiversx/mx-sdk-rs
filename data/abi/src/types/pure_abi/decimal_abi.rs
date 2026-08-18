use crate::{
    AbiType, AbiTypeFrom, HasUnmanaged, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName,
};
use alloc::format;

/// Pure ABI counterpart of `ManagedDecimal<M, NumDecimals>` (variable number of decimals).
///
/// Provides a stable, framework-agnostic type representation for fixed-point decimals
/// with a runtime-determined number of decimal places.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct DecimalAbi;

impl AbiTypeFrom<Self> for DecimalAbi {}

impl AbiType for DecimalAbi {
    fn type_name() -> TypeName {
        TypeName::from("ManagedDecimal<usize>")
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbiFrom<Self> for DecimalAbi {}

impl TypeAbi for DecimalAbi {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        TypeName::from("ManagedDecimalAbi")
    }
}

#[cfg(feature = "num-bigint")]
impl HasUnmanaged for DecimalAbi {
    type Unmanaged = crate::codec::num_bigint::BigUint;
}

/// Pure ABI counterpart of `ManagedDecimal<M, ConstDecimals<DECIMALS>>` (compile-time fixed number of decimals).
///
/// The const parameter `DECIMALS` encodes the number of decimal places at the type level.
///
/// Provides a stable, framework-agnostic type representation for fixed-point decimals
/// with a compile-time-fixed number of decimal places.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct DecimalConstAbi<const DECIMALS: usize>;

impl<const DECIMALS: usize> AbiTypeFrom<Self> for DecimalConstAbi<DECIMALS> {}

impl<const DECIMALS: usize> AbiType for DecimalConstAbi<DECIMALS> {
    fn type_name() -> TypeName {
        format!("ManagedDecimal<{DECIMALS}>")
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl<const DECIMALS: usize> TypeAbiFrom<Self> for DecimalConstAbi<DECIMALS> {}

impl<const DECIMALS: usize> TypeAbi for DecimalConstAbi<DECIMALS> {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        format!("ManagedDecimalConstAbi<{DECIMALS}>")
    }
}

#[cfg(feature = "num-bigint")]
impl<const DECIMALS: usize> HasUnmanaged for DecimalConstAbi<DECIMALS> {
    type Unmanaged = crate::codec::num_bigint::BigUint;
}
