#![no_std]

extern crate alloc;

mod bytes_de;
mod bytes_err;
mod bytes_ser;

pub use bytes_de::{bytes_to_number, from_bytes};
pub use bytes_err::SDError;
pub use bytes_ser::{to_bytes, u64_to_bytes};

#[cfg(test)]
pub mod tests {
    use super::*;
    use alloc::vec::Vec;
    use core::fmt::Debug;
    use serde::{Deserialize, Serialize};

    pub fn the_same<V>(element: V)
    where
        V: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    {
        let serialized_bytes = to_bytes(&element).unwrap();
        let deserialized: V = from_bytes(serialized_bytes.as_slice()).unwrap();
        assert_eq!(deserialized, element);
    }

    pub fn check_top_encode_decode<V>(element: V, expected_bytes: &[u8])
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
        check_top_encode_decode(0u8, &[]);
        check_top_encode_decode(0u16, &[]);
        check_top_encode_decode(0u32, &[]);
        check_top_encode_decode(0u64, &[]);
        check_top_encode_decode(0usize, &[]);
        // unsigned positive
        check_top_encode_decode(5u8, &[5]);
        check_top_encode_decode(5u16, &[5]);
        check_top_encode_decode(5u32, &[5]);
        check_top_encode_decode(5u64, &[5]);
        check_top_encode_decode(5usize, &[5]);
        // signed positive
        check_top_encode_decode(5i8, &[5]);
        check_top_encode_decode(5i16, &[5]);
        check_top_encode_decode(5i32, &[5]);
        check_top_encode_decode(5i64, &[5]);
        check_top_encode_decode(5isize, &[5]);
        // signed negative
        check_top_encode_decode(-5i8, &[251]);
        check_top_encode_decode(-5i16, &[251]);
        check_top_encode_decode(-5i32, &[251]);
        check_top_encode_decode(-5i64, &[251]);
        check_top_encode_decode(-5isize, &[251]);
    }

    #[test]
    fn test_top_compacted_bool() {
        check_top_encode_decode(true, &[1]);
        check_top_encode_decode(false, &[]);
    }

    #[test]
    fn test_top_bytes_compacted() {
        check_top_encode_decode(Vec::<u8>::new(), &[]);
        check_top_encode_decode([1u8, 2u8, 3u8].to_vec(), &[1u8, 2u8, 3u8]);
    }

    #[test]
    fn test_vec_i32_compacted() {
        let v = [1i32, 2i32, 3i32].to_vec();
        let expected: &[u8] = &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3];
        check_top_encode_decode(v, expected);
    }

    #[test]
    fn test_option_vec_i32() {
        let some_v = Some([1i32, 2i32, 3i32].to_vec());
        let expected: &[u8] = &[
            /*opt*/ 1, /*size*/ 0, 0, 0, 3, /*data*/ 0, 0, 0, 1, 0, 0, 0, 2, 0, 0,
            0, 3,
        ];
        check_top_encode_decode(some_v, expected);

        let none_v: Option<Vec<i32>> = None;
        check_top_encode_decode(none_v, &[0]);
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
        check_top_encode_decode(wa, &[1, 2, 3, 4, 5]);

        let mut v: Vec<WrappedArray> = Vec::new();
        v.push(wa);
        v.push(WrappedArray([6, 7, 8, 9, 0]));
        check_top_encode_decode(v, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    }
}
