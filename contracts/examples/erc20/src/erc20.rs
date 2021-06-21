#![no_std]

elrond_wasm::imports!();

#[elrond_wasm_derive::contract]
pub trait SimpleErc20Token {
	// STORAGE

	/// Total number of tokens in existence.
	#[view(totalSupply)]
	#[storage_get("total_supply")]
	fn get_total_supply(&self) -> Self::BigUint;

	#[storage_set("total_supply")]
	fn set_total_supply(&self, total_supply: &Self::BigUint);

	/// Gets the balance of the specified address.
	///
	/// Arguments:
	///
	/// * `address` The address to query the the balance of
	///
	#[view(balanceOf)]
	#[storage_get("balance")]
	fn get_token_balance(&self, address: &Address) -> Self::BigUint;

	#[storage_set("balance")]
	fn set_token_balance(&self, address: &Address, balance: &Self::BigUint);

	/// The amount of tokens that an owner allowed to a spender.
	///
	/// Arguments:
	///
	/// * `owner` The address that owns the funds.
	/// * `spender` The address that will spend the funds.
	///
	#[view(allowance)]
	#[storage_get("allowance")]
	fn get_allowance(&self, owner: &Address, spender: &Address) -> Self::BigUint;

	#[storage_set("allowance")]
	fn set_allowance(&self, owner: &Address, spender: &Address, allowance: &Self::BigUint);

	// FUNCTIONALITY

	/// Constructor, is called immediately after the contract is created
	/// Will set the fixed global token supply and give all the supply to the creator.
	#[init]
	fn init(&self, total_supply: &Self::BigUint) {
		let creator = self.blockchain().get_caller();

		// save total supply
		self.set_total_supply(total_supply);

		// deployer initially receives the total supply
		let mut creator_balance = self.get_token_balance(&creator);
		creator_balance += total_supply;
		self.set_token_balance(&creator, &creator_balance);
	}

	/// This method is private, deduplicates logic from transfer and transferFrom.
	fn perform_transfer(
		&self,
		sender: Address,
		recipient: Address,
		amount: Self::BigUint,
	) -> SCResult<()> {
		// check if enough funds & decrease sender balance
		{
			let mut sender_balance = self.get_token_balance(&sender);
			if amount > sender_balance {
				return sc_error!("insufficient funds");
			}

			sender_balance -= &amount;

			self.set_token_balance(&sender, &sender_balance);
		}

		// increase recipient balance
		let mut recipient_balance = self.get_token_balance(&recipient);
		recipient_balance += &amount; // saved automatically at the end of scope
		self.set_token_balance(&recipient, &recipient_balance);

		// log operation
		self.transfer_event(&sender, &recipient, &amount);

		Ok(())
	}

	/// Transfer token to a specified address from sender.
	///
	/// Arguments:
	///
	/// * `to` The address to transfer to.
	///
	#[endpoint]
	fn transfer(&self, to: Address, amount: Self::BigUint) -> SCResult<()> {
		// the sender is the caller
		let sender = self.blockchain().get_caller();
		self.perform_transfer(sender, to, amount)
	}

	/// Use allowance to transfer funds between two accounts.
	///
	/// Arguments:
	///
	/// * `sender` The address to transfer from.
	/// * `recipient` The address to transfer to.
	/// * `amount` the amount of tokens to be transferred.
	///
	#[endpoint(transferFrom)]
	fn transfer_from(
		&self,
		sender: Address,
		recipient: Address,
		amount: Self::BigUint,
	) -> SCResult<()> {
		// get caller
		let caller = self.blockchain().get_caller();

		// load allowance
		let mut allowance = self.get_allowance(&sender, &caller);

		// amount should not exceed allowance
		if amount > allowance {
			return sc_error!("allowance exceeded");
		}

		// update allowance
		allowance -= &amount; // saved automatically at the end of scope
		self.set_allowance(&sender, &caller, &allowance);

		// transfer
		self.perform_transfer(sender, recipient, amount)
	}

	/// Approve the given address to spend the specified amount of tokens on behalf of the sender.
	/// It overwrites any previously existing allowance from sender to beneficiary.
	///
	/// Arguments:
	///
	/// * `spender` The address that will spend the funds.
	/// * `amount` The amount of tokens to be spent.
	///
	#[endpoint]
	fn approve(&self, spender: Address, amount: Self::BigUint) -> SCResult<()> {
		// sender is the caller
		let caller = self.blockchain().get_caller();

		// store allowance
		self.set_allowance(&caller, &spender, &amount);

		// log operation
		self.approve_event(&caller, &spender, &amount);
		Ok(())
	}

	// EVENTS

	#[event("transfer")]
	fn transfer_event(
		&self,
		#[indexed] sender: &Address,
		#[indexed] recipient: &Address,
		amount: &Self::BigUint,
	);

	#[event("approve")]
	fn approve_event(
		&self,
		#[indexed] sender: &Address,
		#[indexed] recipient: &Address,
		amount: &Self::BigUint,
	);
}
