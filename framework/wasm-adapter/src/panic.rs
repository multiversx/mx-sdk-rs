use crate::api::VmApiImpl;
pub use alloc::alloc::Layout;
use multiversx_sc::{
    api::{ErrorApi, ErrorApiImpl},
    types::{ManagedBuffer, ManagedRef, ManagedType},
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

    // downcasting the payload (default panic behavior in std) doesn't work in this scenario
    // the payload is always a dummy value
    // panic runtime is irrelevant for no_std applications
    // let payload = if let Some(payload) = panic_info.payload().downcast_ref::<&'static str>() {
    //     *payload
    // } else {
    //     "unknown panic occurred"
    // };
    // panic_msg.append_str(payload);

    // write full panic
    core::fmt::write(&mut panic_msg, format_args!("{:?}", panic_info))
        .expect("Failed to write panic payload");

    // take str
    // let str = format_args!("{:?}", panic_info).as_str();

    // extract message
    // let message = extract_panic_message(full_panic_str);

    // overwrite buf
    // match message {
    //     Some(val) => panic_msg.overwrite(val.as_bytes()),
    //     None => panic_msg.overwrite(b"unknown panic"),
    // }
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

    fn _overwrite(&mut self, s: &str) {
        self.buffer.overwrite(s.as_bytes());
    }

    fn _buffer(&self) -> ManagedRef<VmApiImpl, ManagedBuffer<VmApiImpl>> {
        self.buffer.as_ref()
    }
}

impl core::fmt::Write for ManagedPanicMessage {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.append_str(s);
        Ok(())
    }
}
