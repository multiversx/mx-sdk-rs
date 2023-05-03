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
    if let Some(args) = panic_info.message() {
        panic_msg.append_str("panic occurred: ");
        let _ = core::fmt::write(&mut panic_msg, *args);
    } else {
        panic_msg.append_str("unknown panic occurred");
    };

    VmApiImpl::error_api_impl().signal_error_from_buffer(panic_msg.buffer.get_handle())
}

#[derive(Default)]
struct ManagedPanicMessage {
    buffer: ManagedBuffer<VmApiImpl>,
}

impl ManagedPanicMessage {
    fn append_str(&mut self, s: &str) {
        self.buffer.append_bytes(s.as_bytes());
    }
}

impl core::fmt::Write for ManagedPanicMessage {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.append_str(s);
        Ok(())
    }
}
