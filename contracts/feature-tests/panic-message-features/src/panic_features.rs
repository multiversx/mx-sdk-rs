#![no_std]

multiversx_sc::imports!();

/// Explores panic messaging.
/// Sending panic messages to the VM is possible, as shown in this contract,
/// but it greatly inflates the bytecode size.
#[multiversx_sc::contract]
pub trait PanicMessageFeatures {
    #[init]
    fn init(&self) {}

    #[endpoint(panicWithMessage)]
    fn panic_with_message(&self) {
        panic!("example panic message");
    }
}
