#![no_std]
#![feature(new_uninit)]
// Required to replace the global allocator.
#![feature(alloc_error_handler, lang_items)]
// Only relevant if the `panic-message` flag is on.
#![feature(panic_info_message)]

// Allows us to use alloc::vec::Vec;
// TODO: get rid of the legacy API and also of this.
extern crate alloc;

mod api;
pub mod error_hook;
mod node_macros;

#[cfg(feature = "wasm-output-mode")]
mod wasm_alloc;

#[cfg(feature = "wasm-output-mode")]
mod wasm_panic;

pub use api::VmApiImpl;

pub use mx_sc;

/// Provides an API instance.
pub fn vm_api() -> VmApiImpl {
    VmApiImpl {}
}
