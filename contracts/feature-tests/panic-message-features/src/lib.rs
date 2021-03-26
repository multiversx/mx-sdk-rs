#![no_std]
#![allow(clippy::string_lit_as_bytes)]
#![allow(clippy::redundant_clone)]

elrond_wasm::imports!();

/// Explores panic messaging.
/// Sending panic messages to the VM is possible, as shown in this contract,
/// but it greatly inflates the bytecode size.
#[elrond_wasm_derive::contract(PanicMessageFeaturesImpl)]
pub trait PanicMessageFeatures {
	#[endpoint(panicWithMessage)]
	fn panic_with_message(&self) {
		panic!("example panic message");
	}
}
