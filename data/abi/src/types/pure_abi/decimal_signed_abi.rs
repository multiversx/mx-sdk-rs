use crate::{
    AbiType, AbiTypeFrom, DecimalAbiSpec, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName,
};
use alloc::format;
use core::marker::PhantomData;

/// Pure ABI counterpart of `ManagedDecimalSigned<M, D>`.
///
/// Provides a stable, framework-agnostic type representation for signed fixed-point decimals
/// with a decimal precision determined by `D`.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct DecimalSignedAbi<D>(PhantomData<D>);

impl<D: DecimalAbiSpec> AbiTypeFrom<Self> for DecimalSignedAbi<D> {}

impl<D: DecimalAbiSpec> AbiType for DecimalSignedAbi<D> {
    fn type_name() -> TypeName {
        format!("ManagedDecimalSigned<{}>", D::decimal_abi_name())
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl<D: DecimalAbiSpec> TypeAbiFrom<Self> for DecimalSignedAbi<D> {}

impl<D: DecimalAbiSpec> TypeAbi for DecimalSignedAbi<D> {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        TypeName::from("ManagedDecimalSignedAbi")
    }
}

/// Pure ABI counterpart of `ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>` (compile-time fixed number of decimals).
///
/// The const parameter `DECIMALS` encodes the number of decimal places at the type level.
///
/// Provides a stable, framework-agnostic type representation for signed fixed-point decimals
/// with a compile-time-fixed number of decimal places.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct DecimalSignedConstAbi<const DECIMALS: usize>;

impl<const DECIMALS: usize> AbiTypeFrom<Self> for DecimalSignedConstAbi<DECIMALS> {}

impl<const DECIMALS: usize> AbiType for DecimalSignedConstAbi<DECIMALS> {
    fn type_name() -> TypeName {
        format!("ManagedDecimalSigned<{DECIMALS}>")
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl<const DECIMALS: usize> TypeAbiFrom<Self> for DecimalSignedConstAbi<DECIMALS> {}

impl<const DECIMALS: usize> TypeAbi for DecimalSignedConstAbi<DECIMALS> {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        format!("ManagedDecimalSignedConstAbi<{DECIMALS}>")
    }
}
