
use alloc::vec::Vec;
use super::codec_err::DecodeError;
use super::TypeInfo;
use arrayvec::ArrayVec;

/// Trait that allows reading of data into a slice.
pub trait Input {
	/// Should return the remaining length of the input data. If no information about the input
	/// length is available, `None` should be returned.
	///
	/// The length is used to constrain the preallocation while decoding. Returning a garbage
	/// length can open the doors for a denial of service attack to your application.
	/// Otherwise, returning `None` can decrease the performance of your application.
    fn remaining_len(&mut self) -> usize;
    
    fn empty(&mut self)-> bool {
        self.remaining_len() == 0
    }

	/// Read the exact number of bytes required to fill the given buffer.
    fn read_into(&mut self, into: &mut [u8]) -> Result<(), DecodeError>;

	/// Read a single byte from the input.
	fn read_byte(&mut self) -> Result<u8, DecodeError> {
		let mut buf = [0u8];
		self.read_into(&mut buf[..])?;
		Ok(buf[0])
    }

    /// Read the exact number of bytes required to fill the given buffer.
	fn read_slice(&mut self, length: usize) -> Result<&[u8], DecodeError>;
    
    fn flush(&mut self) -> Result<&[u8], DecodeError>;

}

impl<'a> Input for &'a [u8] {
	fn remaining_len(&mut self) -> usize {
		self.len()
    }

	fn read_into(&mut self, into: &mut [u8]) -> Result<(), DecodeError> {
		if into.len() > self.len() {
			return Err(DecodeError::InputTooShort);
		}
		let len = into.len();
		into.copy_from_slice(&self[..len]);
		*self = &self[len..];
		Ok(())
    }

    fn read_slice(&mut self, length: usize) -> Result<&[u8], DecodeError> {
        if length > self.len() {
            return Err(DecodeError::InputTooShort);
        }

        let (result, rest) = self.split_at(length);
        *self = rest;
        return Ok(result);
    }
    
    fn flush(&mut self) -> Result<&[u8], DecodeError> {
        let result = &self[..];
        *self = &[];
        return Ok(result);
    }
}

/// Trait that allows zero-copy read of value-references from slices in LE format.
pub trait Decode: Sized {
	// !INTERNAL USE ONLY!
	// This const helps SCALE to optimize the encoding/decoding by doing fake specialization.
	#[doc(hidden)]
	const TYPE_INFO: TypeInfo = TypeInfo::Unknown;
    
    /// Attempt to deserialise the value from input.
	fn top_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        let result = Self::dep_decode(input)?;
        if input.remaining_len() > 0 {
            return Err(DecodeError::InputTooLong);
        }
        Ok(result)
    }

    /// Attempt to deserialise the value from input,
    /// using the format of an object nested inside another structure.
    /// In case of success returns the deserialized value and the number of bytes consumed during the operation.
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError>;
}

/// Convenience method, to avoid having to specify type when calling `top_decode`.
/// Especially useful in the macros.
#[inline]
pub fn decode_from_byte_slice<D: Decode>(input: &[u8]) -> Result<D, DecodeError> {
    // the input doesn't need to be mutable because we are not changing the underlying data 
    D::top_decode(&mut &*input)
}

impl Decode for () {
    const TYPE_INFO: TypeInfo = TypeInfo::Unit;

	fn dep_decode<I: Input>(_: &mut I) -> Result<(), DecodeError> {
		Ok(())
	}
}

impl Decode for u8 {
    const TYPE_INFO: TypeInfo = TypeInfo::U8;
    
	fn top_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        let bytes = input.flush()?;
        match bytes.len() {
            0 => Ok(0u8),
            1 => Ok(bytes[0]),
            _ => Err(DecodeError::InputTooLong),
        }
    }
    
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        input.read_byte()
    }
}

impl<T: Decode> Decode for Vec<T> {
	fn top_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        match T::TYPE_INFO {
			TypeInfo::U8 => {
                let bytes = input.flush()?;
                let bytes_copy = bytes.to_vec(); // copy is needed because result might outlive input
                let cast_vec: Vec<T> = unsafe { core::mem::transmute(bytes_copy) };
                Ok(cast_vec)
			},
			_ => {
                let mut result: Vec<T> = Vec::new();
                while input.remaining_len() > 0 {
                    result.push(T::dep_decode(input)?);
                }
                Ok(result)
			}
        }
    }
    
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        let size = usize::dep_decode(input)?;
        match T::TYPE_INFO {
			TypeInfo::U8 => {
                let bytes = input.read_slice(size)?;
                let bytes_copy = bytes.to_vec(); // copy is needed because result might outlive input
                let cast_vec: Vec<T> = unsafe { core::mem::transmute(bytes_copy) };
                Ok(cast_vec)
			},
			_ => {
                let mut result: Vec<T> = Vec::with_capacity(size);
				for _ in 0..size {
                    result.push(T::dep_decode(input)?);
                }
                Ok(result)
			}
        }
    }
}

/// Handles both signed and unsigned of any length.
/// No generics here, because we want the executable binary as small as possible.
pub fn bytes_to_number(bytes: &[u8], signed: bool) -> u64 {
    if bytes.len() == 0 {
        return 0;
    }
    let negative = signed && bytes[0] >> 7 == 1;
    let mut result = 
        if negative {
            // start with all bits set to 1, 
            // to ensure that if there are fewer bytes than the result type width,
            // the leading bits will be 1 instead of 0
            0xffffffffffffffffu64 
        } else { 
            0u64 
        };
    for byte in bytes.iter() {
        result <<= 8;
        result |= *byte as u64;
    }
    result
}

macro_rules! impl_nums {
    ($ty:ty, $num_bytes:expr, $signed:expr, $type_info:expr) => {
        impl Decode for $ty {
            const TYPE_INFO: TypeInfo = $type_info;
            
            fn top_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
                let bytes = input.flush()?;
                if bytes.len() > $num_bytes {
                    return Err(DecodeError::InputTooLong)
                }
                let num = bytes_to_number(bytes, $signed) as $ty;
                Ok(num)
            }
            
            fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
                let bytes = input.read_slice($num_bytes)?;
                let num = bytes_to_number(bytes, $signed) as $ty;
                Ok(num)
            }
        }
    }
}

impl_nums!(u16, 2, false, TypeInfo::U16);
impl_nums!(u32, 4, false, TypeInfo::U32);
impl_nums!(usize, 4, false, TypeInfo::U32);
impl_nums!(u64, 8, false, TypeInfo::U64);

impl_nums!(i8 , 1, true, TypeInfo::I8);
impl_nums!(i16, 2, true, TypeInfo::I16);
impl_nums!(i32, 4, true, TypeInfo::I32);
impl_nums!(isize, 4, true, TypeInfo::I32);
impl_nums!(i64, 8, true, TypeInfo::I64);

impl Decode for bool {
    const TYPE_INFO: TypeInfo = TypeInfo::Bool;
    
	fn top_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        let bytes = input.flush()?;
        match bytes.len() {
            0 => Ok(false),
            1 => match bytes[0] {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(DecodeError::InvalidValue),
            }
            _ => Err(DecodeError::InputTooLong),
        }
    }
    
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        match input.read_byte()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(DecodeError::InvalidValue),
        }
    }
}

impl<T: Decode> Decode for Option<T> {
	fn top_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        if input.empty() {
            Ok(None)
        } else {
            let result = Self::dep_decode(input);
            if input.remaining_len() > 0 {
                return Err(DecodeError::InputTooLong);
            }
            result
        }
    }
    
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        match input.read_byte()? {
			0 => Ok(None),
			1 => Ok(Some(T::dep_decode(input)?)),
			_ => Err(DecodeError::InvalidValue),
		}
    }
}

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> Decode for ($($name,)+)
            where
                $($name: Decode,)+
            {
                fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
                    let tuple = (
                        $(
                            $name::dep_decode(input)?,
                        )+
                    );
                    Ok(tuple)
                }
            }
        )+
    }
}

tuple_impls! {
    1 => (0 T0)
    2 => (0 T0 1 T1)
    3 => (0 T0 1 T1 2 T2)
    4 => (0 T0 1 T1 2 T2 3 T3)
    5 => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

macro_rules! array_impls {
    ($($n: tt,)+) => {
        $(
            impl<T: Decode> Decode for [T; $n] {
				fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
					let mut r = ArrayVec::new();
					for _ in 0..$n {
						r.push(T::dep_decode(input)?);
					}
					let i = r.into_inner();

					match i {
						Ok(a) => Ok(a),
						Err(_) => Err(DecodeError::ArrayDecodeErr),
					}
				}
            }
        )+
    }
}

array_impls!(
	1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
	17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
	32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
	52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71,
	72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91,
	92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108,
	109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124,
	125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140,
	141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156,
	157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172,
	173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188,
	189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204,
	205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220,
	221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236,
	237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252,
	253, 254, 255, 256, 384, 512, 768, 1024, 2048, 4096, 8192, 16384, 32768,
);

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
	use super::super::test_struct::*;
    use core::fmt::Debug;

    fn deser_ok<V>(element: V, bytes: &[u8])
    where
        V: Decode + PartialEq + Debug + 'static,
    {
        let input = bytes.to_vec();
        let deserialized: V = V::top_decode(&mut &input[..]).unwrap();
        assert_eq!(deserialized, element);
    }

    #[test]
    fn test_top_numbers_decompacted() {
        // unsigned positive
        deser_ok(5u8, &[5]);
        deser_ok(5u16, &[5]);
        deser_ok(5u32, &[5]);
        deser_ok(5u64, &[5]);
        deser_ok(5usize, &[5]);
        // signed positive
        deser_ok(5i8, &[5]);
        deser_ok(5i16, &[5]);
        deser_ok(5i32, &[5]);
        deser_ok(5i64, &[5]);
        deser_ok(5isize, &[5]);
        // signed negative
        deser_ok(-5i8, &[251]);
        deser_ok(-5i16, &[251]);
        deser_ok(-5i32, &[251]);
        deser_ok(-5i64, &[251]);
        deser_ok(-5isize, &[251]);
    }

    

    #[test]
    fn test_struct() {
        let test = Test {
            int: 1,
            seq: [5, 6].to_vec(),
            another_byte: 7,
        };
        deser_ok(test, &[0, 1, 0, 0, 0, 2, 5, 6, 7]);
    }

    #[test]
    fn test_enum() {
        let u = E::Unit;
        let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 0];
        deser_ok(u, expected);

        let n = E::Newtype(1);
        let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 1, /*data*/ 0, 0, 0, 1];
        deser_ok(n, expected);

        let t = E::Tuple(1, 2);
        let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 2, /*(*/ 0, 0, 0, 1, /*,*/ 0, 0, 0, 2 /*)*/];
        deser_ok(t, expected);

        let s = E::Struct { a: 1 };
        let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 3, /*data*/ 0, 0, 0, 1];
        deser_ok(s, expected);
    }
}
