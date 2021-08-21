use crate::types::{BoxedBytes, MessageHashType, H256};
use alloc::boxed::Box;

pub trait CryptoApi {
    fn sha256(&self, data: &[u8]) -> H256;

    fn keccak256(&self, data: &[u8]) -> H256;

    fn ripemd160(&self, data: &[u8]) -> Box<[u8; 20]>;

    fn verify_bls(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool;

    fn verify_ed25519(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool;

    /// Note: the signature is minimum 2 bytes in length,
    /// the second byte encodes the length of the remaining signature bytes.
    fn verify_secp256k1(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool;

    fn verify_custom_secp256k1(
        &self,
        key: &[u8],
        message: &[u8],
        signature: &[u8],
        hash_type: MessageHashType,
    ) -> bool;

    fn encode_secp256k1_der_signature(&self, r: &[u8], s: &[u8]) -> BoxedBytes;
}
