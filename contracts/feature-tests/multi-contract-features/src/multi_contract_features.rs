#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait MultiContractFeatures {
    #[init]
    fn init(&self, sample_value: BigUint) {
        self.sample_value().set(sample_value);
    }

    #[external_view]
    fn external_pure(&self) -> i32 {
        1
    }

    #[view]
    #[storage_mapper("sample-value")]
    fn sample_value(&self) -> SingleValueMapper<BigUint>;

    #[external_view]
    fn sample_value_external_get(&self) -> BigUint {
        self.sample_value().get()
    }

    /// This is not really a view/
    /// Designed to check what happens if we try to write to storage from an external view.
    #[external_view]
    fn sample_value_external_set(&self, sample_value: BigUint) {
        self.sample_value().set(sample_value);
    }
}
