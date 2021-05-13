use super::StorageMapper;
use crate::abi::{TypeAbi, TypeDescriptionContainer, TypeName};
use crate::api::{EndpointFinishApi, ErrorApi, StorageReadApi, StorageWriteApi};
use crate::io::EndpointResult;
use crate::storage::{storage_get, storage_set};
use crate::types::BoxedBytes;
use core::marker::PhantomData;
use elrond_codec::{TopDecode, TopEncode};

/// Manages a single serializable item in storage.
pub struct SingleValueMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode + 'static,
{
	api: SA,
	key: BoxedBytes,
	_phantom: core::marker::PhantomData<T>,
}

impl<SA, T> StorageMapper<SA> for SingleValueMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode,
{
	fn new(api: SA, key: BoxedBytes) -> Self {
		SingleValueMapper {
			api,
			key,
			_phantom: PhantomData,
		}
	}
}

impl<SA, T> SingleValueMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode,
{
	/// Retrieves current value from storage.
	pub fn get(&self) -> T {
		storage_get(self.api.clone(), self.key.as_slice())
	}

	/// Saves argument to storage.
	pub fn set(&self, new_value: &T) {
		storage_set(self.api.clone(), self.key.as_slice(), new_value);
	}

	/// Returns whether the storage managed by this is empty
	pub fn is_empty(&self) -> bool {
		self.api.storage_load_len(self.key.as_slice()) == 0
	}

	/// Clears the storage for this mapper
	pub fn clear(&self) {
		self.api.storage_store_slice_u8(self.key.as_slice(), &[]);
	}

	/// Syntactic sugar, to more compactly express a get, update and set in one line.
	/// Takes whatever lies in storage, apples the given closure and saves the final value back to storage.
	/// Propagates the return value of the given function.
	pub fn update<R, F: FnOnce(&mut T) -> R>(&self, f: F) -> R {
		let mut value = self.get();
		let result = f(&mut value);
		self.set(&value);
		result
	}
}

impl<SA, T> EndpointResult for SingleValueMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode + EndpointResult,
{
	type DecodeAs = T::DecodeAs;

	fn finish<FA>(&self, api: FA)
	where
		FA: EndpointFinishApi + Clone + 'static,
	{
		self.get().finish(api);
	}
}

impl<SA, T> TypeAbi for SingleValueMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode + TypeAbi,
{
	fn type_name() -> TypeName {
		T::type_name()
	}

	fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
		T::provide_type_descriptions(accumulator)
	}
}
