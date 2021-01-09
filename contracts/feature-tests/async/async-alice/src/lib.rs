#![no_std]
#![allow(non_snake_case)]

imports!();

use hex_literal::hex;

static HARDCODED_ADDRESS: [u8; 32] =
	hex!("fefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe");

#[elrond_wasm_derive::callable(PayMeProxy)]
pub trait PayMe {
	#[payable]
	fn payMe(&self, #[payment] _payment: BigUint, _arg1: i64);

	#[payable]
	#[callback(payCallback)]
	fn payMeWithResult(&self, #[payment] _payment: BigUint, _arg1: i64);
}

#[elrond_wasm_derive::callable(MessageMeProxy)]
pub trait MessageMe {
	fn messageMe(&self, arg1: i64, arg2: &BigUint, arg3: Vec<u8>, arg4: &Address);
}

#[elrond_wasm_derive::callable(MessageMeProxy)]
pub trait MessageMeWithCallback {
	#[callback(messageCallback)]
	fn messageMe(&self, arg1: i64, arg2: BigUint, arg3: Vec<u8>, arg4: Address);
}

#[elrond_wasm_derive::contract(AliceImpl)]
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

	#[payable]
	#[endpoint]
	fn forwardToOtherContract(&self, #[payment] payment: BigUint) {
		let other_contract = self.get_other_contract();

		let target_contract = contract_proxy!(self, &other_contract, PayMe);
		target_contract.payMe(payment, 0x56);
	}

	#[payable]
	#[endpoint]
	fn forwardToOtherContractWithCallback(&self, #[payment] payment: BigUint) {
		let other_contract = self.get_other_contract();

		let target_contract = contract_proxy!(self, &other_contract, PayMe);
		target_contract.payMeWithResult(payment, 0x56);
	}

	#[endpoint]
	fn messageOtherContract(&self) {
		let other_contract = self.get_other_contract();

		let target_contract = contract_proxy!(self, &other_contract, MessageMe);
		target_contract.messageMe(
			0x01,
			&BigUint::from(0x02u64),
			create_a_vec(),
			&HARDCODED_ADDRESS.into(),
		);
	}

	#[endpoint]
	fn messageOtherContractWithCallback(&self) {
		let other_contract = self.get_other_contract();

		let target_contract = contract_proxy!(self, &other_contract, MessageMeWithCallback);
		target_contract.messageMe(
			0x01,
			BigUint::from(0x02u64),
			create_a_vec(),
			HARDCODED_ADDRESS.into(),
		);
	}

	#[callback]
	fn payCallback(&self, call_result: AsyncCallResult<i64>) {
		match call_result {
			AsyncCallResult::Ok(cb_arg) => {
				self.set_callback_info(cb_arg);
			},
			AsyncCallResult::Err(_) => {},
		}
	}

	#[callback]
	fn messageCallback(&self, _call_result: AsyncCallResult<()>) {
		self.set_callback_info(0x5555);
	}
}

fn create_a_vec() -> Vec<u8> {
	let mut res = Vec::with_capacity(3);
	res.push(3);
	res.push(3);
	res.push(3);
	res
}
