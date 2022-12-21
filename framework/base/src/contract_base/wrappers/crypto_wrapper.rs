use core::marker::PhantomData;

use crate::{
    api::{
        CryptoApi, CryptoApiImpl, StaticVarApiImpl, ED25519_KEY_BYTE_LEN,
        ED25519_SIGNATURE_BYTE_LEN, KECCAK256_RESULT_LEN, SHA256_RESULT_LEN,
    },
    types::{ManagedBuffer, ManagedByteArray, ManagedType, MessageHashType},
};

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

    pub fn sha256<B: core::borrow::Borrow<ManagedBuffer<A>>>(
        &self,
        data: B,
    ) -> ManagedByteArray<A, SHA256_RESULT_LEN> {
        let new_handle: A::ManagedBufferHandle = A::static_var_api_impl().next_handle();
        A::crypto_api_impl().sha256_managed(new_handle.clone(), data.borrow().get_handle());
        ManagedByteArray::from_handle(new_handle)
    }

    #[cfg(feature = "alloc")]
    pub fn sha256_legacy_alloc(&self, data: &[u8]) -> crate::types::H256 {
        crate::types::H256::from(A::crypto_api_impl().sha256_legacy(data))
    }

    #[deprecated(
        since = "0.31.0",
        note = "Method no longer needed, use `sha256` instead, functionality is available on mainnet."
    )]
    pub fn sha256_legacy_managed<const MAX_INPUT_LEN: usize>(
        &self,
        data: &ManagedBuffer<A>,
    ) -> ManagedByteArray<A, SHA256_RESULT_LEN> {
        let mut data_buffer = [0u8; MAX_INPUT_LEN];
        let data_buffer_slice = data.load_to_byte_array(&mut data_buffer);
        ManagedByteArray::new_from_bytes(&A::crypto_api_impl().sha256_legacy(data_buffer_slice))
    }

    pub fn keccak256<B: core::borrow::Borrow<ManagedBuffer<A>>>(
        &self,
        data: B,
    ) -> ManagedByteArray<A, KECCAK256_RESULT_LEN> {
        let new_handle: A::ManagedBufferHandle = A::static_var_api_impl().next_handle();
        A::crypto_api_impl().keccak256_managed(new_handle.clone(), data.borrow().get_handle());
        ManagedByteArray::from_handle(new_handle)
    }

    #[cfg(feature = "alloc")]
    pub fn keccak256_legacy_alloc(&self, data: &[u8]) -> crate::types::H256 {
        crate::types::H256::from(A::crypto_api_impl().keccak256_legacy(data))
    }

    #[deprecated(
        since = "0.31.0",
        note = "Method no longer needed, use `keccak256` instead, functionality is available on mainnet."
    )]
    pub fn keccak256_legacy_managed<const MAX_INPUT_LEN: usize>(
        &self,
        data: &ManagedBuffer<A>,
    ) -> ManagedByteArray<A, KECCAK256_RESULT_LEN> {
        let mut data_buffer = [0u8; MAX_INPUT_LEN];
        let data_buffer_slice = data.load_to_byte_array(&mut data_buffer);
        ManagedByteArray::new_from_bytes(&A::crypto_api_impl().keccak256_legacy(data_buffer_slice))
    }

    #[cfg(feature = "alloc")]
    pub fn ripemd160_legacy(&self, data: &[u8]) -> crate::types::Box<[u8; 20]> {
        crate::types::Box::new(A::crypto_api_impl().ripemd160_legacy(data))
    }

    pub fn ripemd160<B: core::borrow::Borrow<ManagedBuffer<A>>>(
        &self,
        data: B,
    ) -> ManagedByteArray<A, { crate::api::RIPEMD_RESULT_LEN }> {
        let new_handle: A::ManagedBufferHandle = A::static_var_api_impl().next_handle();
        A::crypto_api_impl().ripemd160_managed(new_handle.clone(), data.borrow().get_handle());
        ManagedByteArray::from_handle(new_handle)
    }

    pub fn verify_bls_legacy(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        A::crypto_api_impl().verify_bls_legacy(key, message, signature)
    }

    pub fn verify_bls(
        &self,
        key: &ManagedBuffer<A>,
        message: &ManagedBuffer<A>,
        signature: &ManagedBuffer<A>,
    ) -> bool {
        A::crypto_api_impl().verify_bls_managed(
            key.get_handle(),
            message.get_handle(),
            signature.get_handle(),
        )
    }

    pub fn verify_ed25519_legacy(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        A::crypto_api_impl().verify_ed25519_legacy(key, message, signature)
    }

    pub fn verify_ed25519_legacy_managed<const MAX_MESSAGE_LEN: usize>(
        &self,
        key: &ManagedByteArray<A, ED25519_KEY_BYTE_LEN>,
        message: &ManagedBuffer<A>,
        signature: &ManagedByteArray<A, ED25519_SIGNATURE_BYTE_LEN>,
    ) -> bool {
        let key_bytes = key.to_byte_array();
        let mut message_byte_buffer = [0u8; MAX_MESSAGE_LEN];
        let message_byte_slice = message.load_to_byte_array(&mut message_byte_buffer);
        let sig_bytes = signature.to_byte_array();

        A::crypto_api_impl().verify_ed25519_legacy(
            &key_bytes[..],
            message_byte_slice,
            &sig_bytes[..],
        )
    }

    pub fn verify_ed25519(
        &self,
        key: &ManagedBuffer<A>,
        message: &ManagedBuffer<A>,
        signature: &ManagedBuffer<A>,
    ) -> bool {
        A::crypto_api_impl().verify_ed25519_managed(
            key.get_handle(),
            message.get_handle(),
            signature.get_handle(),
        )
    }

    /// Note: the signature is minimum 2 bytes in length,
    /// the second byte encodes the length of the remaining signature bytes.
    pub fn verify_secp256k1_legacy(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        A::crypto_api_impl().verify_secp256k1_legacy(key, message, signature)
    }

    pub fn verify_secp256k1(
        &self,
        key: &ManagedBuffer<A>,
        message: &ManagedBuffer<A>,
        signature: &ManagedBuffer<A>,
    ) -> bool {
        A::crypto_api_impl().verify_secp256k1_managed(
            key.get_handle(),
            message.get_handle(),
            signature.get_handle(),
        )
    }

    pub fn verify_custom_secp256k1_legacy(
        &self,
        key: &[u8],
        message: &[u8],
        signature: &[u8],
        hash_type: MessageHashType,
    ) -> bool {
        A::crypto_api_impl().verify_custom_secp256k1_legacy(key, message, signature, hash_type)
    }

    pub fn verify_custom_secp256k1(
        &self,
        key: &ManagedBuffer<A>,
        message: &ManagedBuffer<A>,
        signature: &ManagedBuffer<A>,
        hash_type: MessageHashType,
    ) -> bool {
        A::crypto_api_impl().verify_custom_secp256k1_managed(
            key.get_handle(),
            message.get_handle(),
            signature.get_handle(),
            hash_type,
        )
    }

    #[cfg(feature = "alloc")]
    pub fn encode_secp256k1_der_signature_legacy(
        &self,
        r: &[u8],
        s: &[u8],
    ) -> crate::types::BoxedBytes {
        A::crypto_api_impl().encode_secp256k1_der_signature_legacy(r, s)
    }

    pub fn encode_secp256k1_der_signature(
        &self,
        r: &ManagedBuffer<A>,
        s: &ManagedBuffer<A>,
    ) -> ManagedBuffer<A> {
        let new_handle: A::ManagedBufferHandle = A::static_var_api_impl().next_handle();
        A::crypto_api_impl().encode_secp256k1_der_signature_managed(
            r.get_handle(),
            s.get_handle(),
            new_handle.clone(),
        );
        ManagedBuffer::from_handle(new_handle)
    }
}
