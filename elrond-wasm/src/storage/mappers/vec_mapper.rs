use super::StorageMapper;
use crate::abi::{TypeAbi, TypeDescriptionContainer, TypeName};
use crate::api::{EndpointFinishApi, ErrorApi, StorageReadApi, StorageWriteApi};
use crate::io::EndpointResult;
use crate::storage::{storage_get, storage_set};
use crate::types::{BoxedBytes, MultiResultVec};
use alloc::vec::Vec;
use core::marker::PhantomData;
use elrond_codec::{TopDecode, TopEncode};

/// The item count gets stored under what would be index 0.
const COUNT_KEY_INDEX: usize = 0;

fn compute_key(prefix: &[u8], index: usize) -> BoxedBytes {
	// cast to u32, so it also works correctly in debug mode on x64 architectures
	// would be nice to go via the framework serialization, but it currently has a little overhead over this
	let index_bytes = (index as u32).to_be_bytes();
	BoxedBytes::from_concat(&[prefix, &index_bytes[..]])
}

/// Manages a list of items of the same type.
/// Saves each of the items under a separate key in storage.
/// To produce each individual key, it concatenates the main key with a serialized 4-byte index.
/// Indexes start from 1, instead of 0. (We avoid 0-value indexes to prevent confusion between an uninitialized variable and zero.)
/// It also stores the count separately, at what would be index 0.
/// The count is always kept in sync automatically.
pub struct VecMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode + 'static,
{
	api: SA,
	main_key: BoxedBytes,
	count: usize,
	_phantom: core::marker::PhantomData<T>,
}

impl<SA, T> StorageMapper<SA> for VecMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode,
{
	fn new(api: SA, main_key: BoxedBytes) -> Self {
		let count_key = compute_key(main_key.as_slice(), COUNT_KEY_INDEX);
		let count: usize = storage_get(api.clone(), count_key.as_slice());
		VecMapper {
			api,
			main_key,
			count,
			_phantom: PhantomData,
		}
	}
}

impl<SA, T> VecMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode,
{
	fn get_key(&self, index: usize) -> BoxedBytes {
		compute_key(self.main_key.as_slice(), index)
	}

	fn save_count(&self) {
		storage_set(
			self.api.clone(),
			self.get_key(COUNT_KEY_INDEX).as_slice(),
			&self.count,
		);
	}

	/// Add one item at the end of the list.
	pub fn push(&mut self, item: &T) {
		self.count += 1;
		storage_set(self.api.clone(), self.get_key(self.count).as_slice(), item);
		self.save_count();
	}

	/// Adds multiple items at the end of the list.
	/// Cheaper than multiple `push`-es because the count only gets updated once at the end.
	pub fn extend_from_slice(&mut self, items: &[T]) {
		for item in items {
			self.count += 1;
			storage_set(self.api.clone(), self.get_key(self.count).as_slice(), item);
		}
		self.save_count();
	}

	/// Get item at index from storage.
	/// Index must be valid (1 <= index <= count).
	pub fn get(&self, index: usize) -> T {
		if index == 0 || index > self.count {
			self.api.signal_error(&b"index out of range"[..]);
		}
		storage_get(self.api.clone(), self.get_key(index).as_slice())
	}

	/// Get item at index from storage.
	/// If index is valid (1 <= index <= count), returns value at index,
	/// else calls lambda given as argument.
	/// The lambda only gets called lazily if the index is not valid.
	pub fn get_or_else<F: FnOnce() -> T>(self, index: usize, or_else: F) -> T {
		if index == 0 || index > self.count {
			or_else()
		} else {
			storage_get(self.api.clone(), self.get_key(index).as_slice())
		}
	}

	/// Number of items managed by the mapper.
	pub fn len(&self) -> usize {
		self.count
	}

	/// Loads all items from storage and places them in a Vec.
	/// Can easily consume a lot of gas.
	pub fn load_as_vec(&self) -> Vec<T> {
		let mut result = Vec::with_capacity(self.count);
		for i in 1..=self.count {
			result.push(self.get(i));
		}
		result
	}
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, FA, T> EndpointResult<FA> for VecMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	FA: EndpointFinishApi + Clone + 'static,
	T: TopEncode + TopDecode + EndpointResult<FA>,
{
	fn finish(&self, api: FA) {
		let v = self.load_as_vec();
		MultiResultVec::<T>::from(v).finish(api);
	}
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, T> TypeAbi for VecMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode + TypeAbi,
{
	fn type_name() -> TypeName {
		crate::types::MultiResultVec::<T>::type_name()
	}

	fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
		T::provide_type_descriptions(accumulator);
	}

	fn is_multi_arg_or_result() -> bool {
		true
	}
}
