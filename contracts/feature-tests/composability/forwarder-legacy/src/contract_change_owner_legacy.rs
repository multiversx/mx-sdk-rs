multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ChangeOwnerModule {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::Api>;

    #[endpoint(changeOwnerAddress)]
    fn change_owner(
        &self,
        child_sc_address: ManagedAddress,
        new_owner: ManagedAddress,
    ) -> ManagedAddress {
        let () = self
            .send()
            .change_owner_address(child_sc_address.clone(), &new_owner)
            .execute_on_dest_context();

        self.get_owner_of_vault_contract(child_sc_address)
    }

    fn get_owner_of_vault_contract(&self, address: ManagedAddress) -> ManagedAddress {
        self.vault_proxy()
            .contract(address)
            .get_owner_address()
            .execute_on_dest_context()
    }
}
