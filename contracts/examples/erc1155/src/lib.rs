#![no_std]

elrond_wasm::imports!();

const INTERFACE_SIGNATURE_ERC165: u32 = 0x01ffc9a7;
const INTERFACE_SIGNATURE_ERC1155: u32 = 0xd9b67a26;

/// One of the simplest smart contracts possible,
/// it holds a single variable in storage, which anyone can increment.
#[elrond_wasm_derive::contract(Erc1155Impl)]
pub trait Erc1155 {
	#[init]
	fn init(&self) {}

	// endpoints

	#[endpoint(safeTransferFrom)]
	fn safe_transfer_from(
		&self,
		from: Address,
		to: Address,
		id: BigUint,
		amount: BigUint,
		_data: &[u8],
	) -> SCResult<()> {
		let caller = self.get_caller();
		let balance = match self.get_balance_mapper(&from).get(&id) {
			Some(b) => b,
			None => return sc_error!("Address has no tokens of that type"),
		};

		require!(
			caller == from || self.get_is_approved(&caller, &from),
			"Caller is not approved to transfer tokens from address"
		);
		require!(to != Address::zero(), "Can't transfer to address zero");
		require!(amount > 0, "Must transfer more than 0");
		require!(amount <= balance, "Not enough balance for id");

		self.perform_transfer(&from, &to, &id, &amount);

		// self.transfer_single_event(&caller, &from, &to, &id, &amount);

		if self.is_smart_contract_address(&to) {
			// TODO: async-call
		}

		Ok(())
	}

	#[endpoint(safeBatchTransferFrom)]
	fn safe_batch_transfer_from(
		&self,
		from: Address,
		to: Address,
		ids: &[BigUint],
		amounts: &[BigUint],
		_data: &[u8],
	) -> SCResult<()> {
		let caller = self.get_caller();

		require!(
			caller == from || self.get_is_approved(&caller, &from),
			"Caller is not approved to transfer tokens from address"
		);
		require!(to != Address::zero(), "Can't transfer to address zero");
		require!(
			ids.len() == amounts.len(),
			"Id and amount lenghts do not match"
		);

		for i in 0..ids.len() {
			let balance = match self.get_balance_mapper(&from).get(&ids[i]) {
				Some(b) => b,
				None => return sc_error!("Address has no tokens of that type"),
			};

			require!(amounts[i] > 0, "Must transfer more than 0");
			require!(amounts[i] <= balance, "Not enough balance for id");
		}

		self.perform_batch_transfer(&from, &to, ids, amounts);

		// self.transfer_batch_event(&caller, &from, &to, ids, amounts);

		if self.is_smart_contract_address(&to) {
			// TODO: async-call
		}

		Ok(())
	}

	#[endpoint(setApprovalForAll)]
	fn set_approved_for_all(&self, operator: Address, approved: bool) {
		let caller = self.get_caller();

		self.set_is_approved(&operator, &caller, approved);

		// self.approval_for_all_event(&caller, &operator, approved);
	}

	// returns assigned id
	#[endpoint(createNonFungible)]
	fn create_non_fungible(&self, _uri: &[u8]) -> BigUint {
		// non-fungible tokens have a starting supply of 1 and cannot be minted
		let initial_supply = BigUint::from(1u32);
		let id = self.create_token(&self.get_caller(), &initial_supply);

		self.set_is_fungible(&id, false);

		// self.uri_event(uri, &id);

		id
	}

	// returns assigned id
	#[endpoint(createFungible)]
	fn create_fungible(&self, _uri: &[u8], initial_supply: BigUint) -> BigUint {
		let id = self.create_token(&self.get_caller(), &initial_supply);

		self.set_is_fungible(&id, true);

		// self.uri_event(uri, &id);

		id
	}

	// views

	#[view(supportsInterface)]
	fn supports_interface(&self, interface_id: u32) -> bool {
		interface_id == INTERFACE_SIGNATURE_ERC165 || interface_id == INTERFACE_SIGNATURE_ERC1155
	}

	#[view(balanceOf)]
	fn balance_of(&self, owner: &Address, id: &BigUint) -> BigUint {
		self.get_balance_mapper(&owner)
			.get(&id)
			.unwrap_or_else(|| BigUint::zero())
	}

	// returns balance for each (owner, id) pair
	#[view(balanceOfBatch)]
	fn balance_of_batch(&self, owners: &[Address], ids: &[BigUint]) -> Vec<BigUint> {
		let mut batch_balance = Vec::new();

		for i in 0..owners.len() {
			batch_balance.push(self.balance_of(&owners[i], &ids[i]));
		}

		batch_balance
	}

	#[view(isApprovedForAll)]
	fn is_approval_for_all(&self, owner: Address, operator: Address) -> bool {
		self.get_is_approved(&operator, &owner)
	}

	// private

	// mock
	fn is_smart_contract_address(&self, _address: &Address) -> bool {
		false
	}

	fn perform_transfer(&self, from: &Address, to: &Address, id: &BigUint, amount: &BigUint) {
		let mut from_balance_mapper = self.get_balance_mapper(&from);
		let mut to_balance_mapper = self.get_balance_mapper(&to);

		let mut from_balance = from_balance_mapper.get(id).unwrap();
		let mut to_balance = to_balance_mapper.get(id).unwrap_or_else(|| BigUint::zero());

		from_balance -= amount;
		to_balance += amount;

		from_balance_mapper.insert(id.clone(), from_balance);
		to_balance_mapper.insert(id.clone(), to_balance);
	}

	fn perform_batch_transfer(
		&self,
		from: &Address,
		to: &Address,
		ids: &[BigUint],
		amounts: &[BigUint],
	) {
		for i in 0..ids.len() {
			self.perform_transfer(from, to, &ids[i], &amounts[i]);
		}
	}

	fn create_token(&self, creator: &Address, initial_supply: &BigUint) -> BigUint {
		let id = self.get_last_valid_id() + BigUint::from(1u32);

		self.get_balance_mapper(&creator)
			.insert(id.clone(), initial_supply.clone());
		self.set_token_creator(&id, &creator);

		self.set_last_valid_id(&id);

		id
	}

	// storage

	// map for address -> id -> amount
	#[storage_mapper("balanceOf")]
	fn get_balance_mapper(&self, owner: &Address) -> MapMapper<Self::Storage, BigUint, BigUint>;

	// token creator

	#[storage_get("tokenCreator")]
	fn get_token_creator(&self, id: &BigUint) -> Address;

	#[storage_set("tokenCreator")]
	fn set_token_creator(&self, id: &BigUint, creator: &Address);

	// last valid id

	#[storage_get("lastValidId")]
	fn get_last_valid_id(&self) -> BigUint;

	#[storage_set("lastValidId")]
	fn set_last_valid_id(&self, last_valid_id: &BigUint);

	// check if an operator is approved. Default is false.

	#[storage_get("isApproved")]
	fn get_is_approved(&self, operator: &Address, owner: &Address) -> bool;

	#[storage_set("isApproved")]
	fn set_is_approved(&self, operator: &Address, owner: &Address, is_approved: bool);

	// check if a token is fungible. Non-fungible tokens cannot be minted. Default is false.

	#[storage_get("isFungible")]
	fn get_is_fungible(&self, id: &BigUint) -> bool;

	#[storage_set("isFungible")]
	fn set_is_fungible(&self, id: &BigUint, is_fungible: bool);

	// Events

	/*
	#[event("0x0000000000000000000000000000000000000000000000000000000000000001")]
	fn transfer_single_event(&self, operator: &Address, from: &Address, to: &Address, id: &BigUint, amount: &BigUint);

	#[event("0x0000000000000000000000000000000000000000000000000000000000000002")]
	fn transfer_batch_event(&self, operator: &Address, from: &Address, to: &Address, ids: &Vec<BigUint>, amounts: &Vec<BigUint>);

	#[event("0x0000000000000000000000000000000000000000000000000000000000000003")]
	fn approval_for_all_event(&self, owner: &Address, operator: &Address, approved: bool);

	#[event("0x0000000000000000000000000000000000000000000000000000000000000004")]
	fn uri_event(&self, uri: &[u8], id: &BigUint); // maybe use &str
	*/
}
