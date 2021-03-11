#![no_std]
#![allow(unused_attributes)]

elrond_wasm::imports!();

mod user_status;

use user_status::UserStatus;

/// Derived empirically.
const PONG_ALL_LOW_GAS_LIMIT: u64 = 3_000_000;

#[elrond_wasm_derive::contract(PingPongImpl)]
pub trait PingPong {
	#[init]
	fn init(
		&self,
		ping_amount: &BigUint,
		duration_in_seconds: u64,
		opt_activation_timestamp: Option<u64>,
		#[var_args] max_funds: OptionalArg<BigUint>,
	) {
		self.ping_amount().set(ping_amount);
		let activation_timestamp =
			opt_activation_timestamp.unwrap_or_else(|| self.get_block_timestamp());
		let deadline = activation_timestamp + duration_in_seconds;
		self.deadline().set(&deadline);
		self.activation_timestamp().set(&activation_timestamp);
		self.max_funds().set(&max_funds.into_option());
	}

	#[payable("EGLD")]
	#[endpoint]
	fn ping(&self, #[payment] payment: &BigUint, _data: BoxedBytes) -> SCResult<()> {
		require!(
			payment == &self.ping_amount().get(),
			"the payment must match the fixed sum"
		);

		let block_timestamp = self.get_block_timestamp();
		require!(
			self.activation_timestamp().get() <= block_timestamp,
			"smart contract not active yet"
		);

		require!(
			block_timestamp < self.deadline().get(),
			"deadline has passed"
		);

		if let Some(max_funds) = self.max_funds().get() {
			require!(
				&self.get_sc_balance() + payment <= max_funds,
				"smart contract full"
			);
		}

		let caller = self.get_caller();
		let user_id = self.user_mapper().get_or_create_user(&caller);
		let user_status = self.user_status(user_id).get();
		match user_status {
			UserStatus::New => {
				self.user_status(user_id).set(&UserStatus::Registered);
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
		let user_status = self.user_status(user_id).get();
		match user_status {
			UserStatus::New => {
				sc_error!("can't pong, never pinged")
			},
			UserStatus::Registered => {
				self.user_status(user_id).set(&UserStatus::Withdrawn);
				if let Some(user_address) = self.user_mapper().get_user_address(user_id) {
					self.send()
						.direct_egld(&user_address, &self.ping_amount().get(), b"pong");
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
			self.get_block_timestamp() >= self.deadline().get(),
			"can't withdraw before deadline"
		);

		let caller = self.get_caller();
		let user_id = self.user_mapper().get_user_id(&caller);
		self.pong_by_user_id(user_id)
	}

	#[endpoint(pongAll)]
	fn pong_all(&self) -> SCResult<OperationCompletionStatus> {
		require!(
			self.get_block_timestamp() >= self.deadline().get(),
			"can't withdraw before deadline"
		);

		let num_users = self.user_mapper().get_user_count();
		let mut pong_all_last_user = self.pong_all_last_user().get();
		loop {
			if pong_all_last_user >= num_users {
				// clear field and reset to 0
				pong_all_last_user = 0;
				self.pong_all_last_user().set(&pong_all_last_user);
				return Ok(OperationCompletionStatus::Finished);
			}

			if self.get_gas_left() < PONG_ALL_LOW_GAS_LIMIT {
				self.pong_all_last_user().set(&pong_all_last_user);
				return Ok(OperationCompletionStatus::InterruptedBeforeOutOfGas);
			}

			pong_all_last_user += 1;
			let _ = self.pong_by_user_id(pong_all_last_user);
		}
	}

	#[view(getUserAddresses)]
	fn get_user_addresses(&self) -> MultiResultVec<Address> {
		self.user_mapper().get_all_addresses().into()
	}

	#[view]
	fn get_sc_balance_view(&self) -> BigUint {
		self.get_sc_balance()
	}

	// storage

	#[view(getPingAmount)]
	#[storage_mapper("ping_amount")]
	fn ping_amount(&self) -> SingleValueMapper<Self::Storage, BigUint>;

	#[view(getDeadline)]
	#[storage_mapper("deadline")]
	fn deadline(&self) -> SingleValueMapper<Self::Storage, u64>;

	#[view(getActivationTimestamp)]
	#[storage_mapper("activation_timestamp")]
	fn activation_timestamp(&self) -> SingleValueMapper<Self::Storage, u64>;

	#[view(getMaxFunds)]
	#[storage_mapper("max_funds")]
	fn max_funds(&self) -> SingleValueMapper<Self::Storage, Option<BigUint>>;

	#[storage_mapper("user")]
	fn user_mapper(&self) -> UserMapper<Self::Storage>;

	#[view(getUserStatus)]
	#[storage_mapper("user_status")]
	fn user_status(&self, user_id: usize) -> SingleValueMapper<Self::Storage, UserStatus>;

	#[view(pongAllLastUser)]
	#[storage_mapper("pong_all_last_user")]
	fn pong_all_last_user(&self) -> SingleValueMapper<Self::Storage, usize>;
}
