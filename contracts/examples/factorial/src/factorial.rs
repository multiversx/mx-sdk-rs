#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait Factorial {
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn factorial(&self, value: BigUint) -> BigUint {
        let one = self.types().big_uint_from(1u32);
        if value == 0 {
            return one;
        }

        let mut result = self.types().big_uint_from(1u32);
        let mut x = self.types().big_uint_from(1u32);
        while x <= value {
            result *= &x;
            x += &one;
        }

        result
    }
}
