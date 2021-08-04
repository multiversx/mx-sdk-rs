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

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::test_util::ser_deser_ok;
    use alloc::vec::Vec;
    use core::num::NonZeroUsize;

    #[test]
    fn test_top_compacted_numbers() {
        // zero
        ser_deser_ok(0u8, &[]);
        ser_deser_ok(0u16, &[]);
        ser_deser_ok(0u32, &[]);
        ser_deser_ok(0u64, &[]);
        ser_deser_ok(0usize, &[]);
        // unsigned positive
        ser_deser_ok(5u8, &[5]);
        ser_deser_ok(5u16, &[5]);
        ser_deser_ok(5u32, &[5]);
        ser_deser_ok(5u64, &[5]);
        ser_deser_ok(5usize, &[5]);
        // signed positive
        ser_deser_ok(5i8, &[5]);
        ser_deser_ok(5i16, &[5]);
        ser_deser_ok(5i32, &[5]);
        ser_deser_ok(5i64, &[5]);
        ser_deser_ok(5isize, &[5]);
        // signed negative
        ser_deser_ok(-5i8, &[251]);
        ser_deser_ok(-5i16, &[251]);
        ser_deser_ok(-5i32, &[251]);
        ser_deser_ok(-5i64, &[251]);
        ser_deser_ok(-5isize, &[251]);
        // non zero usize
        ser_deser_ok(NonZeroUsize::new(5).unwrap(), &[5]);
    }

    #[test]
    fn test_top_compacted_bool() {
        ser_deser_ok(true, &[1]);
        ser_deser_ok(false, &[]);
    }

    #[test]
    fn test_top_bytes_compacted() {
        ser_deser_ok(Vec::<u8>::new(), &[]);
        ser_deser_ok([1u8, 2u8, 3u8].to_vec(), &[1u8, 2u8, 3u8]);
    }

    #[test]
    fn test_vec_i32_compacted() {
        let v = [1i32, 2i32, 3i32].to_vec();
        let expected: &[u8] = &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3];
        ser_deser_ok(v, expected);
    }

    #[test]
    fn test_option_vec_i32() {
        let some_v = Some([1i32, 2i32, 3i32].to_vec());
        let expected: &[u8] = &[
            /*opt*/ 1, /*size*/ 0, 0, 0, 3, /*data*/ 0, 0, 0, 1, 0, 0, 0, 2, 0, 0,
            0, 3,
        ];
        ser_deser_ok(some_v, expected);

        let none_v: Option<Vec<i32>> = None;
        ser_deser_ok(none_v, &[]);
    }
}
