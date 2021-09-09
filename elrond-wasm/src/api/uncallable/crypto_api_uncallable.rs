use super::UncallableApi;
use crate::{
    api::CryptoApi,
    types::{BoxedBytes, MessageHashType, H256},
};
use alloc::boxed::Box;

impl CryptoApi for UncallableApi {
    fn sha256(&self, data: &[u8]) -> H256 {
        unreachable!()
    }

    fn keccak256(&self, data: &[u8]) -> H256 {
        unreachable!()
    }

    fn ripemd160(&self, data: &[u8]) -> Box<[u8; 20]> {
        unreachable!()
    }

    fn verify_bls(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        unreachable!()
    }

    fn verify_ed25519(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        unreachable!()
    }

    fn verify_secp256k1(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        unreachable!()
    }

    fn verify_custom_secp256k1(
        &self,
        key: &[u8],
        message: &[u8],
        signature: &[u8],
        hash_type: MessageHashType,
    ) -> bool {
        unreachable!()
    }

    fn encode_secp256k1_der_signature(&self, r: &[u8], s: &[u8]) -> BoxedBytes {
        unreachable!()
    }
}
