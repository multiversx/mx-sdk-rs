use core::marker::PhantomData;

use crate::{
    abi::TypeAbiFrom,
    codec::{
        EncodeErrorHandler, TopEncodeMulti, TopEncodeMultiOutput, multi_encode_iter_or_handle_err,
    },
};

use super::{
    StorageMapper, StorageMapperFromAddress,
    source::{CurrentStorage, StorageAddress},
};
use crate::{
    abi::{TypeAbi, TypeName},
    api::StorageMapperApi,
    storage::{StorageKey, storage_set},
    types::{ManagedAddress, ManagedType, ManagedVec, MultiValueEncoded},
};

const ADDRESS_TO_ID_SUFFIX: &str = "_address_to_id";
const ID_TO_ADDRESS_SUFFIX: &str = "_id_to_address";
const COUNT_SUFFIX: &str = "_count";

/// A specialized bidirectional mapper for managing smart contract users with auto-incrementing IDs.
///
/// # Storage Layout
///
/// The `UserMapper` maintains three storage patterns for efficient user management:
///
/// 1. **Address to ID mapping**:
///    - `base_key + "_address_to_id" + encoded_address` → user ID (usize)
///
/// 2. **ID to address mapping**:
///    - `base_key + "_id_to_address" + id` → user address
///
/// 3. **User counter**:
///    - `base_key + "_count"` → total number of registered users
///
/// # User ID Assignment
///
/// - IDs start from 1 and increment sequentially (never 0)
/// - ID 0 represents "no user" or "user not found"
/// - Once assigned, user IDs are **permanent** and never reused
/// - No removal functionality - users cannot be deleted once registered
///
/// # Main Operations
///
/// - **Auto-register**: `get_or_create_user(address)` - Gets ID or assigns new one. O(1).
/// - **Batch register**: `get_or_create_users(addresses, callback)` - Registers multiple users efficiently.
/// - **Lookup ID**: `get_user_id(address)` - Returns user ID (0 if not found). O(1).
/// - **Lookup Address**: `get_user_address(id)` - Returns address for ID. O(1).
/// - **Count**: `get_user_count()` - Returns total number of registered users. O(1).
/// - **Bulk Access**: `get_all_addresses()` - Returns all user addresses (expensive). O(n).
///
/// # Trade-offs
///
/// - **Pros**: Sequential IDs enable efficient iteration; permanent user registry; bidirectional lookup;
///   auto-incrementing simplifies user management; built-in user counting.
/// - **Cons**: No user removal; IDs never reused; `get_all_addresses()` can be expensive for large user bases;
///   slightly higher storage overhead than simple address lists.
///
/// # Comparison with AddressToIdMapper
///
/// - **UserMapper**: Specialized for users; includes user count; batch operations; no removal
/// - **AddressToIdMapper**: Generic address mapping; supports removal; no built-in counting
///
/// # Use Cases
///
/// - User registration and management systems
/// - Participant tracking in contracts (staking, voting, etc.)
/// - Whitelist management with sequential user numbering
/// - Any scenario requiring both address-based lookup and iteration by user ID
/// - Loyalty programs or membership systems
///
/// # Example
///
/// ```rust
/// # use multiversx_sc::storage::mappers::{StorageMapper, UserMapper};
/// # use multiversx_sc::types::ManagedAddress;
/// # fn example<SA: multiversx_sc::api::StorageMapperApi>(
/// #     user1: ManagedAddress<SA>,
/// #     user2: ManagedAddress<SA>,
/// #     user3: ManagedAddress<SA>
/// # ) {
/// # let mapper = UserMapper::<SA>::new(
/// #     multiversx_sc::storage::StorageKey::new(&b"users"[..])
/// # );
/// // Register users (auto-assign IDs)
/// let id1 = mapper.get_or_create_user(&user1);  // Returns 1
/// let id2 = mapper.get_or_create_user(&user2);  // Returns 2
/// let id1_again = mapper.get_or_create_user(&user1);  // Returns 1 (existing)
///
/// assert_eq!(id1, 1);
/// assert_eq!(id2, 2);
/// assert_eq!(id1_again, 1);
/// assert_eq!(mapper.get_user_count(), 2);
///
/// // Lookup by address
/// assert_eq!(mapper.get_user_id(&user1), 1);
/// assert_eq!(mapper.get_user_id(&user3), 0);  // Not registered
///
/// // Lookup by ID
/// assert_eq!(mapper.get_user_address(1), Some(user1.clone()));
/// assert_eq!(mapper.get_user_address(999), None);  // Invalid ID
///
/// // Safe address lookup (returns zero address if invalid)
/// let addr = mapper.get_user_address_or_zero(1);
/// assert_eq!(addr, user1);
///
/// let zero_addr = mapper.get_user_address_or_zero(999);
/// assert!(zero_addr.is_zero());
///
/// // Batch registration with callback
/// let addresses = vec![user2.clone(), user3.clone()];
/// mapper.get_or_create_users(addresses.into_iter(), |id, is_new| {
///     if is_new {
///         // Handle new user registration
///     } else {
///         // Handle existing user
///     }
/// });
///
/// assert_eq!(mapper.get_user_count(), 3);
///
/// // Get all users (expensive for large lists)
/// let all_users = mapper.get_all_addresses();
/// assert_eq!(all_users.len(), 3);
///
/// // Note: No removal functionality - users are permanent
/// # }
/// ```
pub struct UserMapper<SA, A = CurrentStorage>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
{
    _phantom_api: PhantomData<SA>,
    address: A,
    base_key: StorageKey<SA>,
}

impl<SA> StorageMapper<SA> for UserMapper<SA>
where
    SA: StorageMapperApi,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        UserMapper {
            _phantom_api: PhantomData,
            address: CurrentStorage,
            base_key,
        }
    }
}

impl<SA> StorageMapperFromAddress<SA> for UserMapper<SA, ManagedAddress<SA>>
where
    SA: StorageMapperApi,
{
    fn new_from_address(address: ManagedAddress<SA>, base_key: StorageKey<SA>) -> Self {
        UserMapper {
            _phantom_api: PhantomData,
            address,
            base_key,
        }
    }
}

impl<SA, A> UserMapper<SA, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
{
    fn get_user_id_key(&self, address: &ManagedAddress<SA>) -> StorageKey<SA> {
        let mut user_id_key = self.base_key.clone();
        user_id_key.append_bytes(ADDRESS_TO_ID_SUFFIX.as_bytes());
        user_id_key.append_item(address);
        user_id_key
    }

    fn get_user_address_key(&self, id: usize) -> StorageKey<SA> {
        let mut user_address_key = self.base_key.clone();
        user_address_key.append_bytes(ID_TO_ADDRESS_SUFFIX.as_bytes());
        user_address_key.append_item(&id);
        user_address_key
    }

    fn get_user_count_key(&self) -> StorageKey<SA> {
        let mut user_count_key = self.base_key.clone();
        user_count_key.append_bytes(COUNT_SUFFIX.as_bytes());
        user_count_key
    }

    /// Yields the user id for a given address.
    /// Will return 0 if the address is not known to the contract.
    pub fn get_user_id(&self, address: &ManagedAddress<SA>) -> usize {
        self.address
            .address_storage_get(self.get_user_id_key(address).as_ref())
    }

    /// Yields the user address for a given id, if the id is valid.
    pub fn get_user_address(&self, id: usize) -> Option<ManagedAddress<SA>> {
        let key = self.get_user_address_key(id);
        // TODO: optimize, storage_load_managed_buffer_len is currently called twice

        if self.address.address_storage_get_len(key.as_ref()) > 0 {
            Some(self.address.address_storage_get(key.as_ref()))
        } else {
            None
        }
    }

    /// Yields the user address for a given id.
    /// Will cause a deserialization error if the id is invalid.
    pub fn get_user_address_unchecked(&self, id: usize) -> ManagedAddress<SA> {
        self.address
            .address_storage_get(self.get_user_address_key(id).as_ref())
    }

    /// Yields the user address for a given id, if the id is valid.
    /// Otherwise returns the zero address (0x000...)
    pub fn get_user_address_or_zero(&self, id: usize) -> ManagedAddress<SA> {
        let key = self.get_user_address_key(id);
        // TODO: optimize, storage_load_managed_buffer_len is currently called twice
        if self.address.address_storage_get_len(key.as_ref()) > 0 {
            self.address.address_storage_get(key.as_ref())
        } else {
            ManagedAddress::zero()
        }
    }

    /// Number of users.
    pub fn get_user_count(&self) -> usize {
        self.address
            .address_storage_get(self.get_user_count_key().as_ref())
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

impl<SA> UserMapper<SA, CurrentStorage>
where
    SA: StorageMapperApi,
{
    fn set_user_id(&self, address: &ManagedAddress<SA>, id: usize) {
        storage_set(self.get_user_id_key(address).as_ref(), &id);
    }

    fn set_user_address(&self, id: usize, address: &ManagedAddress<SA>) {
        storage_set(self.get_user_address_key(id).as_ref(), address);
    }

    fn set_user_count(&self, user_count: usize) {
        storage_set(self.get_user_count_key().as_ref(), &user_count);
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
}

/// Behaves like a MultiResultVec<Address> when an endpoint result,
/// and lists all users addresses.
impl<SA> TopEncodeMulti for UserMapper<SA, CurrentStorage>
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

impl<SA> TypeAbiFrom<UserMapper<SA, CurrentStorage>> for MultiValueEncoded<SA, ManagedAddress<SA>> where
    SA: StorageMapperApi
{
}

impl<SA> TypeAbiFrom<Self> for UserMapper<SA, CurrentStorage> where SA: StorageMapperApi {}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA> TypeAbi for UserMapper<SA, CurrentStorage>
where
    SA: StorageMapperApi,
{
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        crate::abi::type_name_variadic::<ManagedAddress<SA>>()
    }

    fn type_name_rust() -> TypeName {
        crate::abi::type_name_multi_value_encoded::<ManagedAddress<SA>>()
    }

    fn is_variadic() -> bool {
        true
    }
}
