pub use alloc::alloc::Layout;
pub use core::panic::PanicInfo;
pub use wee_alloc::WeeAlloc;

pub fn alloc_error_handler(_layout: Layout) -> ! {
    crate::error_hook::signal_error(&b"allocation error"[..])
}

pub fn panic_fmt(_: &PanicInfo) -> ! {
    crate::error_hook::signal_error(multiversx_sc::err_msg::PANIC_OCCURRED.as_bytes())
}

pub fn panic_fmt_with_message(panic_info: &PanicInfo) -> ! {
    use alloc::string::String;
    let panic_msg = if let Some(s) = panic_info.message() {
        alloc::format!("panic occurred: {s:?}")
    } else {
        String::from("unknown panic occurred")
    };

    crate::error_hook::signal_error(panic_msg.as_bytes())
}
