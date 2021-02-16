#![no_std]
#![allow(unused_attributes)]

elrond_wasm::imports!(); 

#[elrond_wasm_derive::contract(TransferEsdtHookImpl)]
pub trait TransferEsdtHook {
	#[init]
	fn init() {}

	#[endpoint(acceptEsdt)]
	#[payable("*")]
	fn accept_esdt(&self, #[payment] _payment: BigUint, #[payment_token] _token: TokenIdentifier) {}

	#[endpoint(transferEsdtOnce)]
	fn transfer_esdt_once(&self, to: Address, token: TokenIdentifier, amount: BigUint) {
		self.send()
			.direct_esdt(&to, token.as_slice(), &amount, b"transfer once");
	}

	#[endpoint(transferEsdtMultiple)]
	fn transfer_esdt_multiple(
		&self,
		to: Address,
		token: TokenIdentifier,
		amount: BigUint,
		nr_of_transfers: u32,
	) {
		for _ in 0..nr_of_transfers {
			self.send()
				.direct_esdt(&to, token.as_slice(), &amount, b"transfer multiple");
		}
	}
}
