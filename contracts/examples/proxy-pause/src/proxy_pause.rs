#![no_std]

multiversx_sc::imports!();

mod pause_proxy {
    multiversx_sc::imports!();

    #[multiversx_sc::proxy]
    pub trait Pausable {
        #[endpoint]
        fn pause(&self);

        #[endpoint]
        fn unpause(&self);
    }
}

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
        F: Fn(pause_proxy::Proxy<Self::Api>),
    {
        for contract_address in self.contracts().iter() {
            f(self.pausable_contract().contract(contract_address));
        }
    }

    #[endpoint]
    fn pause(&self) {
        self.require_owner();
        self.for_each_contract(|mut contract| contract.pause().execute_on_dest_context());
    }

    #[endpoint]
    fn unpause(&self) {
        self.require_owner();
        self.for_each_contract(|mut contract| contract.unpause().execute_on_dest_context());
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

    #[proxy]
    fn pausable_contract(&self) -> pause_proxy::Proxy<Self::Api>;
}
