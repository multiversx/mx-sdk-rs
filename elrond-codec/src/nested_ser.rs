use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::num::NonZeroUsize;

use crate::codec_err::EncodeError;
use crate::nested_ser_output::NestedEncodeOutput;
use crate::TypeInfo;

/// Most types will be encoded without any possibility of error.
/// The trait is used to provide these implementations.
/// This is currently not a substitute for implementing a proper TopEncode.
pub trait NestedEncodeNoErr: Sized {
	fn dep_encode_no_err<O: NestedEncodeOutput>(&self, dest: &mut O);
}

/// Trait that allows zero-copy write of value-references to slices in LE format.
///
/// Implementations should override `using_top_encoded` for value types and `dep_encode` and `size_hint` for allocating types.
/// Wrapper types should override all methods.
pub trait NestedEncode: Sized {
	// !INTERNAL USE ONLY!
	// This const helps SCALE to optimize the encoding/decoding by doing fake specialization.
	#[doc(hidden)]
	const TYPE_INFO: TypeInfo = TypeInfo::Unknown;

	/// NestedEncode to output, using the format of an object nested inside another structure.
	/// Does not provide compact version.
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError>;

	/// Version of `top_decode` that exits quickly in case of error.
	/// Its purpose is to create smaller implementations
	/// in cases where the application is supposed to exit directly on decode error.
	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		match self.dep_encode(dest) {
			Ok(v) => v,
			Err(e) => exit(c, e),
		}
	}
}

macro_rules! dep_encode_from_no_err {
	($type:ty, $type_info:expr) => {
		impl NestedEncode for $type {
			const TYPE_INFO: TypeInfo = $type_info;

			#[inline]
			fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
				self.dep_encode_no_err(dest);
				Ok(())
			}

			#[inline]
			fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
				&self,
				dest: &mut O,
				_: ExitCtx,
				_: fn(ExitCtx, EncodeError) -> !,
			) {
				self.dep_encode_no_err(dest);
			}
		}
	};
}

/// Convenience function for getting an object nested-encoded to a Vec<u8> directly.
pub fn dep_encode_to_vec<T: NestedEncode>(obj: &T) -> Result<Vec<u8>, EncodeError> {
	let mut bytes = Vec::<u8>::new();
	obj.dep_encode(&mut bytes)?;
	Ok(bytes)
}

/// Adds the concantenated encoded contents of a slice to an output buffer,
/// without serializing the slice length.
/// Byte slice is treated separately, via direct transmute.
pub fn dep_encode_slice_contents<T: NestedEncode, O: NestedEncodeOutput>(
	slice: &[T],
	dest: &mut O,
) -> Result<(), EncodeError> {
	match T::TYPE_INFO {
		TypeInfo::U8 => {
			// cast &[T] to &[u8]
			let slice: &[u8] =
				unsafe { core::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len()) };
			dest.write(slice);
		},
		_ => {
			for x in slice {
				x.dep_encode(dest)?;
			}
		},
	}
	Ok(())
}

pub fn dep_encode_slice_contents_or_exit<T, O, ExitCtx>(
	slice: &[T],
	dest: &mut O,
	c: ExitCtx,
	exit: fn(ExitCtx, EncodeError) -> !,
) where
	T: NestedEncode,
	O: NestedEncodeOutput,
	ExitCtx: Clone,
{
	match T::TYPE_INFO {
		TypeInfo::U8 => {
			// cast &[T] to &[u8]
			let slice: &[u8] =
				unsafe { core::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len()) };
			dest.write(slice);
		},
		_ => {
			for x in slice {
				x.dep_encode_or_exit(dest, c.clone(), exit);
			}
		},
	}
}

impl NestedEncodeNoErr for () {
	fn dep_encode_no_err<O: NestedEncodeOutput>(&self, _: &mut O) {}
}

dep_encode_from_no_err! {(), TypeInfo::Unit}

impl<T: NestedEncode> NestedEncode for &[T] {
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		// push size
		self.len().dep_encode(dest)?;
		// actual data
		dep_encode_slice_contents(self, dest)
	}

	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		// push size
		self.len().dep_encode_or_exit(dest, c.clone(), exit);
		// actual data
		dep_encode_slice_contents_or_exit(self, dest, c, exit);
	}
}

impl<T: NestedEncode> NestedEncode for &T {
	#[inline]
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		(*self).dep_encode(dest)
	}

	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		(*self).dep_encode_or_exit(dest, c, exit);
	}
}

impl NestedEncode for &str {
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.as_bytes().dep_encode(dest)
	}

	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_bytes().dep_encode_or_exit(dest, c, exit);
	}
}

impl<T: NestedEncode> NestedEncode for Vec<T> {
	#[inline]
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.as_slice().dep_encode(dest)
	}

	#[inline]
	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_slice().dep_encode_or_exit(dest, c, exit);
	}
}

impl NestedEncode for String {
	#[inline]
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.as_bytes().dep_encode(dest)
	}

	#[inline]
	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_bytes().dep_encode_or_exit(dest, c, exit);
	}
}

impl NestedEncode for Box<str> {
	#[inline]
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.as_ref().as_bytes().dep_encode(dest)
	}

	#[inline]
	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_ref().as_bytes().dep_encode_or_exit(dest, c, exit);
	}
}

// The main unsigned types need to be reversed before serializing.
macro_rules! encode_num_unsigned {
	($num_type:ty, $size_in_bits:expr, $type_info:expr) => {
		impl NestedEncodeNoErr for $num_type {
			#[inline(never)]
			fn dep_encode_no_err<O: NestedEncodeOutput>(&self, dest: &mut O) {
				dest.write(&self.to_be_bytes()[..]);
			}
		}

		dep_encode_from_no_err! {$num_type, $type_info}
	};
}

encode_num_unsigned! {u64, 64, TypeInfo::U64}
encode_num_unsigned! {u32, 32, TypeInfo::U32}
encode_num_unsigned! {u16, 16, TypeInfo::U16}

// No reversing needed for u8, because it is a single byte.
impl NestedEncodeNoErr for u8 {
	fn dep_encode_no_err<O: NestedEncodeOutput>(&self, dest: &mut O) {
		dest.push_byte(*self as u8);
	}
}

dep_encode_from_no_err! {u8, TypeInfo::U8}

// Derive the implementation of the other types by casting.
macro_rules! encode_num_mimic {
	($num_type:ty, $mimic_type:ident, $type_info:expr) => {
		impl NestedEncodeNoErr for $num_type {
			#[inline]
			fn dep_encode_no_err<O: NestedEncodeOutput>(&self, dest: &mut O) {
				(*self as $mimic_type).dep_encode_no_err(dest)
			}
		}

		dep_encode_from_no_err! {$num_type, $type_info}
	};
}

encode_num_mimic! {usize, u32, TypeInfo::USIZE}
encode_num_mimic! {i64, u64, TypeInfo::I64}
encode_num_mimic! {i32, u32, TypeInfo::I32}
encode_num_mimic! {isize, u32, TypeInfo::ISIZE}
encode_num_mimic! {i16, u16, TypeInfo::I16}
encode_num_mimic! {i8, u8, TypeInfo::I8}
encode_num_mimic! {bool, u8, TypeInfo::Bool}

impl<T: NestedEncode> NestedEncode for Option<T> {
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		match self {
			Some(v) => {
				dest.push_byte(1u8);
				v.dep_encode(dest)
			},
			None => {
				dest.push_byte(0u8);
				Ok(())
			},
		}
	}

	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		match self {
			Some(v) => {
				dest.push_byte(1u8);
				v.dep_encode_or_exit(dest, c, exit);
			},
			None => {
				dest.push_byte(0u8);
			},
		}
	}
}

impl<T: NestedEncode> NestedEncode for Box<T> {
	#[inline(never)]
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.as_ref().dep_encode(dest)
	}

	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_ref().dep_encode_or_exit(dest, c, exit);
	}
}

impl<T: NestedEncode> NestedEncode for Box<[T]> {
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.as_ref().dep_encode(dest)
	}

	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_ref().dep_encode_or_exit(dest, c, exit);
	}
}

macro_rules! tuple_impls {
    ($(($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> NestedEncode for ($($name,)+)
            where
                $($name: NestedEncode,)+
            {
				fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
					$(
                        self.$n.dep_encode(dest)?;
                    )+
					Ok(())
				}

				fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(&self, dest: &mut O, c: ExitCtx, exit: fn(ExitCtx, EncodeError) -> !) {
					$(
                        self.$n.dep_encode_or_exit(dest, c.clone(), exit);
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

impl NestedEncode for NonZeroUsize {
	#[inline]
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.get().dep_encode(dest)
	}

	#[inline]
	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.get().dep_encode_or_exit(dest, c, exit);
	}
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use super::super::test_struct::*;
	use super::*;
	use crate::test_util::check_dep_encode;
	use core::fmt::Debug;

	fn ser_ok<V>(element: V, expected_bytes: &[u8])
	where
		V: NestedEncode + PartialEq + Debug + 'static,
	{
		let bytes = check_dep_encode(&element);
		assert_eq!(bytes.as_slice(), expected_bytes);
	}

	#[test]
	fn test_dep_encode_numbers() {
		// unsigned positive
		ser_ok(5u8, &[5]);
		ser_ok(5u16, &[0, 5]);
		ser_ok(5u32, &[0, 0, 0, 5]);
		ser_ok(5usize, &[0, 0, 0, 5]);
		ser_ok(5u64, &[0, 0, 0, 0, 0, 0, 0, 5]);
		// signed positive
		ser_ok(5i8, &[5]);
		ser_ok(5i16, &[0, 5]);
		ser_ok(5i32, &[0, 0, 0, 5]);
		ser_ok(5isize, &[0, 0, 0, 5]);
		ser_ok(5i64, &[0, 0, 0, 0, 0, 0, 0, 5]);
		// signed negative
		ser_ok(-5i8, &[251]);
		ser_ok(-5i16, &[255, 251]);
		ser_ok(-5i32, &[255, 255, 255, 251]);
		ser_ok(-5isize, &[255, 255, 255, 251]);
		ser_ok(-5i64, &[255, 255, 255, 255, 255, 255, 255, 251]);
		// non zero usize
		ser_ok(NonZeroUsize::new(5).unwrap(), &[0, 0, 0, 5]);
	}

	#[test]
	fn test_dep_encode_bool() {
		ser_ok(true, &[1]);
		ser_ok(false, &[0]);
	}

	#[test]
	fn test_dep_encode_empty_bytes() {
		let empty_byte_slice: &[u8] = &[];
		ser_ok(empty_byte_slice, &[0, 0, 0, 0]);
	}

	#[test]
	fn test_dep_encode_bytes() {
		ser_ok(&[1u8, 2u8, 3u8][..], &[0, 0, 0, 3, 1u8, 2u8, 3u8]);
	}

	#[test]
	fn test_dep_encode_vec_u8() {
		let some_vec = [1u8, 2u8, 3u8].to_vec();
		ser_ok(some_vec, &[0, 0, 0, 3, 1u8, 2u8, 3u8]);
	}

	#[test]
	#[rustfmt::skip]
	fn test_dep_encode_str() {
		let s = "abc";
		ser_ok(s, &[0, 0, 0, 3, b'a', b'b', b'c']);
		ser_ok(String::from(s), &[0, 0, 0, 3, b'a', b'b', b'c']);
		ser_ok(String::from(s).into_boxed_str(), &[0, 0, 0, 3, b'a', b'b', b'c']);
	}

	#[test]
	fn test_dep_encode_vec_i32() {
		let some_vec = [1i32, 2i32, 3i32].to_vec();
		let expected: &[u8] = &[0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3];
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
		let expected: &[u8] = &[
			/*variant index*/ 0, 0, 0, 2, /*(*/ 0, 0, 0, 1, /*,*/ 0, 0, 0,
			2, /*)*/
		];
		ser_ok(t, expected);

		let s = E::Struct { a: 1 };
		let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 3, /*data*/ 0, 0, 0, 1];
		ser_ok(s, expected);
	}
}
