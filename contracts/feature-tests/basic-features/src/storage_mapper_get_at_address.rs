use multiversx_sc::storage::StorageKey;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

/// Module that calls another contract to read the content of a SetMapper remotely
#[multiversx_sc::module]
pub trait StorageMapperGetAtAddress {
    #[storage_mapper("contract_address")]
    fn contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[endpoint]
    fn set_contract_address(&self, address: ManagedAddress) {
        self.contract_address().set(address)
    }

    #[endpoint]
    fn is_empty_at_address(&self) -> bool {
        let address = self.contract_address().get();
        let mapper: SetMapper<u32, _> =
            SetMapper::new_from_address(address, StorageKey::from("set_mapper"));
        mapper.is_empty()
    }

    #[endpoint]
    fn contains_at_address(&self, item: u32) -> bool {
        let address = self.contract_address().get();
        let mapper: SetMapper<u32, _> =
            SetMapper::new_from_address(address, StorageKey::from("set_mapper"));
        mapper.contains(&item)
    }

    #[endpoint]
    fn len_at_address(&self) -> usize {
        let address = self.contract_address().get();
        let mapper: SetMapper<u32, _> =
            SetMapper::new_from_address(address, StorageKey::from("set_mapper"));
        mapper.len()
    }
}
