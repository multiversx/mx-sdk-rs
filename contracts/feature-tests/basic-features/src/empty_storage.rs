multiversx_sc::imports!();
multiversx_sc::derive_imports!();

/// A contract with an empty storage. Will try to read from a different contract.
#[multiversx_sc::module]
pub trait EmptyStorage {
    #[view]
    #[storage_mapper("empty_set_mapper")]
    fn empty_set_mapper(&self) -> SetMapper<u32>;

    #[view]
    #[storage_mapper("other_contract_address")]
    fn other_contract_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[endpoint]
    fn set_other_contract_address(&self, address: ManagedAddress) {
        self.other_contract_address().set(address)
    }

    #[endpoint]
    fn contains_at_address_endpoint(&self, item: u32) -> bool {
        let set = self.empty_set_mapper();
        let other_contract_address = self.other_contract_address().get();
        set.contains_at_address(&other_contract_address, &item)
    }

    #[endpoint]
    fn next_at_address_endpoint(&self, item: u32) -> u32 {
        let set = self.empty_set_mapper();
        let other_contract_address = self.other_contract_address().get();
        set.next_at_address(&other_contract_address, &item).unwrap()
    }

    #[endpoint]
    fn previous_at_address_endpoint(&self, item: u32) -> u32 {
        let set = self.empty_set_mapper();
        let other_contract_address = self.other_contract_address().get();
        set.previous_at_address(&other_contract_address, &item)
            .unwrap_or_default()
    }

    #[endpoint]
    fn is_empty_at_address_endpoint(&self) -> bool {
        let set = self.empty_set_mapper();
        let other_contract_address = self.other_contract_address().get();
        set.is_empty_at_address(&other_contract_address)
    }

    #[endpoint]
    fn front_at_address_endpoint(&self) -> u32 {
        let set = self.empty_set_mapper();
        let other_contract_address = self.other_contract_address().get();
        set.front_at_address(&other_contract_address)
            .unwrap_or_default()
    }

    #[endpoint]
    fn back_at_address_endpoint(&self) -> u32 {
        let set = self.empty_set_mapper();
        let other_contract_address = self.other_contract_address().get();
        set.back_at_address(&other_contract_address)
            .unwrap_or_default()
    }

    #[endpoint]
    fn len_at_address_endpoint(&self) -> usize {
        let set = self.empty_set_mapper();
        let other_contract_address = self.other_contract_address().get();
        set.len_at_address(&other_contract_address)
    }

    #[endpoint]
    fn check_internal_consistency_at_address_endpoint(&self) -> bool {
        let set = self.empty_set_mapper();
        let other_contract_address = self.other_contract_address().get();
        set.check_internal_consistency_at_address(&other_contract_address)
    }
}
