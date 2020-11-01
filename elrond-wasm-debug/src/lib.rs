#![allow(dead_code)]
#![allow(unused_imports)]

mod ext_mock;
mod big_int_mock;
mod big_uint_mock;
mod contract_map;
mod execute_mandos;
mod display_util;
mod blockchain_mock;
mod async_data;

pub use ext_mock::*;
pub use big_int_mock::*;
pub use big_uint_mock::*;
pub use contract_map::*;
pub use execute_mandos::*;
pub use display_util::*;
pub use blockchain_mock::*;
pub use async_data::*;

#[macro_use]
extern crate alloc;
pub use alloc::boxed::Box;
pub use alloc::vec::Vec;

pub use std::collections::HashMap;

#[cfg(test)]
mod elrond_codec_tests {
    use super::*;
    use elrond_wasm::elrond_codec::*;
    use core::fmt::Debug;

    pub fn ser_deser_ok<V>(element: V, expected_bytes: &[u8])
    where
        V: TopEncode + TopDecode + PartialEq + Debug + 'static,
    {
        // serialize
        let serialized_bytes = top_encode_to_vec(&element).unwrap();
        assert_eq!(serialized_bytes.as_slice(), expected_bytes);

        // deserialize
        let deserialized: V = V::top_decode(&serialized_bytes[..], |res| res.unwrap());
        assert_eq!(deserialized, element);
    }

    #[test]
    fn test_big_int_serialization() {
        ser_deser_ok(RustBigInt::from(5), &[5u8]);
        ser_deser_ok(RustBigInt::from(-5), &[251u8]);
    }

    #[test]
    fn test_big_uint_serialization() {
        ser_deser_ok(RustBigUint::from(5u32), &[5u8]);
    }
}
