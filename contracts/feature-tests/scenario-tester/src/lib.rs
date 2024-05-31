#![no_std]

use multiversx_sc::imports::*;

pub mod scenario_tester_proxy;

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[multiversx_sc::contract]
pub trait ScenarioTester {
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

    /// Tests "from" conversion for MultiValueN parameters
    #[endpoint]
    fn multi_param(&self, _value: MultiValue2<BigUint, BigUint>) {}

    /// Tests "from" conversion for MultiValueN return function
    #[endpoint]
    fn multi_return(&self, value: BigUint) -> MultiValue2<BigUint, BigUint> {
        let value_plus_one = &value + 1u32;
        (value, value_plus_one).into()
    }
}
