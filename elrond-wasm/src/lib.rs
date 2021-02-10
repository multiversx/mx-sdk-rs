#![no_std]

// re-export basic heap types
extern crate alloc;
pub use alloc::boxed::Box;
pub use alloc::string::String;
pub use alloc::vec::Vec;

pub use elrond_codec;

pub mod abi;
pub mod api;
pub mod err_msg;
pub mod hex_call_data;
pub mod io;
mod macros;
pub mod non_zero_util;
mod proxy;
pub mod storage;
pub mod types;

pub use hex_call_data::*;
pub use io::*;
pub use proxy::OtherContractHandle;
pub use storage::{storage_get, storage_set};
pub use types::*;
