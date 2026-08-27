use crate::{AbiType, AbiTypeFrom, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName};
use alloc::format;
use core::marker::PhantomData;

/// Provides the ABI name fragment for a decimal precision specification.
pub trait DecimalAbiSpec {
    fn decimal_abi_name() -> TypeName;
}

impl DecimalAbiSpec for usize {
    fn decimal_abi_name() -> TypeName {
        TypeName::from("usize")
    }
}

/// Pure ABI counterpart of `ManagedDecimal<M, D>`.
///
/// Provides a stable, framework-agnostic type representation for fixed-point decimals
/// with a decimal precision determined by `D`.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct DecimalAbi<D>(PhantomData<D>);

impl<D: DecimalAbiSpec> AbiTypeFrom<Self> for DecimalAbi<D> {}

impl<D: DecimalAbiSpec> AbiType for DecimalAbi<D> {
    fn type_name() -> TypeName {
        format!("ManagedDecimal<{}>", D::decimal_abi_name())
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl<D: DecimalAbiSpec> TypeAbiFrom<Self> for DecimalAbi<D> {}

impl<D: DecimalAbiSpec> TypeAbi for DecimalAbi<D> {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        TypeName::from("ManagedDecimalAbi")
    }
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
