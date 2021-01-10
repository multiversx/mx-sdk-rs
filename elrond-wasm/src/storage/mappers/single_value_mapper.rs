use super::StorageMapper;
use crate::abi::{TypeAbi, TypeDescriptionContainer, TypeName};
use crate::api::{EndpointFinishApi, ErrorApi, StorageReadApi, StorageWriteApi};
use crate::io::EndpointResult;
use crate::storage::{storage_get, storage_set};
use crate::types::BoxedBytes;
use elrond_codec::{TopDecode, TopEncode};

/// Manages a single serializable item in storage.
pub struct SingleValueMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode + 'static,
{
	api: SA,
	key: BoxedBytes,
	pub value: T,
}

impl<SA, T> StorageMapper<SA> for SingleValueMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode,
{
	fn new(api: SA, key: BoxedBytes) -> Self {
		let value: T = storage_get(api.clone(), key.as_slice());
		SingleValueMapper { api, key, value }
	}
}

impl<SA, T> SingleValueMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode,
{
	/// Saves current value from memory to storage.
	pub fn save(&self) {
		storage_set(self.api.clone(), self.key.as_slice(), &self.value);
	}
}

impl<SA, FA, T> EndpointResult<FA> for SingleValueMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	FA: EndpointFinishApi + 'static,
	T: TopEncode + TopDecode + EndpointResult<FA>,
{
	fn finish(&self, api: FA) {
		self.value.finish(api);
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
