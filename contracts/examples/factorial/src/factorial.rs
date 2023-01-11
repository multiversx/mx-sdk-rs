#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait Factorial {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn factorial(&self, value: BigUint) -> BigUint {
        let one = BigUint::from(1u32);
        if value == 0 {
            return one;
        }

        let mut result = BigUint::from(1u32);
        let mut x = BigUint::from(1u32);
        while x <= value {
            result *= &x;
            x += &one;
        }

        result
    }
}
