use std::cell::RefCell;

use multiversx_sc::{
    api::{PrintApi, PrintApiImpl},
    types::ManagedBufferCachedBuilder,
};

use crate::api::{VMHooksApi, VMHooksApiBackend};

thread_local!(
    static PRINTED_MESSAGES: RefCell<Vec<String>> = RefCell::new(Vec::new())
);

impl<VHB: VMHooksApiBackend> VMHooksApi<VHB> {
    /// Clears static buffer used for testing.
    pub fn printed_messages_clear() {
        PRINTED_MESSAGES.with(|cell| cell.replace(Vec::new()));
    }

    /// Whenever using `sc_print!`, the message gets printed to console, but also gets saved in a static buffer, for testing.
    ///
    /// This method retrieves a copy of the contents of that static print message buffer.
    pub fn printed_messages() -> Vec<String> {
        PRINTED_MESSAGES.with(|cell| cell.borrow().clone())
    }
}

impl<VHB: VMHooksApiBackend> PrintApi for VMHooksApi<VHB> {
    type PrintApiImpl = Self;

    fn print_api_impl() -> Self::PrintApiImpl {
        Self::api_impl()
    }
}

impl<VHB: VMHooksApiBackend> PrintApiImpl for VMHooksApi<VHB> {
    type Buffer = ManagedBufferCachedBuilder<Self>;

    fn print_buffer(&self, buffer: Self::Buffer) {
        let bytes = buffer.into_managed_buffer().to_boxed_bytes();
        let s = String::from_utf8_lossy(bytes.as_slice());
        println!("{:?}", &s);
        PRINTED_MESSAGES.with(|cell| cell.borrow_mut().push(s.into_owned()));
    }
}
