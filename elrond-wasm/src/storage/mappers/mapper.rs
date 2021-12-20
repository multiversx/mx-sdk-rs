use crate::{
    api::{
        ErrorApi, ManagedTypeApi, StorageReadApi, StorageReadApiImpl, StorageWriteApi,
        StorageWriteApiImpl,
    },
    storage::StorageKey,
};

pub trait StorageMapper<SA>: 'static
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
{
    /// Will be called automatically by the `#[storage_mapper]` annotation generated code.
    fn new(base_key: StorageKey<SA>) -> Self;
}

pub trait StorageClearable {
    /// Clears all the entries owned by the storage.
    fn clear(&mut self);
}
