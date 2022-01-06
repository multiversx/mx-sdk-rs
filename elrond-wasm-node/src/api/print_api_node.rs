use crate::VmApiImpl;
use elrond_wasm::api::{Handle, PrintApi, PrintApiImpl};

impl PrintApi for VmApiImpl {
    type PrintApiImpl = VmApiImpl;

    fn print_api_impl() -> Self::PrintApiImpl {
        VmApiImpl {}
    }
}

impl PrintApiImpl for VmApiImpl {
    #[inline]
    fn print_biguint(&self, _bu_handle: Handle) {}
}
