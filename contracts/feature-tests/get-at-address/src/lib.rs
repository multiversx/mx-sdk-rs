#![no_std]

multiversx_sc::imports!();
/// This contract's storage gets called by the StorageMapperGetAtAddress module of basic-features
#[multiversx_sc::contract]
pub trait GetAtAddress {
    #[storage_mapper("set_mapper")]
    fn set_mapper(&self) -> SetMapper<u32>;

    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {
        self.init();
    }

    #[endpoint]
    fn fill_set_mapper(&self, value: u32) {
        for item in 1u32..=value {
            self.set_mapper().insert(item);
        }
    }
}
