use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::num::NonZeroUsize;

use crate::codec_err::DecodeError;
use crate::nested_de::*;
use crate::top_de_input::TopDecodeInput;
use crate::transmute::*;
use crate::TypeInfo;

/// Trait that allows zero-copy read of values from an underlying API in big endian format.
///
/// 'Top' stands for the fact that values are deserialized on their own,
/// so we have the benefit of knowing their length.
/// This is useful in many scnearios, such as not having to encode Vec length and others.
///
/// The opther optimization that can be done when deserializing top-level objects
/// is using special functions from the underlying API that do some of the work for the deserializer.
/// These include getting values directly as i64/u64 or wrapping them directly into an owned Box<[u8]>.
///
/// BigInt/BigUint handling is not included here, because these are API-dependent
/// and would overly complicate the trait.
///
pub trait TopDecode: Sized {
	#[doc(hidden)]
	const TYPE_INFO: TypeInfo = TypeInfo::Unknown;

	/// Attempt to deserialize the value from input.
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError>;

	/// Version of `top_decode` that exits quickly in case of error.
	/// Its purpose is to create smaller implementations
	/// in cases where the application is supposed to exit directly on decode error.
	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		match Self::top_decode(input) {
			Ok(v) => v,
			Err(e) => exit(c, e),
		}
	}

	/// Allows types to provide optimized implementations for their boxed version.
	/// Especially designed for byte arrays that can be transmuted directly from the input sometimes.
	#[doc(hidden)]
	#[inline]
	fn top_decode_boxed<I: TopDecodeInput>(input: I) -> Result<Box<Self>, DecodeError> {
		Ok(Box::new(Self::top_decode(input)?))
	}

	#[doc(hidden)]
	#[inline]
	fn top_decode_boxed_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Box<Self> {
		Box::new(Self::top_decode_or_exit(input, c, exit))
	}
}

/// Top-decodes the result using the NestedDecode implementation.
pub fn top_decode_from_nested<T, I>(input: I) -> Result<T, DecodeError>
where
	I: TopDecodeInput,
	T: NestedDecode,
{
	let bytes = input.into_boxed_slice_u8();
	let mut_slice = &mut &*bytes;
	let result = T::dep_decode(mut_slice)?;
	if !mut_slice.is_empty() {
		return Err(DecodeError::INPUT_TOO_LONG);
	}
	Ok(result)
}

/// Top-decodes the result using the NestedDecode implementation.
/// Uses the fast-exit mechanism in case of error.
pub fn top_decode_from_nested_or_exit<T, I, ExitCtx: Clone>(
	input: I,
	c: ExitCtx,
	exit: fn(ExitCtx, DecodeError) -> !,
) -> T
where
	I: TopDecodeInput,
	T: NestedDecode,
{
	let bytes = input.into_boxed_slice_u8();
	let mut_slice = &mut &*bytes;
	let result = T::dep_decode_or_exit(mut_slice, c.clone(), exit);
	if !mut_slice.is_empty() {
		exit(c, DecodeError::INPUT_TOO_LONG);
	}
	result
}

impl TopDecode for () {
	const TYPE_INFO: TypeInfo = TypeInfo::Unit;

	fn top_decode<I: TopDecodeInput>(_: I) -> Result<Self, DecodeError> {
		Ok(())
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		_: I,
		_: ExitCtx,
		_: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
	}
}

impl<T: TopDecode> TopDecode for Box<T> {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		T::top_decode_boxed(input)
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		T::top_decode_boxed_or_exit(input, c, exit)
	}
}

// Allowed to implement this because [T] cannot implement NestedDecode, being ?Sized.
impl<T: NestedDecode> TopDecode for Box<[T]> {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		if let TypeInfo::U8 = T::TYPE_INFO {
			let bytes = input.into_boxed_slice_u8();
			let cast_bytes: Box<[T]> = unsafe { core::mem::transmute(bytes) };
			Ok(cast_bytes)
		} else {
			let vec = Vec::<T>::top_decode(input)?;
			Ok(vec_into_boxed_slice(vec))
		}
	}

	/// Quick exit for any of the contained types
	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		if let TypeInfo::U8 = T::TYPE_INFO {
			let bytes = input.into_boxed_slice_u8();
			let cast_bytes: Box<[T]> = unsafe { core::mem::transmute(bytes) };
			cast_bytes
		} else {
			let vec = Vec::<T>::top_decode_or_exit(input, c, exit);
			vec_into_boxed_slice(vec)
		}
	}
}

impl<T: NestedDecode> TopDecode for Vec<T> {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		if let TypeInfo::U8 = T::TYPE_INFO {
			let bytes = input.into_boxed_slice_u8();
			let bytes_vec = boxed_slice_into_vec(bytes);
			let cast_vec: Vec<T> = unsafe { core::mem::transmute(bytes_vec) };
			Ok(cast_vec)
		} else {
			let bytes = input.into_boxed_slice_u8();
			let mut_slice = &mut &*bytes;
			let mut result: Vec<T> = Vec::new();
			while !mut_slice.is_empty() {
				result.push(T::dep_decode(mut_slice)?);
			}
			Ok(result)
		}
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		if let TypeInfo::U8 = T::TYPE_INFO {
			let bytes = input.into_boxed_slice_u8();
			let bytes_vec = boxed_slice_into_vec(bytes);
			let cast_vec: Vec<T> = unsafe { core::mem::transmute(bytes_vec) };
			cast_vec
		} else {
			let bytes = input.into_boxed_slice_u8();
			let mut_slice = &mut &*bytes;
			let mut result: Vec<T> = Vec::new();
			while !mut_slice.is_empty() {
				result.push(T::dep_decode_or_exit(mut_slice, c.clone(), exit));
			}
			result
		}
	}
}

impl TopDecode for String {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		let raw = Vec::<u8>::top_decode(input)?;
		match String::from_utf8(raw) {
			Ok(s) => Ok(s),
			Err(_) => Err(DecodeError::UTF8_DECODE_ERROR),
		}
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		let raw = Vec::<u8>::top_decode_or_exit(input, c.clone(), exit);
		match String::from_utf8(raw) {
			Ok(s) => s,
			Err(_) => exit(c, DecodeError::UTF8_DECODE_ERROR),
		}
	}
}

impl TopDecode for Box<str> {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		Ok(String::top_decode(input)?.into_boxed_str())
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		String::top_decode_or_exit(input, c, exit).into_boxed_str()
	}
}

macro_rules! decode_num_unsigned {
	($ty:ty, $bounds_ty:ty, $type_info:expr) => {
		impl TopDecode for $ty {
			const TYPE_INFO: TypeInfo = $type_info;

			fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
				let arg_u64 = input.into_u64();
				let max = <$bounds_ty>::MAX as u64;
				if arg_u64 > max {
					Err(DecodeError::INPUT_TOO_LONG)
				} else {
					Ok(arg_u64 as $ty)
				}
			}

			fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
				input: I,
				c: ExitCtx,
				exit: fn(ExitCtx, DecodeError) -> !,
			) -> Self {
				let arg_u64 = input.into_u64();
				let max = <$bounds_ty>::MAX as u64;
				if arg_u64 > max {
					exit(c, DecodeError::INPUT_TOO_LONG)
				} else {
					arg_u64 as $ty
				}
			}
		}
	};
}

decode_num_unsigned!(u8, u8, TypeInfo::U8);
decode_num_unsigned!(u16, u16, TypeInfo::U16);
decode_num_unsigned!(u32, u32, TypeInfo::U32);
decode_num_unsigned!(usize, u32, TypeInfo::USIZE); // even if usize can be 64 bits on some platforms, we always deserialize as max 32 bits
decode_num_unsigned!(u64, u64, TypeInfo::U64);

macro_rules! decode_num_signed {
	($ty:ty, $bounds_ty:ty, $type_info:expr) => {
		impl TopDecode for $ty {
			const TYPE_INFO: TypeInfo = $type_info;

			fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
				let arg_i64 = input.into_i64();
				let min = <$bounds_ty>::MIN as i64;
				let max = <$bounds_ty>::MAX as i64;
				if arg_i64 < min || arg_i64 > max {
					Err(DecodeError::INPUT_OUT_OF_RANGE)
				} else {
					Ok(arg_i64 as $ty)
				}
			}

			fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
				input: I,
				c: ExitCtx,
				exit: fn(ExitCtx, DecodeError) -> !,
			) -> Self {
				let arg_i64 = input.into_i64();
				let min = <$bounds_ty>::MIN as i64;
				let max = <$bounds_ty>::MAX as i64;
				if arg_i64 < min || arg_i64 > max {
					exit(c, DecodeError::INPUT_OUT_OF_RANGE)
				} else {
					arg_i64 as $ty
				}
			}
		}
	};
}

decode_num_signed!(i8, i8, TypeInfo::I8);
decode_num_signed!(i16, i16, TypeInfo::I16);
decode_num_signed!(i32, i32, TypeInfo::I32);
decode_num_signed!(isize, i32, TypeInfo::ISIZE); // even if isize can be 64 bits on some platforms, we always deserialize as max 32 bits
decode_num_signed!(i64, i64, TypeInfo::I64);

impl TopDecode for bool {
	const TYPE_INFO: TypeInfo = TypeInfo::Bool;

	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		match input.into_u64() {
			0 => Ok(false),
			1 => Ok(true),
			_ => Err(DecodeError::INPUT_OUT_OF_RANGE),
		}
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		match input.into_u64() {
			0 => false,
			1 => true,
			_ => exit(c, DecodeError::INPUT_OUT_OF_RANGE),
		}
	}
}

impl<T: NestedDecode> TopDecode for Option<T> {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		let bytes = input.into_boxed_slice_u8();
		if bytes.is_empty() {
			Ok(None)
		} else {
			let item = dep_decode_from_byte_slice::<T>(&bytes[1..])?;
			Ok(Some(item))
		}
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		let bytes = input.into_boxed_slice_u8();
		if bytes.is_empty() {
			None
		} else {
			let item = dep_decode_from_byte_slice_or_exit(&bytes[1..], c, exit);
			Some(item)
		}
	}
}

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> TopDecode for ($($name,)+)
            where
                $($name: NestedDecode,)+
            {
                fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
                    top_decode_from_nested(input)
                }

                fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(input: I, c: ExitCtx, exit: fn(ExitCtx, DecodeError) -> !) -> Self {
                    top_decode_from_nested_or_exit(input, c, exit)
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

impl TopDecode for NonZeroUsize {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		if let Some(nz) = NonZeroUsize::new(usize::top_decode(input)?) {
			Ok(nz)
		} else {
			Err(DecodeError::INVALID_VALUE)
		}
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		if let Some(nz) = NonZeroUsize::new(usize::top_decode_or_exit(input, c.clone(), exit)) {
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
	use crate::test_util::check_top_decode;
	use core::fmt::Debug;

	fn deser_ok<V>(element: V, bytes: &[u8])
	where
		V: TopDecode + PartialEq + Debug + 'static,
	{
		let deserialized: V = check_top_decode::<V>(&bytes[..]);
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
		// non zero usize
		deser_ok(NonZeroUsize::new(5).unwrap(), &[5]);
	}

	#[test]
	fn test_top_numbers_decompacted_2() {
		deser_ok(-1i32, &[255]);
		deser_ok(-1i32, &[255, 255]);
		deser_ok(-1i32, &[255, 255, 255, 255]);
		deser_ok(-1i64, &[255, 255, 255, 255, 255, 255, 255, 255]);
	}

	#[test]
	fn test_top_decode_str() {
		deser_ok(String::from("abc"), &[b'a', b'b', b'c']);
		deser_ok(String::from("abc").into_boxed_str(), &[b'a', b'b', b'c']);
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
