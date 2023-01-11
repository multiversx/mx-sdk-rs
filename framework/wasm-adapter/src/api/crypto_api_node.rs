use super::VmApiImpl;
use multiversx_sc::{
    api::{CryptoApi, CryptoApiImpl},
    types::{heap::BoxedBytes, MessageHashType},
};

extern "C" {
    fn sha256(dataOffset: *const u8, length: i32, resultOffset: *mut u8) -> i32;

    fn managedSha256(inputHandle: i32, outputHandle: i32) -> i32;

    fn keccak256(dataOffset: *const u8, length: i32, resultOffset: *mut u8) -> i32;

    fn managedKeccak256(inputHandle: i32, outputHandle: i32) -> i32;

    fn ripemd160(dataOffset: *const u8, length: i32, resultOffset: *mut u8) -> i32;

    fn managedRipemd160(inputHandle: i32, outputHandle: i32) -> i32;

    fn verifyBLS(
        keyOffset: *const u8,
        messageOffset: *const u8,
        messageLength: i32,
        sigOffset: *const u8,
    ) -> i32;

    fn managedVerifyBLS(keyHandle: i32, messageHandle: i32, sigHandle: i32) -> i32;

    fn verifyEd25519(
        keyOffset: *const u8,
        messageOffset: *const u8,
        messageLength: i32,
        sigOffset: *const u8,
    ) -> i32;

    fn managedVerifyEd25519(keyHandle: i32, messageHandle: i32, sigHandle: i32) -> i32;

    fn verifySecp256k1(
        keyOffset: *const u8,
        keyLength: i32,
        messageOffset: *const u8,
        messageLength: i32,
        sigOffset: *const u8,
    ) -> i32;

    fn managedVerifySecp256k1(keyHandle: i32, messageHandle: i32, sigHandle: i32) -> i32;

    fn verifyCustomSecp256k1(
        keyOffset: *const u8,
        keyLength: i32,
        messageOffset: *const u8,
        messageLength: i32,
        sigOffset: *const u8,
        hashType: i32,
    ) -> i32;

    fn managedVerifyCustomSecp256k1(
        keyHandle: i32,
        messageHandle: i32,
        sigHandle: i32,
        hashType: i32,
    ) -> i32;

    fn encodeSecp256k1DerSignature(
        rOffset: *const u8,
        rLength: i32,
        sOffset: *const u8,
        sLength: i32,
        sigOffset: *const u8,
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
    #[inline]
    fn sha256_legacy(&self, data: &[u8]) -> [u8; 32] {
        unsafe {
            let mut res = [0u8; 32];
            sha256(data.as_ptr(), data.len() as i32, res.as_mut_ptr());
            res
        }
    }

    fn sha256_managed(
        &self,
        result_handle: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        unsafe {
            let _ = managedSha256(data_handle, result_handle);
        }
    }

    #[inline]
    fn keccak256_legacy(&self, data: &[u8]) -> [u8; 32] {
        unsafe {
            let mut res = [0u8; 32];
            keccak256(data.as_ptr(), data.len() as i32, res.as_mut_ptr());
            res
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
    fn ripemd160_legacy(&self, data: &[u8]) -> [u8; 20] {
        unsafe {
            let mut res = [0u8; 20];
            ripemd160(data.as_ptr(), data.len() as i32, res.as_mut_ptr());
            res
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

    // the verify functions return 0 if valid signature, -1 if invalid

    fn verify_bls_legacy(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        unsafe {
            verifyBLS(
                key.as_ptr(),
                message.as_ptr(),
                message.len() as i32,
                signature.as_ptr(),
            ) == 0
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
    fn verify_ed25519_legacy(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        unsafe {
            verifyEd25519(
                key.as_ptr(),
                message.as_ptr(),
                message.len() as i32,
                signature.as_ptr(),
            ) == 0
        }
    }

    #[inline]
    fn verify_ed25519_managed(
        &self,
        key: Self::ManagedBufferHandle,
        message: Self::ManagedBufferHandle,
        signature: Self::ManagedBufferHandle,
    ) -> bool {
        unsafe { managedVerifyEd25519(key, message, signature) == 0 }
    }

    #[inline]
    fn verify_secp256k1_legacy(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        unsafe {
            verifySecp256k1(
                key.as_ptr(),
                key.len() as i32,
                message.as_ptr(),
                message.len() as i32,
                signature.as_ptr(),
            ) == 0
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
    fn verify_custom_secp256k1_legacy(
        &self,
        key: &[u8],
        message: &[u8],
        signature: &[u8],
        hash_type: MessageHashType,
    ) -> bool {
        unsafe {
            verifyCustomSecp256k1(
                key.as_ptr(),
                key.len() as i32,
                message.as_ptr(),
                message.len() as i32,
                signature.as_ptr(),
                hash_type.as_u8() as i32,
            ) == 0
        }
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

    fn encode_secp256k1_der_signature_legacy(&self, r: &[u8], s: &[u8]) -> BoxedBytes {
        unsafe {
            // 3 for "magic" numbers in the signature + 3 for lengths: total_sig_length, r_length, s_length
            let mut sig_length = 6 + r.len() + s.len();
            let mask = 0x80;

            // 1 additional zero-byte is added for r and s if they could be misinterpreted as a negative number
            if r[0] & mask != 0 {
                sig_length += 1;
            }
            if s[0] & mask != 0 {
                sig_length += 1;
            }

            let mut sig_output = BoxedBytes::allocate(sig_length);

            encodeSecp256k1DerSignature(
                r.as_ptr(),
                r.len() as i32,
                s.as_ptr(),
                s.len() as i32,
                sig_output.as_mut_ptr(),
            );

            sig_output
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
