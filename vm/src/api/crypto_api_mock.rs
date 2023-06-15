use crate::DebugApi;
use ed25519_dalek::*;
use multiversx_sc::{
    api::{
        CryptoApi, CryptoApiImpl, ManagedBufferApiImpl, KECCAK256_RESULT_LEN, SHA256_RESULT_LEN,
    },
    types::MessageHashType,
};
use sha2::Sha256;
use sha3::{Digest, Keccak256};

impl CryptoApi for DebugApi {
    type CryptoApiImpl = DebugApi;

    fn crypto_api_impl() -> Self::CryptoApiImpl {
        DebugApi::new_from_static()
    }
}

impl DebugApi {
    fn sha256_legacy(&self, data: &[u8]) -> [u8; SHA256_RESULT_LEN] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    fn keccak256_legacy(&self, data: &[u8]) -> [u8; KECCAK256_RESULT_LEN] {
        let mut hasher = Keccak256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    /// There is a test that uses this directly.
    /// 
    /// TODO: switch that test to the managed version, or better still directly to the VM hooks implementation.
    pub fn verify_ed25519_legacy(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        let public = PublicKey::from_bytes(key);
        if public.is_err() {
            return false;
        }

        let sig = Signature::from_bytes(signature);
        if sig.is_err() {
            return false;
        }

        public.unwrap().verify(message, &sig.unwrap()).is_ok()
    }
}

impl CryptoApiImpl for DebugApi {
    fn sha256_managed(
        &self,
        dest: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        // default implementation used in debugger
        // the VM has a dedicated hook
        let result_bytes = self.sha256_legacy(self.mb_to_boxed_bytes(data_handle).as_slice());
        self.mb_overwrite(dest, &result_bytes[..]);
    }

    fn keccak256_managed(
        &self,
        dest: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        // default implementation used in debugger
        // the VM has a dedicated hook
        let result_bytes = self.keccak256_legacy(self.mb_to_boxed_bytes(data_handle).as_slice());
        self.mb_overwrite(dest, &result_bytes[..]);
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
        self.verify_ed25519_legacy(
            self.mb_to_boxed_bytes(key).as_slice(),
            self.mb_to_boxed_bytes(message).as_slice(),
            self.mb_to_boxed_bytes(signature).as_slice(),
        )
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
