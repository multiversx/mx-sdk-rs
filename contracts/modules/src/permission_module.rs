multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait PermissionsModule {
    #[only_owner]
    #[endpoint(addAdmin)]
    fn add_admin(&self, address: &ManagedAddress) {
        self.admins().add(address);
    }

    #[only_owner]
    #[endpoint(removeAdmin)]
    fn remove_admin(&self, address: &ManagedAddress) {
        self.admins().remove(address);
    }

    fn require_admin(&self, address: &ManagedAddress) {
        require!(self.admins().contains(address), "Caller not an admin");
    }

    #[storage_mapper("admins")]
    fn admins(&self) -> WhitelistMapper<ManagedAddress>;
}
