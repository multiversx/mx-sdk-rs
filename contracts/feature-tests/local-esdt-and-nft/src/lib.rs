#![no_std]

elrond_wasm::imports!();

#[elrond_wasm_derive::contract(LocalEsdtAndEsdtNftImpl)]
pub trait LocalEsdtAndEsdtNft {
	#[init]
	fn init(&self) {}
}
