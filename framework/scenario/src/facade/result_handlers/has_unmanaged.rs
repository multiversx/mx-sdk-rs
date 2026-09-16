use multiversx_sc::abi::TypeAbi;

/// Gives the "unmanaged" representation of a type: the plain Rust equivalent, without any
/// managed API type parameters, e.g. `num_bigint::BigUint` for `BigUint<M>`.
///
/// Only implemented for the types that [`ReturnsResultUnmanaged`](super::ReturnsResultUnmanaged)
/// is realistically used with. It is not, and does not need to be, implemented for all
/// [`TypeAbi`] types.
pub trait HasUnmanaged: TypeAbi {
    type Unmanaged;
}
