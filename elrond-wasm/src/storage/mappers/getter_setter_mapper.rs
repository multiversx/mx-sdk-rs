use super::StorageMapper;
use crate::abi::{TypeAbi, TypeDescriptionContainer, TypeName};
use crate::api::{EndpointFinishApi, ErrorApi, StorageReadApi, StorageWriteApi};
use crate::io::EndpointResult;
use crate::storage::{storage_get, storage_set};
use crate::types::BoxedBytes;
use crate::BorrowedMutStorage;
use core::marker::PhantomData;
use elrond_codec::{TopDecode, TopEncode};

/// Provides a the get and set methods for a serializable item in storage.
pub struct GetterSetterMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode + 'static,
{
	api: SA,
	key: BoxedBytes,
	phantom_data: core::marker::PhantomData<T>,
}

impl<SA, T> StorageMapper<SA> for GetterSetterMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode,
{
	fn new(api: SA, key: BoxedBytes) -> Self {
		Self {
			api,
			key,
			phantom_data: PhantomData,
		}
	}
}

impl<SA, T> GetterSetterMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode,
{
	// Get the value from storage.
	pub fn get(&self) -> T {
		storage_get(self.api.clone(), self.key.as_slice())
	}

	// Get the value as mutable. See BorrowedMutStorage.
	pub fn get_mut(&self) -> BorrowedMutStorage<SA, T> {
		<BorrowedMutStorage<SA, T>>::with_generated_key(
			self.api.clone(),
			self.key.as_slice().into(),
		)
	}

	/// Saves the value to storage.
	pub fn set(&self, value: T) {
		storage_set(self.api.clone(), self.key.as_slice(), &value);
	}
}

impl<SA, FA, T> EndpointResult<FA> for GetterSetterMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	FA: EndpointFinishApi + 'static,
	T: TopEncode + TopDecode + EndpointResult<FA>,
{
	fn finish(&self, api: FA) {
		self.get().finish(api);
	}
}

impl<SA, T> TypeAbi for GetterSetterMapper<SA, T>
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
