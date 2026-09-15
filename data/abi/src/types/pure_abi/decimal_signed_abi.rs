use crate::{ConstDecimals, Decimals, NumDecimals, TypeAbi, TypeAbiFrom, TypeName};
use alloc::format;
use core::marker::PhantomData;
use typenum::Unsigned;

/// Pure ABI counterpart of `ManagedDecimalSigned<M, D>`.
///
/// Provides a stable, framework-agnostic type representation for signed fixed-point decimals.
/// Using this type ensures ABI compatibility across multiple versions of the framework
/// or across different framework implementations entirely.
///
/// Mirrors `ManagedDecimalSigned`'s own `D: Decimals` type parameter, so that
/// `DecimalSignedAbi<NumDecimals>` and `DecimalSignedAbi<ConstDecimals<DECIMALS>>` map directly
/// to `ManagedDecimalSigned<M, NumDecimals>` and `ManagedDecimalSigned<M, ConstDecimals<DECIMALS>>`
/// respectively.
pub struct DecimalSignedAbi<D: Decimals> {
    _phantom: PhantomData<D>,
}

impl<D: Decimals> TypeAbiFrom<Self> for DecimalSignedAbi<D> {}

impl TypeAbi for DecimalSignedAbi<NumDecimals> {
    type Unmanaged = Self;
    type Abi = Self;

    fn type_name() -> TypeName {
        TypeName::from("ManagedDecimalSigned<usize>")
    }

    fn type_name_rust() -> TypeName {
        TypeName::from("DecimalSignedAbi<NumDecimals>")
    }
}

impl<DECIMALS: Unsigned> TypeAbi for DecimalSignedAbi<ConstDecimals<DECIMALS>> {
    type Unmanaged = Self;
    type Abi = Self;

    fn type_name() -> TypeName {
        format!("ManagedDecimalSigned<{}>", DECIMALS::to_usize())
    }

    fn type_name_rust() -> TypeName {
        format!("DecimalSignedAbi<ConstDecimals<U{}>>", DECIMALS::to_usize())
    }
}
