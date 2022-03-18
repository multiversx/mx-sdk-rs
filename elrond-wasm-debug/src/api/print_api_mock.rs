use crate::{BigUintPrinter, DebugApi};
use elrond_wasm::{
    api::{Handle, PrintApi, PrintApiImpl},
    types::{BigUint, ManagedBufferCachedBuilder, ManagedType},
};

impl PrintApi for DebugApi {
    type PrintApiImpl = DebugApi;

    fn print_api_impl() -> Self::PrintApiImpl {
        DebugApi::new_from_static()
    }
}

impl PrintApiImpl for DebugApi {
    type PrintFormatBuffer = ManagedBufferCachedBuilder<DebugApi>;

    fn print_biguint(&self, bu_handle: Handle) {
        println!(
            "{:?}",
            BigUintPrinter {
                value: BigUint::<Self>::from_raw_handle(bu_handle)
            }
        );
    }

    fn print_buffer(&self, buffer: Self::PrintFormatBuffer) {
        let bytes = buffer.into_managed_buffer().to_boxed_bytes();
        let s = String::from_utf8_lossy(bytes.as_slice());
        println!("{:?}", &s);
        self.printed_messages.borrow_mut().push(s.into_owned());
    }
}
