use multiversx_sc::api::ManagedTypeApi;
use multiversx_sc::types::{BigInt, BigUint};
use crate::data::types::native::NativeConvertible;

impl<M: ManagedTypeApi> NativeConvertible for BigInt<M> {
    type Native = num_bigint::BigInt;

    fn to_native(&self) -> Self::Native {
        num_bigint::BigInt::from_signed_bytes_be(self.to_signed_bytes_be().as_slice())
    }
}

impl<M: ManagedTypeApi> NativeConvertible for BigUint<M> {
    type Native = num_bigint::BigUint;

    fn to_native(&self) -> Self::Native {
        num_bigint::BigUint::from_bytes_be(self.to_bytes_be().as_slice())
    }
}