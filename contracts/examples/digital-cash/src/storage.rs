multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::deposit_info::*;

#[multiversx_sc::module]
pub trait StorageModule {
    #[view]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &ManagedAddress) -> SingleValueMapper<DepositInfo<Self::Api>>;

    #[storage_mapper("fee")]
    fn fee(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("feeToken")]
    fn fee_token(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    #[storage_mapper("collectedFees")]
    fn collected_fees(&self) -> SingleValueMapper<BigUint>;
}
