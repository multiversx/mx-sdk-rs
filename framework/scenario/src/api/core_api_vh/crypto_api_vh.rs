use multiversx_sc::{
    api::{CryptoApi, CryptoApiImpl, HandleConstraints},
    types::MessageHashType,
};

use crate::api::{VMHooksApi, VMHooksApiBackend};

impl<VHB: VMHooksApiBackend> CryptoApi for VMHooksApi<VHB> {
    type CryptoApiImpl = Self;

    fn crypto_api_impl() -> Self::CryptoApiImpl {
        Self::api_impl()
    }
}

impl<VHB: VMHooksApiBackend> CryptoApiImpl for VMHooksApi<VHB> {
    fn sha256_managed(
        &self,
        result_handle: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks_ctx_2(&result_handle, &data_handle, |vh| {
            vh.managed_sha256(
                data_handle.get_raw_handle_unchecked(),
                result_handle.get_raw_handle_unchecked(),
            )
        });
    }

    fn keccak256_managed(
        &self,
        result_handle: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks_ctx_2(&result_handle, &data_handle, |vh| {
            vh.managed_keccak256(
                data_handle.get_raw_handle_unchecked(),
                result_handle.get_raw_handle_unchecked(),
            )
        });
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
    ) {
        self.with_vm_hooks_ctx_3(&key, &message, &signature, |vh| {
            vh.managed_verify_ed25519(
                key.get_raw_handle_unchecked(),
                message.get_raw_handle_unchecked(),
                signature.get_raw_handle_unchecked(),
            )
        });
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
