#![no_std]

elrond_wasm::imports!();

#[elrond_wasm_derive::contract]
pub trait NftStoragePrepay {
	#[init]
	fn init(&self) {}
}
