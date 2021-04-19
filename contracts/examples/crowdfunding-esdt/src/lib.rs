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

#[elrond_wasm_derive::contract(CrowdfundingImpl)]
pub trait Crowdfunding {
	#[init]
	fn init(&self, target: BigUint, deadline: u64, esdt_token_name: TokenIdentifier) {
		let my_address: Address = self.blockchain().get_caller();
		self.set_owner(&my_address);
		self.set_target(&target);
		self.set_deadline(deadline);
		self.set_cf_esdt_token_name(&esdt_token_name);
	}

	#[endpoint]
	#[payable("*")]
	fn fund(
		&self,
		#[payment] payment: BigUint,
		#[payment_token] token: TokenIdentifier,
	) -> SCResult<()> {
		if self.blockchain().get_block_nonce() > self.get_deadline() {
			return sc_error!("cannot fund after deadline");
		}

		require!(token == self.get_cf_esdt_token_name(), "wrong esdt token");

		let caller = self.blockchain().get_caller();
		let mut deposit = self.get_deposit(&caller);
		let mut balance = self.get_esdt_balance_storage();

		deposit += payment.clone();
		balance += payment;

		self.set_deposit(&caller, &deposit);
		self.set_esdt_balance_storage(&balance);

		Ok(())
	}

	#[view]
	fn status(&self) -> Status {
		if self.blockchain().get_block_nonce() <= self.get_deadline() {
			Status::FundingPeriod
		} else if self.get_esdt_balance_storage() >= self.get_target() {
			Status::Successful
		} else {
			Status::Failed
		}
	}

	#[view(currentFunds)]
	fn current_funds(&self) -> SCResult<BigUint> {
		Ok(self.get_esdt_balance_storage())
	}

	#[endpoint]
	fn claim(&self) -> SCResult<()> {
		match self.status() {
			Status::FundingPeriod => sc_error!("cannot claim before deadline"),
			Status::Successful => {
				let caller = self.blockchain().get_caller();
				if caller != self.get_owner() {
					return sc_error!("only owner can claim successful funding");
				}

				let esdt_token_name = self.get_cf_esdt_token_name();
				let esdt_balance = self.get_esdt_balance_storage();

				self.set_esdt_balance_storage(&BigUint::zero());
				self.send()
					.direct(&caller, &esdt_token_name, &esdt_balance, &[]);

				Ok(())
			},
			Status::Failed => {
				let caller = self.blockchain().get_caller();
				let deposit = self.get_deposit(&caller);

				if deposit > 0 {
					let esdt_token_name = self.get_cf_esdt_token_name();
					let mut esdt_balance = self.get_esdt_balance_storage();

					esdt_balance -= deposit.clone();

					self.set_esdt_balance_storage(&esdt_balance);
					self.set_deposit(&caller, &BigUint::zero());
					self.send().direct(&caller, &esdt_token_name, &deposit, &[]);
				}
				Ok(())
			},
		}
	}

	// storage

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
	fn set_esdt_balance_storage(&self, esdt_balance: &BigUint);

	#[view]
	#[storage_get("esdtBalance")]
	fn get_esdt_balance_storage(&self) -> BigUint;

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
	fn set_cf_esdt_token_name(&self, esdt_token_name: &TokenIdentifier);

	#[view]
	#[storage_get("esdtTokenName")]
	fn get_cf_esdt_token_name(&self) -> TokenIdentifier;
}
