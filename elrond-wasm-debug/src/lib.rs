//#![no_std]

#![allow(dead_code)]

mod ext_mock;
mod big_int_mock;

pub use ext_mock::*;
pub use big_int_mock::*;

#[macro_use]
extern crate alloc;
pub use alloc::boxed::Box;
pub use alloc::vec::Vec;

//pub use hashbrown::HashMap;
pub use std::collections::HashMap;
