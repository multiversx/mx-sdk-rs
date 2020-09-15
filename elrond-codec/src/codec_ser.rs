use alloc::vec::Vec;
use crate::codec_err::EncodeError;
use crate::TypeInfo;

/// Trait that allows writing of data.
pub trait Output {
	/// Write to the output.
	fn write(&mut self, bytes: &[u8]);

	/// Write a single byte to the output.
	fn push_byte(&mut self, byte: u8) {
		self.write(&[byte]);
	}
}

impl Output for Vec<u8> {
	fn write(&mut self, bytes: &[u8]) {
		self.extend_from_slice(bytes)
	}
}

/// Trait that allows zero-copy write of value-references to slices in LE format.
///
/// Implementations should override `using_top_encoded` for value types and `dep_encode_to` and `size_hint` for allocating types.
/// Wrapper types should override all methods.
pub trait Encode: Sized {
	// !INTERNAL USE ONLY!
	// This const helps SCALE to optimize the encoding/decoding by doing fake specialization.
	#[doc(hidden)]
	const TYPE_INFO: TypeInfo = TypeInfo::Unknown;

	/// Encode to output, using the format of an object nested inside another structure.
	/// Does not provide compact version.
	fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.using_top_encoded(|buf| dest.write(buf))
	}

	/// Convert self to an owned vector.
	/// Allowed to provide compact version.
	/// Do not call for nested objects.
	fn top_encode(&self) -> Result<Vec<u8>, EncodeError> {
		let mut dest = Vec::new();
		self.using_top_encoded(|buf| dest.write(buf))?;
		Ok(dest)
	}

	/// Convert self to a slice and then invoke the given closure with it.
	/// Allowed to provide compact version.
	/// Do not call for nested objects.
	fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) -> Result<(), EncodeError> {
		// default implementation simply use dep_encode_to
		let mut dest: Vec<u8> = Vec::new();
		self.dep_encode_to(&mut dest)?;
		f(dest.as_slice());
		Ok(())
	}

	#[inline]
	fn top_encode_as_i64(&self) -> Option<Result<i64, EncodeError>> {
		None
	}
}

// TODO: consider removing altogether when possible
impl Encode for () {
	const TYPE_INFO: TypeInfo = TypeInfo::Unit;

	fn dep_encode_to<O: Output>(&self, _dest: &mut O) -> Result<(), EncodeError> {
		Ok(())
	}

	fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) -> Result<(), EncodeError> {
		f(&[]);
		Ok(())
	}

	fn top_encode(&self) -> Result<Vec<u8>, EncodeError> {
		Ok(Vec::with_capacity(0))
	}
}

impl Encode for u8 {
	const TYPE_INFO: TypeInfo = TypeInfo::U8;

	fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
		dest.write(&[*self as u8][..]);
		Ok(())
	}

	fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) -> Result<(), EncodeError> {
		if *self == 0u8 {
			f(&[]);
		} else {
			f(&[*self][..]);
		}
		Ok(())
	}
}

impl<T: Encode> Encode for &[T] {
	fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
		// push size
		using_encoded_number(self.len() as u64, 32, false, false, |buf| dest.write(buf));
		// actual data
		match T::TYPE_INFO {
			TypeInfo::U8 => {
				// cast &[T] to &[u8]
				let slice: &[u8] = unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len()) };
				dest.write(slice);
			},
			_ => {
				for x in *self {
					x.dep_encode_to(dest)?;
				}
			}
		}
		Ok(())
	}

	#[inline]
	fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) -> Result<(), EncodeError> {
		match T::TYPE_INFO {
			TypeInfo::U8 => {
				// cast Vec<T> to &[u8]
				let slice: &[u8] = unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len()) };
				f(slice);
			},
			_ => {
				let mut result: Vec<u8> = Vec::new();
				for x in *self {
					x.dep_encode_to(&mut result)?;
				}
				f(result.as_slice());
			}
		}
		Ok(())
	}
}

impl<T: Encode> Encode for &T {
	#[inline]
	fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
		(*self).dep_encode_to(dest)
	}

	#[inline]
	fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) -> Result<(), EncodeError> {
		(*self).using_top_encoded(f)
	}
}

impl Encode for &str {
	fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
		// push size
		using_encoded_number(self.len() as u64, 32, false, false, |buf| dest.write(buf));
		// actual data
		dest.write(self.as_bytes());
		Ok(())
	}

	fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) -> Result<(), EncodeError> {
		f(self.as_bytes());
		Ok(())
	}
}

impl<T: Encode> Encode for Vec<T> {
	#[inline]
	fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.as_slice().dep_encode_to(dest)
	}

	#[inline]
	fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) -> Result<(), EncodeError> {
		self.as_slice().using_top_encoded(f)
	}
}


/// Adds number to output buffer.
/// No argument generics here, because we want the executable binary as small as possible.
/// Smaller types need to be converted to u64 before using this function.
/// TODO: there might be a quicker version of this using transmute + reverse bytes.
pub fn using_encoded_number<F: FnOnce(&[u8])>(x: u64, size_in_bits: usize, signed: bool, mut compact: bool, f: F) {
	let mut result = [0u8; 8];
	let mut result_size = 0usize;
	let negative = 
		compact && // only relevant when compact flag
		signed &&  // only possible when signed flag
		x >> (size_in_bits - 1) & 1 == 1; // compute by checking first bit
	
	let irrelevant_byte = if negative { 0xffu8 } else { 0x00u8 };
	let mut bit_offset = size_in_bits as isize - 8;
	while bit_offset >= 0 {
		// going byte by byte from most to least significant
		let byte = (x >> (bit_offset as usize) & 0xffu64) as u8;
		
		if compact {
			// compact means ignoring irrelvant leading bytes
			// that is 000... for positives and fff... for negatives
			if byte != irrelevant_byte {
				result[result_size] = byte;
				result_size += 1;
				compact = false;
			}
		} else {
			result[result_size] = byte;
			result_size += 1;
		}
		
		bit_offset -= 8;
	}

	f(&result[0..result_size])
}

macro_rules! encode_num_signed {
    ($num_type:ident, $size_in_bits:expr, $type_info:expr) => {
		impl Encode for $num_type {
			const TYPE_INFO: TypeInfo = $type_info;

			#[inline]
            fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
				using_encoded_number(*self as u64, $size_in_bits, true, false, |buf| dest.write(buf));
				Ok(())
			}
		
			#[inline]
            fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) -> Result<(), EncodeError> {
				using_encoded_number(*self as u64, $size_in_bits, true, true, f);
				Ok(())
			}

			#[inline]
            fn top_encode_as_i64(&self) -> Option<Result<i64, EncodeError>> {
				Some(Ok(*self as i64))
			}
		}
    }
}

macro_rules! encode_num_unsigned {
    ($num_type:ident, $size_in_bits:expr, $type_info:expr) => {
		impl Encode for $num_type {
			const TYPE_INFO: TypeInfo = $type_info;

			#[inline]
            fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
				using_encoded_number(*self as u64, $size_in_bits, false, false, |buf| dest.write(buf));
				Ok(())
			}
		
			#[inline]
            fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) -> Result<(), EncodeError> {
				using_encoded_number(*self as u64, $size_in_bits, false, true, f);
				Ok(())
			}
		}
    }
}

encode_num_unsigned!{u64, 64, TypeInfo::U64}
encode_num_unsigned!{u32, 32, TypeInfo::U32}
encode_num_unsigned!{usize, 32, TypeInfo::USIZE}
encode_num_unsigned!{u16, 16, TypeInfo::U16}

encode_num_signed!{i64, 64, TypeInfo::I64}
encode_num_signed!{i32, 32, TypeInfo::I32}
encode_num_signed!{isize, 32, TypeInfo::ISIZE}
encode_num_signed!{i16, 16, TypeInfo::I16}
encode_num_signed!{i8, 8, TypeInfo::I8}

impl Encode for bool {
	const TYPE_INFO: TypeInfo = TypeInfo::Bool;

	fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
		dest.write(&[*self as u8][..]);
		Ok(())
	}

	fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) -> Result<(), EncodeError> {
		if *self {
			f(&[1u8][..]);
		} else {
			f(&[]);
		}
		Ok(())
	}

	#[inline]
	fn top_encode_as_i64(&self) -> Option<Result<i64, EncodeError>> {
		Some(if *self {
			Ok(1i64)
		} else {
			Ok(0i64)
		})
	}
}

impl<T: Encode> Encode for Option<T> {
	fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
		match self {
			Some(v) => {
				using_encoded_number(1u64, 8, false, false, |buf| dest.write(buf));
				v.dep_encode_to(dest)
			},
			None => {
				using_encoded_number(0u64, 8, false, false, |buf| dest.write(buf));
				Ok(())
			}
		}
	}

	/// Allow None to be serialized to empty bytes, but leave the leading "1" for Some,
	/// to allow disambiguation between e.g. Some(0) and None.
	fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) -> Result<(), EncodeError> {
		match self {
			Some(v) => {
				let mut dest: Vec<u8> = Vec::new();
				dest.push(1u8);
				v.dep_encode_to(&mut dest)?;
				f(dest.as_slice());
			},
			None => {
				f(&[]);
			}
		}
		Ok(())
	}
}

macro_rules! tuple_impls {
    ($(($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> Encode for ($($name,)+)
            where
                $($name: Encode,)+
            {
				#[inline]
				fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
					$(
                        self.$n.dep_encode_to(dest)?;
                    )+
					Ok(())
				}
            }
        )+
    }
}

tuple_impls! {
    (0 T0)
    (0 T0 1 T1)
    (0 T0 1 T1 2 T2)
    (0 T0 1 T1 2 T2 3 T3)
    (0 T0 1 T1 2 T2 3 T3 4 T4)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

macro_rules! array_impls {
    ($($n: tt,)+) => {
        $(
            impl<T: Encode> Encode for [T; $n] {
				#[inline]
				fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
					// the top encoded slice does not serialize its length, so just like the array
					(&self[..]).using_top_encoded(|buf| dest.write(buf))
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

    fn ser_ok<V>(element: V, expected_bytes: &[u8])
    where
        V: Encode + PartialEq + Debug + 'static,
    {
		V::using_top_encoded(&element, |bytes| {
			assert_eq!(bytes, expected_bytes);
		}).unwrap();
        
    }

    #[test]
    fn test_top_compacted_numbers() {
        // unsigned positive
        ser_ok(5u8, &[5]);
        ser_ok(5u16, &[5]);
        ser_ok(5u32, &[5]);
        ser_ok(5u64, &[5]);
        ser_ok(5usize, &[5]);
        // signed positive
        ser_ok(5i8, &[5]);
        ser_ok(5i16, &[5]);
        ser_ok(5i32, &[5]);
        ser_ok(5i64, &[5]);
        ser_ok(5isize, &[5]);
        // signed negative
        ser_ok(-5i8, &[251]);
        ser_ok(-5i16, &[251]);
        ser_ok(-5i32, &[251]);
        ser_ok(-5i64, &[251]);
        ser_ok(-5isize, &[251]);
    }

    #[test]
    fn test_top_compacted_bool() {
        ser_ok(true,    &[1]);
        ser_ok(false,   &[]);
    }

    #[test]
    fn test_top_compacted_empty_bytes() {
        let empty_byte_slice: &[u8] = &[];
        ser_ok(empty_byte_slice, empty_byte_slice);
    }

    #[test]
    fn test_top_compacted_bytes() {
        ser_ok(&[1u8, 2u8, 3u8][..], &[1u8, 2u8, 3u8]);
    }

    #[test]
    fn test_top_compacted_vec_u8() {
        let some_vec = [1u8, 2u8, 3u8].to_vec();
        ser_ok(some_vec, &[1u8, 2u8, 3u8]);
    }

    #[test]
    fn test_top_compacted_vec_i32() {
        let some_vec = [1i32, 2i32, 3i32].to_vec();
        let expected: &[u8] = &[0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3];
        ser_ok(some_vec, expected);
	}

    #[test]
    fn test_struct() {
		let test = Test {
            int: 1,
            seq: [5, 6].to_vec(),
            another_byte: 7,
        };

        ser_ok(test, &[0, 1, 0, 0, 0, 2, 5, 6, 7]);
    }

    #[test]
    fn test_tuple() {
        ser_ok((7u32, -2i16), &[0, 0, 0, 7, 255, 254]);
    }

    #[test]
    fn test_unit() {
        ser_ok((), &[]);
    }

    #[test]
    fn test_enum() {
        let u = E::Unit;
        let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 0];
        ser_ok(u, expected);

        let n = E::Newtype(1);
        let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 1, /*data*/ 0, 0, 0, 1];
        ser_ok(n, expected);

        let t = E::Tuple(1, 2);
        let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 2, /*(*/ 0, 0, 0, 1, /*,*/ 0, 0, 0, 2 /*)*/];
        ser_ok(t, expected);

        let s = E::Struct { a: 1 };
        let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 3, /*data*/ 0, 0, 0, 1];
        ser_ok(s, expected);
    }
}
