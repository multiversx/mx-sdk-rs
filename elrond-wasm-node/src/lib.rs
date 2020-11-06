
#![no_std]
#![feature(new_uninit)]

#![allow(dead_code)] // TODO: remove

mod ext;
pub mod ext_error;
mod big_int;
mod big_uint;

pub use ext::*;
pub use big_int::*;
pub use big_uint::*;

#[macro_use]
extern crate alloc;
pub use alloc::boxed::Box;
pub use alloc::vec::Vec;
pub use alloc::string::String;
