use super::UncallableApi;
use crate::{
    api::{
        CryptoApi, CryptoApiImpl, Handle, KECCAK256_RESULT_LEN, RIPEMD_RESULT_LEN,
        SHA256_RESULT_LEN,
    },
    types::{heap::BoxedBytes, MessageHashType},
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

    fn sha256_managed(&self, _dest: Handle, _data_handle: Handle) {
        unreachable!()
    }

    fn keccak256_legacy(&self, _data: &[u8]) -> [u8; KECCAK256_RESULT_LEN] {
        unreachable!()
    }

    fn keccak256_managed(&self, _dest: Handle, _data_handle: Handle) {
        unreachable!()
    }

    fn ripemd160_legacy(&self, _data: &[u8]) -> [u8; RIPEMD_RESULT_LEN] {
        unreachable!()
    }

    fn ripemd160_managed(&self, _dest: Handle, _data_handle: Handle) {
        unreachable!()
    }

    fn verify_bls_legacy(&self, _key: &[u8], _message: &[u8], _signature: &[u8]) -> bool {
        unreachable!()
    }

    fn verify_bls_managed(&self, _key: Handle, _message: Handle, _signature: Handle) -> bool {
        unreachable!()
    }

    fn verify_ed25519_legacy(&self, _key: &[u8], _message: &[u8], _signature: &[u8]) -> bool {
        unreachable!()
    }

    fn verify_ed25519_managed(&self, _key: Handle, _message: Handle, _signature: Handle) -> bool {
        unreachable!()
    }

    fn verify_secp256k1_legacy(&self, _key: &[u8], _message: &[u8], _signature: &[u8]) -> bool {
        unreachable!()
    }

    fn verify_secp256k1_managed(&self, _key: Handle, _message: Handle, _signature: Handle) -> bool {
        unreachable!()
    }

    fn verify_custom_secp256k1_legacy(
        &self,
        _key: &[u8],
        _message: &[u8],
        _signature: &[u8],
        _hash_type: MessageHashType,
    ) -> bool {
        unreachable!()
    }

    fn verify_custom_secp256k1_managed(
        &self,
        _key: Handle,
        _message: Handle,
        _signature: Handle,
        _hash_type: MessageHashType,
    ) -> bool {
        unreachable!()
    }

    fn encode_secp256k1_der_signature_legacy(&self, _r: &[u8], _s: &[u8]) -> BoxedBytes {
        unreachable!()
    }

    fn encode_secp256k1_der_signature_managed(&self, _r: Handle, _s: Handle, _dest: Handle) {
        unreachable!()
    }
}
