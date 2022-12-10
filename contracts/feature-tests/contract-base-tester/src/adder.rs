#![no_std]

mod adder_endpoints;
mod storage;
elrond_wasm::imports!();

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[elrond_wasm::contract]
pub trait Adder:
    elrond_wasm::contract_base::ContractBase + storage::StorageModule + adder_endpoints::EndpointsModule
{
    #[init]
    fn init(&self, initial_value: BigUint) {
        self.sum().set(initial_value);
    }
}
