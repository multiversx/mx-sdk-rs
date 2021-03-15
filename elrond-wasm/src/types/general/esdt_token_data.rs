use crate::{abi::TypeAbi, api::BigUintApi};
use alloc::string::String;
use elrond_codec::*;

use super::{Address, BoxedBytes, EsdtTokenType, H256};

pub struct EsdtTokenData<BigUint: BigUintApi> {
	pub token_type: EsdtTokenType,
	pub amount: BigUint,
	pub frozen: bool,
	pub hash: H256,
	pub name: BoxedBytes,
	pub attributes: BoxedBytes,
	pub creator: Address,
	pub royalties: BigUint,
	pub uris: Vec<BoxedBytes>,
}

impl<BigUint: BigUintApi> EsdtTokenData<BigUint> {
	
}

impl<BigUint: BigUintApi> NestedEncode for EsdtTokenData<BigUint> {
	#[inline]
	fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
		self.token_type.dep_encode(dest)?;
		self.amount.dep_encode(dest)?;
		self.frozen.dep_encode(dest)?;
		self.hash.dep_encode(dest)?;
		self.name.dep_encode(dest)?;
		self.attributes.dep_encode(dest)?;
		self.creator.dep_encode(dest)?;
		self.royalties.dep_encode(dest)?;
		self.uris.dep_encode(dest)?;

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
		self.amount.dep_encode_or_exit(dest, c.clone(), exit);
		self.frozen.dep_encode_or_exit(dest, c.clone(), exit);
		self.hash.dep_encode_or_exit(dest, c.clone(), exit);
		self.name.dep_encode_or_exit(dest, c.clone(), exit);
		self.attributes.dep_encode_or_exit(dest, c.clone(), exit);
		self.creator.dep_encode_or_exit(dest, c.clone(), exit);
		self.royalties.dep_encode_or_exit(dest, c.clone(), exit);
		self.uris.dep_encode_or_exit(dest, c.clone(), exit);
	}
}

impl<BigUint: BigUintApi> TopEncode for EsdtTokenData<BigUint> {
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

impl<BigUint: BigUintApi> NestedDecode for EsdtTokenData<BigUint> {
	fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
		let token_type = EsdtTokenType::dep_decode(input)?;
		let amount = BigUint::dep_decode(input)?;
		let frozen = bool::dep_decode(input)?;
		let hash = H256::dep_decode(input)?;
		let name = BoxedBytes::dep_decode(input)?;
		let attributes = BoxedBytes::dep_decode(input)?;
		let creator = Address::dep_decode(input)?;
		let royalties = BigUint::dep_decode(input)?;
		let uris = Vec::<BoxedBytes>::dep_decode(input)?;

		Ok(Self {
			token_type,
			amount,
			frozen,
			hash,
			name,
			attributes,
			creator,
			royalties,
			uris
		})
	}

	fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
		input: &mut I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		let token_type = EsdtTokenType::dep_decode_or_exit(input, c.clone(), exit);
		let amount = BigUint::dep_decode_or_exit(input, c.clone(), exit);
		let frozen = bool::dep_decode_or_exit(input, c.clone(), exit);
		let hash = H256::dep_decode_or_exit(input, c.clone(), exit);
		let name = BoxedBytes::dep_decode_or_exit(input, c.clone(), exit);
		let attributes = BoxedBytes::dep_decode_or_exit(input, c.clone(), exit);
		let creator = Address::dep_decode_or_exit(input, c.clone(), exit);
		let royalties = BigUint::dep_decode_or_exit(input, c.clone(), exit);
		let uris = Vec::<BoxedBytes>::dep_decode_or_exit(input, c.clone(), exit);

		Self {
			token_type,
			amount,
			frozen,
			hash,
			name,
			attributes,
			creator,
			royalties,
			uris
		}
	}
}

impl<BigUint: BigUintApi> TopDecode for EsdtTokenData<BigUint> {
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

impl<BigUint: BigUintApi> TypeAbi for EsdtTokenData<BigUint> {
	fn type_name() -> String {
		"EsdtTokenData".into()
	}
}
