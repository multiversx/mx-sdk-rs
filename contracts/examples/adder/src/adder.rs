// 0 true,
// 1 true,
// 2 true,
// 3 true,
// 4 false,
// 5 true,
// 6 true,
// 7 true,
// 8 true,
// 9 true,
//10 true,
//11 true,
//12 true,
//13 true,
//14 false,
//15 true,
//16 true,
//17 false,
//18 true,
//19 false,
//20 true,

#![no_std]

use multiversx_sc::imports::*;

pub mod adder_proxy;

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
    #[view]
    fn add(&self, value: BigUint) {
        self.sum().update(|sum| *sum += value);
    }
}
