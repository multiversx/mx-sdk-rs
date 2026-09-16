use alloc::{string::String, vec::Vec};

#[cfg(feature = "num-bigint")]
use crate::abi::{BigIntAbi, BigUintAbi};
use crate::{
    abi::{BigFloatAbi, BytesReadToEndAbi, CountedVariadicAbi, ListAbi, StringAbi, VariadicAbi},
    codec::{MultiValueConstLength, multi_types::MultiValueVec},
};

use crate::types::HasUnmanaged;

// Plain, non-managed unmanaged counterparts, when one is available.

impl HasUnmanaged for BigFloatAbi {
    type Unmanaged = f64;
}

#[cfg(feature = "num-bigint")]
impl HasUnmanaged for BigIntAbi {
    type Unmanaged = crate::codec::num_bigint::BigInt;
}

#[cfg(feature = "num-bigint")]
impl HasUnmanaged for BigUintAbi {
    type Unmanaged = crate::codec::num_bigint::BigUint;
}

impl HasUnmanaged for BytesReadToEndAbi {
    type Unmanaged = Vec<u8>;
}

impl HasUnmanaged for StringAbi {
    type Unmanaged = String;
}

// The other pure-ABI marker types (`EgldOrEsdtTokenIdentifierAbi`, `TokenIdAbi`, `DecimalAbi`,
// etc.) have no type parameter to condition an unmanaged form on, and no plain Rust counterpart
// either, so they deliberately don't implement `HasUnmanaged` here. Their managed types
// implement it directly (see `has_unmanaged_tokens.rs`/`has_unmanaged_managed.rs`), gated on
// `UnmanagedApi`.

// Container ABI markers: resolve in terms of their item's unmanaged counterpart.

impl<T: HasUnmanaged> HasUnmanaged for ListAbi<T> {
    type Unmanaged = Vec<T::Unmanaged>;
}

impl<T: HasUnmanaged> HasUnmanaged for VariadicAbi<T> {
    type Unmanaged = MultiValueVec<T::Unmanaged>;
}

impl<T: HasUnmanaged + MultiValueConstLength> HasUnmanaged for CountedVariadicAbi<T> {
    type Unmanaged = MultiValueVec<T::Unmanaged>;
}
