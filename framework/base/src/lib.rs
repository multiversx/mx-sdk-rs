#![no_std]
#![allow(deprecated)]
pub use multiversx_sc_derive::{self as derive, contract, module, proxy};

// re-export basic heap types
extern crate alloc;

/// The current version of `multiversx_sc_codec`, re-exported.
pub use multiversx_sc_codec as codec;

// Re-exporting the VM-core, for convenience.
pub use multiversx_chain_core as chain_core;

/// Reexported for convenience.
pub use crate::codec::arrayvec;

/// Reexported for convenience.
pub use generic_array::typenum;

// TEMP: should adjust imports.
pub use multiversx_sc_abi as abi;

/// Must be called from here, to capture the framework crate info, and not the abi crate info.
pub fn framework_build_abi() -> abi::FrameworkBuildAbi {
    abi::FrameworkBuildAbi::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    )
}

pub mod api;
pub mod contract_base;
pub mod err_msg;
pub mod external_view_contract;
pub mod formatter;
pub mod hex_call_data;
pub mod io;
pub mod log_util;
mod macros;
pub mod non_zero_util;
pub mod storage;
pub mod tuple_util;
pub mod types;

#[cfg(feature = "std")]
mod std_impl;

pub use hex_call_data::*;
pub use storage::{storage_clear, storage_get, storage_get_len, storage_set};

/// Conveniently groups all framework imports required by a smart contract form the framework.
pub mod imports;

/// Conveniently groups all imports required for deriving framework-related traits for types.
pub mod derive_imports;

/// Conveniently groups all imports required for generated proxies.
pub mod proxy_imports {
    pub use super::{derive_imports::*, imports::*};
}
