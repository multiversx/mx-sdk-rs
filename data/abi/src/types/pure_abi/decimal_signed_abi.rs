use crate::{TypeAbi, TypeAbiFrom, TypeName};
use alloc::format;

/// Pure ABI counterpart of `ManagedDecimalSigned<M, NumDecimals>` (variable number of decimals).
///
/// Provides a stable, framework-agnostic type representation for signed fixed-point decimals
/// with a runtime-determined number of decimal places.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct DecimalSignedAbi;

impl TypeAbiFrom<Self> for DecimalSignedAbi {}

impl TypeAbi for DecimalSignedAbi {
    type Unmanaged = Self;
    type Abi = Self;

    fn type_name() -> TypeName {
        TypeName::from("ManagedDecimalSigned<usize>")
    }

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

impl<const DECIMALS: usize> TypeAbiFrom<Self> for DecimalSignedConstAbi<DECIMALS> {}

impl<const DECIMALS: usize> TypeAbi for DecimalSignedConstAbi<DECIMALS> {
    type Unmanaged = Self;
    type Abi = Self;

    fn type_name() -> TypeName {
        format!("ManagedDecimalSigned<{DECIMALS}>")
    }

    fn type_name_rust() -> TypeName {
        format!("ManagedDecimalSignedConstAbi<{DECIMALS}>")
    }
}
