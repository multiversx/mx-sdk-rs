// Need to provide a tiny `panic_fmt` lang-item implementation for `#![no_std]`.
// This implementation will translate panics into traps in the resulting
// WebAssembly.
// #[lang = "panic_fmt"]
// extern "C" fn panic_fmt(
//     _args: ::core::fmt::Arguments,
//     _file: &'static str,
//     _line: u32
// ) -> ! {
//     use core::intrinsics;
//     unsafe {
//         intrinsics::abort();
//     }
// }

#[alloc_error_handler]
fn alloc_error_handler(_layout: alloc::alloc::Layout) -> ! {
    crate::error_hook::signal_error(&b"allocation error"[..])
}

// for future reference, the PanicInfo struct looks like this:
// PanicInfo {
//     payload: Any,
//     message: Some(
//         example panic message,
//     ),
//     location: Location {
//         file: "features/src/lib.rs",
//         line: 19,
//         col: 9,
//     },
// }

#[cfg(feature = "panic-message")]
#[panic_handler]
fn panic_fmt(panic_info: &core::panic::PanicInfo) -> ! {
    use alloc::string::String;
    let panic_msg = if let Some(s) = panic_info.message() {
        alloc::format!("panic occurred: {:?}", s)
    } else {
        String::from("unknown panic occurred")
    };

    crate::error_hook::signal_error(panic_msg.as_bytes())
}

#[cfg(not(feature = "panic-message"))]
#[panic_handler]
fn panic_fmt(_: &core::panic::PanicInfo) -> ! {
    crate::error_hook::signal_error(&b"panic occurred"[..])
}

#[lang = "eh_personality"]
fn eh_personality() {}
