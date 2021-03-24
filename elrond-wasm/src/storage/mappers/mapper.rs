use crate::api::{ErrorApi, StorageReadApi, StorageWriteApi};
use crate::types::BoxedBytes;

pub trait StorageMapper<SA>: 'static
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
{
	/// Will be called automatically by the `#[storage_mapper]` annotation generated code.
	fn new(api: SA, key: BoxedBytes) -> Self;
}

pub trait StorageClearable {
	/// Clears all the entries owned by the storage.
	fn clear(&mut self);
}
