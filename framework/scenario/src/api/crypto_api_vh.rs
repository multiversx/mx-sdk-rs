use multiversx_sc::{
    api::{CryptoApi, CryptoApiImpl},
    types::MessageHashType,
};

use super::{VMHooksApi, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> CryptoApi for VMHooksApi<BACKEND_TYPE> {
    type CryptoApiImpl = Self;

    fn crypto_api_impl() -> Self::CryptoApiImpl {
        Self::api_impl()
    }
}

impl<const BACKEND_TYPE: VMHooksBackendType> CryptoApiImpl for VMHooksApi<BACKEND_TYPE> {
    fn sha256_managed(
        &self,
        result_handle: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks(|vh| vh.managed_sha256(data_handle, result_handle));
    }

    fn keccak256_managed(
        &self,
        result_handle: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks(|vh| vh.managed_keccak256(data_handle, result_handle));
    }

    fn ripemd160_managed(
        &self,
        _dest: Self::ManagedBufferHandle,
        _data_handle: Self::ManagedBufferHandle,
    ) {
        panic!("ripemd160 not implemented yet!")
    }

    fn verify_bls_managed(
        &self,
        _key: Self::ManagedBufferHandle,
        _message: Self::ManagedBufferHandle,
        _signature: Self::ManagedBufferHandle,
    ) -> bool {
        panic!("verify_bls not implemented yet!")
    }

    fn verify_ed25519_managed(
        &self,
        key: Self::ManagedBufferHandle,
        message: Self::ManagedBufferHandle,
        signature: Self::ManagedBufferHandle,
    ) -> bool {
        let result = self.with_vm_hooks(|vh| vh.managed_verify_ed25519(key, message, signature));
        result == 0
    }

    fn verify_secp256k1_managed(
        &self,
        _key: Self::ManagedBufferHandle,
        _message: Self::ManagedBufferHandle,
        _signature: Self::ManagedBufferHandle,
    ) -> bool {
        panic!("verify_secp256k1 not implemented yet!")
    }

    fn verify_custom_secp256k1_managed(
        &self,
        _key: Self::ManagedBufferHandle,
        _message: Self::ManagedBufferHandle,
        _signature: Self::ManagedBufferHandle,
        _hash_type: MessageHashType,
    ) -> bool {
        panic!("verify_custom_secp256k1 not implemented yet!")
    }

    fn encode_secp256k1_der_signature_managed(
        &self,
        _r: Self::ManagedBufferHandle,
        _s: Self::ManagedBufferHandle,
        _dest: Self::ManagedBufferHandle,
    ) {
        panic!("encode_secp256k1_signature not implemented yet!")
    }
}
