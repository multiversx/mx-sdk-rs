#![no_std]
#![allow(non_snake_case)]

elrond_wasm::imports!();

use hex_literal::hex;

static HARDCODED_ADDRESS: [u8; 32] =
	hex!("fefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe");

#[elrond_wasm_derive::callable(PayMeProxy)]
pub trait PayMe {
	#[payable("EGLD")]
	fn payMe(&self, #[payment] _payment: BigUint, _arg1: i64) -> ContractCall<BigUint, ()>;

	#[payable("EGLD")]
	fn payMeWithResult(
		&self,
		#[payment] _payment: BigUint,
		_arg1: i64,
	) -> ContractCall<BigUint, ()>;
}

#[elrond_wasm_derive::callable(MessageMeProxy)]
pub trait MessageMe {
	fn messageMe(
		&self,
		arg1: i64,
		arg2: &BigUint,
		arg3: Vec<u8>,
		arg4: &Address,
	) -> ContractCall<BigUint, ()>;
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

	#[payable("EGLD")]
	#[endpoint]
	fn forwardToOtherContract(&self, #[payment] payment: BigUint) -> AsyncCall<BigUint> {
		let other_contract = self.get_other_contract();
		contract_call!(self, other_contract, PayMeProxy)
			.payMe(payment, 0x56)
			.async_call()
	}

	#[payable("EGLD")]
	#[endpoint]
	fn forwardToOtherContractWithCallback(
		&self,
		#[payment] payment: BigUint,
	) -> AsyncCall<BigUint> {
		let other_contract = self.get_other_contract();

		contract_call!(self, other_contract, PayMeProxy)
			.payMeWithResult(payment, 0x56)
			.async_call()
			.with_callback(self.callbacks().payCallback())
	}

	#[endpoint]
	fn messageOtherContract(&self) -> AsyncCall<BigUint> {
		let other_contract = self.get_other_contract();

		contract_call!(self, other_contract, MessageMeProxy)
			.messageMe(
				0x01,
				&BigUint::from(0x02u64),
				[3u8; 3].to_vec(),
				&HARDCODED_ADDRESS.into(),
			)
			.async_call()
	}

	#[endpoint]
	fn messageOtherContractWithCallback(&self) -> AsyncCall<BigUint> {
		let other_contract = self.get_other_contract();

		contract_call!(self, other_contract, MessageMeProxy)
			.messageMe(
				0x01,
				&BigUint::from(0x02u64),
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
