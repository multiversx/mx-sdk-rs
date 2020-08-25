
#![no_std]

#![allow(unused_imports)]
#![allow(stable_features)]

// Required to replace the global allocator.
#![feature(global_allocator)]
#![feature(alloc_error_handler, lang_items)]

// Required to use the `alloc` crate and its types, the `abort` intrinsic, and a
// custom panic handler.
#![feature(alloc, core_intrinsics)]

#![feature(panic_info_message)]

// Use `wee_alloc` as the global allocator.
// more info: https://os.phil-opp.com/heap-allocation/#local-and-static-variables
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_use]
extern crate alloc;

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


#[cfg(feature = "wasm-output-mode")]
#[alloc_error_handler]
fn alloc_error_handler(_layout: alloc::alloc::Layout) -> ! {
    elrond_wasm_node::ext_error::signal_error(&b"allocation error"[..])
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

#[cfg(feature = "wasm-output-mode")]
#[panic_handler]
fn panic_fmt(info: &core::panic::PanicInfo) -> ! {
    use alloc::string::String;
    let panic_msg =
        if let Some(s) = info.message() {
            format!("panic occurred: {:?}", s)
        } else {
            String::from("unknown panic occurred")
        };

    elrond_wasm_node::ext_error::signal_error(panic_msg.as_bytes())
}

#[cfg(feature = "wasm-output-mode")]
#[lang = "eh_personality"] fn eh_personality() {}
