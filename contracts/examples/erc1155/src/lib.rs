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
		data: &[u8],
	) -> SCResult<()> {
		Ok(())
	}

	#[endpoint(setApprovalForAll)]
	fn set_approved_for_all(&self, operator: Address, approved: bool) -> SCResult<()> {
		Ok(())
	}

	// views

	#[view(supportsInterface)]
	fn supports_interface(&self, interface_id: u32) -> bool {
		interface_id == INTERFACE_SIGNATURE_ERC165 || interface_id == INTERFACE_SIGNATURE_ERC1155
	}

	#[view(balanceOf)]
	fn balance_of(&self, owner: Address, id: BigUint) -> BigUint {
		BigUint::zero()
	}

	// returns balance for each (owner, id) pair
	#[view(balanceOfBatch)]
	fn balance_of_batch(&self, owners: &[Address], ids: &[BigUint]) -> Vec<BigUint> {
		Vec::new()
	}

	#[view(isApprovedForAll)]
	fn is_approval_for_all(&self, owner: Address, operator: Address) -> bool {
		true
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

	// storage

	// map for address -> id -> amount
	#[storage_mapper("balanceOf")]
	fn get_balance_mapper(&self, owner: &Address) -> MapMapper<Self::Storage, BigUint, BigUint>;

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
	fn uri_event(&self, new_uri: &[u8], id: &BigUint); // maybe use &str
	*/
}
