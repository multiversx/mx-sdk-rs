#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait Factorial {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn factorial(&self, mut value: BigUint) -> BigUint {
        let one = BigUint::from(1u32);
        let mut result = BigUint::from(1u32);
        while value > 0 {
            result *= &value;
            value -= &one;
        }

        result
    }
}
