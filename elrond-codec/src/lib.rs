#![no_std]

extern crate alloc;

#[cfg(feature = "elrond-codec-derive")]
pub use elrond_codec_derive;

/// Reexport needed by derive.
pub use alloc::vec::Vec;

mod codec_err;
mod default_traits;
mod impl_for_types;
mod nested_de;
mod nested_de_input;
mod nested_ser;
mod nested_ser_output;
mod num_conv;
pub mod test_util;
mod top_de;
mod top_de_input;
mod top_ser;
mod top_ser_output;
mod transmute;

pub use crate::nested_de_input::NestedDecodeInput;
pub use crate::nested_ser_output::NestedEncodeOutput;
pub use crate::num_conv::{bytes_to_number, top_encode_number_to_output, using_encoded_number};
pub use codec_err::{DecodeError, EncodeError};
pub use default_traits::{DecodeDefault, EncodeDefault};
pub use nested_de::{dep_decode_from_byte_slice, dep_decode_from_byte_slice_or_exit, NestedDecode};
pub use nested_ser::{dep_encode_to_vec, NestedEncode, NestedEncodeNoErr};
pub use top_de::{top_decode_from_nested, top_decode_from_nested_or_exit, TopDecode};
pub use top_de_input::TopDecodeInput;
pub use top_ser::{
    top_encode_from_nested, top_encode_from_nested_or_exit, top_encode_no_err, top_encode_to_vec,
    TopEncode,
};
pub use top_ser_output::TopEncodeOutput;
pub use transmute::{boxed_slice_into_vec, vec_into_boxed_slice};

/// !INTERNAL USE ONLY!
///
/// This enum provides type information to optimize encoding/decoding by doing fake specialization.
#[doc(hidden)]
#[allow(clippy::upper_case_acronyms)]
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
    BigUint,
    BigInt,
    Unit,
}
