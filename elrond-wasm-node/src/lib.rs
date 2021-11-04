#![no_std]
#![feature(new_uninit)]

mod api;
mod node_macros;
pub mod error_hook;

extern crate alloc;
pub use alloc::{boxed::Box, string::String, vec::Vec};

pub use api::VmApiImpl;

/// Provides an API instance.
pub fn vm_api() -> VmApiImpl {
    VmApiImpl {}
}
