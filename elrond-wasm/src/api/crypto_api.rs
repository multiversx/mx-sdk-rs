use crate::types::{BoxedBytes, ManagedBuffer, ManagedByteArray, MessageHashType, H256};
use alloc::boxed::Box;

use super::ManagedTypeApi;

pub trait CryptoApi: ManagedTypeApi {
    fn sha256_legacy(&self, data: &[u8]) -> H256;

    fn sha256(&self, data: &ManagedBuffer<Self>) -> ManagedByteArray<Self, 32> {
        self.sha256_legacy(data.to_boxed_bytes().as_slice())
            .as_array()
            .into()
    }

    fn keccak256_legacy(&self, data: &[u8]) -> H256;

    fn keccak256(&self, data: &ManagedBuffer<Self>) -> ManagedByteArray<Self, 32> {
        self.keccak256_legacy(data.to_boxed_bytes().as_slice())
            .as_array()
            .into()
    }

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
