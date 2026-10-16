mod has_managed_basic;
mod has_managed_managed;
mod has_managed_multi_value;
mod has_managed_pure_abi;
mod has_managed_tokens;
mod has_managed_vm_core;

use crate::{abi::TypeAbi, api::ManagedTypeApi};

/// Gives the canonical managed representation of a type under a specific `ManagedTypeApi`: the
/// framework managed type it decodes into, e.g. `BigUint<M>` for `BigUintAbi`.
///
/// The complement of [`HasUnmanaged`](super::HasUnmanaged): where `HasUnmanaged` erases a
/// type's managed API parameter into a plain Rust value, `HasManaged` goes the other way,
/// reconstructing the real managed type for a chosen `M` — most useful when `Original` is a
/// pure ABI marker type (`BigUintAbi`, `ListAbi<T>`, ...), e.g. when calling another contract
/// purely by its ABI, with no dependency on its Rust types.
///
/// Only implemented for the types [`ReturnsResultManaged`](super::ReturnsResultManaged) is
/// realistically used with. It is not, and does not need to be, implemented for all [`TypeAbi`]
/// types.
pub trait HasManaged<M: ManagedTypeApi>: TypeAbi {
    type Managed;
}
