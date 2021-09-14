use crate::{
    api::CryptoApi,
    types::{BoxedBytes, MessageHashType, H256},
};
use alloc::boxed::Box;

pub struct CryptoWrapper<A>
where
    A: CryptoApi,
{
    pub(crate) api: A,
}

impl<A> CryptoWrapper<A>
where
    A: CryptoApi,
{
    pub(crate) fn new(api: A) -> Self {
        CryptoWrapper { api }
    }

    pub fn sha256(&self, data: &[u8]) -> H256 {
        self.api.sha256(data)
    }

    pub fn keccak256(&self, data: &[u8]) -> H256 {
        self.api.keccak256(data)
    }

    pub fn ripemd160(&self, data: &[u8]) -> Box<[u8; 20]> {
        self.api.ripemd160(data)
    }

    pub fn verify_bls(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        self.api.verify_bls(key, message, signature)
    }

    pub fn verify_ed25519(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        self.api.verify_ed25519(key, message, signature)
    }

    /// Note: the signature is minimum 2 bytes in length,
    /// the second byte encodes the length of the remaining signature bytes.
    pub fn verify_secp256k1(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        self.api.verify_secp256k1(key, message, signature)
    }

    pub fn verify_custom_secp256k1(
        &self,
        key: &[u8],
        message: &[u8],
        signature: &[u8],
        hash_type: MessageHashType,
    ) -> bool {
        self.api
            .verify_custom_secp256k1(key, message, signature, hash_type)
    }

    pub fn encode_secp256k1_der_signature(&self, r: &[u8], s: &[u8]) -> BoxedBytes {
        self.api.encode_secp256k1_der_signature(r, s)
    }
}
