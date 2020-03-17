
#![no_std]

#![allow(dead_code)]
#![allow(stable_features)]


// Required to replace the global allocator.
#![feature(global_allocator)]

#![feature(alloc_error_handler, lang_items)]

// Required to use the `alloc` crate and its types, the `abort` intrinsic, and a
// custom panic handler.
#![feature(alloc, core_intrinsics)]

#![feature(panic_info_message)]

mod ext;
mod ext_int64;
mod big_int;
mod big_uint;

pub use ext::*;
pub use ext_int64::*;
pub use big_int::*;
pub use big_uint::*;

#[macro_use]
extern crate alloc;
pub use alloc::boxed::Box;
pub use alloc::vec::Vec;
pub use alloc::string::String;

// Use `wee_alloc` as the global allocator.
// more info: https://os.phil-opp.com/heap-allocation/#local-and-static-variables
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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




extern {
    fn signalError(messageOffset: *const u8, messageLength: i32) -> !;
}

#[cfg(target_arch = "wasm32")]
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
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

#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic_fmt(info: &core::panic::PanicInfo) -> ! {
    let panic_msg =
        if let Some(s) = info.message() {
            format!("panic occurred: {:?}", s)
        } else {
            String::from("unknown panic occurred")
        };

    unsafe {
        signalError(panic_msg.as_ptr(), panic_msg.len() as i32) 
    }
}

#[cfg(target_arch = "wasm32")]
#[lang = "eh_personality"] fn eh_personality() {}

