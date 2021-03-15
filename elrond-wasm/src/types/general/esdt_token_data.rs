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

impl<BigUint: BigUintApi> TypeAbi for EsdtTokenData<BigUint> {
	fn type_name() -> String {
		"EsdtTokenData".into()
	}
}
