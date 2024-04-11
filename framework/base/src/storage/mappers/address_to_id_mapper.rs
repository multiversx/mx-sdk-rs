use core::marker::PhantomData;

use super::{
    set_mapper::{CurrentStorage, StorageAddress},
    StorageMapper,
};
use crate::{
    api::{ErrorApiImpl, StorageMapperApi},
    storage::{storage_clear, storage_set, StorageKey},
    types::{ManagedAddress, ManagedType},
};

static ID_SUFFIX: &[u8] = b"addrId";
static ADDRESS_SUFFIX: &[u8] = b"addr";
static LAST_ID_SUFFIX: &[u8] = b"lastId";

static UNKNOWN_ADDR_ERR_MSG: &[u8] = b"Unknown address";

pub type AddressId = u64;
pub const NULL_ID: AddressId = 0;

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

impl<SA> AddressToIdMapper<SA, ManagedAddress<SA>>
where
    SA: StorageMapperApi,
{
    pub fn new_from_address(address: ManagedAddress<SA>, base_key: StorageKey<SA>) -> Self {
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
            SA::error_api_impl().signal_error(UNKNOWN_ADDR_ERR_MSG);
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
        item_key.append_bytes(ID_SUFFIX);
        item_key.append_item(&id);

        item_key
    }

    fn address_to_id_key(&self, address: &ManagedAddress<SA>) -> StorageKey<SA> {
        let mut item_key = self.base_key.clone();
        item_key.append_bytes(ADDRESS_SUFFIX);
        item_key.append_item(address);

        item_key
    }

    fn last_id_key(&self) -> StorageKey<SA> {
        let mut item_key = self.base_key.clone();
        item_key.append_bytes(LAST_ID_SUFFIX);

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
