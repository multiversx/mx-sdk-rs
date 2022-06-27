#![no_std]

elrond_wasm::imports!();

pub mod big_float_methods;
pub mod big_float_operators;
pub mod big_float_wrapped;

#[elrond_wasm::contract]
pub trait BigFloatFeatures:
    big_float_methods::BigFloatMethods
    + big_float_operators::BigFloatOperators
    + big_float_wrapped::BigFloatWrappedEndpoints
{
    #[init]
    fn init(&self) {}
}
