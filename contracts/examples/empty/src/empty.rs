#![no_std]

elrond_wasm::imports!();

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[elrond_wasm::derive::contract]
pub trait EmptyContract {
    #[init]
    fn init(&self) {}
}
