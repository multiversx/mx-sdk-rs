
#![no_std]
#![allow(non_snake_case)]

imports!();

#[elrond_wasm_derive::contract(BobImpl)]
pub trait Bob {

    #[init]
    fn init(&self) {
    }

    #[endpoint]
    fn messageMe(&self, value: BigUint) {
        self.set_value(value);
    }
    
    #[storage_set("value")]
    fn set_value(&self, value: BigUint);

    #[view]
    #[storage_get("value")]
    fn get_value(&self) -> BigUint;
}
