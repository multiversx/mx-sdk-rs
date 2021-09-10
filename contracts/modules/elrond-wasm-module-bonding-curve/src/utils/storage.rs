elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::structs::{BondingCurve, TokenOwnershipData};

#[elrond_wasm::module]
pub trait StorageModule {
    #[storage_mapper("token_details")]
    fn token_details(
        &self,
        token: &TokenIdentifier,
    ) -> SingleValueMapper<TokenOwnershipData<Self::Api>>;

    #[storage_mapper("bonding_curve")]
    fn bonding_curve(&self, token: &TokenIdentifier) -> SingleValueMapper<BondingCurve<Self::Api>>;

    #[storage_mapper("owned_tokens")]
    fn owned_tokens(&self, owner: &ManagedAddress) -> SetMapper<TokenIdentifier>;

    #[storage_mapper("nonce_amount")]
    fn nonce_amount(&self, identifier: &TokenIdentifier, nonce: u64) -> SingleValueMapper<BigUint>;
}
