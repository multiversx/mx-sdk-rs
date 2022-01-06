use super::UncallableApi;
use crate::{
    api::{CryptoApi, CryptoApiImpl},
    types::{BoxedBytes, MessageHashType, H256},
};
use alloc::boxed::Box;

impl CryptoApi for UncallableApi {
    type CryptoApiImpl = UncallableApi;

    fn crypto_api_impl() -> Self::CryptoApiImpl {
        unreachable!()
    }
}

impl CryptoApiImpl for UncallableApi {
    fn sha256_legacy(&self, _data: &[u8]) -> H256 {
        unreachable!()
    }

    fn keccak256_legacy(&self, _data: &[u8]) -> H256 {
        unreachable!()
    }

    fn ripemd160(&self, _data: &[u8]) -> Box<[u8; 20]> {
        unreachable!()
    }

    fn verify_bls(&self, _key: &[u8], _message: &[u8], _signature: &[u8]) -> bool {
        unreachable!()
    }

    fn verify_ed25519(&self, _key: &[u8], _message: &[u8], _signature: &[u8]) -> bool {
        unreachable!()
    }

    fn verify_secp256k1(&self, _key: &[u8], _message: &[u8], _signature: &[u8]) -> bool {
        unreachable!()
    }

    fn verify_custom_secp256k1(
        &self,
        _key: &[u8],
        _message: &[u8],
        _signature: &[u8],
        _hash_type: MessageHashType,
    ) -> bool {
        unreachable!()
    }

    fn encode_secp256k1_der_signature(&self, _r: &[u8], _s: &[u8]) -> BoxedBytes {
        unreachable!()
    }
}
