#![no_std]
#![feature(panic_info_message)]

// Allows us to use alloc::vec::Vec;
// TODO: get rid of the legacy API and also of this.
extern crate alloc;

pub use multiversx_sc;

pub mod api;
pub mod error_hook;
pub mod wasm_deps;
mod wasm_macros;
