#![no_std]
#![allow(unused_attributes)]

elrond_wasm::imports!();

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
		#[var_args] max_funds: OptionalArg<BigUint>,
	) {
		self.set_fixed_sum(fixed_sum);
		let computed_beginning = beginning.unwrap_or_else(|| self.get_block_timestamp());
		let deadline = computed_beginning + duration;
		self.set_deadline(deadline);
		self.set_beginning(computed_beginning);
		self.set_max_funds(max_funds.into_option());
	}

	#[payable("EGLD")]
	#[endpoint]
	fn ping(&self, #[payment] payment: &BigUint, _data: BoxedBytes) -> SCResult<()> {
		require!(
			payment == &self.get_fixed_sum(),
			"the payment must match the fixed sum"
		);

		require!(
			self.get_beginning() <= self.get_block_timestamp(),
			"smart contract not active yet"
		);

		require!(
			self.get_block_timestamp() < self.get_deadline(),
			"deadline has passed"
		);

		if let Some(max_funds) = self.get_max_funds() {
			require!(
				&self.get_sc_balance() + payment > max_funds,
				"smart contract full"
			);
		}

		let caller = self.get_caller();
		let user_id = self.user_mapper().get_or_create_user(&caller);
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
				if let Some(user_address) = self.user_mapper().get_user_address(user_id) {
					self.send()
						.direct_egld(&user_address, &self.get_fixed_sum(), b"pong");
					Ok(())
				} else {
					sc_error!("unknown user")
				}
			},
			UserStatus::Withdrawn => {
				sc_error!("already withdrawn")
			},
		}
	}

	#[endpoint]
	fn pong(&self) -> SCResult<()> {
		require!(
			self.get_block_timestamp() >= self.get_deadline(),
			"can't withdraw before deadline"
		);

		let caller = self.get_caller();
		let user_id = self.user_mapper().get_user_id(&caller);
		self.pong_by_user_id(user_id)
	}

	#[endpoint]
	fn pong_all(&self) -> SCResult<()> {
		require!(
			self.get_block_timestamp() >= self.get_deadline(),
			"can't withdraw before deadline"
		);

		let num_users = self.user_mapper().get_user_count();
		for user_id in 1..=num_users {
			let _ = self.pong_by_user_id(user_id);
		}
		Ok(())
	}

	#[view]
	fn get_user_addresses(&self) -> MultiResultVec<Address> {
		self.user_mapper().get_all_addresses().into()
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

	#[storage_mapper("user")]
	fn user_mapper(&self) -> UserMapper<Self::Storage>;

	#[storage_set("user_status")]
	fn set_user_status(&self, user_id: usize, user_status: &UserStatus);

	#[view]
	#[storage_get("user_status")]
	fn get_user_status(&self, user_id: usize) -> UserStatus;
}
