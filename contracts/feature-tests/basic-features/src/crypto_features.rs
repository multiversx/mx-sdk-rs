elrond_wasm::imports!();

/// All crypto functions provided by Arwen exposed here.
#[elrond_wasm::module]
pub trait CryptoFeatures {
    #[endpoint]
    #[allow(deprecated)]
    fn compute_sha256_legacy_managed(
        &self,
        input: ManagedBuffer,
    ) -> ManagedByteArray<Self::Api, 32> {
        self.crypto().sha256_legacy_managed::<100>(&input)
    }

    #[endpoint]
    fn compute_sha256(&self, input: ManagedBuffer) -> ManagedByteArray<Self::Api, 32> {
        self.crypto().sha256(&input)
    }

    #[endpoint]
    #[allow(deprecated)]
    fn compute_keccak256_legacy_managed(
        &self,
        input: ManagedBuffer,
    ) -> ManagedByteArray<Self::Api, 32> {
        self.crypto().keccak256_legacy_managed::<100>(&input)
    }

    #[endpoint]
    fn compute_keccak256(&self, input: ManagedBuffer) -> ManagedByteArray<Self::Api, 32> {
        self.crypto().keccak256(&input)
    }
}
