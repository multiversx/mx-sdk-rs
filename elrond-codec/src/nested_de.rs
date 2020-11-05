use alloc::boxed::Box;
use alloc::vec::Vec;
use arrayvec::ArrayVec;
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
			1 => Some(T::dep_decode_or_exit(input, c.clone(), exit)),
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

macro_rules! array_impls {
    ($($n: tt,)+) => {
        $(
            impl<T: NestedDecode> NestedDecode for [T; $n] {
                fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
					let mut r = ArrayVec::new();
					for _ in 0..$n {
						r.push(T::dep_decode(input)?);
					}
					let i = r.into_inner();

					match i {
						Ok(a) => Ok(a),
						Err(_) => Err(DecodeError::ARRAY_DECODE_ERROR),
					}
                }

                fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(input: &mut I, c: ExitCtx, exit: fn(ExitCtx, DecodeError) -> !) -> Self {
                    let mut r = ArrayVec::new();
					for _ in 0..$n {
						r.push(T::dep_decode_or_exit(input, c.clone(), exit));
					}
					let i = r.into_inner();

					match i {
						Ok(a) => a,
						Err(_) => exit(c, DecodeError::ARRAY_DECODE_ERROR),
					}
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
