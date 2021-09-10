#![no_std]
#![feature(never_type)]
#![feature(try_trait_v2)]
#![feature(control_flow_enum)]
#![allow(clippy::type_complexity)]
#![allow(deprecated)]
#![allow(unused)] // TODO: temporary

#[cfg(feature = "derive")]
pub use elrond_wasm_derive::{self as derive, contract, module, proxy};

// re-export basic heap types
extern crate alloc;
pub use alloc::boxed::Box;
pub use alloc::string::String;
pub use alloc::vec::Vec;

pub use elrond_codec;

pub mod abi;
pub mod api;
pub mod err_msg;
pub mod esdt;
pub mod hex_call_data;
pub mod io;
pub mod log_util;
mod macros;
pub mod non_zero_util;
pub mod storage;
pub mod types;

pub use hex_call_data::*;
pub use io::*;
pub use storage::{storage_get, storage_set};
