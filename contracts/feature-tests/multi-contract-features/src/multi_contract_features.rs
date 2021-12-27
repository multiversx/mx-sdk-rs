#![no_std]
#![allow(clippy::type_complexity)]

elrond_wasm::imports!();

/// Contract that only tests the call value features,
/// i.e. the framework/Arwen functionality for accepting EGLD and ESDT payments.
#[elrond_wasm::contract]
pub trait MultiContractFeatures {
    #[init]
    fn init(&self) {}

    #[external_view]
    fn external_pure(&self) -> i32 {
        1
    }
}
