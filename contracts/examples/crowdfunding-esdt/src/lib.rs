#![no_std]
#![allow(unused_attributes)]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone, Copy)]
pub enum Status {
	FundingPeriod,
	Successful,
	Failed,
}

#[elrond_wasm_derive::contract]
pub trait Crowdfunding {
	#[init]
	fn init(&self, target: Self::BigUint, deadline: u64, token_name: TokenIdentifier) {
		self.target().set(&target);
		self.deadline().set(&deadline);
		self.cf_token_name().set(&token_name);
	}

	#[endpoint]
	#[payable("*")]
	fn fund(
		&self,
		#[payment] payment: Self::BigUint,
		#[payment_token] token: TokenIdentifier,
	) -> SCResult<()> {
		require!(
			self.status() == Status::FundingPeriod,
			"cannot fund after deadline"
		);
		require!(token == self.cf_token_name().get(), "wrong token");

		let caller = self.blockchain().get_caller();
		self.deposit(&caller).update(|deposit| *deposit += payment);

		Ok(())
	}

	#[view]
	fn status(&self) -> Status {
		if self.get_current_time() < self.deadline().get() {
			Status::FundingPeriod
		} else if self.get_current_funds() >= self.target().get() {
			Status::Successful
		} else {
			Status::Failed
		}
	}

	#[view(getCurrentFunds)]
	fn get_current_funds(&self) -> Self::BigUint {
		let token = self.cf_token_name().get();
		let sc_address = self.blockchain().get_sc_address();

		if token.is_egld() {
			self.blockchain().get_sc_balance()
		} else {
			self.blockchain()
				.get_esdt_balance(&sc_address, token.as_esdt_identifier(), 0)
		}
	}

	#[endpoint]
	fn claim(&self) -> SCResult<()> {
		match self.status() {
			Status::FundingPeriod => sc_error!("cannot claim before deadline"),
			Status::Successful => {
				let caller = self.blockchain().get_caller();
				require!(
					caller == self.blockchain().get_owner_address(),
					"only owner can claim successful funding"
				);

				let token_name = self.cf_token_name().get();
				let sc_balance = self.get_current_funds();

				self.send().direct(&caller, &token_name, &sc_balance, &[]);

				Ok(())
			},
			Status::Failed => {
				let caller = self.blockchain().get_caller();
				let deposit = self.deposit(&caller).get();

				if deposit > 0 {
					let token_name = self.cf_token_name().get();

					self.deposit(&caller).clear();
					self.send().direct(&caller, &token_name, &deposit, &[]);
				}

				Ok(())
			},
		}
	}

	// private

	fn get_current_time(&self) -> u64 {
		self.blockchain().get_block_timestamp()
	}

	// storage

	#[view(getTarget)]
	#[storage_mapper("target")]
	fn target(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

	#[view(getDeadline)]
	#[storage_mapper("deadline")]
	fn deadline(&self) -> SingleValueMapper<Self::Storage, u64>;

	#[view(getDeposit)]
	#[storage_mapper("deposit")]
	fn deposit(&self, donor: &Address) -> SingleValueMapper<Self::Storage, Self::BigUint>;

	#[view(getCrowdfundingTokenName)]
	#[storage_mapper("tokenName")]
	fn cf_token_name(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;
}
