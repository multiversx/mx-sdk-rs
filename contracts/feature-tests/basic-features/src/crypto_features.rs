multiversx_sc::imports!();

/// All crypto functions provided by Arwen exposed here.
#[multiversx_sc::module]
pub trait CryptoFeatures {
    #[endpoint]
    fn compute_sha256(&self, input: ManagedBuffer) -> ManagedByteArray<Self::Api, 32> {
        self.crypto().sha256(&input)
    }

    #[endpoint]
    fn compute_keccak256(&self, input: ManagedBuffer) -> ManagedByteArray<Self::Api, 32> {
        self.crypto().keccak256(&input)
    }

    #[endpoint]
    fn compute_ripemd160(&self, input: ManagedBuffer) -> ManagedByteArray<Self::Api, 20> {
        self.crypto().ripemd160(&input)
    }

    #[endpoint]
    fn verify_bls_signature(
        &self,
        key: ManagedBuffer,
        message: ManagedBuffer,
        signature: ManagedBuffer,
    ) -> bool {
        self.crypto().verify_bls(&key, &message, &signature)
    }

    #[endpoint]
    fn verify_ed25519_signature(
        &self,
        key: ManagedBuffer,
        message: ManagedBuffer,
        signature: ManagedBuffer,
    ) {
        self.crypto().verify_ed25519(&key, &message, &signature);
    }

    #[endpoint]
    fn verify_secp256k1_signature(
        &self,
        key: ManagedBuffer,
        message: ManagedBuffer,
        signature: ManagedBuffer,
    ) -> bool {
        self.crypto().verify_secp256k1(&key, &message, &signature)
    }

    #[endpoint]
    fn verify_custom_secp256k1_signature(
        &self,
        key: ManagedBuffer,
        message: ManagedBuffer,
        signature: ManagedBuffer,
        hash_type: MessageHashType,
    ) -> bool {
        self.crypto()
            .verify_custom_secp256k1(&key, &message, &signature, hash_type)
    }

    #[endpoint]
    fn compute_secp256k1_der_signature(&self, r: ManagedBuffer, s: ManagedBuffer) -> ManagedBuffer {
        self.crypto().encode_secp256k1_der_signature(&r, &s)
    }
}
