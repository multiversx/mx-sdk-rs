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

    #[view(getOtherMapper)]
    #[storage_mapper("otherMapper")]
    fn other_mapper(&self) -> SingleValueMapper<ManagedBuffer>;

    /// Return value for testing reasons.
    #[init]
    fn init(&self, initial_value: BigUint) -> &'static str {
        self.sum().set(initial_value);
        "init-result"
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

    /// Sets a value at another key
    #[endpoint]
    fn set_other_mapper(&self, value: ManagedBuffer) {
        self.other_mapper().set(value);
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

    #[view]
    fn sc_panic(&self) {
        sc_panic!("sc_panic! example");
    }

    // Trigger warning message in terminal: "Forbidden opcodes detected in endpoint"
    // Report available in *.mxsc.json
    #[endpoint]
    #[inline(never)]
    #[label("forbidden-opcodes")]
    fn mul_floats(&self, arg: i32) -> i32 {
        (arg as f32 * 1.5f32) as i32
    }
}
