use multiversx_sc::{
    api::ManagedTypeApi,
    codec::num_bigint,
    types::{BigFloat, BigInt, BigUint, NonZeroBigUint},
};

use crate::{api::StaticApi, facade::result_handlers::HasUnmanaged};

impl HasUnmanaged for num_bigint::BigUint {
    type Unmanaged = Self;
}

impl HasUnmanaged for num_bigint::BigInt {
    type Unmanaged = Self;
}

impl<M: ManagedTypeApi> HasUnmanaged for BigUint<M> {
    type Unmanaged = num_bigint::BigUint;
}

impl<M: ManagedTypeApi> HasUnmanaged for BigInt<M> {
    type Unmanaged = num_bigint::BigInt;
}

impl<M: ManagedTypeApi> HasUnmanaged for BigFloat<M> {
    type Unmanaged = f64;
}

impl<M: ManagedTypeApi> HasUnmanaged for NonZeroBigUint<M> {
    type Unmanaged = NonZeroBigUint<StaticApi>;
}
