use super::StorageMapper;
use crate::abi::{TypeAbi, TypeName};
use crate::api::{EndpointFinishApi, ErrorApi, StorageReadApi, StorageWriteApi};
use crate::io::EndpointResult;
use crate::storage::{storage_get, storage_set};
use crate::types::{Address, BoxedBytes, MultiResultVec};
use alloc::vec::Vec;

const ADDRESS_TO_ID_SUFFIX: &[u8] = b"_address_to_id";
const ID_TO_ADDRESS_SUFFIX: &[u8] = b"_id_to_address";
const COUNT_SUFFIX: &[u8] = b"_count";

/// Very widely used mapper, that manages the users of a smart contract.
/// It holds a bi-directional map, from addresses to ids and viceversa.
/// This is so we can easily iterate over all users, using their ids.
/// Also holds the user count in sync. This is also necessary for iteration.
///
/// This particular implementation of a user mapper doesn't contain any additional
/// user data other than address/id.
///
/// It also doesn't allow removing users. Once in, their ids are reserved forever.
pub struct UserMapper<SA>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
{
	api: SA,
	main_key: BoxedBytes,
}

impl<SA> StorageMapper<SA> for UserMapper<SA>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
{
	fn new(api: SA, main_key: BoxedBytes) -> Self {
		UserMapper { api, main_key }
	}
}

impl<SA> UserMapper<SA>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
{
	fn get_user_id_key(&self, address: &Address) -> BoxedBytes {
		// TODO: build this more elegantly from elrond-codec
		BoxedBytes::from_concat(&[
			self.main_key.as_slice(),
			ADDRESS_TO_ID_SUFFIX,
			address.as_bytes(),
		])
	}

	/// Yields the user id for a given address.
	/// Will return 0 if the address is not known to the contract.
	pub fn get_user_id(&self, address: &Address) -> usize {
		storage_get(self.api.clone(), self.get_user_id_key(address).as_slice())
	}

	fn set_user_id(&self, address: &Address, id: usize) {
		storage_set(
			self.api.clone(),
			self.get_user_id_key(address).as_slice(),
			&id,
		);
	}

	fn get_user_address_key(&self, id: usize) -> BoxedBytes {
		// TODO: build this more elegantly from elrond-codec
		let id_bytes = (id as u32).to_be_bytes();
		BoxedBytes::from_concat(&[
			self.main_key.as_slice(),
			ID_TO_ADDRESS_SUFFIX,
			&id_bytes[..],
		])
	}

	/// Yields the user address for a given id, if the id is valid.
	pub fn get_user_address(&self, id: usize) -> Option<Address> {
		let key = self.get_user_address_key(id);
		// TODO: optimize, storage_load_len is currently called twice
		if self.api.storage_load_len(key.as_slice()) > 0 {
			Some(storage_get(self.api.clone(), key.as_slice()))
		} else {
			None
		}
	}

	/// Yields the user address for a given id.
	/// Will cause a deserialization error if the id is invalid.
	pub fn get_user_address_unchecked(&self, id: usize) -> Address {
		let key = self.get_user_address_key(id);
		storage_get(self.api.clone(), key.as_slice())
	}

	/// Yields the user address for a given id, if the id is valid.
	/// Otherwise returns the zero address (0x000...)
	pub fn get_user_address_or_zero(&self, id: usize) -> Address {
		let key = self.get_user_address_key(id);
		// TODO: optimize, storage_load_len is currently called twice
		if self.api.storage_load_len(key.as_slice()) > 0 {
			storage_get(self.api.clone(), key.as_slice())
		} else {
			Address::zero()
		}
	}

	fn set_user_address(&self, id: usize, address: &Address) {
		storage_set(
			self.api.clone(),
			self.get_user_address_key(id).as_slice(),
			address,
		);
	}

	fn get_user_count_key(&self) -> BoxedBytes {
		BoxedBytes::from_concat(&[self.main_key.as_slice(), COUNT_SUFFIX])
	}

	/// Number of users.
	pub fn get_user_count(&self) -> usize {
		storage_get(self.api.clone(), self.get_user_count_key().as_slice())
	}

	fn set_user_count(&self, user_count: usize) {
		storage_set(
			self.api.clone(),
			self.get_user_count_key().as_slice(),
			&user_count,
		);
	}

	/// Yields the user id for a given address, or creates a new user id if there isn't one.
	/// Will safely keep the user count in sync.
	pub fn get_or_create_user(&self, address: &Address) -> usize {
		let mut user_id = self.get_user_id(&address);
		if user_id == 0 {
			let mut user_count = self.get_user_count();
			user_count += 1;
			self.set_user_count(user_count);
			user_id = user_count;
			self.set_user_id(&address, user_id);
			self.set_user_address(user_id, &address);
		}
		user_id
	}

	/// Tries to insert a number of addresses.
	/// Calls a lambda function for each, with the new user id and whether of nor the user was already present.
	pub fn get_or_create_users<F: FnMut(usize, bool)>(
		&self,
		addresses: &[Address],
		mut user_id_lambda: F,
	) {
		let mut user_count = self.get_user_count();
		for address in addresses {
			let mut user_id = self.get_user_id(&address);
			if user_id > 0 {
				user_id_lambda(user_id, false);
			} else {
				user_count += 1;
				user_id = user_count;
				self.set_user_id(&address, user_id);
				self.set_user_address(user_id, &address);
				user_id_lambda(user_id, true);
			}
		}
		self.set_user_count(user_count);
	}

	/// Loads all addresses from storage and places them in a Vec.
	/// Can easily consume a lot of gas.
	pub fn get_all_addresses(&self) -> Vec<Address> {
		let user_count = self.get_user_count();
		let mut result = Vec::with_capacity(user_count);
		for i in 1..=user_count {
			result.push(self.get_user_address_or_zero(i));
		}
		result
	}
}

/// Behaves like a MultiResultVec<Address> when an endpoint result,
/// and lists all users addresses.
impl<SA> EndpointResult for UserMapper<SA>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
{
	type DecodeAs = MultiResultVec<Address>;

	fn finish<FA>(&self, api: FA)
	where
		FA: EndpointFinishApi + Clone + 'static,
	{
		let addr_vec = self.get_all_addresses();
		MultiResultVec::<Address>::from(addr_vec).finish(api);
	}
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA> TypeAbi for UserMapper<SA>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
{
	fn type_name() -> TypeName {
		crate::types::MultiResultVec::<Address>::type_name()
	}

	fn is_multi_arg_or_result() -> bool {
		true
	}
}
