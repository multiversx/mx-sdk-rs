use super::VmApiImpl;
use multiversx_sc::{
    api::{CryptoApi, CryptoApiImpl},
    types::MessageHashType,
};

extern "C" {
    fn managedSha256(inputHandle: i32, outputHandle: i32) -> i32;

    fn managedKeccak256(inputHandle: i32, outputHandle: i32) -> i32;

    fn managedRipemd160(inputHandle: i32, outputHandle: i32) -> i32;

    fn managedVerifyBLS(keyHandle: i32, messageHandle: i32, sigHandle: i32) -> i32;

    fn managedVerifyEd25519(keyHandle: i32, messageHandle: i32, sigHandle: i32) -> i32;

    fn managedVerifySecp256k1(keyHandle: i32, messageHandle: i32, sigHandle: i32) -> i32;

    fn managedVerifyCustomSecp256k1(
        keyHandle: i32,
        messageHandle: i32,
        sigHandle: i32,
        hashType: i32,
    ) -> i32;

    fn managedEncodeSecp256k1DerSignature(rHandle: i32, sHandle: i32, sigHandle: i32) -> i32;
}

impl CryptoApi for VmApiImpl {
    type CryptoApiImpl = VmApiImpl;

    #[inline]
    fn crypto_api_impl() -> Self::CryptoApiImpl {
        VmApiImpl {}
    }
}

impl CryptoApiImpl for VmApiImpl {
    fn sha256_managed(
        &self,
        result_handle: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        unsafe {
            let _ = managedSha256(data_handle, result_handle);
        }
    }

    fn keccak256_managed(
        &self,
        result_handle: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        unsafe {
            let _ = managedKeccak256(data_handle, result_handle);
        }
    }

    #[inline]
    fn ripemd160_managed(
        &self,
        dest: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        unsafe {
            let _ = managedRipemd160(data_handle, dest);
        }
    }

    #[inline]
    fn verify_bls_managed(
        &self,
        key: Self::ManagedBufferHandle,
        message: Self::ManagedBufferHandle,
        signature: Self::ManagedBufferHandle,
    ) -> bool {
        unsafe { managedVerifyBLS(key, message, signature) == 0 }
    }

    #[inline]
    fn verify_ed25519_managed(
        &self,
        key: Self::ManagedBufferHandle,
        message: Self::ManagedBufferHandle,
        signature: Self::ManagedBufferHandle,
    ) {
        unsafe {
            managedVerifyEd25519(key, message, signature);
        }
    }

    #[inline]
    fn verify_secp256k1_managed(
        &self,
        key: Self::ManagedBufferHandle,
        message: Self::ManagedBufferHandle,
        signature: Self::ManagedBufferHandle,
    ) -> bool {
        unsafe { managedVerifySecp256k1(key, message, signature) == 0 }
    }

    #[inline]
    fn verify_custom_secp256k1_managed(
        &self,
        key: Self::ManagedBufferHandle,
        message: Self::ManagedBufferHandle,
        signature: Self::ManagedBufferHandle,
        hash_type: MessageHashType,
    ) -> bool {
        unsafe {
            managedVerifyCustomSecp256k1(key, message, signature, hash_type.as_u8() as i32) == 0
        }
    }

    fn encode_secp256k1_der_signature_managed(
        &self,
        r: Self::ManagedBufferHandle,
        s: Self::ManagedBufferHandle,
        dest_sig_handle: Self::ManagedBufferHandle,
    ) {
        unsafe {
            let _ = managedEncodeSecp256k1DerSignature(r, s, dest_sig_handle);
        }
    }
}
