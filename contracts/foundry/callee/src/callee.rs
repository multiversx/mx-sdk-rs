#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait CalleeContract {
    #[init]
    fn init(&self) {}

    #[endpoint]
    #[payable("EGLD")]
    fn fail_if_neg(&self, value: i64) -> ManagedBuffer {
        require!(value >= 0, "negative");

        if value == 0 {
            ManagedBuffer::from("zero")
        } else {
            ManagedBuffer::from("positive")
        }
    }
}
