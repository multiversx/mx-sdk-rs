#![no_std]
#![allow(non_snake_case)]

elrond_wasm::imports!();

use hex_literal::hex;

static HARDCODED_ADDRESS: [u8; 32] =
	hex!("fefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe");

mod pay_me_proxy {
	elrond_wasm::imports!();

	#[elrond_wasm_derive::proxy]
	pub trait PayMe {
		#[payable("EGLD")]
		#[endpoint]
		fn payMe(&self, #[payment] payment: Self::BigUint, arg1: i64);

		#[payable("EGLD")]
		#[endpoint]
		fn payMeWithResult(&self, #[payment] payment: Self::BigUint, arg1: i64);
	}
}

mod message_me_proxy {
	elrond_wasm::imports!();

	#[elrond_wasm_derive::proxy]
	pub trait MessageMe {
		#[endpoint]
		fn messageMe(&self, arg1: i64, arg2: &Self::BigUint, arg3: Vec<u8>, arg4: &Address);
	}
}

use message_me_proxy::Proxy as _; // currently needed for contract calls, TODO: better syntax
use pay_me_proxy::Proxy as _; // currently needed for contract calls, TODO: better syntax

#[elrond_wasm_derive::contract]
pub trait Alice {
	#[storage_get("other_contract")]
	fn get_other_contract(&self) -> Address;

	#[storage_set("other_contract")]
	fn set_other_contract(&self, other_contract: &Address);

	#[storage_set("callback_info")]
	fn set_callback_info(&self, callback_info: i64);

	#[init]
	fn init(&self, calee_address: &Address) {
		self.set_other_contract(calee_address);
	}

	#[payable("EGLD")]
	#[endpoint]
	fn forwardToOtherContract(
		&self,
		#[payment] payment: Self::BigUint,
	) -> AsyncCall<Self::SendApi> {
		let other_contract = self.get_other_contract();
		pay_me_proxy::ProxyObj::new_proxy_obj(self.send(), other_contract)
			.payMe(payment, 0x56)
			.async_call()
	}

	#[payable("EGLD")]
	#[endpoint]
	fn forwardToOtherContractWithCallback(
		&self,
		#[payment] payment: Self::BigUint,
	) -> AsyncCall<Self::SendApi> {
		let other_contract = self.get_other_contract();
		pay_me_proxy::ProxyObj::new_proxy_obj(self.send(), other_contract)
			.payMeWithResult(payment, 0x56)
			.async_call()
			.with_callback(self.callbacks().payCallback())
	}

	#[endpoint]
	fn messageOtherContract(&self) -> AsyncCall<Self::SendApi> {
		let other_contract = self.get_other_contract();
		message_me_proxy::ProxyObj::new_proxy_obj(self.send(), other_contract)
			.messageMe(
				0x01,
				&Self::BigUint::from(0x02u64),
				[3u8; 3].to_vec(),
				&HARDCODED_ADDRESS.into(),
			)
			.async_call()
	}

	#[endpoint]
	fn messageOtherContractWithCallback(&self) -> AsyncCall<Self::SendApi> {
		let other_contract = self.get_other_contract();
		message_me_proxy::ProxyObj::new_proxy_obj(self.send(), other_contract)
			.messageMe(
				0x01,
				&Self::BigUint::from(0x02u64),
				[3u8; 3].to_vec(),
				&HARDCODED_ADDRESS.into(),
			)
			.async_call()
			.with_callback(self.callbacks().message_callback())
	}

	#[callback]
	fn payCallback(&self, #[call_result] call_result: AsyncCallResult<i64>) {
		match call_result {
			AsyncCallResult::Ok(cb_arg) => {
				self.set_callback_info(cb_arg);
			},
			AsyncCallResult::Err(_) => {},
		}
	}

	#[callback]
	fn message_callback(&self, #[call_result] _call_result: AsyncCallResult<()>) {
		self.set_callback_info(0x5555);
	}
}
