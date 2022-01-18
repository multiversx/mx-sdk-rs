#![no_std]

elrond_wasm::imports!();

mod pause_proxy {
    elrond_wasm::imports!();

    #[elrond_wasm::proxy]
    pub trait Pausable {
        #[endpoint]
        fn pause(&self);

        #[endpoint]
        fn unpause(&self);
    }
}

#[elrond_wasm::derive::contract]
pub trait PauseProxy {
    #[init]
    fn init(&self) {
        self.owners().insert(self.blockchain().get_caller());
    }

    #[endpoint(addContracts)]
    fn add_contracts(&self, #[var_args] contracts: ManagedVarArgs<ManagedAddress>) -> SCResult<()> {
        self.require_owner()?;
        self.contracts().extend(contracts);
        Ok(())
    }

    #[endpoint(removeContracts)]
    fn remove_contracts(
        &self,
        #[var_args] contracts: ManagedVarArgs<ManagedAddress>,
    ) -> SCResult<()> {
        self.require_owner()?;
        self.contracts().remove_all(contracts);
        Ok(())
    }

    #[endpoint(addOwners)]
    fn add_owners(&self, #[var_args] owners: ManagedVarArgs<ManagedAddress>) -> SCResult<()> {
        self.require_owner()?;
        self.owners().extend(owners);
        Ok(())
    }

    #[endpoint(removeOwners)]
    fn remove_owners(&self, #[var_args] owners: ManagedVarArgs<ManagedAddress>) -> SCResult<()> {
        self.require_owner()?;
        self.owners().remove_all(owners);
        Ok(())
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
    fn pause(&self) -> SCResult<()> {
        self.require_owner()?;
        self.for_each_contract(|contract| contract.pause().execute_on_dest_context());
        Ok(())
    }

    #[endpoint]
    fn unpause(&self) -> SCResult<()> {
        self.require_owner()?;
        self.for_each_contract(|contract| contract.unpause().execute_on_dest_context());
        Ok(())
    }

    fn require_owner(&self) -> SCResult<()> {
        require!(
            self.owners().contains(&self.blockchain().get_caller()),
            "caller is not an owner"
        );
        Ok(())
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
