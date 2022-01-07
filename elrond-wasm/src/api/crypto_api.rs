use crate::types::{BoxedBytes, MessageHashType, H256};
use alloc::boxed::Box;

use super::{Handle, ManagedTypeApi, ManagedTypeApiImpl};

pub trait CryptoApi: ManagedTypeApi {
    type CryptoApiImpl: CryptoApiImpl;

    fn crypto_api_impl() -> Self::CryptoApiImpl;
}

pub trait CryptoApiImpl: ManagedTypeApiImpl {
    fn sha256_legacy(&self, data: &[u8]) -> H256;

    fn sha256(&self, data_handle: Handle) -> Handle {
        // default implementation used in debugger
        // the VM has a dedicated hook
        self.mb_new_from_bytes(
            self.sha256_legacy(self.mb_to_boxed_bytes(data_handle).as_slice())
                .as_array(),
        )
    }

    fn keccak256_legacy(&self, data: &[u8]) -> H256;

    fn keccak256(&self, data_handle: Handle) -> Handle {
        // default implementation used in debugger
        // the VM has a dedicated hook
        self.mb_new_from_bytes(
            self.keccak256_legacy(self.mb_to_boxed_bytes(data_handle).as_slice())
                .as_array(),
        )
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
