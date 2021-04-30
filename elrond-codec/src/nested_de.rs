use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::num::NonZeroUsize;

use crate::codec_err::DecodeError;
use crate::nested_de_input::NestedDecodeInput;
use crate::num_conv::bytes_to_number;
use crate::TypeInfo;

/// Trait that allows zero-copy read of value-references from slices in LE format.
pub trait NestedDecode: Sized {
	// !INTERNAL USE ONLY!
	// This const helps elrond-wasm to optimize the encoding/decoding by doing fake specialization.
	#[doc(hidden)]
	const TYPE_INFO: TypeInfo = TypeInfo::Unknown;

	/// Attempt to deserialise the value from input,
	/// using the format of an object nested inside another structure.
	/// In case of success returns the deserialized value and the number of bytes consumed during the operation.
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError>;

	/// Version of `top_decode` that exits quickly in case of error.
	/// Its purpose is to create smaller implementations
	/// in cases where the application is supposed to exit directly on decode error.
	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		match Self::dep_decode(input) {
			Ok(v) => v,
			Err(e) => exit(c, e),
		}
	}
}

/// Convenience method, to avoid having to specify type when calling `dep_decode`.
/// Especially useful in the macros.
/// Also checks that the entire slice was used.
/// The input doesn't need to be mutable because we are not changing the underlying data.
pub fn dep_decode_from_byte_slice<D: NestedDecode>(input: &[u8]) -> Result<D, DecodeError> {
	let mut_slice = &mut &*input;
	let result = D::dep_decode(mut_slice);
	if !mut_slice.is_empty() {
		return Err(DecodeError::INPUT_TOO_LONG);
	}
	result
}

pub fn dep_decode_from_byte_slice_or_exit<D: NestedDecode, ExitCtx: Clone>(
	input: &[u8],
	c: ExitCtx,
	exit: fn(ExitCtx, DecodeError) -> !,
) -> D {
	let mut_slice = &mut &*input;
	let result = D::dep_decode_or_exit(mut_slice, c.clone(), exit);
	if !mut_slice.is_empty() {
		exit(c, DecodeError::INPUT_TOO_LONG);
	}
	result
}

impl NestedDecode for () {
	const TYPE_INFO: TypeInfo = TypeInfo::Unit;

	fn dep_decode<I: NestedDecodeInput>(_: &mut I) -> Result<(), DecodeError> {
		Ok(())
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		_: &mut I,
		_: ExitCtx,
		_: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
	}
}

impl NestedDecode for u8 {
	const TYPE_INFO: TypeInfo = TypeInfo::U8;

	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		input.read_byte()
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		input.read_byte_or_exit(c, exit)
	}
}

impl<T: NestedDecode> NestedDecode for Vec<T> {
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
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
			},
		}
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		let size = usize::dep_decode_or_exit(input, c.clone(), exit);
		match T::TYPE_INFO {
			TypeInfo::U8 => {
				let bytes = input.read_slice_or_exit(size, c, exit);
				let bytes_copy = bytes.to_vec(); // copy is needed because result might outlive input
				let cast_vec: Vec<T> = unsafe { core::mem::transmute(bytes_copy) };
				cast_vec
			},
			_ => {
				let mut result: Vec<T> = Vec::with_capacity(size);
				for _ in 0..size {
					result.push(T::dep_decode_or_exit(input, c.clone(), exit));
				}
				result
			},
		}
	}
}

impl NestedDecode for String {
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		let raw = Vec::<u8>::dep_decode(input)?;
		match String::from_utf8(raw) {
			Ok(s) => Ok(s),
			Err(_) => Err(DecodeError::UTF8_DECODE_ERROR),
		}
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		let raw = Vec::<u8>::dep_decode_or_exit(input, c.clone(), exit);
		match String::from_utf8(raw) {
			Ok(s) => s,
			Err(_) => exit(c, DecodeError::UTF8_DECODE_ERROR),
		}
	}
}

impl NestedDecode for Box<str> {
	#[inline]
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		Ok(String::dep_decode(input)?.into_boxed_str())
	}

	#[inline]
	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		String::dep_decode_or_exit(input, c, exit).into_boxed_str()
	}
}

macro_rules! decode_num_unsigned {
	($ty:ty, $num_bytes:expr, $type_info:expr) => {
		impl NestedDecode for $ty {
			const TYPE_INFO: TypeInfo = $type_info;

			fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
				let bytes = input.read_slice($num_bytes)?;
				let num = bytes_to_number(bytes, false) as $ty;
				Ok(num)
			}

			fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
				input: &mut I,
				c: ExitCtx,
				exit: fn(ExitCtx, DecodeError) -> !,
			) -> Self {
				let bytes = input.read_slice_or_exit($num_bytes, c, exit);
				let num = bytes_to_number(bytes, false) as $ty;
				num
			}
		}
	};
}

decode_num_unsigned!(u16, 2, TypeInfo::U16);
decode_num_unsigned!(u32, 4, TypeInfo::U32);
decode_num_unsigned!(usize, 4, TypeInfo::USIZE);
decode_num_unsigned!(u64, 8, TypeInfo::U64);

macro_rules! decode_num_signed {
	($ty:ty, $num_bytes:expr, $type_info:expr) => {
		impl NestedDecode for $ty {
			const TYPE_INFO: TypeInfo = $type_info;

			fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
				let bytes = input.read_slice($num_bytes)?;
				let num = bytes_to_number(bytes, true) as $ty;
				Ok(num)
			}

			fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
				input: &mut I,
				c: ExitCtx,
				exit: fn(ExitCtx, DecodeError) -> !,
			) -> Self {
				let bytes = input.read_slice_or_exit($num_bytes, c, exit);
				let num = bytes_to_number(bytes, true) as $ty;
				num
			}
		}
	};
}

decode_num_signed!(i8, 1, TypeInfo::I8);
decode_num_signed!(i16, 2, TypeInfo::I16);
decode_num_signed!(i32, 4, TypeInfo::I32);
decode_num_signed!(isize, 4, TypeInfo::ISIZE);
decode_num_signed!(i64, 8, TypeInfo::I64);

impl NestedDecode for bool {
	const TYPE_INFO: TypeInfo = TypeInfo::Bool;

	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		match input.read_byte()? {
			0 => Ok(false),
			1 => Ok(true),
			_ => Err(DecodeError::INVALID_VALUE),
		}
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		match input.read_byte_or_exit(c.clone(), exit) {
			0 => false,
			1 => true,
			_ => exit(c, DecodeError::INVALID_VALUE),
		}
	}
}

impl<T: NestedDecode> NestedDecode for Option<T> {
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		match input.read_byte()? {
			0 => Ok(None),
			1 => Ok(Some(T::dep_decode(input)?)),
			_ => Err(DecodeError::INVALID_VALUE),
		}
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		match input.read_byte_or_exit(c.clone(), exit) {
			0 => None,
			1 => Some(T::dep_decode_or_exit(input, c, exit)),
			_ => exit(c, DecodeError::INVALID_VALUE),
		}
	}
}

impl<T: NestedDecode> NestedDecode for Box<T> {
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		Ok(Box::new(T::dep_decode(input)?))
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		Box::new(T::dep_decode_or_exit(input, c, exit))
	}
}

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> NestedDecode for ($($name,)+)
            where
                $($name: NestedDecode,)+
            {
                fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
                    Ok((
                        $(
                            $name::dep_decode(input)?,
                        )+
                    ))
                }

                fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(input: &mut I, c: ExitCtx, exit: fn(ExitCtx, DecodeError) -> !) -> Self {
                    (
                        $(
                            $name::dep_decode_or_exit(input, c.clone(), exit),
                        )+
                    )
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

impl NestedDecode for NonZeroUsize {
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		if let Some(nz) = NonZeroUsize::new(usize::dep_decode(input)?) {
			Ok(nz)
		} else {
			Err(DecodeError::INVALID_VALUE)
		}
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		if let Some(nz) = NonZeroUsize::new(usize::dep_decode_or_exit(input, c.clone(), exit)) {
			nz
		} else {
			exit(c, DecodeError::INVALID_VALUE)
		}
	}
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
	use super::super::test_struct::*;
	use super::*;
	use crate::test_util::check_dep_decode;
	use core::fmt::Debug;

	fn deser_ok<V>(element: V, bytes: &[u8])
	where
		V: NestedDecode + PartialEq + Debug + 'static,
	{
		let input = bytes.to_vec();
		let deserialized: V = check_dep_decode::<V>(&input[..]);
		assert_eq!(deserialized, element);
	}

	#[test]
	fn test_dep_decode_numbers() {
		// unsigned positive
		deser_ok(5u8, &[5]);
		deser_ok(5u16, &[0, 5]);
		deser_ok(5u32, &[0, 0, 0, 5]);
		deser_ok(5usize, &[0, 0, 0, 5]);
		deser_ok(5u64, &[0, 0, 0, 0, 0, 0, 0, 5]);
		// signed positive
		deser_ok(5i8, &[5]);
		deser_ok(5i16, &[0, 5]);
		deser_ok(5i32, &[0, 0, 0, 5]);
		deser_ok(5isize, &[0, 0, 0, 5]);
		deser_ok(5i64, &[0, 0, 0, 0, 0, 0, 0, 5]);
		// signed negative
		deser_ok(-5i8, &[251]);
		deser_ok(-5i16, &[255, 251]);
		deser_ok(-5i32, &[255, 255, 255, 251]);
		deser_ok(-5isize, &[255, 255, 255, 251]);
		deser_ok(-5i64, &[255, 255, 255, 255, 255, 255, 255, 251]);
		// non zero usize
		deser_ok(NonZeroUsize::new(5).unwrap(), &[0, 0, 0, 5]);
	}

	#[test]
	#[rustfmt::skip]
	fn test_dep_decode_str() {
		deser_ok(String::from("abc"), &[0, 0, 0, 3, b'a', b'b', b'c']);
		deser_ok(String::from("abc").into_boxed_str(), &[0, 0, 0, 3, b'a', b'b', b'c']);
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
		let expected: &[u8] = &[
			/*variant index*/ 0, 0, 0, 2, /*(*/ 0, 0, 0, 1, /*,*/ 0, 0, 0,
			2, /*)*/
		];
		deser_ok(t, expected);

		let s = E::Struct { a: 1 };
		let expected: &[u8] = &[/*variant index*/ 0, 0, 0, 3, /*data*/ 0, 0, 0, 1];
		deser_ok(s, expected);
	}
}
