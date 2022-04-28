#![no_std]
#![feature(try_trait_v2)]
#![feature(never_type)]
#![feature(exhaustive_patterns)]
#![feature(auto_traits)]
#![feature(negative_impls)]

extern crate alloc;

#[cfg(feature = "elrond-codec-derive")]
pub use elrond_codec_derive;

/// Reexport needed by derive.
pub use alloc::vec::Vec;

/// Reexported for convenience.
pub use arrayvec;

/// Reexported for convenience.
#[cfg(feature = "num-bigint")]
pub use num_bigint;

// TODO: group into smaller sub-modules

mod codec_err;
mod codec_err_handler;
mod default_traits;
mod equivalent;
mod impl_for_types;
mod multi;
pub mod multi_types;
mod num_conv;
mod single;
pub mod test_util;
mod transmute;
mod try_static_cast;

pub use crate::{
    num_conv::{top_encode_number, universal_decode_number},
    try_static_cast::{
        try_cast_execute_or_else, try_cast_ref, try_execute_then_cast, TryStaticCast,
    },
};
pub use codec_err::{DecodeError, EncodeError};
pub use codec_err_handler::*;
pub use default_traits::{DecodeDefault, EncodeDefault};
pub use equivalent::*;
pub use impl_for_types::impl_empty::Empty;
pub use multi::*;
pub use single::*;

pub use transmute::{boxed_slice_into_vec, vec_into_boxed_slice};

/// !INTERNAL USE ONLY!
///
/// This enum provides type information to optimize encoding/decoding by doing fake specialization.
#[doc(hidden)]
#[allow(clippy::upper_case_acronyms)]
#[derive(PartialEq)]
pub enum TypeInfo {
    /// Default value of [`NestedEncode::TYPE_INFO`] to not require implementors to set this value in the trait.
    Unknown,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    USIZE,
    ISIZE,
    U64,
    I64,
    Bool,
    Unit,
}
