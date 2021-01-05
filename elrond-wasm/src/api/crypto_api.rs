use crate::types::H256;

pub trait CryptoApi {
	fn sha256(&self, data: &[u8]) -> H256;

	fn keccak256(&self, data: &[u8]) -> H256;
}
