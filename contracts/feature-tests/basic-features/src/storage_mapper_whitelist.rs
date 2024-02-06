use multiversx_sc::storage::StorageKey;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait StorageMapperWhitelistFeatures {
    #[endpoint]
    fn add_to_whitelist(&self, item: ManagedBuffer) {
        self.whitelist_mapper().add(&item);
    }

    #[endpoint]
    fn remove_from_whitelist(&self, item: ManagedBuffer) {
        self.whitelist_mapper().remove(&item);
    }

    #[endpoint]
    fn check_contains(&self, item: ManagedBuffer) -> bool {
        self.whitelist_mapper().contains(&item)
    }

    #[endpoint]
    fn check_contains_at_address(&self, address: ManagedAddress, item: ManagedBuffer) -> bool {
        let mapper =
            WhitelistMapper::new_from_address(address, StorageKey::from("whitelistMapper"));
        mapper.contains(&item)
    }

    #[endpoint]
    fn require_contains(&self, item: ManagedBuffer) {
        self.whitelist_mapper().require_whitelisted(&item);
    }

    #[endpoint]
    fn require_contains_at_address(&self, address: ManagedAddress, item: ManagedBuffer) {
        let mapper =
            WhitelistMapper::new_from_address(address, StorageKey::from("whitelistMapper"));
        mapper.require_whitelisted(&item)
    }

    #[storage_mapper("whitelistMapper")]
    fn whitelist_mapper(&self) -> WhitelistMapper<ManagedBuffer>;
}
