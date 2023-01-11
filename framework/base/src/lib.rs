#![no_std]
#![feature(never_type)]
#![feature(exhaustive_patterns)]
#![feature(try_trait_v2)]
#![feature(control_flow_enum)]
#![allow(clippy::type_complexity)]
#![allow(deprecated)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(negative_impls)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![feature(slice_partition_dedup)]
#![feature(is_sorted)]
pub use multiversx_sc_derive::{self as derive, contract, module, proxy};

// re-export basic heap types
extern crate alloc;

/// The current version of `multiversx_sc_codec`, re-exported.
pub use multiversx_sc_codec as codec;

/// Reexported for convenience.
pub use crate::codec::arrayvec;

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
