#![no_std]
#![feature(never_type)]
#![feature(exhaustive_patterns)]
#![feature(try_trait_v2)]
#![feature(control_flow_enum)]
#![allow(clippy::type_complexity)]
#![allow(deprecated)]
#![feature(generic_associated_types)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]

pub use elrond_wasm_derive::{self as derive, contract, module, proxy};

// re-export basic heap types
extern crate alloc;

/// Reexported for convenience.
pub use elrond_codec::arrayvec;

pub use elrond_codec;

pub mod abi;
pub mod api;
pub mod contract_base;
pub mod err_msg;
pub mod esdt;
pub mod external_view_contract;
pub mod formatter;
pub mod hex_call_data;
pub mod io;
pub mod log_util;
mod macros;
pub mod non_zero_util;
pub mod storage;
pub mod types;

pub use hex_call_data::*;
pub use hex_literal;
pub use storage::{storage_clear, storage_get, storage_get_len, storage_set};
