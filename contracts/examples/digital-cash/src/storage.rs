use multiversx_sc::{derive_imports::*, imports::*};

use crate::DepositKey;

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct DepositInfo<M: ManagedTypeApi> {
    pub depositor_address: ManagedAddress<M>,
    pub funds: ManagedVec<M, Payment<M>>,
    pub expiration: TimestampMillis,
    pub fees: Fee<M>,
}

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct Fee<M: ManagedTypeApi> {
    pub num_token_to_transfer: usize,
    pub value: Payment<M>,
}

#[multiversx_sc::module]
pub trait StorageModule {
    #[view]
    #[storage_mapper("deposit")]
    fn deposit(
        &self,
        deposit_key: &DepositKey<Self::Api>,
    ) -> SingleValueMapper<DepositInfo<Self::Api>>;

    #[storage_mapper("fee")]
    fn fee(&self, token: &TokenId) -> SingleValueMapper<BigUint>;

    #[storage_mapper("collectedFees")]
    fn collected_fees(&self, token: &TokenId) -> SingleValueMapper<BigUint>;

    #[storage_mapper("whitelistedFeeTokens")]
    fn whitelisted_fee_tokens(&self) -> UnorderedSetMapper<TokenId>;

    #[storage_mapper("allTimeFeeTokens")]
    fn all_time_fee_tokens(&self) -> UnorderedSetMapper<TokenId>;
}
