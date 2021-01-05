use crate::{TxContext, TxPanic};
use alloc::vec::Vec;
use elrond_wasm::api::CryptoApi;
use elrond_wasm::types::H256;
use sha3::{Digest, Keccak256, Sha3_256};

impl CryptoApi for TxContext {
	fn sha256(&self, data: &[u8]) -> H256 {
		let mut hasher = Sha3_256::new();
		hasher.input(data);
		let hash: [u8; 32] = hasher.result().into();
		hash.into()
	}

	fn keccak256(&self, data: &[u8]) -> H256 {
		let mut hasher = Keccak256::new();
		hasher.input(data);
		let hash: [u8; 32] = hasher.result().into();
		hash.into()
	}
}
