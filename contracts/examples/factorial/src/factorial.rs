#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait Factorial {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn factorial(&self, value: BaseBigUint) -> BaseBigUint {
        let one = BaseBigUint::from(1u32);
        if value == 0 {
            return one;
        }

        let mut result = BaseBigUint::from(1u32);
        let mut x = BaseBigUint::from(1u32);
        while x <= value {
            result *= &x;
            x += &one;
        }

        result
    }
}
