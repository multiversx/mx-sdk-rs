#![no_std]

extern crate alloc;

mod codec_ser;
mod codec_de;
mod codec_err;
pub mod test_util;

pub use codec_ser::*;
pub use codec_de::*;
pub use codec_err::{EncodeError, DecodeError};

/// !INTERNAL USE ONLY!
///
/// This enum provides type information to optimize encoding/decoding by doing fake specialization.
#[doc(hidden)]
pub enum TypeInfo {
	/// Default value of [`Encode::TYPE_INFO`] to not require implementors to set this value in the trait.
	Unknown,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    Bool,
    BigUint,
    BigInt,
    Unit,
}

/// Until we have derive capabilities, here are some structures with explicit encode/decode, for testing.
#[cfg(test)]
pub mod test_struct {
    use super::*;
    use core::fmt::Debug;
    use alloc::vec::Vec;

    #[derive(PartialEq, Debug)]
	pub struct Test {
		pub int: u16,
		pub seq: Vec<u8>,
		pub another_byte: u8,
	}

	impl Encode for Test {
		fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
			self.int.dep_encode_to(dest)?;
			self.seq.dep_encode_to(dest)?;
            self.another_byte.dep_encode_to(dest)?;
            Ok(())
		}
    }
    
    impl Decode for Test {
        fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
            Ok(Test{
                int: u16::dep_decode(input)?,
                seq: Vec::<u8>::dep_decode(input)?,
                another_byte: u8::dep_decode(input)?,
            })
        }
    }

    #[derive(PartialEq, Clone, Debug)]
    pub enum E {
        Unit,
        Newtype(u32),
        Tuple(u32, u32),
        Struct { a: u32 },
    }

    impl Encode for E {
		fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
            match self {
                E::Unit => {
                    using_encoded_number(0u64, 32, false, false, |buf| dest.write(buf));
                },
                E::Newtype(arg1) => {
                    using_encoded_number(1u64, 32, false, false, |buf| dest.write(buf));
                    using_encoded_number(*arg1 as u64, 32, false, false, |buf| dest.write(buf));
                },
                E::Tuple(arg1, arg2) => {
                    using_encoded_number(2u64, 32, false, false, |buf| dest.write(buf));
                    using_encoded_number(*arg1 as u64, 32, false, false, |buf| dest.write(buf));
                    using_encoded_number(*arg2 as u64, 32, false, false, |buf| dest.write(buf));
                },
                E::Struct { a } => {
                    using_encoded_number(3u64, 32, false, false, |buf| dest.write(buf));
                    using_encoded_number(*a as u64, 32, false, false, |buf| dest.write(buf));
                },
            }
            Ok(())
		}
    }
    
    impl Decode for E {
        fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
            match u32::dep_decode(input)? {
                0 => Ok(E::Unit),
                1 => Ok(E::Newtype(u32::dep_decode(input)?)),
                2 => Ok(E::Tuple(u32::dep_decode(input)?, u32::dep_decode(input)?)),
                3 => Ok(E::Struct{ a: u32::dep_decode(input)? }),
                _ => Err(DecodeError::InvalidValue),
            }
        }
    }

    #[derive(PartialEq, Debug, Clone, Copy)]
    pub struct WrappedArray(pub [u8; 5]);

    impl Encode for WrappedArray {
		fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
            dest.write(&self.0[..]);
            Ok(())
		}
    }
    
    impl Decode for WrappedArray {
        fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
            let mut arr = [0u8; 5];
            input.read_into(&mut arr)?;
            Ok(WrappedArray(arr))
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use super::test_struct::*;
    use crate::test_util::ser_deser_ok;
    use core::fmt::Debug;
    use alloc::vec::Vec;

    pub fn the_same<V>(element: V)
    where
        V: Encode + Decode + PartialEq + Debug + 'static,
    {
        let serialized_bytes = element.top_encode().unwrap();
        let deserialized: V = decode_from_byte_slice(&mut &serialized_bytes[..]).unwrap();
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
    fn test_array_16384() {
        let arr = [7i32; 16384];
        let mut expected_bytes = Vec::<u8>::with_capacity(16384 * 4);
        for _ in 0..16384 {
            expected_bytes.push(0);
            expected_bytes.push(0);
            expected_bytes.push(0);
            expected_bytes.push(7);
        }

        // serialize
        let serialized_bytes = arr.top_encode().unwrap();
        assert_eq!(serialized_bytes, expected_bytes);

        // deserialize
        let deserialized = <[i32; 16384]>::top_decode(&mut &serialized_bytes[..]).unwrap();
        for i in 0..16384 {
            assert_eq!(deserialized[i], 7i32);
        }
    }

    

    #[test]
    fn test_option_vec_i32() {
        let some_v = Some([1i32, 2i32, 3i32].to_vec());
        let expected: &[u8] = &[/*opt*/ 1, /*size*/ 0, 0, 0, 3, /*data*/ 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3];
        ser_deser_ok(some_v, expected);

        let none_v: Option<Vec<i32>> = None;
        ser_deser_ok(none_v, &[]);
    }

    #[test]
    fn test_struct() {
        let test = Test {
            int: 1,
            seq: [5, 6].to_vec(),
            another_byte: 7,
        };
        the_same(test);
    }

    #[test]
    fn test_wrapped_array() {
        let wa = WrappedArray([1, 2, 3, 4, 5]);
        ser_deser_ok(wa, &[1, 2, 3, 4, 5]);

        let mut v: Vec<WrappedArray> = Vec::new();
        v.push(wa);
        v.push(WrappedArray([6, 7, 8, 9, 0]));
        ser_deser_ok(v, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    }

    #[test]
    fn test_tuple() {
        let t = (1i8, 2u32, (), 3i16);
        let expected: &[u8] = &[1, 0, 0, 0, 2, 0, 3];
        ser_deser_ok(t, expected);
    }

}
