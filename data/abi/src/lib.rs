// re-export basic heap types
extern crate alloc;

mod contract_abi;
mod types;

pub use contract_abi::*;
pub use types::*;

/// The current version of `multiversx_sc_codec`, re-exported.
pub use multiversx_sc_codec as codec;

/// Re-exported for easier import in derive macros.
pub use alloc::vec::Vec;
