use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::num::NonZeroUsize;

use crate::codec_err::EncodeError;
use crate::nested_ser::{dep_encode_slice_contents, NestedEncode};
use crate::nested_ser_output::NestedEncodeOutput;
use crate::top_ser_output::TopEncodeOutput;
use crate::TypeInfo;

/// Most types will be encoded without any possibility of error.
/// The trait is used to provide these implementations.
/// This is currently not a substitute for implementing a proper TopEncode.
pub trait TopEncodeNoErr: Sized {
	fn top_encode_no_err<O: TopEncodeOutput>(&self, output: O);
}

pub trait TopEncode: Sized {
	// !INTERNAL USE ONLY!
	#[doc(hidden)]
	const TYPE_INFO: TypeInfo = TypeInfo::Unknown;

	/// Attempt to serialize the value to ouput.
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError>;

	/// Version of `top_decode` that exits quickly in case of error.
	/// Its purpose is to create smaller bytecode implementations
	/// in cases where the application is supposed to exit directly on decode error.
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		match self.top_encode(output) {
			Ok(v) => v,
			Err(e) => exit(c, e),
		}
	}
}

pub fn top_encode_from_nested<T, O>(obj: &T, output: O) -> Result<(), EncodeError>
where
	O: TopEncodeOutput,
	T: NestedEncode,
{
	let mut bytes = Vec::<u8>::new();
	obj.dep_encode(&mut bytes)?;
	output.set_slice_u8(&bytes[..]);
	Ok(())
}

pub fn top_encode_from_nested_or_exit<T, O, ExitCtx>(
	obj: &T,
	output: O,
	c: ExitCtx,
	exit: fn(ExitCtx, EncodeError) -> !,
) where
	O: TopEncodeOutput,
	T: NestedEncode,
	ExitCtx: Clone,
{
	let mut bytes = Vec::<u8>::new();
	obj.dep_encode_or_exit(&mut bytes, c, exit);
	output.set_slice_u8(&bytes[..]);
}

macro_rules! top_encode_from_no_err {
	($type:ty, $type_info:expr) => {
		impl TopEncode for $type {
			const TYPE_INFO: TypeInfo = $type_info;

			#[inline]
			fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
				self.top_encode_no_err(output);
				Ok(())
			}

			#[inline]
			fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
				&self,
				output: O,
				_: ExitCtx,
				_: fn(ExitCtx, EncodeError) -> !,
			) {
				self.top_encode_no_err(output);
			}
		}
	};
}

pub fn top_encode_to_vec<T: TopEncode>(obj: &T) -> Result<Vec<u8>, EncodeError> {
	let mut bytes = Vec::<u8>::new();
	obj.top_encode(&mut bytes)?;
	Ok(bytes)
}

impl TopEncodeNoErr for () {
	#[inline]
	fn top_encode_no_err<O: TopEncodeOutput>(&self, output: O) {
		output.set_unit();
	}
}

top_encode_from_no_err! {(), TypeInfo::Unit}

impl<T: NestedEncode> TopEncode for &[T] {
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		match T::TYPE_INFO {
			TypeInfo::U8 => {
				// transmute to &[u8]
				// save directly, without passing through the buffer
				let slice: &[u8] =
					unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len()) };
				output.set_slice_u8(slice);
			},
			_ => {
				// only using `dep_encode_slice_contents` for non-u8,
				// because it always appends to the buffer,
				// which is not necessary above
				let mut buffer = Vec::<u8>::new();
				dep_encode_slice_contents(self, &mut buffer)?;
				output.set_slice_u8(&buffer[..]);
			},
		}
		Ok(())
	}

	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		match T::TYPE_INFO {
			TypeInfo::U8 => {
				// transmute to &[u8]
				// save directly, without passing through the buffer
				let slice: &[u8] =
					unsafe { core::slice::from_raw_parts(self.as_ptr() as *const u8, self.len()) };
				output.set_slice_u8(slice);
			},
			_ => {
				// only using `dep_encode_slice_contents` for non-u8,
				// because it always appends to the buffer,
				// which is not necessary above
				let mut buffer = Vec::<u8>::new();
				for x in *self {
					x.dep_encode_or_exit(&mut buffer, c.clone(), exit);
				}
				output.set_slice_u8(&buffer[..]);
			},
		}
	}
}

impl<T: TopEncode> TopEncode for &T {
	#[inline]
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		(*self).top_encode(output)
	}

	#[inline]
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		(*self).top_encode_or_exit(output, c, exit);
	}
}

impl TopEncode for &str {
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		output.set_slice_u8(self.as_bytes());
		Ok(())
	}

	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		_: ExitCtx,
		_: fn(ExitCtx, EncodeError) -> !,
	) {
		output.set_slice_u8(self.as_bytes());
	}
}

impl<T: NestedEncode> TopEncode for Vec<T> {
	#[inline]
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		self.as_slice().top_encode(output)
	}

	#[inline]
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_slice().top_encode_or_exit(output, c, exit);
	}
}

impl TopEncode for String {
	#[inline]
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		self.as_bytes().top_encode(output)
	}

	#[inline]
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_bytes().top_encode_or_exit(output, c, exit);
	}
}

impl TopEncode for Box<str> {
	#[inline]
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		self.as_ref().as_bytes().top_encode(output)
	}

	#[inline]
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_ref().as_bytes().top_encode_or_exit(output, c, exit);
	}
}

macro_rules! encode_num_unsigned {
	($num_type:ty, $size_in_bits:expr, $type_info:expr) => {
		impl TopEncodeNoErr for $num_type {
			#[inline]
			fn top_encode_no_err<O: TopEncodeOutput>(&self, output: O) {
				output.set_u64(*self as u64);
			}
		}

		top_encode_from_no_err! {$num_type, $type_info}
	};
}

encode_num_unsigned! {u64, 64, TypeInfo::U64}
encode_num_unsigned! {u32, 32, TypeInfo::U32}
encode_num_unsigned! {usize, 32, TypeInfo::USIZE}
encode_num_unsigned! {u16, 16, TypeInfo::U16}
encode_num_unsigned! {u8, 8, TypeInfo::U8}

macro_rules! encode_num_signed {
	($num_type:ty, $size_in_bits:expr, $type_info:expr) => {
		impl TopEncodeNoErr for $num_type {
			#[inline]
			fn top_encode_no_err<O: TopEncodeOutput>(&self, output: O) {
				output.set_i64(*self as i64);
			}
		}

		top_encode_from_no_err! {$num_type, $type_info}
	};
}

encode_num_signed! {i64, 64, TypeInfo::I64}
encode_num_signed! {i32, 32, TypeInfo::I32}
encode_num_signed! {isize, 32, TypeInfo::ISIZE}
encode_num_signed! {i16, 16, TypeInfo::I16}
encode_num_signed! {i8, 8, TypeInfo::I8}

impl TopEncodeNoErr for bool {
	fn top_encode_no_err<O: TopEncodeOutput>(&self, output: O) {
		// only using signed because this one is implemented in Arwen, unsigned is not
		// TODO: change to set_u64
		output.set_i64(if *self { 1i64 } else { 0i64 });
	}
}

top_encode_from_no_err! {bool, TypeInfo::Bool}

impl<T: NestedEncode> TopEncode for Option<T> {
	/// Allow None to be serialized to empty bytes, but leave the leading "1" for Some,
	/// to allow disambiguation between e.g. Some(0) and None.
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		match self {
			Some(v) => {
				let mut buffer = Vec::<u8>::new();
				buffer.push_byte(1u8);
				v.dep_encode(&mut buffer)?;
				output.set_slice_u8(&buffer[..]);
			},
			None => {
				output.set_slice_u8(&[]);
			},
		}
		Ok(())
	}

	/// Allow None to be serialized to empty bytes, but leave the leading "1" for Some,
	/// to allow disambiguation between e.g. Some(0) and None.
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		match self {
			Some(v) => {
				let mut buffer = Vec::<u8>::new();
				buffer.push_byte(1u8);
				v.dep_encode_or_exit(&mut buffer, c, exit);
				output.set_slice_u8(&buffer[..]);
			},
			None => {
				output.set_slice_u8(&[]);
			},
		}
	}
}

impl<T: TopEncode> TopEncode for Box<T> {
	#[inline]
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		self.as_ref().top_encode(output)
	}

	#[inline]
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_ref().top_encode_or_exit(output, c, exit);
	}
}

impl<T: NestedEncode> TopEncode for Box<[T]> {
	#[inline]
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		self.as_ref().top_encode(output)
	}

	#[inline]
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.as_ref().top_encode_or_exit(output, c, exit);
	}
}

macro_rules! tuple_impls {
    ($(($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> TopEncode for ($($name,)+)
            where
                $($name: NestedEncode,)+
            {
				fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
					let mut buffer = Vec::<u8>::new();
					$(
                        self.$n.dep_encode(&mut buffer)?;
                    )+
					output.set_slice_u8(&buffer[..]);
					Ok(())
				}

				fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(&self, output: O, c: ExitCtx, exit: fn(ExitCtx, EncodeError) -> !) {
					let mut buffer = Vec::<u8>::new();
					$(
                        self.$n.dep_encode_or_exit(&mut buffer, c.clone(), exit);
                    )+
					output.set_slice_u8(&buffer[..]);
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
            impl<T: NestedEncode> TopEncode for [T; $n] {
				#[inline]
				fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
					// the top encoded slice does not serialize its length, so just like the array
					(&self[..]).top_encode(output)
				}

				#[inline]
				fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(&self, output: O, c: ExitCtx, exit: fn(ExitCtx, EncodeError) -> !) {
					(&self[..]).top_encode_or_exit(output, c, exit);
				}
            }
        )+
    }
}

#[rustfmt::skip]
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

impl TopEncode for NonZeroUsize {
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		self.get().top_encode(output)
	}

	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.get().top_encode_or_exit(output, c, exit);
	}
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use super::super::test_struct::*;
	use super::*;
	use crate::test_util::check_top_encode;
	use core::fmt::Debug;

	fn ser_ok<V>(element: V, expected_bytes: &[u8])
	where
		V: TopEncode + PartialEq + Debug + 'static,
	{
		let bytes = check_top_encode(&element);
		assert_eq!(bytes.as_slice(), expected_bytes);
	}

	#[test]
	fn test_serialize_top_compacted_numbers() {
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
		// non zero usize
		ser_ok(NonZeroUsize::new(5).unwrap(), &[5]);
	}

	#[test]
	fn test_serialize_top_compacted_numbers_msb_ok() {
		ser_ok(127i32, &[127]);
		ser_ok(128i32, &[0, 128]);
		ser_ok(255i32, &[0, 255]);

		ser_ok(-1i32, &[255]);
		ser_ok(-128i32, &[128]);
		ser_ok(-129i32, &[255, 127]);
		ser_ok(-256i32, &[255, 0]);
		ser_ok(-257i32, &[254, 255]);
	}

	#[test]
	fn test_top_compacted_bool() {
		ser_ok(true, &[1]);
		ser_ok(false, &[]);
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
	fn test_top_encode_str() {
		let s = "abc";
		ser_ok(s, &[b'a', b'b', b'c']);
		ser_ok(String::from(s), &[b'a', b'b', b'c']);
		ser_ok(String::from(s).into_boxed_str(), &[b'a', b'b', b'c']);
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
