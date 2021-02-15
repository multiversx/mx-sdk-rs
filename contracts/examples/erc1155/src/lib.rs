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

	#[endpoint(safeTransferFromFungible)]
	fn safe_transfer_from_fungible(
		&self,
		from: Address,
		to: Address,
		type_id: BigUint,
		amount: BigUint,
		_data: &[u8],
	) -> SCResult<()> {
		let caller = self.get_caller();
		
		require!(to != Address::zero(), "Can't transfer to address zero");
		require!(amount > 0, "Must transfer more than 0");
		require!(self.is_valid_type_id(&type_id), "Toke id is invalid");
		require!(
			self.get_is_fungible(&type_id) == true,
			"Token is not fungible"
		);
		require!(
			caller == from || self.get_is_approved(&caller, &from),
			"Caller is not approved to transfer tokens from address"
		);

		let balance = match self.get_balance_mapper(&from).get(&type_id) {
			Some(b) => b,
			None => return sc_error!("Address has no tokens of that type"),
		};
		require!(amount <= balance, "Not enough balance for id");

		self.perform_transfer_fungible(&from, &to, &type_id, &amount);

		// self.transfer_single_event(&caller, &from, &to, &id, &amount);

		if self.is_smart_contract_address(&to) {
			// TODO: async-call
		}

		Ok(())
	}

	#[endpoint(safeTransferFromNonFungible)]
	fn safe_transfer_from_non_fungible(
		&self,
		from: Address,
		to: Address,
		type_id: BigUint,
		token_id: BigUint,
		_data: &[u8],
	) -> SCResult<()> {
		let caller = self.get_caller();
		
		require!(to != Address::zero(), "Can't transfer to address zero");
		require!(self.is_valid_token_id(&type_id, &token_id), "Token type-id pair is not valid");
		require!(
			self.get_is_fungible(&type_id) == false,
			"Token is fungible"
		);
		require!(self.get_token_owner(&type_id, &token_id) == from, "_from_ is not the owner of the token");
		require!(
			caller == from || self.get_is_approved(&caller, &from),
			"Caller is not approved to transfer tokens from address"
		);

		self.set_token_owner(&type_id, &token_id, &to);

		// self.transfer_single_event(&caller, &from, &to, &id, &amount);

		if self.is_smart_contract_address(&to) {
			// TODO: async-call
		}

		Ok(())
	}

	// value is amount for fungible, token_id for non-fungible
	#[endpoint(safeBatchTransferFrom)]
	fn safe_batch_transfer_from(
		&self,
		from: Address,
		to: Address,
		type_ids: &[BigUint],
		values: &[BigUint],
		_data: &[u8],
	) -> SCResult<()> {
		let caller = self.get_caller();

		require!(
			caller == from || self.get_is_approved(&caller, &from),
			"Caller is not approved to transfer tokens from address"
		);
		require!(to != Address::zero(), "Can't transfer to address zero");
		require!(
			type_ids.len() == values.len(),
			"Id and value lenghts do not match"
		);

		for i in 0..type_ids.len() {
			let type_id = &type_ids[i];

			if self.get_is_fungible(type_id) == true {
				let amount = &values[i];

				let balance = match self.get_balance_mapper(&from).get(type_id) {
					Some(b) => b,
					None => return sc_error!("Address has no tokens of that type"),
				};

				require!(amount > &0, "Must transfer more than 0");
				require!(amount <= &balance, "Not enough balance for id");
			}
			else {
				let token_id = &values[i];

				require!(self.get_token_owner(&type_id, &token_id) == from, "_from_ is not the owner of the token");
			}
		}

		self.perform_batch_transfer(&from, &to, type_ids, values);

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
	fn create_non_fungible(&self, _uri: &[u8], initial_supply: BigUint) -> BigUint {
		let big_uint_one = BigUint::from(1u32);

		let creator = self.get_caller();
		let type_id = self.get_last_valid_type_id() + big_uint_one.clone();
		let mut token_id = big_uint_one.clone();

		while token_id < initial_supply {
			self.set_token_owner(&type_id, &token_id, &creator);

			token_id += &big_uint_one;
		}

		self.set_token_type_creator(&type_id, &creator);
		self.set_last_valid_type_id(&type_id);

		self.set_is_fungible(&type_id, false);

		// self.uri_event(uri, &id);

		type_id
	}

	// returns assigned id
	#[endpoint(createFungible)]
	fn create_fungible(&self, _uri: &[u8], initial_supply: BigUint) -> BigUint {
		let type_id = self.get_last_valid_type_id() + BigUint::from(1u32);
		let creator = self.get_caller();

		self.get_balance_mapper(&creator)
			.insert(type_id.clone(), initial_supply.clone());
		self.set_token_type_creator(&type_id, &creator);
		self.set_is_fungible(&type_id, true);

		self.set_last_valid_type_id(&type_id);

		// self.uri_event(uri, &id);

		type_id
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

	fn perform_transfer_fungible(&self, from: &Address, to: &Address, type_id: &BigUint, amount: &BigUint) {
		let mut from_balance_mapper = self.get_balance_mapper(&from);
		let mut to_balance_mapper = self.get_balance_mapper(&to);

		let mut from_balance = from_balance_mapper.get(type_id).unwrap();
		let mut to_balance = to_balance_mapper.get(type_id).unwrap_or_else(|| BigUint::zero());

		from_balance -= amount;
		to_balance += amount;

		from_balance_mapper.insert(type_id.clone(), from_balance);
		to_balance_mapper.insert(type_id.clone(), to_balance);
	}

	fn perform_batch_transfer(
		&self,
		from: &Address,
		to: &Address,
		type_ids: &[BigUint],
		values: &[BigUint],
	) {
		for i in 0..type_ids.len() {
			//self.perform_transfer(from, to, &type_ids[i], &values[i]);
			let type_id = &type_ids[i];
			if self.get_is_fungible(&type_id) == true {
				let amount = &values[i];

				self.perform_transfer_fungible(&from, &to, type_id, amount);
			}
			else {
				let token_id = &values[i];
				
				self.set_token_owner(type_id, token_id, &to);
			}
		}
	}

	fn is_valid_type_id(&self, type_id: &BigUint) -> bool {
		type_id > &0 && type_id <= &self.get_last_valid_type_id()
	}

	fn is_valid_token_id(&self, type_id: &BigUint, token_id: &BigUint) -> bool {
		self.is_valid_type_id(type_id)
			&& token_id > &0
			&& token_id <= &self.get_last_valid_token_id_for_type(type_id)
	}

	// storage

	// map for address -> type_id -> amount
	// for fungible

	#[storage_mapper("balanceOf")]
	fn get_balance_mapper(&self, owner: &Address) -> MapMapper<Self::Storage, BigUint, BigUint>;

	// token owner
	// for non-fungible

	#[view(getTokenOwner)]
	#[storage_get("tokenOwner")]
	fn get_token_owner(&self, type_id: &BigUint, token_id: &BigUint) -> Address;

	#[storage_set("tokenOwner")]
	fn set_token_owner(&self, type_id: &BigUint, token_id: &BigUint, owner: &Address);

	// token creator

	#[view(getTokenTypeCreator)]
	#[storage_get("tokenTypeCreator")]
	fn get_token_type_creator(&self, type_id: &BigUint) -> Address;

	#[storage_set("tokenTypeCreator")]
	fn set_token_type_creator(&self, type_id: &BigUint, creator: &Address);

	// check if a token is fungible

	#[view(isFungible)]
	#[storage_get("isFungible")]
	fn get_is_fungible(&self, type_id: &BigUint) -> bool;

	#[storage_set("isFungible")]
	fn set_is_fungible(&self, type_id: &BigUint, is_fungible: bool);

	// last valid id

	#[storage_get("lastValidTypeId")]
	fn get_last_valid_type_id(&self) -> BigUint;

	#[storage_set("lastValidTypeId")]
	fn set_last_valid_type_id(&self, last_valid_type_id: &BigUint);

	#[storage_get("lastValidTokenIdForType")]
	fn get_last_valid_token_id_for_type(&self, type_id: &BigUint) -> BigUint;

	#[storage_set("lastValidTokenIdForType")]
	fn set_last_valid_token_id_for_type(&self, type_id: &BigUint, last_valid_token_id: &BigUint);

	// check if an operator is approved. Default is false.

	#[storage_get("isApproved")]
	fn get_is_approved(&self, operator: &Address, owner: &Address) -> bool;

	#[storage_set("isApproved")]
	fn set_is_approved(&self, operator: &Address, owner: &Address, is_approved: bool);

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
