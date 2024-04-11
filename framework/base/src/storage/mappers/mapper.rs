use crate::{api::StorageMapperApi, storage::StorageKey};

pub trait StorageMapper<'a, SA>: 'static
where
    SA: StorageMapperApi<'a>,
{
    /// Will be called automatically by the `#[storage_mapper]` annotation generated code.
    fn new(base_key: StorageKey<'a, SA>) -> Self;
}

pub trait StorageClearable {
    /// Clears all the entries owned by the storage.
    fn clear(&mut self);
}
