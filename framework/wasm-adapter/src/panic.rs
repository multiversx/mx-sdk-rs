use crate::api::VmApiImpl;
pub use alloc::alloc::Layout;
use multiversx_sc::{
    api::{ErrorApi, ErrorApiImpl},
    types::{ManagedBuffer, ManagedType},
};

/// Also used in wasm crate macros.
pub use core::panic::PanicInfo;

/// Default panic handler for all contracts.
pub fn panic_fmt(_: &PanicInfo) -> ! {
    crate::error_hook::signal_error(multiversx_sc::err_msg::PANIC_OCCURRED.as_bytes())
}

/// Panic handler that formats and sends the original message.
///
/// Mostly used for debugging, the additional code is normally not deemed to be worth it.
pub fn panic_fmt_with_message(panic_info: &PanicInfo) -> ! {
    let mut panic_msg = ManagedPanicMessage::default();
    panic_msg.append_str("panic occurred: ");

    core::fmt::write(&mut panic_msg, format_args!("{panic_info}"))
        .unwrap_or_else(|_| panic_msg.append_str("unable to write panic"));

    signal_error_with_managed_panic_message(panic_msg)
}

pub fn signal_error_with_managed_panic_message(panic_msg: ManagedPanicMessage) -> ! {
    VmApiImpl::error_api_impl().signal_error_from_buffer(panic_msg.buffer.get_handle())
}

#[derive(Default)]
pub struct ManagedPanicMessage {
    buffer: ManagedBuffer<VmApiImpl>,
}

impl ManagedPanicMessage {
    pub fn append_str(&mut self, s: &str) {
        self.buffer.append_bytes(s.as_bytes());
    }
}

impl core::fmt::Write for ManagedPanicMessage {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let file_name = match s.rfind('/') {
            Some(index) => &s[index + 1..],
            None => s,
        };
        self.append_str(file_name);
        Ok(())
    }
}
