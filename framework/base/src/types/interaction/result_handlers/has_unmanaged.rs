mod has_unmanaged_basic;
mod has_unmanaged_managed;
mod has_unmanaged_multi_value;
mod has_unmanaged_numbers;
mod has_unmanaged_pure_abi;
mod has_unmanaged_tokens;
mod has_unmanaged_vm_core;

use crate::abi::TypeAbi;

/// Gives the "unmanaged" representation of a type: the plain Rust equivalent, without any
/// managed API type parameters, e.g. `num_bigint::BigUint` for `BigUint<M>`.
///
/// Only implemented for the types that [`ReturnsResultUnmanaged`](super::ReturnsResultUnmanaged)
/// is realistically used with. It is not, and does not need to be, implemented for all
/// [`TypeAbi`] types.
///
/// Custom types that are generic over a [`ManagedTypeApi`](crate::api::ManagedTypeApi) (e.g. a
/// struct defined in a smart contract) can implement this trait for themselves, with
/// `type Unmanaged = Self`, restricted to APIs that implement
/// [`UnmanagedApi`](crate::api::UnmanagedApi):
///
/// ```ignore
/// impl<M: UnmanagedApi> HasUnmanaged for MyType<M> {
///     type Unmanaged = Self;
/// }
/// ```
///
/// This is legal to write directly in the crate that defines `MyType`, since `MyType` is local
/// to it, even though both `HasUnmanaged` and `UnmanagedApi` are defined elsewhere.
pub trait HasUnmanaged: TypeAbi {
    type Unmanaged;
}
