use crate::DebugApi;
use multiversx_sc::{
    api::{PrintApi, PrintApiImpl},
    types::ManagedBufferCachedBuilder,
};

impl PrintApi for DebugApi {
    type PrintApiImpl = DebugApi;

    fn print_api_impl() -> Self::PrintApiImpl {
        DebugApi::new_from_static()
    }
}

impl PrintApiImpl for DebugApi {
    type Buffer = ManagedBufferCachedBuilder<DebugApi>;

    fn print_buffer(&self, buffer: Self::Buffer) {
        let bytes = buffer.into_managed_buffer().to_boxed_bytes();
        let s = String::from_utf8_lossy(bytes.as_slice());
        println!("{:?}", &s);
        self.printed_messages.borrow_mut().push(s.into_owned());
    }
}
