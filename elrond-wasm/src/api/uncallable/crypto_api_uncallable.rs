use super::UncallableApi;
use crate::{
    api::{
        CryptoApi, CryptoApiImpl, Handle, KECCAK256_RESULT_LEN, RIPEMD_RESULT_LEN,
        SHA256_RESULT_LEN,
    },
    types::{BoxedBytes, MessageHashType},
};

impl CryptoApi for UncallableApi {
    type CryptoApiImpl = UncallableApi;

    fn crypto_api_impl() -> Self::CryptoApiImpl {
        unreachable!()
    }
}

impl CryptoApiImpl for UncallableApi {
    fn sha256_legacy(&self, _data: &[u8]) -> [u8; SHA256_RESULT_LEN] {
        unreachable!()
    }

    fn sha256(&self, _data_handle: Handle) -> Handle {
        unreachable!()
    }

    fn keccak256_legacy(&self, _data: &[u8]) -> [u8; KECCAK256_RESULT_LEN] {
        unreachable!()
    }

    fn keccak256(&self, _data_handle: Handle) -> Handle {
        unreachable!()
    }

    fn ripemd160(&self, _data: &[u8]) -> [u8; RIPEMD_RESULT_LEN] {
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
