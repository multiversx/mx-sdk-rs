#![no_std]

elrond_wasm::imports!();

pub mod big_float_methods;
pub mod big_float_operators;
pub mod big_float_to_big_int_result;

#[elrond_wasm::contract]
pub trait BigFloatFeatures:
    big_float_methods::BigFloatMethods
    + big_float_operators::BigFloatOperators
    + big_float_to_big_int_result::BigFloatWrappedEndpoints
{
    #[init]
    fn init(&self) {}
}
