#![no_std]
#![feature(new_uninit)]

mod api;
pub mod error_hook;

extern crate alloc;
pub use alloc::{boxed::Box, string::String, vec::Vec};

pub use api::VmApiImpl;

/// Provides an API instance.
pub fn arwen_api() -> VmApiImpl {
    VmApiImpl {}
}
