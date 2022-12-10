use crate::storage;
elrond_wasm::imports!();

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[elrond_wasm::module]
pub trait EndpointsModule:
    elrond_wasm::contract_base::ContractBase + storage::StorageModule {

    /// Add desired amount to the storage variable.
    #[endpoint]
    fn add(&self, value: BigUint) {
        self.sum().update(|sum| *sum += value);
    }

}
