use crate::{BigUintPrinter, DebugApi};
use elrond_wasm::{
    api::{PrintApi, PrintApiImpl},
    types::BigUint,
};

impl PrintApi for DebugApi {
    type PrintApiImpl = DebugApi;

    fn print_api_impl() -> Self::PrintApiImpl {
        DebugApi::new_from_static()
    }
}

impl PrintApiImpl for DebugApi {
    type ManagedTypeApi = DebugApi;

    fn print_biguint(&self, biguint: &BigUint<Self>) {
        println!(
            "{:?}",
            BigUintPrinter {
                value: biguint.clone()
            }
        );
    }
}
