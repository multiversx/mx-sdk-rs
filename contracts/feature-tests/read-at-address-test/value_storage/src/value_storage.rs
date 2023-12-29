#![no_std]

multiversx_sc::imports!();

/// A contract with storage. Will be read from the other contract
#[multiversx_sc::contract]
pub trait ValueStorage {
    #[init]
    fn init(&self) {}

    #[view]
    #[storage_mapper("value_set_mapper")]
    fn value_set_mapper(&self) -> SetMapper<u32>;

    #[endpoint]
    fn fill_set_mapper(&self, elements: MultiValueEncoded<u32>) {
        for elem in elements {
            self.value_set_mapper().insert(elem);
        }
    }
}
