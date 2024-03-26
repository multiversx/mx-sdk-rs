use crate::vault_proxy;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ChangeOwnerModule {
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
        self.tx()
            .to(&address)
            .typed(vault_proxy::VaultProxy)
            .get_owner_address()
            .returns(ReturnsExact)
            .sync_call()
    }
}
