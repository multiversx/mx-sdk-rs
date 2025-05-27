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
        self.set_mapper_from_address(address).is_empty()
    }

    #[endpoint]
    fn contains_at_address(&self, item: u32) -> bool {
        let address = self.contract_address().get();
        self.set_mapper_from_address(address).contains(&item)
    }

    #[endpoint]
    fn len_at_address(&self) -> usize {
        let address = self.contract_address().get();
        self.set_mapper_from_address(address).len()
    }

    #[endpoint]
    fn next_at_address(&self, item: u32) -> u32 {
        let address = self.contract_address().get();
        self.set_mapper_from_address(address).next(&item).unwrap()
    }

    #[endpoint]
    fn previous_at_address(&self, item: u32) -> u32 {
        let address = self.contract_address().get();
        self.set_mapper_from_address(address)
            .previous(&item)
            .unwrap()
    }

    #[endpoint]
    fn front_at_address(&self) -> u32 {
        let address = self.contract_address().get();
        self.set_mapper_from_address(address).front().unwrap()
    }

    #[endpoint]
    fn back_at_address(&self) -> u32 {
        let address = self.contract_address().get();
        self.set_mapper_from_address(address).back().unwrap()
    }

    #[endpoint]
    fn keys_at_address(&self) -> ManagedVec<u32> {
        let address = self.contract_address().get();
        self.map_mapper_from_address(address).keys().collect()
    }

    #[endpoint]
    fn values_at_address(&self) -> ManagedVec<u32> {
        let address = self.contract_address().get();
        self.map_mapper_from_address(address).values().collect()
    }

    #[endpoint]
    fn contains_unordered_at_address(&self, item: u32) -> bool {
        let address = self.contract_address().get();
        self.unordered_set_mapper_from_address(address)
            .contains(&item)
    }

    #[endpoint]
    fn get_by_index(&self, index: usize) -> u32 {
        let address = self.contract_address().get();
        self.unordered_set_mapper_from_address(address)
            .get_by_index(index)
    }

    /// Storage to be called. For testing, this contract is deployed twice,
    /// and this module acts both as caller and receiver
    #[storage_mapper("set_mapper")]
    fn set_mapper(&self) -> SetMapper<u32>;

    #[storage_mapper_from_address("set_mapper")]
    fn set_mapper_from_address(&self, address: ManagedAddress) -> SetMapper<u32, ManagedAddress>;

    #[storage_mapper("map_mapper")]
    fn map_mapper(&self) -> MapMapper<u32, u32>;

    #[storage_mapper_from_address("map_mapper")]
    fn map_mapper_from_address(
        &self,
        address: ManagedAddress,
    ) -> MapMapper<u32, u32, ManagedAddress>;

    #[storage_mapper("unordered_set_mapper")]
    fn unordered_set_mapper(&self) -> UnorderedSetMapper<u32>;

    #[storage_mapper_from_address("unordered_set_mapper")]
    fn unordered_set_mapper_from_address(
        &self,
        address: ManagedAddress,
    ) -> UnorderedSetMapper<u32, ManagedAddress>;

    #[endpoint]
    fn fill_set_mapper(&self, value: u32) {
        for item in 1u32..=value {
            self.set_mapper().insert(item);
        }
    }

    #[endpoint]
    fn fill_map_mapper(&self, value: u32) {
        for item in 1u32..=value {
            let key = 10_000u32 + item;
            self.map_mapper().insert(key, item);
        }
    }

    #[endpoint]
    fn fill_unordered_set_mapper(&self, value: u32) {
        for item in 1u32..=value {
            self.unordered_set_mapper().insert(item);
        }
    }

    #[storage_mapper_from_address("single_value_mapper_with_key")]
    fn single_value_from_address_with_keys(
        &self,
        address: ManagedAddress,
        extra_key: usize,
    ) -> SingleValueMapper<ManagedBuffer, ManagedAddress>;

    #[view]
    fn get_value_from_address_with_keys(
        &self,
        address: ManagedAddress,
        extra_key: usize,
    ) -> ManagedBuffer {
        self.single_value_from_address_with_keys(address, extra_key)
            .get()
    }

    #[storage_mapper_from_address("address_ids")]
    fn address_ids_from_address(
        &self,
        address: ManagedAddress,
    ) -> AddressToIdMapper<ManagedAddress>;

    #[view]
    fn address_to_id_mapper_get_id_from_address(&self, address_arg: ManagedAddress) -> AddressId {
        let address = self.contract_address().get();
        self.address_ids_from_address(address).get_id(&address_arg)
    }
}
