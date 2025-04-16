#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait EmptyContract {
    #[init]
    #[inline(never)]
    fn init(&self, arg: i32) -> i32 {
        (arg as f32 * 1.5f32) as i32
    }
}
