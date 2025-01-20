#![no_std]

use multiversx_sc::derive_imports::*;
use multiversx_sc::imports::*;

pub mod adder_proxy;

#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Clone, Debug, PartialEq,
)]
pub struct SovereignConfig<M: ManagedTypeApi> {
    pub min_validators: u64,
    pub max_validators: u64,
    pub min_stake: BigUint<M>,
    pub opt_additional_stake_required: Option<ManagedVec<M, StakeArgs<M>>>,
}

#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Clone, Debug, PartialEq,
)]
pub struct StakeArgs<M: ManagedTypeApi> {
    pub token_id: TokenIdentifier<M>,
    pub amount: BigUint<M>,
}

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[multiversx_sc::contract]
pub trait Adder {
    #[view(getSum)]
    #[storage_mapper("sum")]
    fn sum(&self) -> SingleValueMapper<BigUint>;

    #[view]
    #[storage_mapper("sovereignConfig")]
    fn config(&self) -> SingleValueMapper<SovereignConfig<Self::Api>>;

    #[storage_mapper_from_address("sovereignConfigFromAddress")]
    fn config_from_address(
        &self,
        address: ManagedAddress,
    ) -> SingleValueMapper<SovereignConfig<Self::Api>, ManagedAddress>;

    #[endpoint]
    fn get_storage_from_address(&self, address: ManagedAddress) -> SovereignConfig<Self::Api> {
        self.config_from_address(address).get()
    }

    #[endpoint]
    fn set_storage(&self, config: SovereignConfig<Self::Api>) {
        self.config().set(config)
    }

    #[init]
    fn init(&self, initial_value: BigUint) {
        self.sum().set(initial_value);
    }

    #[upgrade]
    fn upgrade(&self, initial_value: BigUint) {
        self.init(initial_value);
    }

    #[endpoint]
    fn test_endpoint(&self) -> EsdtTokenData<Self::Api> {
        self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            &TokenIdentifier::from_esdt_bytes(b"TESTTOKEN"),
            0u64,
        )
    }

    #[endpoint]
    fn send_back_token(&self) {
        let balance = self.blockchain().get_sc_balance(
            &EgldOrEsdtTokenIdentifier::esdt(TokenIdentifier::from_esdt_bytes(b"TESTTOKEN")),
            0u64,
        );

        self.tx()
            .to(ToCaller)
            .single_esdt(
                &TokenIdentifier::from_esdt_bytes(b"TESTTOKEN"),
                0u64,
                &balance,
            )
            .transfer();
    }

    /// Add desired amount to the storage variable.
    #[endpoint]
    fn add(&self, value: BigUint) {
        self.sum().update(|sum| *sum += value);
    }
}
