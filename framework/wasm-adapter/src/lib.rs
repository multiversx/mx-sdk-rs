#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use multiversx_sc;

pub mod api;
pub mod error_hook;
pub mod panic;
pub mod wasm_alloc;
mod wasm_macros;

#[cfg(feature = "std")]
pub mod panic_std;
