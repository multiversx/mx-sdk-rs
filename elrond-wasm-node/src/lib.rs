#![no_std]
#![feature(new_uninit)]

mod api;
pub mod error_hook;

extern crate alloc;
pub use alloc::{boxed::Box, string::String, vec::Vec};

pub use api::ArwenApiImpl;

/// Provides an API instance.
pub fn arwen_api() -> ArwenApiImpl {
    ArwenApiImpl {}
}
