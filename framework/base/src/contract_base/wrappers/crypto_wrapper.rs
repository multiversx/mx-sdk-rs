use core::marker::PhantomData;

use crate::{
    api::{CryptoApi, CryptoApiImpl, KECCAK256_RESULT_LEN, SHA256_RESULT_LEN},
    types::{ManagedBuffer, ManagedByteArray, ManagedType, ManagedVec, MessageHashType},
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
        unsafe {
            let result = ManagedByteArray::new_uninit();
            A::crypto_api_impl().sha256_managed(result.get_handle(), data.borrow().get_handle());
            result
        }
    }

    pub fn keccak256<B: core::borrow::Borrow<ManagedBuffer<A>>>(
        &self,
        data: B,
    ) -> ManagedByteArray<A, KECCAK256_RESULT_LEN> {
        unsafe {
            let result = ManagedByteArray::new_uninit();
            A::crypto_api_impl().keccak256_managed(result.get_handle(), data.borrow().get_handle());
            result
        }
    }

    pub fn ripemd160<B: core::borrow::Borrow<ManagedBuffer<A>>>(
        &self,
        data: B,
    ) -> ManagedByteArray<A, { crate::api::RIPEMD_RESULT_LEN }> {
        unsafe {
            let result = ManagedByteArray::new_uninit();
            A::crypto_api_impl().ripemd160_managed(result.get_handle(), data.borrow().get_handle());
            result
        }
    }

    pub fn verify_bls(
        &self,
        key: &ManagedBuffer<A>,
        message: &ManagedBuffer<A>,
        signature: &ManagedBuffer<A>,
    ) {
        A::crypto_api_impl().verify_bls_managed(
            key.get_handle(),
            message.get_handle(),
            signature.get_handle(),
        )
    }

    /// Calls the Vm to verify ed25519 signature.
    ///
    /// Does not return result, will fail tx directly!
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
        unsafe {
            let result = ManagedBuffer::new_uninit();
            A::crypto_api_impl().encode_secp256k1_der_signature_managed(
                r.get_handle(),
                s.get_handle(),
                result.get_handle(),
            );
            result
        }
    }

    /// Calls the Vm to verify secp256r1 signature.
    ///
    /// Does not return result, will fail tx directly!
    pub fn verify_secp256r1(
        &self,
        key: &ManagedBuffer<A>,
        message: &ManagedBuffer<A>,
        signature: &ManagedBuffer<A>,
    ) {
        A::crypto_api_impl().verify_secp256r1_managed(
            key.get_handle(),
            message.get_handle(),
            signature.get_handle(),
        )
    }

    /// Calls the Vm to verify BLS signature share.
    ///
    /// Does not return result, will fail tx directly!
    pub fn verify_bls_signature_share(
        &self,
        key: &ManagedBuffer<A>,
        message: &ManagedBuffer<A>,
        signature: &ManagedBuffer<A>,
    ) {
        A::crypto_api_impl().verify_bls_signature_share_managed(
            key.get_handle(),
            message.get_handle(),
            signature.get_handle(),
        )
    }

    /// Calls the Vm to verify BLS aggregated signature.
    ///
    /// Does not return result, will fail tx directly!
    pub fn verify_bls_aggregated_signature(
        &self,
        keys: &ManagedVec<A, ManagedBuffer<A>>,
        message: &ManagedBuffer<A>,
        signature: &ManagedBuffer<A>,
    ) {
        A::crypto_api_impl().verify_bls_aggregated_signature_managed(
            keys.get_handle(),
            message.get_handle(),
            signature.get_handle(),
        )
    }
}
