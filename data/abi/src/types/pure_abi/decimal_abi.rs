use crate::{TypeAbi, TypeAbiFrom, TypeName};
use alloc::format;
use core::marker::PhantomData;
use typenum::Unsigned;

/// Pure ABI counterpart of `ManagedDecimal<M, NumDecimals>` (variable number of decimals).
///
/// Provides a stable, framework-agnostic type representation for fixed-point decimals
/// with a runtime-determined number of decimal places.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct DecimalAbi;

impl TypeAbiFrom<Self> for DecimalAbi {}

impl TypeAbi for DecimalAbi {
    type Unmanaged = Self;
    type Abi = Self;

    fn type_name() -> TypeName {
        TypeName::from("ManagedDecimal<usize>")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("DecimalAbi")
    }
}

/// Pure ABI counterpart of `ManagedDecimal<M, ConstDecimals<DECIMALS>>` (compile-time fixed number of decimals).
///
/// The type-level number `DECIMALS` encodes the number of decimal places at the type level,
/// mirroring `ConstDecimals<DECIMALS>`'s own `Unsigned` type parameter - unlike a plain
/// `const usize` parameter, this can be substituted directly from `ManagedDecimal`'s own
/// `DECIMALS: Unsigned` type parameter in its `TypeAbi::Abi` projection, with no const-generic
/// conversion (which would require the unstable `generic_const_exprs` feature).
///
/// Provides a stable, framework-agnostic type representation for fixed-point decimals
/// with a compile-time-fixed number of decimal places.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
pub struct DecimalConstAbi<DECIMALS: Unsigned> {
    _phantom: PhantomData<DECIMALS>,
}

impl<DECIMALS: Unsigned> TypeAbiFrom<Self> for DecimalConstAbi<DECIMALS> {}

impl<DECIMALS: Unsigned> TypeAbi for DecimalConstAbi<DECIMALS> {
    type Unmanaged = Self;
    type Abi = Self;

    fn type_name() -> TypeName {
        format!("ManagedDecimal<{}>", DECIMALS::to_usize())
    }

    fn type_name_rust() -> TypeName {
        format!("DecimalConstAbi<U{}>", DECIMALS::to_usize())
    }
}
