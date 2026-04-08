use storage_get_from_address::storage_get_len_from_address;

use crate::{
    api::StorageMapperApi,
    codec::TopDecode,
    storage::{StorageKey, storage_get_from_address},
    storage_get, storage_get_len,
    types::{ManagedAddress, ManagedRef, ManagedType},
};

pub trait StorageAddress<SA>
where
    SA: StorageMapperApi,
{
    fn address_storage_get<T: TopDecode>(&self, key: ManagedRef<'_, SA, StorageKey<SA>>) -> T;
    fn address_storage_get_len(&self, key: ManagedRef<'_, SA, StorageKey<SA>>) -> usize;
}

pub struct CurrentStorage;

impl<SA> StorageAddress<SA> for CurrentStorage
where
    SA: StorageMapperApi,
{
    fn address_storage_get<T: TopDecode>(&self, key: ManagedRef<'_, SA, StorageKey<SA>>) -> T {
        storage_get(key)
    }

    fn address_storage_get_len(&self, key: ManagedRef<'_, SA, StorageKey<SA>>) -> usize {
        storage_get_len(key)
    }
}

impl<SA> StorageAddress<SA> for ManagedAddress<SA>
where
    SA: StorageMapperApi,
{
    fn address_storage_get<T: TopDecode>(&self, key: ManagedRef<'_, SA, StorageKey<SA>>) -> T {
        storage_get_from_address(self.as_ref(), key)
    }

    fn address_storage_get_len(&self, key: ManagedRef<'_, SA, StorageKey<SA>>) -> usize {
        storage_get_len_from_address(self.as_ref(), key)
    }
}
