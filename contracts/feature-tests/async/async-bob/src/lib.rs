#![no_std]
#![allow(non_snake_case)]

imports!();

#[elrond_wasm_derive::contract(BobImpl)]
pub trait Bob {
	#[storage_set("last_payment")]
	fn set_last_payment(&self, last_payment: &BigUint);

	#[storage_set("pay_me_arg")]
	fn set_pay_me_arg(&self, arg: i64);

	#[storage_set("message_me_1")]
	fn set_message_me_1(&self, m1: i64);

	#[storage_set("message_me_2")]
	fn set_message_me_2(&self, s2: &BigUint);

	#[storage_set("message_me_3")]
	fn set_message_me_3(&self, s3: &BoxedBytes);

	#[storage_set("message_me_4")]
	fn set_message_me_4(&self, s4: &Address);

	#[init]
	fn init(&self) {}

	#[payable]
	#[endpoint]
	fn payMe(&self, #[payment] payment: BigUint, arg1: i64) {
		self.set_last_payment(&payment);
		self.set_pay_me_arg(arg1);
	}

	#[payable]
	#[endpoint]
	fn payMeWithResult(&self, #[payment] payment: BigUint, arg1: i64) -> i64 {
		self.payMe(payment, arg1);
		0x7777
	}

	#[endpoint]
	fn messageMe(&self, arg1: i64, arg2: &BigUint, arg3: &BoxedBytes, arg4: &Address) {
		self.set_message_me_1(arg1);
		self.set_message_me_2(arg2);
		self.set_message_me_3(arg3);
		self.set_message_me_4(arg4);
	}
}
