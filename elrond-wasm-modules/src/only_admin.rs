elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait OnlyAdminModule {
    #[view(is_admin)]
    fn is_admin(&self, address: ManagedAddress) -> bool {
        self.admins().contains(&address)
    }

    #[only_owner]
    #[endpoint(addAdmin)]
    fn add_admin(&self, address: ManagedAddress) {
        self.admins().add(&address);
        // TODO: event
    }

    #[only_owner]
    #[endpoint(removeAdmin)]
    fn remove_admin(&self, address: ManagedAddress) {
        self.admins().remove(&address);
        // TODO: event
    }

    #[storage_mapper("only_admin_module:admins")]
    fn admins(&self) -> WhitelistMapper<Self::Api, ManagedAddress>;

    fn check_caller_is_admin(&self) {
        require!(
            self.is_admin(self.blockchain().get_caller()),
            "Endpoint can only be called by admins"
        );
    }
}
