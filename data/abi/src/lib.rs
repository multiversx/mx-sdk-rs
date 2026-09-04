#![no_std]

// re-export basic heap types
extern crate alloc;

mod contract_abi;
mod contract_abi_provider;
mod proxy_abi_traits;
mod types;

pub use contract_abi::*;
pub use contract_abi_provider::*;
pub use proxy_abi_traits::*;
pub use types::*;

/// The current version of `multiversx_sc_codec`, re-exported.
pub use multiversx_sc_codec as codec;

/// Re-exported for easier import in derive macros.
pub use alloc::vec::Vec;

pub mod imports;
