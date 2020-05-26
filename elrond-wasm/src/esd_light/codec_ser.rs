


// use super::bytes_err::{SDError, Result};
use alloc::vec::Vec;
use super::TypeInfo;

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
	fn dep_encode_to<O: Output>(&self, dest: &mut O) {
		self.using_top_encoded(|buf| dest.write(buf));
	}

	/// Convert self to an owned vector.
	/// Allowed to provide compact version.
	/// Do not call for nested objects.
	fn top_encode(&self) -> Vec<u8> {
		let mut dest = Vec::new();
		self.using_top_encoded(|buf| dest.write(buf));
		dest
	}

	/// Convert self to a slice and then invoke the given closure with it.
	/// Allowed to provide compact version.
	/// Do not call for nested objects.
	fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) {
		// default implementation simply use dep_encode_to
		let mut dest: Vec<u8> = Vec::new();
		self.dep_encode_to(&mut dest);
		f(dest.as_slice())
	}
}

impl Encode for () {
	fn dep_encode_to<O: Output>(&self, _dest: &mut O) {
	}

	fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) {
		f(&[])
	}

	fn top_encode(&self) -> Vec<u8> {
		Vec::new()
	}
}

impl Encode for u8 {
	const TYPE_INFO: TypeInfo = TypeInfo::U8;

	fn dep_encode_to<O: Output>(&self, dest: &mut O) {
		dest.write(&[*self as u8][..]);
	}

	fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) {
		if *self == 0u8 {
			f(&[])
		} else {
			f(&[*self][..])
		}
	}
}

impl<T: Encode> Encode for &[T] {
	fn dep_encode_to<O: Output>(&self, dest: &mut O) {
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
					x.dep_encode_to(dest);
				}
			}
		}
	}

	#[inline]
	fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) {
		match T::TYPE_INFO {
			TypeInfo::U8 => {
				// cast Vec<T> to &[u8]
				let slice: &[u8] = unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len()) };
				f(slice);
			},
			_ => {
				let mut result: Vec<u8> = Vec::new();
				for x in *self {
					x.dep_encode_to(&mut result);
				}
				f(result.as_slice())
			}
		}
	}
}

impl<T: Encode> Encode for Vec<T> {
	#[inline]
	fn dep_encode_to<O: Output>(&self, dest: &mut O) {
		self.as_slice().dep_encode_to(dest);
	}

	#[inline]
	fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) {
		self.as_slice().using_top_encoded(f);
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

macro_rules! encode_num {
    ($num_type:ident, $size_in_bits:expr, $signed:expr) => {
		impl Encode for $num_type {
			#[inline]
            fn dep_encode_to<O: Output>(&self, dest: &mut O) {
				using_encoded_number(*self as u64, $size_in_bits, $signed, false, |buf| dest.write(buf))
			}
		
			#[inline]
            fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) {
				using_encoded_number(*self as u64, $size_in_bits, $signed, true, f)
			}
		}
    }
}

encode_num!{u64, 64, false}
encode_num!{i64, 64, true}
encode_num!{u32, 32, false}
encode_num!{i32, 32, true}
encode_num!{usize, 32, false}
encode_num!{isize, 32, true}
encode_num!{u16, 16, false}
encode_num!{i16, 16, true}
encode_num!{i8, 8, true}

impl Encode for bool {
	fn dep_encode_to<O: Output>(&self, dest: &mut O) {
		dest.write(&[*self as u8][..]);
	}

	fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) {
		if *self {
			f(&[1u8][..])
		} else {
			f(&[])
		}
	}
}

impl<T: Encode> Encode for Option<T> {
	fn dep_encode_to<O: Output>(&self, dest: &mut O) {
		match self {
			Some(v) => {
				using_encoded_number(1u64, 8, false, false, |buf| dest.write(buf));
				v.dep_encode_to(dest);
			},
			None => {
				using_encoded_number(0u64, 8, false, false, |buf| dest.write(buf));
			}
		}
	}

	// fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) {
	// 	match self {
	// 		Some(v) => {
	// 			v.using_top_encoded(f);
	// 		},
	// 		None => {}
	// 	}
	// }
}

macro_rules! tuple_impls {
    ($(($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> Encode for ($($name,)+)
            where
                $($name: Encode,)+
            {
				#[inline]
				fn dep_encode_to<O: Output>(&self, dest: &mut O) {
					$(
                        self.$n.dep_encode_to(dest);
                    )+
					
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

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use super::*;
	use super::super::test_struct::*;
    use core::fmt::Debug;

    fn ser_ok<V>(element: V, bytes: &[u8])
    where
        V: Encode + PartialEq + Debug + 'static,
    {
        assert_eq!(element.top_encode().as_slice(), bytes);
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
