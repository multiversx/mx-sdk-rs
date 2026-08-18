use crate::{
    AbiType, AbiTypeFrom, HasUnmanaged, TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName,
};
use alloc::format;

/// Pure ABI counterpart of `ManagedDecimalSigned<M, NumDecimals>` (variable number of decimals).
///
/// Provides a stable, framework-agnostic type representation for signed fixed-point decimals
/// with a runtime-determined number of decimal places.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct DecimalSignedAbi;

impl AbiTypeFrom<Self> for DecimalSignedAbi {}

impl AbiType for DecimalSignedAbi {
    fn type_name() -> TypeName {
        TypeName::from("ManagedDecimalSigned<usize>")
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
}

impl TypeAbiFrom<Self> for DecimalSignedAbi {}

impl TypeAbi for DecimalSignedAbi {
    type Abi = Self;

    fn type_name_rust() -> TypeName {
        TypeName::from("ManagedDecimalSignedAbi")
    }
}

#[cfg(feature = "num-bigint")]
impl HasUnmanaged for DecimalSignedAbi {
    type Unmanaged = crate::codec::num_bigint::BigInt;
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

#[cfg(feature = "num-bigint")]
impl<const DECIMALS: usize> HasUnmanaged for DecimalSignedConstAbi<DECIMALS> {
    type Unmanaged = crate::codec::num_bigint::BigInt;
}
