#![no_std]
#![allow(unused_attributes)]

imports!();

mod user_status;

use user_status::UserStatus;

#[elrond_wasm_derive::contract(PingPongImpl)]
pub trait PingPong {
	#[init]
	fn init(
		&self,
		fixed_sum: &BigUint,
		duration: u64,
		beginning: Option<u64>,
		max_funds: Option<BigUint>,
	) {
		self.set_fixed_sum(fixed_sum);
		let computed_beginning = beginning.unwrap_or(self.get_block_nonce());
		let deadline = computed_beginning + duration;
		self.set_deadline(deadline);
		self.set_beginning(computed_beginning);
		self.set_max_funds(max_funds);
	}

	#[payable]
	#[endpoint]
	fn ping(&self, #[payment] payment: &BigUint) -> SCResult<()> {
		require!(
			payment == &self.get_fixed_sum(),
			"the payment must match the fixed sum"
		);

		require!(
			self.get_beginning() <= self.get_block_nonce(),
			"smart contract not active yet"
		);

		require!(
			self.get_max_funds()
				.map_or(true, |max_funds| &self.get_sc_balance() + payment
					> max_funds),
			"smart contract full"
		);

		let caller = self.get_caller();
		let user_status = self.get_user_status(&caller);
		match user_status {
			UserStatus::New => {
				self.set_user_status(&caller, &UserStatus::Registered);
				let new_index = self.get_size() + 1;
				self.set_index(&caller, new_index);
				self.set_address(new_index, &caller);
				self.set_size(new_index);
				Ok(())
			},
			UserStatus::Registered => {
				sc_error!("can only ping once")
			},
			UserStatus::Withdrawn => {
				sc_error!("already withdrawn")
			},
		}
	}

	fn pong_address(&self, caller: &Address) -> SCResult<()> {
		require!(
			self.get_block_nonce() >= self.get_deadline(),
			"can't withdraw before deadline"
		);

		let user_status = self.get_user_status(&caller);
		match user_status {
			UserStatus::New => {
				sc_error!("can't pong, never pinged")
			},
			UserStatus::Registered => {
				self.set_user_status(&caller, &UserStatus::Withdrawn);
				self.send_tx(&caller, &self.get_fixed_sum(), "pong");
				Ok(())
			},
			UserStatus::Withdrawn => {
				sc_error!("already withdrawn")
			},
		}
	}

	#[endpoint]
	fn pong(&self) -> SCResult<()> {
		let caller = self.get_caller();
		self.pong_address(&caller)
	}

	#[endpoint]
	fn pong_all(&self) -> SCResult<()> {
		require!(
			self.get_block_nonce() >= self.get_deadline(),
			"can't withdraw before deadline"
		);

		let size = self.get_size();
		for i in 1..=size {
			let address = self.get_address(i);
			self.pong_address(&address).ok();
		}
		Ok(())
	}

	// storage

	#[storage_set("fixed_sum")]
	fn set_fixed_sum(&self, fixed_sum: &BigUint);

	#[view]
	#[storage_get("fixed_sum")]
	fn get_fixed_sum(&self) -> BigUint;

	#[storage_set("deadline")]
	fn set_deadline(&self, deadline: u64);

	#[view]
	#[storage_get("deadline")]
	fn get_deadline(&self) -> u64;

	#[storage_set("beginning")]
	fn set_beginning(&self, beginning: u64);

	#[view]
	#[storage_get("beginning")]
	fn get_beginning(&self) -> u64;

	#[storage_set("max_funds")]
	fn set_max_funds(&self, max_funds: Option<BigUint>);

	#[view]
	#[storage_get("max_funds")]
	fn get_max_funds(&self) -> Option<BigUint>;

	#[storage_set("user_status")]
	fn set_user_status(&self, user: &Address, user_status: &UserStatus);

	#[view]
	#[storage_get("user_status")]
	fn get_user_status(&self, user: &Address) -> UserStatus;

	#[storage_set("index")]
	fn set_index(&self, user: &Address, index: usize);

	#[view]
	#[storage_get("index")]
	fn get_index(&self, user: &Address) -> usize;

	#[storage_set("address")]
	fn set_address(&self, index: usize, user: &Address);

	#[view]
	#[storage_get("address")]
	fn get_address(&self, index: usize) -> Address;

	#[storage_set("size")]
	fn set_size(&self, size: usize);

	#[view]
	#[storage_get("size")]
	fn get_size(&self) -> usize;
}
