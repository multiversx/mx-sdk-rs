#![no_std]

extern crate alloc;

mod bytes_ser;
mod bytes_de;
mod bytes_err;

pub use bytes_ser::{to_bytes, u64_to_bytes};
pub use bytes_de::{from_bytes, bytes_to_number};
pub use bytes_err::SDError;

#[cfg(test)]
pub mod tests {
    use super::*;
    use core::fmt::Debug;
    use alloc::vec::Vec;
    use serde::{Serialize, Deserialize};

    pub fn the_same<V>(element: V)
    where
        V: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    {
        let serialized_bytes = to_bytes(&element).unwrap();
        let deserialized: V = from_bytes(serialized_bytes.as_slice()).unwrap();
        assert_eq!(deserialized, element);
    }

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
    fn test_top_compacted_numbers() {
        // zero
        ser_deser_ok(0u8,    &[]);
        ser_deser_ok(0u16,   &[]);
        ser_deser_ok(0u32,   &[]);
        ser_deser_ok(0u64,   &[]);
        ser_deser_ok(0usize, &[]);
        // unsigned positive
        ser_deser_ok(5u8,    &[5]);
        ser_deser_ok(5u16,   &[5]);
        ser_deser_ok(5u32,   &[5]);
        ser_deser_ok(5u64,   &[5]);
        ser_deser_ok(5usize, &[5]);
        // signed positive
        ser_deser_ok(5i8,    &[5]);
        ser_deser_ok(5i16,   &[5]);
        ser_deser_ok(5i32,   &[5]);
        ser_deser_ok(5i64,   &[5]);
        ser_deser_ok(5isize, &[5]);
        // signed negative
        ser_deser_ok(-5i8,    &[251]);
        ser_deser_ok(-5i16,   &[251]);
        ser_deser_ok(-5i32,   &[251]);
        ser_deser_ok(-5i64,   &[251]);
        ser_deser_ok(-5isize, &[251]);
    }

    #[test]
    fn test_top_compacted_bool() {
        ser_deser_ok(true,    &[1]);
        ser_deser_ok(false,   &[]);
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
        let expected: &[u8] = &[/*opt*/ 1, /*size*/ 0, 0, 0, 3, /*data*/ 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3];
        ser_deser_ok(some_v, expected);

        let none_v: Option<Vec<i32>> = None;
        ser_deser_ok(none_v, &[0]);
    }

    #[test]
    fn test_struct() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct Test {
            int: u16,
            seq: Vec<u8>,
            another_byte: u8,
        }

        let test = Test {
            int: 1,
            seq: [5, 6].to_vec(),
            another_byte: 7,
        };
        the_same(test);
    }

    #[test]
    fn test_wrapped_array() {
        #[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
        struct WrappedArray([u8; 5]);

        let wa = WrappedArray([1, 2, 3, 4, 5]);
        ser_deser_ok(wa, &[1, 2, 3, 4, 5]);

        let mut v: Vec<WrappedArray> = Vec::new();
        v.push(wa);
        v.push(WrappedArray([6, 7, 8, 9, 0]));
        ser_deser_ok(v, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    }

}