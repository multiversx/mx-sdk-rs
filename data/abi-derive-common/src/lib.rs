// ensure we don't run out of macro stack
#![recursion_limit = "1024"]
// TODO: remove once minimum version is 1.87+
#![allow(unknown_lints)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::manual_is_multiple_of)]

#[macro_use]
extern crate quote;

/// Common code for derive macros, shared between `data/abi-derive` and `framework/derive`.
pub mod parse;

/// Contract-trait model, parsing, validation and ABI codegen,
/// shared between `data/abi-derive` and `framework/derive`.
pub mod contract;

mod type_abi_derive;

pub use type_abi_derive::{TypeAbiImportCrate, type_abi_derive, type_abi_full};
