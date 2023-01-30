#![no_std]

multiversx_sc::imports!();

/// A basic contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait BasicContract {
    #[init]
    fn init(&self) {}
}
