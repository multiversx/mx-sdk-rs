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
		let computed_beginning = beginning.unwrap_or_else(|| self.get_block_nonce());
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
			self.get_block_nonce() < self.get_deadline(),
			"smart contract already ended"
		);

		if let Some(max_funds) = self.get_max_funds() {
			require!(
				&self.get_sc_balance() + payment > max_funds,
				"smart contract full"
			);
		}

		let caller = self.get_caller();
		let user_id = self.get_or_create_user(&caller);
		let user_status = self.get_user_status(user_id);
		match user_status {
			UserStatus::New => {
				self.set_user_status(user_id, &UserStatus::Registered);
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

	fn pong_by_user_id(&self, user_id: usize) -> SCResult<()> {
		let user_status = self.get_user_status(user_id);
		match user_status {
			UserStatus::New => {
				sc_error!("can't pong, never pinged")
			},
			UserStatus::Registered => {
				self.set_user_status(user_id, &UserStatus::Withdrawn);
				let user_address = self.get_user_address(user_id);
				self.send_tx(&user_address, &self.get_fixed_sum(), b"pong");
				Ok(())
			},
			UserStatus::Withdrawn => {
				sc_error!("already withdrawn")
			},
		}
	}

	#[endpoint]
	fn pong(&self) -> SCResult<()> {
		require!(
			self.get_block_nonce() >= self.get_deadline(),
			"can't withdraw before deadline"
		);

		let caller = self.get_caller();
		let user_id = self.get_user_id(&caller);
		self.pong_by_user_id(user_id)
	}

	#[endpoint]
	fn pong_all(&self) -> SCResult<()> {
		require!(
			self.get_block_nonce() >= self.get_deadline(),
			"can't withdraw before deadline"
		);

		let num_users = self.get_num_users();
		for user_id in 1..=num_users {
			let _ = self.pong_by_user_id(user_id);
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
	fn set_user_status(&self, user_id: usize, user_status: &UserStatus);

	#[view]
	#[storage_get("user_status")]
	fn get_user_status(&self, user_id: usize) -> UserStatus;

	#[storage_set("user_id")]
	fn set_user_id(&self, user: &Address, user_id: usize);

	#[view]
	#[storage_get("user_id")]
	fn get_user_id(&self, user: &Address) -> usize;

	#[storage_set("user_address")]
	fn set_user_address(&self, user_id: usize, user_address: &Address);

	#[view]
	#[storage_get("user_address")]
	fn get_user_address(&self, user_id: usize) -> Address;

	#[storage_set("num_users")]
	fn set_num_users(&self, size: usize);

	#[view]
	#[storage_get("num_users")]
	fn get_num_users(&self) -> usize;

	fn get_or_create_user(&self, address: &Address) -> usize {
		let mut user_id = self.get_user_id(&address);
		if user_id == 0 {
			let mut num_users = self.get_num_users();
			num_users += 1;
			self.set_num_users(num_users);
			user_id = num_users;
			self.set_user_id(&address, user_id);
			self.set_user_address(user_id, &address);
		}
		user_id
	}
}
