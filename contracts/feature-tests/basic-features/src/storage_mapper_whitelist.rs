elrond_wasm::imports!();

#[elrond_wasm::module]
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
    fn require_contains(&self, item: ManagedBuffer) {
        self.whitelist_mapper().require_whitelisted(&item);
    }

    #[storage_mapper("whitelistMapper")]
    fn whitelist_mapper(&self) -> WhitelistMapper<Self::Api, ManagedBuffer>;
}
