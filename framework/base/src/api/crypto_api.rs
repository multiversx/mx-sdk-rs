use crate::types::{heap::BoxedBytes, MessageHashType};

use super::{HandleTypeInfo, ManagedTypeApi, ManagedTypeApiImpl};

pub const SHA256_RESULT_LEN: usize = 32;
pub const KECCAK256_RESULT_LEN: usize = 32;
pub const RIPEMD_RESULT_LEN: usize = 20;
pub const ED25519_KEY_BYTE_LEN: usize = 32;
pub const ED25519_SIGNATURE_BYTE_LEN: usize = 64;

pub trait CryptoApi: ManagedTypeApi {
    type CryptoApiImpl: CryptoApiImpl
        + HandleTypeInfo<
            ManagedBufferHandle = Self::ManagedBufferHandle,
            BigIntHandle = Self::BigIntHandle,
            BigFloatHandle = Self::BigFloatHandle,
            EllipticCurveHandle = Self::EllipticCurveHandle,
        >;

    fn crypto_api_impl() -> Self::CryptoApiImpl;
}

pub trait CryptoApiImpl: ManagedTypeApiImpl {
    fn sha256_legacy(&self, data: &[u8]) -> [u8; SHA256_RESULT_LEN];

    fn sha256_managed(
        &self,
        dest: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    );

    fn keccak256_legacy(&self, data: &[u8]) -> [u8; KECCAK256_RESULT_LEN];

    fn keccak256_managed(
        &self,
        dest: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    );

    fn ripemd160_legacy(&self, data: &[u8]) -> [u8; RIPEMD_RESULT_LEN];

    fn ripemd160_managed(
        &self,
        dest: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    );

    fn verify_bls_legacy(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool;

    fn verify_bls_managed(
        &self,
        key: Self::ManagedBufferHandle,
        message: Self::ManagedBufferHandle,
        signature: Self::ManagedBufferHandle,
    ) -> bool;

    fn verify_ed25519_legacy(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool;

    fn verify_ed25519_managed(
        &self,
        key: Self::ManagedBufferHandle,
        message: Self::ManagedBufferHandle,
        signature: Self::ManagedBufferHandle,
    ) -> bool;

    /// Note: the signature is minimum 2 bytes in length,
    /// the second byte encodes the length of the remaining signature bytes.
    fn verify_secp256k1_legacy(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool;

    fn verify_secp256k1_managed(
        &self,
        key: Self::ManagedBufferHandle,
        message: Self::ManagedBufferHandle,
        signature: Self::ManagedBufferHandle,
    ) -> bool;

    fn verify_custom_secp256k1_legacy(
        &self,
        key: &[u8],
        message: &[u8],
        signature: &[u8],
        hash_type: MessageHashType,
    ) -> bool;

    fn verify_custom_secp256k1_managed(
        &self,
        key: Self::ManagedBufferHandle,
        message: Self::ManagedBufferHandle,
        signature: Self::ManagedBufferHandle,
        hash_type: MessageHashType,
    ) -> bool;

    fn encode_secp256k1_der_signature_legacy(&self, r: &[u8], s: &[u8]) -> BoxedBytes;

    fn encode_secp256k1_der_signature_managed(
        &self,
        r: Self::ManagedBufferHandle,
        s: Self::ManagedBufferHandle,
        dest: Self::ManagedBufferHandle,
    );
}
