use super::{StorageClearable, StorageMapper};
use crate::abi::{TypeAbi, TypeDescriptionContainer, TypeName};
use crate::api::{EndpointFinishApi, ErrorApi, StorageReadApi, StorageWriteApi};
use crate::io::EndpointResult;
use crate::storage::{storage_get, storage_set};
use crate::types::{BoxedBytes, MultiResultVec};
use alloc::vec::Vec;
use core::{marker::PhantomData, usize};
use elrond_codec::{TopDecode, TopEncode};

const ITEM_SUFFIX: &[u8] = b".item";
const LEN_SUFFIX: &[u8] = b".len";

fn compute_item_key(prefix: &[u8], index: usize) -> BoxedBytes {
	// cast to u32, so it also works correctly in debug mode on x64 architectures
	// would be nice to go via the framework serialization, but it currently has a little overhead over this
	let index_bytes = (index as u32).to_be_bytes();
	BoxedBytes::from_concat(&[prefix, ITEM_SUFFIX, &index_bytes[..]])
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
	_phantom: core::marker::PhantomData<T>,
}

impl<SA, T> StorageMapper<SA> for VecMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode,
{
	fn new(api: SA, main_key: BoxedBytes) -> Self {
		VecMapper {
			api,
			main_key,
			_phantom: PhantomData,
		}
	}
}

impl<SA, T> StorageClearable for VecMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode,
{
	fn clear(&mut self) {
		self.clear();
	}
}

impl<SA, T> VecMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode,
{
	fn item_key(&self, index: usize) -> BoxedBytes {
		compute_item_key(self.main_key.as_slice(), index)
	}

	fn len_key(&self) -> BoxedBytes {
		BoxedBytes::from_concat(&[self.main_key.as_slice(), LEN_SUFFIX])
	}

	fn save_count(&self, new_len: usize) {
		storage_set(self.api.clone(), self.len_key().as_slice(), &new_len);
	}

	/// Number of items managed by the mapper.
	pub fn len(&self) -> usize {
		storage_get(self.api.clone(), self.len_key().as_slice())
	}

	/// True if no items present in the mapper.
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Add one item at the end of the list.
	/// Returns the index of the newly inserted item, which is also equal to the new number of elements.
	pub fn push(&mut self, item: &T) -> usize {
		let mut len = self.len();
		len += 1;
		storage_set(self.api.clone(), self.item_key(len).as_slice(), item);
		self.save_count(len);
		len
	}

	/// Adds multiple items at the end of the list.
	/// Cheaper than multiple `push`-es because the count only gets updated once at the end.
	/// Returns the index of the last inserted item, which is also equal to the new number of elements.
	pub fn extend_from_slice(&mut self, items: &[T]) -> usize {
		let mut len = self.len();
		for item in items {
			len += 1;
			storage_set(self.api.clone(), self.item_key(len).as_slice(), item);
		}
		self.save_count(len);
		len
	}

	/// Get item at index from storage.
	/// Index must be valid (1 <= index <= count).
	pub fn get(&self, index: usize) -> T {
		if index == 0 || index > self.len() {
			self.api.signal_error(&b"index out of range"[..]);
		}
		self.get_unchecked(index)
	}

	/// Get item at index from storage.
	/// There are no restrictions on the index,
	/// calling for an invalid index will simply return the zero-value.
	pub fn get_unchecked(&self, index: usize) -> T {
		storage_get(self.api.clone(), self.item_key(index).as_slice())
	}

	/// Get item at index from storage.
	/// If index is valid (1 <= index <= count), returns value at index,
	/// else calls lambda given as argument.
	/// The lambda only gets called lazily if the index is not valid.
	pub fn get_or_else<F: FnOnce() -> T>(self, index: usize, or_else: F) -> T {
		if index == 0 || index > self.len() {
			or_else()
		} else {
			self.get_unchecked(index)
		}
	}

	/// Checks whether or not there is anything in storage at index.
	/// There are no restrictions on the index,
	/// calling for an invalid index will simply return `true`.
	pub fn item_is_empty_unchecked(&self, index: usize) -> bool {
		self.api.storage_load_len(self.item_key(index).as_slice()) == 0
	}

	/// Checks whether or not there is anything ins storage at index.
	/// Index must be valid (1 <= index <= count).
	pub fn item_is_empty(&self, index: usize) -> bool {
		if index == 0 || index > self.len() {
			self.api.signal_error(&b"index out of range"[..]);
		}
		self.item_is_empty_unchecked(index)
	}

	/// Get item at index from storage.
	/// Index must be valid (1 <= index <= count).
	pub fn set(&self, index: usize, item: &T) {
		if index == 0 || index > self.len() {
			self.api.signal_error(&b"index out of range"[..]);
		}
		self.set_unchecked(index, item);
	}

	/// Keeping `set_unchecked` private on purpose, so developers don't write out of index limits by accident.
	fn set_unchecked(&self, index: usize, item: &T) {
		storage_set(self.api.clone(), self.item_key(index).as_slice(), item);
	}

	/// Clears item at index from storage.
	/// Index must be valid (1 <= index <= count).
	pub fn clear_entry(&self, index: usize) {
		if index == 0 || index > self.len() {
			self.api.signal_error(&b"index out of range"[..]);
		}
		self.clear_entry_unchecked(index)
	}

	/// Clears item at index from storage.
	/// There are no restrictions on the index,
	/// calling for an invalid index will simply do nothing.
	pub fn clear_entry_unchecked(&self, index: usize) {
		self.api
			.storage_store_slice_u8(self.item_key(index).as_slice(), &[]);
	}

	/// Loads all items from storage and places them in a Vec.
	/// Can easily consume a lot of gas.
	pub fn load_as_vec(&self) -> Vec<T> {
		let len = self.len();
		let mut result = Vec::with_capacity(len);
		for i in 1..=len {
			result.push(self.get(i));
		}
		result
	}

	/// Deletes all contents form storage and sets count to 0.
	/// Can easily consume a lot of gas.
	pub fn clear(&mut self) {
		let len = self.len();
		for i in 1..=len {
			self.api
				.storage_store_slice_u8(self.item_key(i).as_slice(), &[]);
		}
		self.save_count(0);
	}
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, T> EndpointResult for VecMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode + EndpointResult,
{
	type DecodeAs = MultiResultVec<T::DecodeAs>;

	fn finish<FA>(&self, api: FA)
	where
		FA: EndpointFinishApi + Clone + 'static,
	{
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
