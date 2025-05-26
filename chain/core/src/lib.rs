#![no_std]

extern crate alloc;

pub mod builtin_func_names;
pub mod token_identifier_util;
pub mod types;

/// Re-exported for convenience.
pub use multiversx_sc_codec as codec;

/// The equivalent ESDT token identifier for transferring EGLD, the native MultiversX token.
pub const EGLD_000000_TOKEN_IDENTIFIER: &str = "EGLD-000000";
