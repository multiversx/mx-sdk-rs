elrond_wasm::imports!();

/// All crypto functions provided by Arwen exposed here.
#[elrond_wasm::module]
pub trait CryptoFeatures {
    #[endpoint(computeSha256)]
    fn compute_sha256(&self, input: Vec<u8>) -> H256 {
        self.crypto().sha256(&input)
    }

    #[endpoint(computeKeccak256)]
    fn compute_keccak256(&self, input: Vec<u8>) -> H256 {
        self.crypto().keccak256(&input)
    }

    #[endpoint(computeRipemd160)]
    fn compute_ripemd160(&self, input: Vec<u8>) -> Box<[u8; 20]> {
        self.crypto().ripemd160(&input)
    }

    #[endpoint]
    fn verify_bls_signature(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        self.crypto().verify_bls(key, message, signature)
    }

    #[endpoint]
    fn verify_ed25519_signature(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        self.crypto().verify_ed25519(key, message, signature)
    }

    #[endpoint]
    fn verify_secp256k1_signature(&self, key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        self.crypto().verify_secp256k1(key, message, signature)
    }

    #[endpoint]
    fn verify_custom_secp256k1_signature(
        &self,
        key: &[u8],
        message: &[u8],
        signature: &[u8],
        hash_type: MessageHashType,
    ) -> bool {
        self.crypto()
            .verify_custom_secp256k1(key, message, signature, hash_type)
    }

    #[endpoint]
    fn compute_secp256k1_der_signature(&self, r: &[u8], s: &[u8]) -> BoxedBytes {
        self.crypto().encode_secp256k1_der_signature(r, s)
    }
}
