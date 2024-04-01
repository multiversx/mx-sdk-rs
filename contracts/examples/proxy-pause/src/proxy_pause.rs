#![no_std]

use multiversx_sc::imports::*;
pub mod pause_sc_proxy;

#[multiversx_sc::contract]
pub trait PauseProxy {
    #[init]
    fn init(&self) {
        self.owners().insert(self.blockchain().get_caller());
    }

    #[endpoint(addContracts)]
    fn add_contracts(&self, contracts: MultiValueEncoded<ManagedAddress>) {
        self.require_owner();
        self.contracts().extend(contracts);
    }

    #[endpoint(removeContracts)]
    fn remove_contracts(&self, contracts: MultiValueEncoded<ManagedAddress>) {
        self.require_owner();
        self.contracts().remove_all(contracts);
    }

    #[endpoint(addOwners)]
    fn add_owners(&self, owners: MultiValueEncoded<ManagedAddress>) {
        self.require_owner();
        self.owners().extend(owners);
    }

    #[endpoint(removeOwners)]
    fn remove_owners(&self, owners: MultiValueEncoded<ManagedAddress>) {
        self.require_owner();
        self.owners().remove_all(owners);
    }

    fn for_each_contract<F>(&self, f: F)
    where
        F: Fn(pause_sc_proxy::PausableProxyMethods<TxScEnv<Self::Api>, (), &ManagedAddress, ()>),
    {
        for contract_address in self.contracts().iter() {
            f(self
                .tx()
                .to(&contract_address)
                .typed(pause_sc_proxy::PausableProxy));
        }
    }

    #[endpoint]
    fn pause(&self) {
        self.require_owner();
        self.for_each_contract(|contract| contract.pause().sync_call());
    }

    #[endpoint]
    fn unpause(&self) {
        self.require_owner();
        self.for_each_contract(|contract| contract.unpause().sync_call());
    }

    fn require_owner(&self) {
        require!(
            self.owners().contains(&self.blockchain().get_caller()),
            "caller is not an owner"
        );
    }

    #[view]
    #[storage_mapper("owners")]
    fn owners(&self) -> SetMapper<ManagedAddress>;

    #[view]
    #[storage_mapper("contracts")]
    fn contracts(&self) -> SetMapper<ManagedAddress>;
}
