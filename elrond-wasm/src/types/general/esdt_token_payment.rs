use crate::{abi::TypeAbi, api::BigUintApi};
use alloc::string::String;
use elrond_codec::*;

use super::{EsdtTokenType, TokenIdentifier};

pub struct EsdtTokenPayment<BigUint: BigUintApi> {
	pub token_type: EsdtTokenType,
	pub token_name: TokenIdentifier,
	pub token_nonce: u64,
	pub amount: BigUint,
}

#[allow(clippy::redundant_clone)]
impl<BigUint: BigUintApi> NestedEncode for EsdtTokenPayment<BigUint> {
	#[inline]
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.token_type.dep_encode(dest)?;
		self.token_name.dep_encode(dest)?;
		self.token_nonce.dep_encode(dest)?;
		self.amount.dep_encode(dest)?;

		Ok(())
	}

	#[inline]
	fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
		&self,
		dest: &mut O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.token_type.dep_encode_or_exit(dest, c.clone(), exit);
		self.token_name.dep_encode_or_exit(dest, c.clone(), exit);
		self.token_nonce.dep_encode_or_exit(dest, c.clone(), exit);
		self.amount.dep_encode_or_exit(dest, c.clone(), exit);
	}
}

impl<BigUint: BigUintApi> TopEncode for EsdtTokenPayment<BigUint> {
	#[inline]
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		top_encode_from_nested(self, output)
	}

	#[inline]
	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		top_encode_from_nested_or_exit(self, output, c, exit);
	}
}

#[allow(clippy::redundant_clone)]
impl<BigUint: BigUintApi> NestedDecode for EsdtTokenPayment<BigUint> {
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		let token_type = EsdtTokenType::dep_decode(input)?;
		let token_name = TokenIdentifier::dep_decode(input)?;
		let token_nonce = u64::dep_decode(input)?;
		let amount = BigUint::dep_decode(input)?;

		Ok(Self {
			token_type,
			token_name,
			token_nonce,
			amount,
		})
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
        let token_type = EsdtTokenType::dep_decode_or_exit(input, c.clone(), exit);
		let token_name = TokenIdentifier::dep_decode_or_exit(input, c.clone(), exit);
		let token_nonce = u64::dep_decode_or_exit(input, c.clone(), exit);
		let amount = BigUint::dep_decode_or_exit(input, c.clone(), exit);

		Self {
			token_type,
			token_name,
			token_nonce,
			amount,
		}
	}
}

impl<BigUint: BigUintApi> TopDecode for EsdtTokenPayment<BigUint> {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		top_decode_from_nested(input)
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		top_decode_from_nested_or_exit(input, c, exit)
	}
}

impl<BigUint: BigUintApi> TypeAbi for EsdtTokenPayment<BigUint> {
	fn type_name() -> String {
		"EsdtTokenPayment".into()
	}
}
