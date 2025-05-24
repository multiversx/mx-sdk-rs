use crate::{api::StorageMapperApi, storage::StorageKey, types::ManagedAddress};
pub trait StorageMapper<SA>: 'static
where
    SA: StorageMapperApi,
{
    /// Will be called automatically by the `#[storage_mapper]` annotation generated code.
    fn new(base_key: StorageKey<SA>) -> Self;
}

pub trait StorageMapperFromAddress<SA>: 'static
where
    SA: StorageMapperApi,
{
    /// Will be called automatically by the `#[storage_mapper_from_address]`
    /// annotation generated code.
    fn new_from_address(address: ManagedAddress<SA>, base_key: StorageKey<SA>) -> Self;
}

pub trait StorageClearable {
    /// Clears all the entries owned by the storage.
    fn clear(&mut self);
}
