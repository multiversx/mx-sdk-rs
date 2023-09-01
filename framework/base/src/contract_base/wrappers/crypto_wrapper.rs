use core::marker::PhantomData;

use crate::{
    api::{
        use_raw_handle, CryptoApi, CryptoApiImpl, StaticVarApiImpl, KECCAK256_RESULT_LEN,
        SHA256_RESULT_LEN,
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
    pub fn new() -> Self {
        CryptoWrapper {
            _phantom: PhantomData,
        }
    }

    pub fn sha256<B: core::borrow::Borrow<ManagedBuffer<A>>>(
        &self,
        data: B,
    ) -> ManagedByteArray<A, SHA256_RESULT_LEN> {
        let new_handle: A::ManagedBufferHandle =
            use_raw_handle(A::static_var_api_impl().next_handle());
        A::crypto_api_impl().sha256_managed(new_handle.clone(), data.borrow().get_handle());
        ManagedByteArray::from_handle(new_handle)
    }

    pub fn keccak256<B: core::borrow::Borrow<ManagedBuffer<A>>>(
        &self,
        data: B,
    ) -> ManagedByteArray<A, KECCAK256_RESULT_LEN> {
        let new_handle: A::ManagedBufferHandle =
            use_raw_handle(A::static_var_api_impl().next_handle());
        A::crypto_api_impl().keccak256_managed(new_handle.clone(), data.borrow().get_handle());
        ManagedByteArray::from_handle(new_handle)
    }

    pub fn ripemd160<B: core::borrow::Borrow<ManagedBuffer<A>>>(
        &self,
        data: B,
    ) -> ManagedByteArray<A, { crate::api::RIPEMD_RESULT_LEN }> {
        let new_handle: A::ManagedBufferHandle =
            use_raw_handle(A::static_var_api_impl().next_handle());
        A::crypto_api_impl().ripemd160_managed(new_handle.clone(), data.borrow().get_handle());
        ManagedByteArray::from_handle(new_handle)
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

    /// Will crash if the verification fails.
    ///
    /// The error comes straight form the VM, the message is "invalid signature".
    pub fn verify_ed25519(
        &self,
        key: &ManagedBuffer<A>,
        message: &ManagedBuffer<A>,
        signature: &ManagedBuffer<A>,
    ) {
        A::crypto_api_impl().verify_ed25519_managed(
            key.get_handle(),
            message.get_handle(),
            signature.get_handle(),
        )
    }

    /// Note: the signature is minimum 2 bytes in length,
    /// the second byte encodes the length of the remaining signature bytes.
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

    pub fn encode_secp256k1_der_signature(
        &self,
        r: &ManagedBuffer<A>,
        s: &ManagedBuffer<A>,
    ) -> ManagedBuffer<A> {
        let new_handle: A::ManagedBufferHandle =
            use_raw_handle(A::static_var_api_impl().next_handle());
        A::crypto_api_impl().encode_secp256k1_der_signature_managed(
            r.get_handle(),
            s.get_handle(),
            new_handle.clone(),
        );
        ManagedBuffer::from_handle(new_handle)
    }
}
