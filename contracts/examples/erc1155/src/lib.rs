#![no_std]

use elrond_codec::test_util::top_encode_to_vec_or_panic;
use elrond_wasm::types::MultiArg2;
use elrond_wasm::HexCallDataSerializer;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const ON_ERC_RECEIVED_ENDPOINT_NAME: &[u8] = b"onERC1155Received";
const ON_ERC_BATCH_RECEIVED_ENDPOINT_NAME: &[u8] = b"onERC1155BatchReceived";

#[derive(TopEncode, TopDecode)]
pub struct Transfer<BigUint: BigUintApi> {
	pub from: Address,
	pub to: Address,
	pub type_ids: Vec<BigUint>,
	pub values: Vec<BigUint>,
}

#[elrond_wasm_derive::contract(Erc1155Impl)]
pub trait Erc1155 {
	#[init]
	fn init(&self) {}

	// endpoints

	/// `value` is amount for fungible, nft_id for non-fungible
	#[endpoint(safeTransferFrom)]
	fn safe_transfer_from(
		&self,
		from: Address,
		to: Address,
		type_id: BigUint,
		value: BigUint,
		data: &[u8],
	) -> SCResult<()> {
		let caller = self.blockchain().get_caller();

		require!(to != Address::zero(), "Can't transfer to address zero");
		require!(self.is_valid_type_id(&type_id), "Token id is invalid");
		require!(
			caller == from || self.get_is_approved(&caller, &from),
			"Caller is not approved to transfer tokens from address"
		);

		if self.is_fungible(&type_id) {
			let amount = &value;

			sc_try!(self.try_reserve_fungible(&from, &type_id, &amount));

			if self.blockchain().is_smart_contract(&to) {
				self.peform_async_call_single_transfer(from, to, type_id, value, data);
			} else {
				self.increase_balance(&to, &type_id, &amount);
			}
		} else {
			let nft_id = &value;

			sc_try!(self.try_reserve_non_fungible(&from, &type_id, &nft_id));

			if self.blockchain().is_smart_contract(&to) {
				self.peform_async_call_single_transfer(from, to, type_id, value, data);
			} else {
				let amount = BigUint::from(1u32);
				self.increase_balance(&to, &type_id, &amount);
				self.set_token_owner(&type_id, nft_id, &to);
			}
		}

		// self.transfer_single_event(&caller, &from, &to, &id, &amount);

		Ok(())
	}

	/// `value` is amount for fungible, nft_id for non-fungible
	#[endpoint(safeBatchTransferFrom)]
	fn safe_batch_transfer_from(
		&self,
		from: Address,
		to: Address,
		type_ids: &[BigUint],
		values: &[BigUint],
		data: &[u8],
	) -> SCResult<()> {
		let caller = self.blockchain().get_caller();
		let is_receiver_smart_contract = self.blockchain().is_smart_contract(&to);

		require!(
			caller == from || self.get_is_approved(&caller, &from),
			"Caller is not approved to transfer tokens from address"
		);
		require!(to != Address::zero(), "Can't transfer to address zero");
		require!(
			!type_ids.is_empty() && !values.is_empty(),
			"No type_ids and/or values provided"
		);
		require!(
			type_ids.len() == values.len(),
			"Id and value lenghts do not match"
		);

		// storage edits are rolled back in case of SCError,
		// so the reverting is handled automatically if one of the transfers fails
		for (type_id, value) in type_ids.iter().zip(values.iter()) {
			if self.is_fungible(type_id) {
				let amount = value;

				sc_try!(self.try_reserve_fungible(&from, &type_id, &amount));

				if !is_receiver_smart_contract {
					self.increase_balance(&to, &type_id, &amount);
				}
			} else {
				let nft_id = value;

				sc_try!(self.try_reserve_non_fungible(&from, &type_id, &nft_id));

				if !is_receiver_smart_contract {
					let amount = BigUint::from(1u32);
					self.increase_balance(&to, &type_id, &amount);
					self.set_token_owner(&type_id, &nft_id, &to);
				} else {
					self.set_token_owner(&type_id, &nft_id, &Address::zero());
				}
			}
		}

		if is_receiver_smart_contract {
			self.peform_async_call_batch_transfer(from, to, type_ids, values, data);
		}

		// self.transfer_batch_event(&caller, &from, &to, ids, amounts);

		Ok(())
	}

	#[endpoint(setApprovalForAll)]
	fn set_approved_for_all(&self, operator: Address, approved: bool) {
		let caller = self.blockchain().get_caller();

		self.set_is_approved(&operator, &caller, approved);

		// self.approval_for_all_event(&caller, &operator, approved);
	}

	// returns assigned id
	#[endpoint(createToken)]
	fn create_token(
		&self,
		uri: &BoxedBytes,
		initial_supply: BigUint,
		is_fungible: bool,
	) -> BigUint {
		let big_uint_one = BigUint::from(1u32);

		let creator = self.blockchain().get_caller();
		let type_id = &self.get_last_valid_type_id() + &big_uint_one;

		self.set_balance(&creator, &type_id, &initial_supply);
		self.set_token_type_creator(&type_id, &creator);
		self.set_is_fungible(&type_id, is_fungible);

		if !is_fungible {
			self.set_owner_for_range(&type_id, &big_uint_one, &initial_supply, &creator);
			self.set_last_valid_nft_id_for_type(&type_id, &initial_supply);
		}

		self.set_last_valid_type_id(&type_id);
		self.set_token_type_uri(&type_id, uri);

		// self.transfer_single_event(&caller, &from, &to, &id, &amount);

		// uri event

		type_id
	}

	#[endpoint]
	fn mint(&self, type_id: BigUint, amount: BigUint) -> SCResult<()> {
		let creator = self.get_token_type_creator(&type_id);

		require!(
			self.blockchain().get_caller() == creator,
			"Only the token creator may mint more tokens"
		);

		self.increase_balance(&creator, &type_id, &amount);

		if !self.is_fungible(&type_id) {
			let last_valid_id = self.get_last_valid_nft_id_for_type(&type_id);
			let id_first = &last_valid_id + &BigUint::from(1u32);
			let id_last = last_valid_id + amount;

			self.set_owner_for_range(&type_id, &id_first, &id_last, &creator);

			self.set_last_valid_nft_id_for_type(&type_id, &id_last);
		}

		// self.transfer_single_event(&caller, &from, &to, &id, &amount);

		Ok(())
	}

	#[endpoint]
	fn burn(&self, type_id: BigUint, amount: BigUint) -> SCResult<()> {
		require!(
			self.is_fungible(&type_id),
			"Only fungible tokens can be burned"
		);

		let caller = self.blockchain().get_caller();
		let balance = self.balance_of(&caller, &type_id);

		require!(balance >= amount, "Not enough tokens to burn");

		self.decrease_balance(&caller, &type_id, &amount);

		Ok(())
	}

	// views

	#[view(balanceOf)]
	fn balance_of(&self, owner: &Address, type_id: &BigUint) -> BigUint {
		self.get_balance_mapper(&owner)
			.get(&type_id)
			.unwrap_or_else(BigUint::zero)
	}

	// returns balance for each (owner, id) pair
	#[view(balanceOfBatch)]
	fn balance_of_batch(
		&self,
		#[var_args] owner_type_id_pairs: VarArgs<MultiArg2<Address, BigUint>>,
	) -> MultiResultVec<BigUint> {
		let mut batch_balance = Vec::new();
		for multi_arg in owner_type_id_pairs.into_vec() {
			let (owner, type_id) = multi_arg.into_tuple();

			batch_balance.push(self.balance_of(&owner, &type_id));
		}

		batch_balance.into()
	}

	#[view(isApprovedForAll)]
	fn is_approval_for_all(&self, owner: Address, operator: Address) -> bool {
		self.get_is_approved(&operator, &owner)
	}

	// private

	fn is_valid_type_id(&self, type_id: &BigUint) -> bool {
		type_id > &0 && type_id <= &self.get_last_valid_type_id()
	}

	fn is_valid_nft_id(&self, type_id: &BigUint, nft_id: &BigUint) -> bool {
		self.is_valid_type_id(type_id)
			&& nft_id > &0
			&& nft_id <= &self.get_last_valid_nft_id_for_type(type_id)
	}

	fn increase_balance(&self, owner: &Address, type_id: &BigUint, amount: &BigUint) {
		let mut balance = self.balance_of(owner, type_id);
		balance += amount;
		self.set_balance(owner, type_id, &balance);
	}

	fn decrease_balance(&self, owner: &Address, type_id: &BigUint, amount: &BigUint) {
		let mut balance = self.balance_of(owner, type_id);
		balance -= amount;
		self.set_balance(owner, type_id, &balance);
	}

	fn set_balance(&self, owner: &Address, type_id: &BigUint, amount: &BigUint) {
		let mut balance_mapper = self.get_balance_mapper(owner);
		balance_mapper.insert(type_id.clone(), amount.clone());
	}

	fn try_reserve_fungible(
		&self,
		owner: &Address,
		type_id: &BigUint,
		amount: &BigUint,
	) -> SCResult<()> {
		let balance = self.balance_of(&owner, &type_id);

		require!(amount > &0, "Must transfer more than 0");
		require!(amount <= &balance, "Not enough balance for id");

		self.decrease_balance(&owner, &type_id, &amount);

		Ok(())
	}

	fn try_reserve_non_fungible(
		&self,
		owner: &Address,
		type_id: &BigUint,
		nft_id: &BigUint,
	) -> SCResult<()> {
		require!(
			self.is_valid_nft_id(&type_id, &nft_id),
			"Token type-id pair is not valid"
		);
		require!(
			&self.get_token_owner(&type_id, &nft_id) == owner,
			"_from_ is not the owner of the token"
		);

		let amount = BigUint::from(1u32);
		self.decrease_balance(&owner, &type_id, &amount);
		self.set_token_owner(&type_id, nft_id, &Address::zero());

		Ok(())
	}

	/// Range is inclusive for both `start` and `end`
	fn set_owner_for_range(
		&self,
		type_id: &BigUint,
		start: &BigUint,
		end: &BigUint,
		owner: &Address,
	) {
		let big_uint_one = BigUint::from(1u32);
		let mut nft_id = start.clone();

		while &nft_id <= end {
			self.set_token_owner(&type_id, &nft_id, &owner);
			nft_id += &big_uint_one;
		}
	}

	fn peform_async_call_single_transfer(
		&self,
		from: Address,
		to: Address,
		type_id: BigUint,
		value: BigUint,
		data: &[u8],
	) {
		let mut serializer = HexCallDataSerializer::new(ON_ERC_RECEIVED_ENDPOINT_NAME);
		serializer.push_argument_bytes(&self.blockchain().get_caller().as_bytes());
		serializer.push_argument_bytes(&from.as_bytes());
		serializer.push_argument_bytes(&type_id.to_bytes_be());
		serializer.push_argument_bytes(&value.to_bytes_be());
		serializer.push_argument_bytes(data);

		self.set_pending_transfer(
			&self.blockchain().get_tx_hash(),
			&Transfer {
				from,
				to: to.clone(),
				type_ids: [type_id].to_vec(),
				values: [value].to_vec(),
			},
		);

		self.send()
			.async_call_raw(&to, &BigUint::zero(), serializer.as_slice());
	}

	fn peform_async_call_batch_transfer(
		&self,
		from: Address,
		to: Address,
		type_ids: &[BigUint],
		values: &[BigUint],
		data: &[u8],
	) {
		let type_ids_encoded = top_encode_to_vec_or_panic(&type_ids);
		let values_encoded = top_encode_to_vec_or_panic(&values);

		let mut serializer = HexCallDataSerializer::new(ON_ERC_BATCH_RECEIVED_ENDPOINT_NAME);
		serializer.push_argument_bytes(&self.blockchain().get_caller().as_bytes());
		serializer.push_argument_bytes(&from.as_bytes());
		serializer.push_argument_bytes(type_ids_encoded.as_slice());
		serializer.push_argument_bytes(values_encoded.as_slice());
		serializer.push_argument_bytes(data);

		self.set_pending_transfer(
			&self.blockchain().get_tx_hash(),
			&Transfer {
				from,
				to: to.clone(),
				type_ids: type_ids.to_vec(),
				values: values.to_vec(),
			},
		);

		self.send()
			.async_call_raw(&to, &BigUint::zero(), serializer.as_slice());
	}

	// callbacks

	#[callback_raw]
	fn callback_raw(&self, #[var_args] result: AsyncCallResult<VarArgs<BoxedBytes>>) {
		let is_transfer_accepted = result.is_ok();

		let tx_hash = self.blockchain().get_tx_hash();
		let pending_transfer = self.get_pending_transfer(&tx_hash);
		let type_ids = &pending_transfer.type_ids;
		let values = &pending_transfer.values;

		// in case of success, transfer to the intended address, otherwise, return tokens to original owner
		let dest_address = if is_transfer_accepted {
			&pending_transfer.to
		} else {
			&pending_transfer.from
		};

		for (type_id, value) in type_ids.iter().zip(values.iter()) {
			if self.is_fungible(type_id) {
				let amount = value;
				self.increase_balance(&dest_address, &type_id, &amount);
			} else {
				let nft_id = value;
				let amount = BigUint::from(1u32);
				self.increase_balance(&dest_address, &type_id, &amount);
				self.set_token_owner(&type_id, nft_id, &dest_address);
			}
		}

		// for success => emit event, single transfer if len() == 1, batch transfer otherwise

		self.clear_pending_transfer(&tx_hash);
	}

	// storage

	// map for address -> type_id -> amount

	#[storage_mapper("balanceOf")]
	fn get_balance_mapper(&self, owner: &Address) -> MapMapper<Self::Storage, BigUint, BigUint>;

	// token owner
	// for non-fungible

	#[view(getTokenOwner)]
	#[storage_get("tokenOwner")]
	fn get_token_owner(&self, type_id: &BigUint, nft_id: &BigUint) -> Address;

	#[storage_set("tokenOwner")]
	fn set_token_owner(&self, type_id: &BigUint, nft_id: &BigUint, owner: &Address);

	// token creator

	#[view(getTokenTypeCreator)]
	#[storage_get("tokenTypeCreator")]
	fn get_token_type_creator(&self, type_id: &BigUint) -> Address;

	#[storage_set("tokenTypeCreator")]
	fn set_token_type_creator(&self, type_id: &BigUint, creator: &Address);

	// token type uri

	#[view(getTokenTypeUri)]
	#[storage_get("tokenTypeUri")]
	fn get_token_type_uri(&self, type_id: &BigUint) -> BoxedBytes;

	#[storage_set("tokenTypeUri")]
	fn set_token_type_uri(&self, type_id: &BigUint, uri: &BoxedBytes);

	// check if a token is fungible

	#[view(isFungible)]
	#[storage_get("isFungible")]
	fn is_fungible(&self, type_id: &BigUint) -> bool;

	#[storage_set("isFungible")]
	fn set_is_fungible(&self, type_id: &BigUint, is_fungible: bool);

	// last valid id

	#[storage_get("lastValidTypeId")]
	fn get_last_valid_type_id(&self) -> BigUint;

	#[storage_set("lastValidTypeId")]
	fn set_last_valid_type_id(&self, last_valid_type_id: &BigUint);

	#[storage_get("lastValidTokenIdForType")]
	fn get_last_valid_nft_id_for_type(&self, type_id: &BigUint) -> BigUint;

	#[storage_set("lastValidTokenIdForType")]
	fn set_last_valid_nft_id_for_type(&self, type_id: &BigUint, last_valid_nft_id: &BigUint);

	// check if an operator is approved. Default is false.

	#[storage_get("isApproved")]
	fn get_is_approved(&self, operator: &Address, owner: &Address) -> bool;

	#[storage_set("isApproved")]
	fn set_is_approved(&self, operator: &Address, owner: &Address, is_approved: bool);

	// transfer data for callbacks, in case a revert is needed

	#[storage_get("pendingTransfer")]
	fn get_pending_transfer(&self, tx_hash: &H256) -> Transfer<BigUint>;

	#[storage_set("pendingTransfer")]
	fn set_pending_transfer(&self, tx_hash: &H256, pending_transfer: &Transfer<BigUint>);

	#[storage_clear("pendingTransfer")]
	fn clear_pending_transfer(&self, tx_hash: &H256);

	// Events

	/*
	#[event("transfer")]
	fn transfer_single_event(&self, operator: &Address, from: &Address, to: &Address, id: &BigUint, amount: &BigUint);

	#[event("approve")]
	fn transfer_batch_event(&self, operator: &Address, from: &Address, to: &Address, ids: &Vec<BigUint>, amounts: &Vec<BigUint>);

	#[legacy_event("0x0000000000000000000000000000000000000000000000000000000000000003")]
	fn approval_for_all_event(&self, owner: &Address, operator: &Address, approved: bool);

	#[legacy_event("0x0000000000000000000000000000000000000000000000000000000000000004")]
	fn uri_event(&self, uri: &[u8], id: &BigUint); // maybe use &str
	*/
}
