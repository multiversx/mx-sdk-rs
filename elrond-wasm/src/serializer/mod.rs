
mod bytes_ser;
mod bytes_de;
mod bytes_err;

pub use bytes_ser::to_bytes;
pub use bytes_de::from_bytes;


#[cfg(test)]
mod tests {
    use super::*;
    use core::fmt::Debug;
    use alloc::vec::Vec;
    use serde::{Serialize, Deserialize};

    fn the_same<V>(element: V)
    where
        V: serde::Serialize + serde::de::DeserializeOwned + PartialEq + Debug + 'static,
    {
        let serialized_bytes = to_bytes(&element).unwrap();
        let deserialized: V = from_bytes(serialized_bytes.as_slice()).unwrap();
        assert_eq!(deserialized, element);
    }

    #[test]
    fn test_numbers() {
        // unsigned positive
        the_same(5u8);
        the_same(5u16);
        the_same(5u32);
        the_same(5u64);
        the_same(5usize);
        // signed positive
        the_same(5i8);
        the_same(5i16);
        the_same(5i32);
        the_same(5i64);
        the_same(5isize);
        // signed negative
        the_same(-5i8);
        the_same(-5i16);
        the_same(-5i32);
        the_same(-5i64);
        the_same(-5isize);
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

}