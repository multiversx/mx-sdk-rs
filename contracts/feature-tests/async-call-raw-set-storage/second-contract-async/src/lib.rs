#![no_std]
#![allow(unused_attributes)]

elrond_wasm::imports!();

#[elrond_wasm_derive::contract(SecondContractAsyncImpl)]
pub trait SecondContractAsync {
	#[init]
	fn init(&self) {}

	#[endpoint(callMe)]
	fn call_me(&self, arg: u32) -> u32 {
		self.set_call_arg(arg);

		42
	}

	// storage

	#[storage_get("callArg")]
	fn get_call_arg(&self) -> u32;

	#[storage_set("callArg")]
	fn set_call_arg(&self, arg: u32);
}
