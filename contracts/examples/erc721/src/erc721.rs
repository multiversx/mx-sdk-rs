#![no_std]

elrond_wasm::imports!();

#[elrond_wasm_derive::contract]
pub trait NonFungibleTokens {
	#[init]
	fn init(&self, initial_minted: u64) {
		let owner = self.blockchain().get_caller();

		self.set_owner(&owner);
		self.perform_mint(initial_minted, &owner);
	}

	// endpoints

	/// Creates new tokens and sets their ownership to the specified account.
	/// Only the contract owner may call this function.
	#[endpoint]
	fn mint(&self, count: u64, new_token_owner: Address) -> SCResult<()> {
		require!(
			self.blockchain().get_caller() == self.get_owner(),
			"Only owner can mint new tokens!"
		);

		self.perform_mint(count, &new_token_owner);

		Ok(())
	}

	/// Approves an account to transfer the token on behalf of its owner.<br>
	/// Only the owner of the token may call this function.
	#[endpoint]
	fn approve(&self, token_id: u64, approved_address: Address) -> SCResult<()> {
		require!(token_id < self.get_total_minted(), "Token does not exist!");
		require!(
			self.blockchain().get_caller() == self.get_token_owner(token_id),
			"Only the token owner can approve!"
		);

		self.set_approval(token_id, &approved_address);

		Ok(())
	}

	/// Revokes approval for the token.<br>  
	/// Only the owner of the token may call this function.
	#[endpoint]
	fn revoke(&self, token_id: u64) -> SCResult<()> {
		require!(token_id < self.get_total_minted(), "Token does not exist!");
		require!(
			self.blockchain().get_caller() == self.get_token_owner(token_id),
			"Only the token owner can revoke approval!"
		);

		if !self.approval_is_empty(token_id) {
			self.clear_approval(token_id);
		}

		Ok(())
	}

	/// Transfer ownership of the token to a new account.
	#[endpoint]
	fn transfer(&self, token_id: u64, to: Address) -> SCResult<()> {
		require!(token_id < self.get_total_minted(), "Token does not exist!");

		let caller = self.blockchain().get_caller();
		let token_owner = self.get_token_owner(token_id);

		if caller == token_owner {
			self.perform_transfer(token_id, &token_owner, &to);

			return Ok(());
		} else if !self.approval_is_empty(token_id) {
			let approved_address = self.get_approval(token_id);

			if caller == approved_address {
				self.perform_transfer(token_id, &token_owner, &to);

				return Ok(());
			}
		}

		sc_error!("Only the owner or the approved account may transfer the token!")
	}

	// private methods

	fn perform_mint(&self, count: u64, new_token_owner: &Address) {
		let new_owner_current_total = self.get_token_count(new_token_owner);
		let total_minted = self.get_total_minted();
		let first_new_id = total_minted;
		let last_new_id = total_minted + count;

		for id in first_new_id..last_new_id {
			self.set_token_owner(id, new_token_owner);
		}

		self.set_total_minted(total_minted + count);
		self.set_token_count(new_token_owner, new_owner_current_total + count);
	}

	fn perform_transfer(&self, token_id: u64, from: &Address, to: &Address) {
		let prev_owner_token_count = self.get_token_count(from);
		let new_owner_token_count = self.get_token_count(to);

		// new ownership revokes approvals by previous owner
		self.clear_approval(token_id);

		self.set_token_count(from, prev_owner_token_count - 1);
		self.set_token_count(to, new_owner_token_count + 1);
		self.set_token_owner(token_id, to);
	}

	// storage

	#[view(contractOwner)]
	#[storage_get("owner")]
	fn get_owner(&self) -> Address;

	#[storage_set("owner")]
	fn set_owner(&self, owner: &Address);

	#[view(totalMinted)]
	#[storage_get("totalMinted")]
	fn get_total_minted(&self) -> u64;

	#[storage_set("totalMinted")]
	fn set_total_minted(&self, total_minted: u64);

	#[view(tokenOwner)]
	#[storage_get("tokenOwner")]
	fn get_token_owner(&self, token_id: u64) -> Address;

	#[storage_set("tokenOwner")]
	fn set_token_owner(&self, token_id: u64, owner: &Address);

	#[view(tokenCount)]
	#[storage_get("tokenCount")]
	fn get_token_count(&self, owner: &Address) -> u64;

	#[storage_set("tokenCount")]
	fn set_token_count(&self, owner: &Address, token_count: u64);

	#[storage_is_empty("approval")]
	fn approval_is_empty(&self, token_id: u64) -> bool;

	#[view(approval)]
	#[storage_get("approval")]
	fn get_approval(&self, token_id: u64) -> Address;

	#[storage_set("approval")]
	fn set_approval(&self, token_id: u64, approved_address: &Address);

	#[storage_clear("approval")]
	fn clear_approval(&self, token_id: u64);
}
