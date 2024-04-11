use crate::{num_bigint, num_bigint::BigInt};
use multiversx_sc::{
    api::ManagedTypeApi,
    types::{heap::BoxedBytes, BigUint, ManagedType},
};
use std::fmt;

/// Only seems to be used in tests, we can probably remove it.
pub struct BigUintPrinter<M: ManagedTypeApi> {
    pub value: BigUint<M>,
}

impl<M: ManagedTypeApi> fmt::Debug for BigUintPrinter<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let handle = self.value.get_handle();
        let mut bytes = self.value.to_bytes_be();
        if bytes.is_empty() {
            bytes = BoxedBytes::from(vec![0u8]);
        }

        let hex = hex::encode(bytes.as_slice());
        let dec = BigInt::from_bytes_be(num_bigint::Sign::Plus, bytes.as_slice());

        f.debug_struct("BigUint")
            .field("handle", &handle)
            .field("hex", &hex)
            .field("dec", &dec.to_string())
            .finish()
    }
}
