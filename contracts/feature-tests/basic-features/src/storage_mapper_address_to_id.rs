multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait AddressToIdMapperFeatures {
    #[endpoint]
    fn address_to_id_mapper_get_id(&self, address: ManagedAddress) -> AddressId {
        self.address_ids().get_id(&address)
    }

    #[endpoint]
    fn address_to_id_mapper_get_id_non_zero(&self, address: ManagedAddress) -> AddressId {
        self.address_ids().get_id_non_zero(&address)
    }

    #[endpoint]
    fn address_to_id_mapper_get_address(&self, address_id: AddressId) -> Option<ManagedAddress> {
        self.address_ids().get_address(address_id)
    }

    #[endpoint]
    fn address_to_id_mapper_contains(&self, address_id: AddressId) -> bool {
        self.address_ids().contains_id(address_id)
    }

    #[endpoint]
    fn address_to_id_mapper_set(&self, address: ManagedAddress) -> AddressId {
        self.address_ids().insert_new(&address)
    }

    #[endpoint]
    fn address_to_id_mapper_get_id_or_insert(&self, address: ManagedAddress) -> AddressId {
        self.address_ids().get_id_or_insert(&address)
    }

    #[endpoint]
    fn address_to_id_mapper_remove_by_id(&self, address_id: AddressId) -> Option<ManagedAddress> {
        self.address_ids().remove_by_id(address_id)
    }

    #[endpoint]
    fn address_to_id_mapper_remove_by_address(&self, address: ManagedAddress) -> AddressId {
        self.address_ids().remove_by_address(&address)
    }

    #[storage_mapper("address_ids")]
    fn address_ids(&self) -> AddressToIdMapper<Self::Api>;
}
