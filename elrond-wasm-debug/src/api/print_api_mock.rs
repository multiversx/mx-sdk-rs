use crate::{BigUintPrinter, DebugApi};
use elrond_wasm::{
    api::{Handle, ManagedBufferApi, PrintApi, PrintApiImpl},
    types::{BigUint, ManagedType},
};

impl PrintApi for DebugApi {
    type PrintApiImpl = DebugApi;

    fn print_api_impl() -> Self::PrintApiImpl {
        DebugApi::new_from_static()
    }
}

impl PrintApiImpl for DebugApi {
    fn print_biguint(&self, bu_handle: Handle) {
        println!(
            "{:?}",
            BigUintPrinter {
                value: BigUint::<Self>::from_raw_handle(bu_handle)
            }
        );
    }

    fn print_managed_buffer(&self, mb_handle: Handle) {
        let bytes = self.mb_to_boxed_bytes(mb_handle);
        let s = String::from_utf8_lossy(bytes.as_slice());
        println!("{:?}", s);
    }
}
