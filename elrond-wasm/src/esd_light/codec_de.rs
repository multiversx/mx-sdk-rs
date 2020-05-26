
use alloc::vec::Vec;
use super::codec_err::DeError;
use super::TypeInfo;

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
    fn read_into(&mut self, into: &mut [u8]) -> Result<(), DeError>;

	/// Read a single byte from the input.
	fn read_byte(&mut self) -> Result<u8, DeError> {
		let mut buf = [0u8];
		self.read_into(&mut buf[..])?;
		Ok(buf[0])
    }

    /// Read the exact number of bytes required to fill the given buffer.
	fn read_slice(&mut self, length: usize) -> Result<&[u8], DeError>;
    
    fn flush(&mut self) -> Result<&[u8], DeError>;

}

impl<'a> Input for &'a [u8] {
	fn remaining_len(&mut self) -> usize {
		self.len()
    }

	fn read_into(&mut self, into: &mut [u8]) -> Result<(), DeError> {
		if into.len() > self.len() {
			return Err(DeError::InputTooShort);
		}
		let len = into.len();
		into.copy_from_slice(&self[..len]);
		*self = &self[len..];
		Ok(())
    }

    fn read_slice(&mut self, length: usize) -> Result<&[u8], DeError> {
        if length > self.len() {
            return Err(DeError::InputTooShort);
        }

        let (result, rest) = self.split_at(length);
        *self = rest;
        return Ok(result);
    }
    
    fn flush(&mut self) -> Result<&[u8], DeError> {
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
	fn top_decode<I: Input>(input: &mut I) -> Result<Self, DeError> {
        let result = Self::dep_decode(input)?;
        if input.remaining_len() > 0 {
            return Err(DeError::InputTooLong);
        }
        Ok(result)
    }

    /// Attempt to deserialise the value from input,
    /// using the format of an object nested inside another structure.
    /// In case of success returns the deserialized value and the number of bytes consumed during the operation.
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DeError>;
}

impl Decode for () {
	fn dep_decode<I: Input>(_: &mut I) -> Result<(), DeError> {
		Ok(())
	}
}

impl Decode for u8 {
    const TYPE_INFO: TypeInfo = TypeInfo::U8;
    
	fn top_decode<I: Input>(input: &mut I) -> Result<Self, DeError> {
        let bytes = input.flush()?;
        match bytes.len() {
            0 => Ok(0u8),
            1 => Ok(bytes[0]),
            _ => Err(DeError::InputTooLong),
        }
    }
    
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DeError> {
        input.read_byte()
    }
}

impl<T: Decode> Decode for Vec<T> {
	fn top_decode<I: Input>(input: &mut I) -> Result<Self, DeError> {
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
    
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DeError> {
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
    ($ty:ty, $num_bytes:expr, $signed:expr) => {
        impl Decode for $ty {
            fn top_decode<I: Input>(input: &mut I) -> Result<Self, DeError> {
                let bytes = input.flush()?;
                if bytes.len() > $num_bytes {
                    return Err(DeError::InputTooLong)
                }
                let num = bytes_to_number(bytes, $signed) as $ty;
                Ok(num)
            }
            
            fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DeError> {
                let bytes = input.read_slice($num_bytes)?;
                let num = bytes_to_number(bytes, $signed) as $ty;
                Ok(num)
            }
        }
    }
}

impl_nums!(u16, 2, false);
impl_nums!(u32, 4, false);
impl_nums!(usize, 4, false);
impl_nums!(u64, 8, false);


impl_nums!(i8 , 1, true);
impl_nums!(i16, 2, true);
impl_nums!(i32, 4, true);
impl_nums!(isize, 4, true);
impl_nums!(i64, 8, true);

impl Decode for bool {
	fn top_decode<I: Input>(input: &mut I) -> Result<Self, DeError> {
        let bytes = input.flush()?;
        match bytes.len() {
            0 => Ok(false),
            1 => match bytes[0] {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(DeError::InvalidValue),
            }
            _ => Err(DeError::InputTooLong),
        }
    }
    
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DeError> {
        match input.read_byte()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(DeError::InvalidValue),
        }
    }
}

impl<T: Decode> Decode for Option<T> {
	// fn top_decode<I: Input>(input: &mut I) -> Result<Self, DeError> {
    //     if input.empty() {
    //         Ok(None)
    //     } else {
    //         Ok(Some(T::top_decode(input)?))
    //     }
    // }
    
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DeError> {
        match input.read_byte()? {
			0 => Ok(None),
			1 => Ok(Some(T::dep_decode(input)?)),
			_ => Err(DeError::InvalidValue),
		}
    }
}

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
