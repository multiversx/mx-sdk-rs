#[cfg(feature = "num-bigint")]
use crate::types::{BigInt, BigUint};
use crate::{
    api::{ManagedTypeApi, UnmanagedApi},
    types::{BigFloat, NonZeroBigUint},
};

use crate::types::HasUnmanaged;

#[cfg(feature = "num-bigint")]
impl HasUnmanaged for crate::codec::num_bigint::BigUint {
    type Unmanaged = Self;
}

#[cfg(feature = "num-bigint")]
impl HasUnmanaged for crate::codec::num_bigint::BigInt {
    type Unmanaged = Self;
}

#[cfg(feature = "num-bigint")]
impl<M: ManagedTypeApi> HasUnmanaged for BigUint<M> {
    type Unmanaged = crate::codec::num_bigint::BigUint;
}

#[cfg(feature = "num-bigint")]
impl<M: ManagedTypeApi> HasUnmanaged for BigInt<M> {
    type Unmanaged = crate::codec::num_bigint::BigInt;
}

impl<M: ManagedTypeApi> HasUnmanaged for BigFloat<M> {
    type Unmanaged = f64;
}

impl<M: UnmanagedApi> HasUnmanaged for NonZeroBigUint<M> {
    type Unmanaged = Self;
}
