multiversx_sc::imports!();

/// Crypto functions that use the allocator.
/// Move to basic-features when they get upgraded.
#[multiversx_sc::module]
pub trait CryptoFeaturesAlloc {
    #[endpoint]
    fn compute_sha256_legacy_alloc(&self, input: Vec<u8>) -> H256 {
        self.crypto().sha256_legacy_alloc(&input)
    }

    #[endpoint]
    fn compute_keccak256_legacy_alloc(&self, input: Vec<u8>) -> H256 {
        self.crypto().keccak256_legacy_alloc(&input)
    }

    #[endpoint]
    fn compute_ripemd160_legacy(&self, input: Vec<u8>) -> Box<[u8; 20]> {
        self.crypto().ripemd160_legacy(&input)
    }

    #[endpoint]
    fn verify_bls_signature_legacy(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        self.crypto().verify_bls_legacy(key, message, signature)
    }

    #[endpoint]
    fn verify_ed25519_signature_legacy(
        &self,
        key: &[u8],
        message: &[u8],
        signature: &[u8],
    ) -> bool {
        self.crypto().verify_ed25519_legacy(key, message, signature)
    }

    #[endpoint]
    fn verify_secp256k1_signature_legacy(
        &self,
        key: &[u8],
        message: &[u8],
        signature: &[u8],
    ) -> bool {
        self.crypto()
            .verify_secp256k1_legacy(key, message, signature)
    }

    #[endpoint]
    fn verify_custom_secp256k1_signature_legacy(
        &self,
        key: &[u8],
        message: &[u8],
        signature: &[u8],
        hash_type: MessageHashType,
    ) -> bool {
        self.crypto()
            .verify_custom_secp256k1_legacy(key, message, signature, hash_type)
    }

    #[endpoint]
    fn compute_secp256k1_der_signature_legacy(&self, r: &[u8], s: &[u8]) -> BoxedBytes {
        self.crypto().encode_secp256k1_der_signature_legacy(r, s)
    }
}
