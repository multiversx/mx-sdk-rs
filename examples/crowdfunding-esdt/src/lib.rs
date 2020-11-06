#![no_std]
#![allow(unused_attributes)]

imports!();

const ESDT_TRANSFER_STRING: &str = "ESDTTransfer";

#[derive(PartialEq, Clone, Copy)]
pub enum Status {
	FundingPeriod,
	Successful,
	Failed,
}

#[elrond_wasm_derive::contract(CrowdfundingImpl)]
pub trait Crowdfunding {
	#[storage_set("owner")]
	fn set_owner(&self, address: &Address);

	#[view]
	#[storage_get("owner")]
	fn get_owner(&self) -> Address;

	#[storage_set("target")]
	fn set_target(&self, target: &BigUint);

	#[view]
	#[storage_get("target")]
	fn get_target(&self) -> BigUint;

	#[storage_set("esdtBalance")]
	fn set_esdt_balance(&self, esdt_balance: &BigUint);

	#[view]
	#[storage_get("esdtBalance")]
	fn get_esdt_balance(&self) -> BigUint;

	#[storage_set("deadline")]
	fn set_deadline(&self, deadline: u64);

	#[view]
	#[storage_get("deadline")]
	fn get_deadline(&self) -> u64;

	#[storage_set("deposit")]
	fn set_deposit(&self, donor: &Address, amount: &BigUint);

	#[view]
	#[storage_get("deposit")]
	fn get_deposit(&self, donor: &Address) -> BigUint;

	#[storage_set("esdtTokenName")]
	fn set_cf_esdt_token_name(&self, esdt_token_name: &[u8]);

	#[view]
	#[storage_get("esdtTokenName")]
	fn get_cf_esdt_token_name(&self) -> Vec<u8>;

	#[init]
	fn init(&self, target: &BigUint, deadline: u64, esdt_token_name: &Vec<u8>) {
		let my_address: Address = self.get_caller();
		self.set_owner(&my_address);
		self.set_target(target);
		self.set_deadline(deadline);
		self.set_cf_esdt_token_name(esdt_token_name);
	}

	#[endpoint]
	fn fund(&self) -> SCResult<()> {
		if self.get_block_nonce() > self.get_deadline() {
			return sc_error!("cannot fund after deadline");
		}

		let expected_token_name = self.get_cf_esdt_token_name();
		let actual_token_name = self.get_esdt_token_name().unwrap_or_default();

		if expected_token_name != actual_token_name {
			return sc_error!("wrong esdt token");
		}

		let payment = self.get_esdt_value_big_uint();
		let caller = self.get_caller();
		let mut deposit = self.get_deposit(&caller);
		let mut balance = self.get_esdt_balance();

		deposit += payment.clone();
		balance += payment;

		self.set_deposit(&caller, &deposit);
		self.set_esdt_balance(&balance);

		return Ok(());
	}

	#[view]
	fn status(&self) -> Status {
		if self.get_block_nonce() <= self.get_deadline() {
			return Status::FundingPeriod;
		} else if self.get_esdt_balance() >= self.get_target() {
			return Status::Successful;
		} else {
			return Status::Failed;
		}
	}

	#[view(currentFunds)]
	fn current_funds(&self) -> SCResult<BigUint> {
		Ok(self.get_esdt_balance())
	}

	#[endpoint]
	fn claim(&self) -> SCResult<()> {
		match self.status() {
			Status::FundingPeriod => sc_error!("cannot claim before deadline"),
			Status::Successful => {
				let caller = self.get_caller();
				if &caller != &self.get_owner() {
					return sc_error!("only owner can claim successful funding");
				}

				let esdt_token_name = self.get_cf_esdt_token_name();
				let esdt_balance = self.get_esdt_balance();

				self.set_esdt_balance(&BigUint::zero());
				self.pay_esdt(&esdt_token_name, &esdt_balance, &caller);

				Ok(())
			},
			Status::Failed => {
				let caller = self.get_caller();
				let deposit = self.get_deposit(&caller);

				if &deposit > &0 {
					let esdt_token_name = self.get_cf_esdt_token_name();
					let mut esdt_balance = self.get_esdt_balance();

					esdt_balance -= deposit.clone();

					self.set_esdt_balance(&esdt_balance);
					self.set_deposit(&caller, &BigUint::zero());
					self.pay_esdt(&esdt_token_name, &deposit, &caller);
				}
				Ok(())
			},
		}
	}

	fn pay_esdt(&self, esdt_token_name: &Vec<u8>, amount: &BigUint, to: &Address) {
		let mut data = Vec::<u8>::new();

		data.append(ESDT_TRANSFER_STRING.as_bytes().to_vec().as_mut());
		data.push('@' as u8);
		data.append(esdt_token_name.clone().as_mut());
		data.push('@' as u8);
		data.append(amount.to_bytes_be().as_mut());

		self.async_call(&to, &BigUint::zero(), &data);
	}
}

use elrond_wasm::elrond_codec::*;

impl Status {
	pub fn to_u8(&self) -> u8 {
		match self {
			Status::FundingPeriod => 0,
			Status::Successful => 1,
			Status::Failed => 2,
		}
	}

	fn from_u8(v: u8) -> Result<Self, DecodeError> {
		match v {
			0 => core::result::Result::Ok(Status::FundingPeriod),
			1 => core::result::Result::Ok(Status::Successful),
			2 => core::result::Result::Ok(Status::Failed),
			_ => core::result::Result::Err(DecodeError::INVALID_VALUE),
		}
	}
}

impl TopEncode for Status {
	fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
		self.to_u8().top_encode(output)
	}

	fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
		&self,
		output: O,
		c: ExitCtx,
		exit: fn(ExitCtx, EncodeError) -> !,
	) {
		self.to_u8().top_encode_or_exit(output, c, exit)
	}
}

impl TopDecode for Status {
	fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
		Status::from_u8(u8::top_decode(input)?)
	}

	fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
		input: I,
		c: ExitCtx,
		exit: fn(ExitCtx, DecodeError) -> !,
	) -> Self {
		match u8::top_decode_or_exit(input, c.clone(), exit) {
			0 => Status::FundingPeriod,
			1 => Status::Successful,
			2 => Status::Failed,
			_ => exit(c, DecodeError::INVALID_VALUE),
		}
	}
}
