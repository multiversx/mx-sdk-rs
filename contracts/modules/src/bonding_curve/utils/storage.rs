multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use super::structs::TokenOwnershipData;

#[multiversx_sc::module]
pub trait StorageModule {
    #[storage_mapper("token_details")]
    fn token_details(
        &self,
        token: &EsdtTokenIdentifier,
    ) -> SingleValueMapper<TokenOwnershipData<Self::Api>>;

    #[storage_mapper("bonding_curve")]
    fn bonding_curve(&self, token: &EsdtTokenIdentifier) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("owned_tokens")]
    fn owned_tokens(&self, owner: &ManagedAddress) -> SetMapper<EsdtTokenIdentifier>;

    #[storage_mapper("nonce_amount")]
    fn nonce_amount(
        &self,
        identifier: &EsdtTokenIdentifier,
        nonce: u64,
    ) -> SingleValueMapper<BigUint>;
}
