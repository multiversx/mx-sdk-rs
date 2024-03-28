#![no_std]

use multiversx_sc::{
    derive::type_abi,
    imports::*,
    proxy_imports::*,
};
pub mod adder_proxy;

#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, Default)]
pub struct Test<M: ManagedTypeApi> {
    pub field1: u64,
    pub field2: BigUint<M>,
}
/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[multiversx_sc::contract]
pub trait Adder {
    #[view(getSum)]
    #[storage_mapper("sum")]
    fn sum(&self) -> SingleValueMapper<BigUint>;

    #[init]
    fn init(&self, initial_value: BigUint) {
        self.sum().set(initial_value);
    }

    #[upgrade]
    fn upgrade(&self, initial_value: BigUint) {
        self.init(initial_value);
    }

    /// Add desired amount to the storage variable.
    #[endpoint]
    fn add(&self, value: BigUint) {
        self.sum().update(|sum| *sum += value);
    }

    #[endpoint]
    fn takes_struct(&self, _elem: Test<Self::Api>) {}
}
