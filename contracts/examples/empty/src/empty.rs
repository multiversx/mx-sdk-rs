#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

mod adder_proxy;
use adder_proxy::{AdderProxy, Test};

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait EmptyContract {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint]
    fn test_struct(&self) {
        let _ = self.tx()
            .to(&ManagedAddress::zero())
            .typed(adder_proxy::AdderProxy)
            .takes_struct(Test {
                field1: 0u64,
                field2: BigUint::zero(),
            });
    }
}
