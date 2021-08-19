#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait Factorial {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn factorial(&self, value: BigUint) -> BigUint {
        if value == 0 {
            return BigUint::from(1u32);
        }

        let mut result = BigUint::from(1u32);
        let one = BigUint::from(1u32);
        let mut x = BigUint::from(1u32);
        while x <= value {
            result *= &x;
            x += &one;
        }

        result
    }
}
