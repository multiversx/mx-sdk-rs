use core::marker::PhantomData;

use crate::codec::{
    multi_encode_iter_or_handle_err, CodecFrom, EncodeErrorHandler, TopEncodeMulti,
    TopEncodeMultiOutput,
};

use super::StorageMapper;
use crate::{
    abi::{TypeAbi, TypeName},
    api::StorageMapperApi,
    storage::{storage_get, storage_get_len, storage_set, StorageKey},
    types::{ManagedAddress, ManagedType, ManagedVec, MultiValueEncoded},
};

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
    SA: StorageMapperApi,
{
    _phantom_api: PhantomData<SA>,
    base_key: StorageKey<SA>,
}

impl<SA> StorageMapper<SA> for UserMapper<SA>
where
    SA: StorageMapperApi,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        UserMapper {
            _phantom_api: PhantomData,
            base_key,
        }
    }
}

impl<SA> UserMapper<SA>
where
    SA: StorageMapperApi,
{
    fn get_user_id_key(&self, address: &ManagedAddress<SA>) -> StorageKey<SA> {
        let mut user_id_key = self.base_key.clone();
        user_id_key.append_bytes(ADDRESS_TO_ID_SUFFIX);
        user_id_key.append_item(address);
        user_id_key
    }

    fn get_user_address_key(&self, id: usize) -> StorageKey<SA> {
        let mut user_address_key = self.base_key.clone();
        user_address_key.append_bytes(ID_TO_ADDRESS_SUFFIX);
        user_address_key.append_item(&id);
        user_address_key
    }

    fn get_user_count_key(&self) -> StorageKey<SA> {
        let mut user_count_key = self.base_key.clone();
        user_count_key.append_bytes(COUNT_SUFFIX);
        user_count_key
    }

    /// Yields the user id for a given address.
    /// Will return 0 if the address is not known to the contract.
    pub fn get_user_id(&self, address: &ManagedAddress<SA>) -> usize {
        storage_get(self.get_user_id_key(address).as_ref())
    }

    fn set_user_id(&self, address: &ManagedAddress<SA>, id: usize) {
        storage_set(self.get_user_id_key(address).as_ref(), &id);
    }

    /// Yields the user address for a given id, if the id is valid.
    pub fn get_user_address(&self, id: usize) -> Option<ManagedAddress<SA>> {
        let key = self.get_user_address_key(id);
        // TODO: optimize, storage_load_managed_buffer_len is currently called twice

        if storage_get_len(key.as_ref()) > 0 {
            Some(storage_get(key.as_ref()))
        } else {
            None
        }
    }

    /// Yields the user address for a given id.
    /// Will cause a deserialization error if the id is invalid.
    pub fn get_user_address_unchecked(&self, id: usize) -> ManagedAddress<SA> {
        storage_get(self.get_user_address_key(id).as_ref())
    }

    /// Yields the user address for a given id, if the id is valid.
    /// Otherwise returns the zero address (0x000...)
    pub fn get_user_address_or_zero(&self, id: usize) -> ManagedAddress<SA> {
        let key = self.get_user_address_key(id);
        // TODO: optimize, storage_load_managed_buffer_len is currently called twice
        if storage_get_len(key.as_ref()) > 0 {
            storage_get(key.as_ref())
        } else {
            ManagedAddress::zero()
        }
    }

    fn set_user_address(&self, id: usize, address: &ManagedAddress<SA>) {
        storage_set(self.get_user_address_key(id).as_ref(), address);
    }

    /// Number of users.
    pub fn get_user_count(&self) -> usize {
        storage_get(self.get_user_count_key().as_ref())
    }

    fn set_user_count(&self, user_count: usize) {
        storage_set(self.get_user_count_key().as_ref(), &user_count);
    }

    /// Yields the user id for a given address, or creates a new user id if there isn't one.
    /// Will safely keep the user count in sync.
    pub fn get_or_create_user(&self, address: &ManagedAddress<SA>) -> usize {
        let mut user_id = self.get_user_id(address);
        if user_id == 0 {
            let next_user_count = self.get_user_count() + 1;
            self.set_user_count(next_user_count);
            user_id = next_user_count;
            self.set_user_id(address, user_id);
            self.set_user_address(user_id, address);
        }
        user_id
    }

    /// Tries to insert a number of addresses.
    /// Calls a lambda function for each, with the new user id and whether of nor the user was already present.
    pub fn get_or_create_users<AddressIter, F>(
        &self,
        address_iter: AddressIter,
        mut user_id_lambda: F,
    ) where
        AddressIter: Iterator<Item = ManagedAddress<SA>>,
        F: FnMut(usize, bool),
    {
        let mut user_count = self.get_user_count();
        for address in address_iter {
            let user_id = self.get_user_id(&address);
            if user_id > 0 {
                user_id_lambda(user_id, false);
            } else {
                user_count += 1;
                let new_user_id = user_count;
                self.set_user_id(&address, new_user_id);
                self.set_user_address(new_user_id, &address);
                user_id_lambda(new_user_id, true);
            }
        }
        self.set_user_count(user_count);
    }

    /// Loads all addresses from storage and places them in a ManagedVec.
    /// Can easily consume a lot of gas.
    pub fn get_all_addresses(&self) -> ManagedVec<SA, ManagedAddress<SA>> {
        let user_count = self.get_user_count();
        let mut result = ManagedVec::new();
        for i in 1..=user_count {
            result.push(self.get_user_address_or_zero(i));
        }
        result
    }
}

/// Behaves like a MultiResultVec<Address> when an endpoint result,
/// and lists all users addresses.
impl<SA> TopEncodeMulti for UserMapper<SA>
where
    SA: StorageMapperApi,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        let all_addresses = self.get_all_addresses();
        multi_encode_iter_or_handle_err(all_addresses.into_iter(), output, h)
    }
}

impl<SA> CodecFrom<UserMapper<SA>> for MultiValueEncoded<SA, ManagedAddress<SA>> where
    SA: StorageMapperApi
{
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA> TypeAbi for UserMapper<SA>
where
    SA: StorageMapperApi,
{
    fn type_name() -> TypeName {
        crate::abi::type_name_variadic::<ManagedAddress<SA>>()
    }

    fn is_variadic() -> bool {
        true
    }
}
