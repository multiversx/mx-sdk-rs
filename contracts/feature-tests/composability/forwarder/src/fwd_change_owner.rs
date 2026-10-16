multiversx_sc::imports!();

use composability_abi::vault_abi;

#[multiversx_sc::module]
pub trait ChangeOwnerModule {
    #[endpoint(changeOwnerAddress)]
    fn change_owner(
        &self,
        child_sc_address: ManagedAddress,
        new_owner: ManagedAddress,
    ) -> ManagedAddress {
        self.send()
            .change_owner_address(child_sc_address.clone(), &new_owner)
            .sync_call();

        self.get_owner_of_vault_contract(child_sc_address)
    }

    fn get_owner_of_vault_contract(&self, address: ManagedAddress) -> ManagedAddress {
        self.tx()
            .to(&address)
            .abi_typed(vault_abi::VaultCall)
            .get_owner_address()
            .returns(ReturnsResultManaged)
            .sync_call()
    }
}
