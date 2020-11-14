use crate::big_uint_mock::*;

use core::ops::{Add, Div, Mul, Neg, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use num_traits::sign::Signed;

use alloc::vec::Vec;
use elrond_wasm::BigIntApi;

use core::cmp::Ordering;
use num_bigint::{BigInt, Sign};

#[derive(Debug)]
pub struct RustBigInt(pub num_bigint::BigInt);

impl RustBigInt {
	pub fn value(&self) -> &BigInt {
		&self.0
	}
}

impl From<RustBigUint> for RustBigInt {
	fn from(item: RustBigUint) -> Self {
		RustBigInt(item.0)
	}
}

impl From<i64> for RustBigInt {
	fn from(item: i64) -> Self {
		RustBigInt(item.into())
	}
}

impl From<i32> for RustBigInt {
	fn from(item: i32) -> Self {
		RustBigInt(item.into())
	}
}

impl From<BigInt> for RustBigInt {
	fn from(item: BigInt) -> Self {
		RustBigInt(item)
	}
}

impl Clone for RustBigInt {
	fn clone(&self) -> Self {
		RustBigInt(self.0.clone())
	}
}

macro_rules! binary_operator {
	($trait:ident, $method:ident) => {
		impl $trait for RustBigInt {
			type Output = RustBigInt;

			fn $method(self, other: RustBigInt) -> RustBigInt {
				RustBigInt((self.0).$method(other.0))
			}
		}

		impl<'a, 'b> $trait<&'b RustBigInt> for &'a RustBigInt {
			type Output = RustBigInt;

			fn $method(self, other: &RustBigInt) -> RustBigInt {
				RustBigInt(self.0.clone().$method(other.0.clone()))
			}
		}
	};
}

binary_operator! {Add, add}
binary_operator! {Sub, sub}
binary_operator! {Mul, mul}
binary_operator! {Div, div}
binary_operator! {Rem, rem}

macro_rules! binary_assign_operator {
	($trait:ident, $method:ident) => {
		impl $trait<RustBigInt> for RustBigInt {
			fn $method(&mut self, other: Self) {
				BigInt::$method(&mut self.0, other.0)
			}
		}

		impl $trait<&RustBigInt> for RustBigInt {
			fn $method(&mut self, other: &RustBigInt) {
				BigInt::$method(&mut self.0, &other.0)
			}
		}
	};
}

binary_assign_operator! {AddAssign, add_assign}
binary_assign_operator! {SubAssign, sub_assign}
binary_assign_operator! {MulAssign, mul_assign}
binary_assign_operator! {DivAssign, div_assign}
binary_assign_operator! {RemAssign, rem_assign}

impl Neg for RustBigInt {
	type Output = RustBigInt;

	fn neg(self) -> Self::Output {
		RustBigInt(-self.0)
	}
}

impl PartialEq<Self> for RustBigInt {
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		PartialEq::eq(&self.0, &other.0)
	}
}

impl Eq for RustBigInt {}

impl PartialOrd<Self> for RustBigInt {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		PartialOrd::partial_cmp(&self.0, &other.0)
	}
}

impl Ord for RustBigInt {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering {
		Ord::cmp(&self.0, &other.0)
	}
}

impl PartialEq<i64> for RustBigInt {
	#[inline]
	fn eq(&self, other: &i64) -> bool {
		PartialEq::eq(&self.0, &BigInt::from(*other))
	}
}

impl PartialOrd<i64> for RustBigInt {
	#[inline]
	fn partial_cmp(&self, other: &i64) -> Option<Ordering> {
		PartialOrd::partial_cmp(&self.0, &BigInt::from(*other))
	}
}

use elrond_wasm::elrond_codec::*;

impl NestedEncode for RustBigInt {
	const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.to_signed_bytes_be().as_slice().dep_encode(dest)
	}

	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.to_signed_bytes_be()
			.as_slice()
			.dep_encode_or_exit(dest, c, exit);
	}
}

impl TopEncode for RustBigInt {
	const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		self.to_signed_bytes_be().top_encode(output)
	}

	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.to_signed_bytes_be()
			.top_encode_or_exit(output, c, exit)
	}
}

impl NestedDecode for RustBigInt {
	const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		let size = usize::dep_decode(input)?;
		let bytes = input.read_slice(size)?;
		Ok(RustBigInt::from_signed_bytes_be(bytes))
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		let size = usize::dep_decode_or_exit(input, c.clone(), exit);
		let bytes = input.read_slice_or_exit(size, c, exit);
		RustBigInt::from_signed_bytes_be(bytes)
	}
}

impl TopDecode for RustBigInt {
	const TYPE_INFO: TypeInfo = TypeInfo::BigInt;

	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		Ok(RustBigInt::from_signed_bytes_be(
			&*input.into_boxed_slice_u8(),
		))
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		_: ExitCtx,
		_: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		RustBigInt::from_signed_bytes_be(&*input.into_boxed_slice_u8())
	}
}

impl elrond_wasm::BigIntApi<RustBigUint> for RustBigInt {
	fn abs_uint(&self) -> RustBigUint {
		RustBigUint(self.0.abs())
	}

	fn sign(&self) -> elrond_wasm::Sign {
		match self.0.sign() {
			num_bigint::Sign::Minus => elrond_wasm::Sign::NoSign,
			num_bigint::Sign::NoSign => elrond_wasm::Sign::NoSign,
			num_bigint::Sign::Plus => elrond_wasm::Sign::Plus,
		}
	}

	fn to_signed_bytes_be(&self) -> Vec<u8> {
		if self.0.sign() == Sign::NoSign {
			Vec::new()
		} else {
			self.0.to_signed_bytes_be()
		}
	}

	fn from_signed_bytes_be(bytes: &[u8]) -> Self {
		let bi = BigInt::from_signed_bytes_be(bytes);
		bi.into()
	}
}
