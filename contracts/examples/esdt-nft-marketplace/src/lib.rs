#![no_std]

elrond_wasm::imports!();

#[elrond_wasm_derive::contract(EsdtNftMarketplaceImpl)]
pub trait EsdtNftMarketplace {
	#[init]
	fn init(&self) {}
}
