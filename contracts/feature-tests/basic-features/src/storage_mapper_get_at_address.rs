use multiversx_sc::storage::StorageKey;

use multiversx_sc::imports::*;

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

    #[endpoint]
    fn next_at_address(&self, item: u32) -> u32 {
        let address = self.contract_address().get();
        let mapper: SetMapper<u32, _> =
            SetMapper::new_from_address(address, StorageKey::from("set_mapper"));
        mapper.next(&item).unwrap()
    }

    #[endpoint]
    fn previous_at_address(&self, item: u32) -> u32 {
        let address = self.contract_address().get();
        let mapper: SetMapper<u32, _> =
            SetMapper::new_from_address(address, StorageKey::from("set_mapper"));
        mapper.previous(&item).unwrap()
    }

    #[endpoint]
    fn front_at_address(&self) -> u32 {
        let address = self.contract_address().get();
        let mapper: SetMapper<u32, _> =
            SetMapper::new_from_address(address, StorageKey::from("set_mapper"));
        mapper.front().unwrap()
    }

    #[endpoint]
    fn back_at_address(&self) -> u32 {
        let address = self.contract_address().get();
        let mapper: SetMapper<u32, _> =
            SetMapper::new_from_address(address, StorageKey::from("set_mapper"));
        mapper.back().unwrap()
    }

    /// Storage to be called. For testing, this contract is deployed twice,
    /// and this module acts both as caller and receiver
    #[storage_mapper("set_mapper")]
    fn set_mapper(&self) -> SetMapper<u32>;

    #[endpoint]
    fn fill_set_mapper(&self, value: u32) {
        for item in 1u32..=value {
            self.set_mapper().insert(item);
        }
    }
}
