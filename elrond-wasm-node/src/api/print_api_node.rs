use crate::VmApiImpl;
use elrond_wasm::{
    api::{PrintApi, PrintApiImpl},
    types::BigUint,
};

impl PrintApi for VmApiImpl {
    type PrintApiImpl = VmApiImpl;

    fn print_api_impl() -> Self::PrintApiImpl {
        VmApiImpl {}
    }
}

impl PrintApiImpl for VmApiImpl {
    type ManagedTypeApi = VmApiImpl;

    #[inline(always)]
    fn print_biguint(&self, _biguint: &BigUint<Self>) {}
}
