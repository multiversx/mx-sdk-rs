use crate::{ConstDecimals, Decimals, NumDecimals, TypeAbi, TypeAbiFrom, TypeName};
use alloc::format;
use core::marker::PhantomData;
use typenum::Unsigned;

/// Pure ABI counterpart of `ManagedDecimal<M, D>`.
///
/// Provides a stable, framework-agnostic type representation for fixed-point decimals.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
///
/// Mirrors `ManagedDecimal`'s own `D: Decimals` type parameter, so that `DecimalAbi<NumDecimals>`
/// and `DecimalAbi<ConstDecimals<DECIMALS>>` map directly to `ManagedDecimal<M, NumDecimals>`
/// and `ManagedDecimal<M, ConstDecimals<DECIMALS>>` respectively.
pub struct DecimalAbi<D: Decimals> {
    _phantom: PhantomData<D>,
}

impl<D: Decimals> TypeAbiFrom<Self> for DecimalAbi<D> {}

impl TypeAbi for DecimalAbi<NumDecimals> {
    type Unmanaged = Self;
    type Abi = Self;

    fn type_name() -> TypeName {
        TypeName::from("ManagedDecimal<usize>")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("DecimalAbi<NumDecimals>")
    }
}

impl<DECIMALS: Unsigned> TypeAbi for DecimalAbi<ConstDecimals<DECIMALS>> {
    type Unmanaged = Self;
    type Abi = Self;

    fn type_name() -> TypeName {
        format!("ManagedDecimal<{}>", DECIMALS::to_usize())
    }

    fn type_name_rust() -> TypeName {
        format!("DecimalAbi<ConstDecimals<U{}>>", DECIMALS::to_usize())
    }
}
