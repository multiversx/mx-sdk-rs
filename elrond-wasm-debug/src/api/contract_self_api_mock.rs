use super::big_int_api_mock::*;
use super::big_uint_api_mock::*;
use crate::TxContext;

impl elrond_wasm::api::ContractSelfApi<RustBigInt, RustBigUint> for TxContext {
	type Storage = Self;
	type CallValue = Self;
	type SendApi = Self;
	type BlockchainApi = Self;
	type CryptoApi = Self;

	#[inline]
	fn get_storage_raw(&self) -> Self::Storage {
		self.clone()
	}

	#[inline]
	fn call_value(&self) -> Self::CallValue {
		self.clone()
	}

	#[inline]
	fn send(&self) -> Self::SendApi {
		self.clone()
	}

	#[inline]
	fn blockchain(&self) -> Self::BlockchainApi {
		self.clone()
	}

	#[inline]
	fn crypto(&self) -> Self::CryptoApi {
		self.clone()
	}
}
