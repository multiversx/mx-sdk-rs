use crate::{TypeAbi, TypeAbiFrom, TypeName};
use alloc::format;
use core::marker::PhantomData;
use typenum::Unsigned;

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
        TypeName::from("DecimalSignedAbi")
    }
}

/// Pure ABI counterpart of `ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>` (compile-time fixed number of decimals).
///
/// The type-level number `DECIMALS` encodes the number of decimal places at the type level,
/// mirroring `ConstDecimals<DECIMALS>`'s own `Unsigned` type parameter - unlike a plain
/// `const usize` parameter, this can be substituted directly from `ManagedDecimalSigned`'s own
/// `DECIMALS: Unsigned` type parameter in its `TypeAbi::Abi` projection, with no const-generic
/// conversion (which would require the unstable `generic_const_exprs` feature).
///
/// Provides a stable, framework-agnostic type representation for signed fixed-point decimals
/// with a compile-time-fixed number of decimal places.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct DecimalSignedConstAbi<DECIMALS: Unsigned> {
    _phantom: PhantomData<DECIMALS>,
}

impl<DECIMALS: Unsigned> TypeAbiFrom<Self> for DecimalSignedConstAbi<DECIMALS> {}

impl<DECIMALS: Unsigned> TypeAbi for DecimalSignedConstAbi<DECIMALS> {
    type Unmanaged = Self;
    type Abi = Self;

    fn type_name() -> TypeName {
        format!("ManagedDecimalSigned<{}>", DECIMALS::to_usize())
    }

    fn type_name_rust() -> TypeName {
        format!("DecimalSignedConstAbi<U{}>", DECIMALS::to_usize())
    }
}
