use crate::types::H256;

pub trait CryptoApi {
	fn sha256(&self, data: &[u8]) -> H256;

	fn keccak256(&self, data: &[u8]) -> H256;

	fn verify_bls(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool;

	fn verify_ed25519(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool;

	fn verify_secp256k1(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool;
}
