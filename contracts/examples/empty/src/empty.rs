#![no_std]

mod file;

use bitflags::bitflags;
use multiversx_sc::derive::type_abi;

bitflags! {
    #[type_abi]
    #[derive(Clone)]
    pub struct Permission: u32 {
        const NONE = 0;
        const OWNER = 1;
        const ADMIN = 2;
        const PAUSE = 4;
    }
}

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait EmptyContract {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
