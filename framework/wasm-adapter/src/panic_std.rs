use crate::panic::{ManagedPanicMessage, signal_error_with_managed_panic_message};
pub use alloc::alloc::Layout;

pub fn set_panic_hook() {
    std::panic::set_hook(std::boxed::Box::new(panic_hook_std));
}

fn panic_hook_std(_: &std::panic::PanicHookInfo<'_>) {
    crate::error_hook::signal_error("panic occurred".as_bytes())
}

/// Sets a panic handler that formats and sends the original message.
///
/// Mostly used for debugging, will increase code size.
pub fn set_panic_hook_with_message() {
    std::panic::set_hook(std::boxed::Box::new(panic_hook_std_with_message));
}

fn panic_hook_std_with_message(info: &std::panic::PanicHookInfo<'_>) {
    let mut panic_msg = ManagedPanicMessage::default();
    panic_msg.append_str("panic occurred: ");

    core::fmt::write(&mut panic_msg, format_args!("{info}"))
        .unwrap_or_else(|_| panic_msg.append_str("unable to write panic"));

    signal_error_with_managed_panic_message(panic_msg);
}
