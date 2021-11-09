use elrond_wasm::{api::PrintApi, types::BoxedBytes};
use num_bigint::BigInt;

use crate::DebugApi;
use elrond_wasm::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedType},
};

use std::fmt;

struct BigUintPrinter<M: ManagedTypeApi> {
    value: BigUint<M>,
}

impl<M: ManagedTypeApi> fmt::Debug for BigUintPrinter<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let handle = self.value.get_raw_handle();
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

impl PrintApi for DebugApi {
    fn print_biguint(&self, biguint: &BigUint<Self>) {
        println!(
            "{:?}",
            BigUintPrinter {
                value: biguint.clone()
            }
        );
    }
}
