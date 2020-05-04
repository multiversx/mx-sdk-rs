//#![no_std]

#![allow(dead_code)]

mod ext_mock;
mod big_int_mock;
mod big_uint_mock;

pub use ext_mock::*;
pub use big_int_mock::*;
pub use big_uint_mock::*;

#[macro_use]
extern crate alloc;
pub use alloc::boxed::Box;
pub use alloc::vec::Vec;

//pub use hashbrown::HashMap;
pub use std::collections::HashMap;

#[cfg(test)]
mod serialization_tests {
    use super::*;
    use core::fmt::Debug;
    use elrond_wasm::serializer::{to_bytes, from_bytes};

    pub fn ser_deser_ok<V>(element: V, expected_bytes: &[u8])
    where
        V: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    {
        // serialize
        let serialized_bytes = to_bytes(&element).unwrap();
        assert_eq!(serialized_bytes.as_slice(), expected_bytes);

        // deserialize
        let deserialized: V = from_bytes(serialized_bytes.as_slice()).unwrap();
        assert_eq!(deserialized, element);
    }

    #[test]
    fn test_big_int_serialization() {
        ser_deser_ok(RustBigInt::from(5), &[5u8]);
        ser_deser_ok(RustBigInt::from(-5), &[251u8]);
    }

    #[test]
    fn test_big_uint_serialization() {
        ser_deser_ok(RustBigUint::from(5), &[5u8]);
    }
}