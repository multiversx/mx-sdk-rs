use core::{borrow::Borrow, marker::PhantomData};

use crate::{
    api::{CryptoApi, CryptoApiImpl},
    types::{BoxedBytes, ManagedBuffer, ManagedByteArray, ManagedType, MessageHashType, H256},
};
use alloc::boxed::Box;

#[derive(Default)]
pub struct CryptoWrapper<A>
where
    A: CryptoApi,
{
    _phantom: PhantomData<A>,
}

impl<A> CryptoWrapper<A>
where
    A: CryptoApi,
{
    pub(crate) fn new() -> Self {
        CryptoWrapper {
            _phantom: PhantomData,
        }
    }

    pub fn sha256_legacy(&self, data: &[u8]) -> H256 {
        A::crypto_api_impl().sha256_legacy(data)
    }

    pub fn sha256<B: Borrow<ManagedBuffer<A>>>(&self, data: B) -> ManagedByteArray<A, 32> {
        ManagedByteArray::from_raw_handle(
            A::crypto_api_impl().sha256(data.borrow().get_raw_handle()),
        )
    }

    pub fn keccak256_legacy(&self, data: &[u8]) -> H256 {
        A::crypto_api_impl().keccak256_legacy(data)
    }

    pub fn keccak256<B: Borrow<ManagedBuffer<A>>>(&self, data: B) -> ManagedByteArray<A, 32> {
        ManagedByteArray::from_raw_handle(
            A::crypto_api_impl().keccak256(data.borrow().get_raw_handle()),
        )
    }

    pub fn ripemd160(&self, data: &[u8]) -> Box<[u8; 20]> {
        A::crypto_api_impl().ripemd160(data)
    }

    pub fn verify_bls(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        A::crypto_api_impl().verify_bls(key, message, signature)
    }

    pub fn verify_ed25519(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        A::crypto_api_impl().verify_ed25519(key, message, signature)
    }

    /// Note: the signature is minimum 2 bytes in length,
    /// the second byte encodes the length of the remaining signature bytes.
    pub fn verify_secp256k1(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        A::crypto_api_impl().verify_secp256k1(key, message, signature)
    }

    pub fn verify_custom_secp256k1(
        &self,
        key: &[u8],
        message: &[u8],
        signature: &[u8],
        hash_type: MessageHashType,
    ) -> bool {
        A::crypto_api_impl().verify_custom_secp256k1(key, message, signature, hash_type)
    }

    pub fn encode_secp256k1_der_signature(&self, r: &[u8], s: &[u8]) -> BoxedBytes {
        A::crypto_api_impl().encode_secp256k1_der_signature(r, s)
    }
}
