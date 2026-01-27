use core::marker::PhantomData;

use super::{
    StorageMapper, StorageMapperFromAddress,
    source::{CurrentStorage, StorageAddress},
};
use crate::{
    api::{ErrorApiImpl, StorageMapperApi},
    storage::{StorageKey, storage_clear, storage_set},
    types::{ManagedAddress, ManagedType},
};

const ID_SUFFIX: &str = "addrId";
const ADDRESS_SUFFIX: &str = "addr";
const LAST_ID_SUFFIX: &str = "lastId";

const UNKNOWN_ADDR_ERR_MSG: &str = "Unknown address";

pub type AddressId = u64;
pub const NULL_ID: AddressId = 0;

/// A specialized bidirectional mapper between addresses and auto-incrementing numeric IDs.
///
/// # Storage Layout
///
/// The `AddressToIdMapper` maintains bidirectional mappings with sequential ID assignment:
///
/// 1. **Address to ID mapping**:
///    - `base_key + "addr" + encoded_address` → assigned ID (u64)
///
/// 2. **ID to address mapping**:
///    - `base_key + "addrId" + id` → address
///
/// 3. **ID counter**:
///    - `base_key + "lastId"` → highest assigned ID (for generating new IDs)
///
/// # ID Assignment
///
/// - IDs start from 1 and increment sequentially
/// - `NULL_ID` (0) represents "no ID assigned" or "not found"
/// - Once an ID is assigned, it remains associated with that address until explicitly removed
/// - Removed IDs are not reused (IDs only increment, never decrement)
///
/// # Main Operations
///
/// - **Auto-insert**: `get_id_or_insert(address)` - Gets ID or assigns new one if not exists. O(1).
/// - **Strict insert**: `insert_new(address)` - Assigns ID only if address is new, errors if exists. O(1).
/// - **Lookup ID**: `get_id(address)` - Returns ID for address (0 if not found). O(1).
/// - **Lookup Address**: `get_address(id)` - Returns address for ID. O(1).
/// - **Contains**: `contains_id(id)` - Checks if ID is assigned. O(1).
/// - **Remove by ID**: `remove_by_id(id)` - Removes mapping by ID, returns address. O(1).
/// - **Remove by Address**: `remove_by_address(address)` - Removes mapping by address, returns ID. O(1).
/// - **Counter**: `get_last_id()` - Returns the highest assigned ID.
///
/// # Trade-offs
///
/// - **Pros**: Sequential IDs are compact and predictable; auto-increment simplifies ID management;
///   O(1) bidirectional lookup; no duplicate addresses.
/// - **Cons**: IDs are never reused (gaps after removal); ID overflow at u64::MAX (unlikely but possible);
///   slightly less flexible than generic `BiDiMapper`.
///
/// # Comparison with BiDiMapper
///
/// - **AddressToIdMapper**: Specialized for addresses; auto-incrementing IDs; single type for IDs (u64)
/// - **BiDiMapper**: Generic; manual ID/value assignment; supports any types for both sides
///
/// # Use Cases
///
/// - User registration systems (address → user ID)
/// - Whitelist/participant management with sequential numbering
/// - Address indexing for efficient iteration
/// - Mapping external addresses to internal compact IDs
/// - Any scenario where addresses need numeric identifiers
///
/// # Example
///
/// ```rust,ignore
/// # use multiversx_sc::storage::mappers::{StorageMapper, AddressToIdMapper};
/// # use multiversx_sc::types::ManagedAddress;
/// # fn example<SA: multiversx_sc::api::StorageMapperApi>(
/// #     addr1: ManagedAddress<SA>,
/// #     addr2: ManagedAddress<SA>,
/// #     addr3: ManagedAddress<SA>
/// # ) {
/// # let mapper = AddressToIdMapper::<SA>::new(
/// #     multiversx_sc::storage::StorageKey::new(&b"users"[..])
/// # );
/// // Auto-assign IDs (get or create)
/// let id1 = mapper.get_id_or_insert(&addr1);  // Returns 1
/// let id2 = mapper.get_id_or_insert(&addr2);  // Returns 2
/// let id1_again = mapper.get_id_or_insert(&addr1);  // Returns 1 (existing)
///
/// assert_eq!(id1, 1);
/// assert_eq!(id2, 2);
/// assert_eq!(id1_again, 1);
/// assert_eq!(mapper.get_last_id(), 2);
///
/// // Strict insert (errors if address already exists)
/// let id3 = mapper.insert_new(&addr3);  // Returns 3
/// assert_eq!(id3, 3);
/// // mapper.insert_new(&addr1);  // Would error: "Address already registered"
///
/// // Bidirectional lookup
/// assert_eq!(mapper.get_id(&addr1), 1);
/// assert_eq!(mapper.get_address(1), Some(addr1.clone()));
///
/// // Check existence
/// assert!(mapper.contains_id(2));
/// assert_eq!(mapper.get_id(&addr2), 2);  // Returns 2
///
/// // Non-zero lookup (errors if not found)
/// let id = mapper.get_id_non_zero(&addr1);  // Returns 1
/// assert_eq!(id, 1);
/// // mapper.get_id_non_zero(&unknown_addr);  // Would error: "Unknown address"
///
/// // Remove by address
/// let removed_id = mapper.remove_by_address(&addr2);
/// assert_eq!(removed_id, 2);
/// assert_eq!(mapper.get_id(&addr2), 0);  // Now returns NULL_ID
/// assert!(!mapper.contains_id(2));
///
/// // Remove by ID
/// let removed_addr = mapper.remove_by_id(1);
/// assert_eq!(removed_addr, Some(addr1.clone()));
/// assert_eq!(mapper.get_id(&addr1), 0);
///
/// // Note: next inserted address gets ID 4, not 2 (IDs never reused)
/// # }
/// ```
pub struct AddressToIdMapper<SA, A = CurrentStorage>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
{
    _phantom_api: PhantomData<SA>,
    address: A,
    base_key: StorageKey<SA>,
}

impl<SA> StorageMapper<SA> for AddressToIdMapper<SA>
where
    SA: StorageMapperApi,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        AddressToIdMapper {
            _phantom_api: PhantomData,
            address: CurrentStorage,
            base_key,
        }
    }
}

impl<SA> StorageMapperFromAddress<SA> for AddressToIdMapper<SA, ManagedAddress<SA>>
where
    SA: StorageMapperApi,
{
    fn new_from_address(address: ManagedAddress<SA>, base_key: StorageKey<SA>) -> Self {
        AddressToIdMapper {
            _phantom_api: PhantomData,
            address,
            base_key,
        }
    }
}

impl<SA, A> AddressToIdMapper<SA, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
{
    pub fn contains_id(&self, id: AddressId) -> bool {
        let key = self.id_to_address_key(id);
        self.address.address_storage_get_len(key.as_ref()) != 0
    }

    pub fn get_id(&self, address: &ManagedAddress<SA>) -> AddressId {
        let key = self.address_to_id_key(address);
        self.address.address_storage_get(key.as_ref())
    }

    pub fn get_id_non_zero(&self, address: &ManagedAddress<SA>) -> AddressId {
        let id = self.get_id(address);
        if id == NULL_ID {
            SA::error_api_impl().signal_error(UNKNOWN_ADDR_ERR_MSG.as_bytes());
        }

        id
    }

    pub fn get_address(&self, id: AddressId) -> Option<ManagedAddress<SA>> {
        let key = self.id_to_address_key(id);
        if self.address.address_storage_get_len(key.as_ref()) == 0 {
            return None;
        }

        let addr = self.address.address_storage_get(key.as_ref());
        Some(addr)
    }

    fn id_to_address_key(&self, id: AddressId) -> StorageKey<SA> {
        let mut item_key = self.base_key.clone();
        item_key.append_bytes(ID_SUFFIX.as_bytes());
        item_key.append_item(&id);

        item_key
    }

    fn address_to_id_key(&self, address: &ManagedAddress<SA>) -> StorageKey<SA> {
        let mut item_key = self.base_key.clone();
        item_key.append_bytes(ADDRESS_SUFFIX.as_bytes());
        item_key.append_item(address);

        item_key
    }

    fn last_id_key(&self) -> StorageKey<SA> {
        let mut item_key = self.base_key.clone();
        item_key.append_bytes(LAST_ID_SUFFIX.as_bytes());

        item_key
    }

    pub fn get_last_id(&self) -> AddressId {
        self.address
            .address_storage_get(self.last_id_key().as_ref())
    }
}

impl<SA> AddressToIdMapper<SA, CurrentStorage>
where
    SA: StorageMapperApi,
{
    pub fn get_id_or_insert(&self, address: &ManagedAddress<SA>) -> AddressId {
        let current_id = self
            .address
            .address_storage_get(self.address_to_id_key(address).as_ref());
        if current_id != 0 {
            return current_id;
        }

        self.insert_address(address)
    }

    pub fn insert_new(&self, address: &ManagedAddress<SA>) -> AddressId {
        let existing_id = self.get_id(address);
        if existing_id != NULL_ID {
            SA::error_api_impl().signal_error(b"Address already registered");
        }

        self.insert_address(address)
    }

    pub fn remove_by_id(&self, id: AddressId) -> Option<ManagedAddress<SA>> {
        let address = self.get_address(id)?;
        self.remove_entry(id, &address);

        Some(address)
    }

    pub fn remove_by_address(&self, address: &ManagedAddress<SA>) -> AddressId {
        let current_id = self.get_id(address);
        if current_id != NULL_ID {
            self.remove_entry(current_id, address);
        }

        current_id
    }

    fn insert_address(&self, address: &ManagedAddress<SA>) -> AddressId {
        let new_id = self.get_last_id() + 1;
        storage_set(self.address_to_id_key(address).as_ref(), &new_id);
        storage_set(self.id_to_address_key(new_id).as_ref(), address);

        self.set_last_id(new_id);

        new_id
    }

    fn set_last_id(&self, last_id: AddressId) {
        if last_id == 0 {
            SA::error_api_impl().signal_error(b"ID Overflow");
        }

        storage_set(self.last_id_key().as_ref(), &last_id);
    }

    fn remove_entry(&self, id: AddressId, address: &ManagedAddress<SA>) {
        storage_clear(self.address_to_id_key(address).as_ref());
        storage_clear(self.id_to_address_key(id).as_ref());
    }
}
