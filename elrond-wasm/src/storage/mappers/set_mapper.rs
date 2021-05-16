pub use super::linked_list_mapper::Iter;
use super::{LinkedListMapper, StorageClearable, StorageMapper};
use crate::abi::{TypeAbi, TypeDescriptionContainer, TypeName};
use crate::api::{EndpointFinishApi, ErrorApi, StorageReadApi, StorageWriteApi};
use crate::io::EndpointResult;
use crate::storage::{storage_get, storage_set};
use crate::types::{BoxedBytes, MultiResultVec};
use alloc::vec::Vec;
use elrond_codec::{top_encode_to_vec, TopDecode, TopEncode};

const NULL_ENTRY: u32 = 0;
const NODE_ID_IDENTIFIER: &[u8] = b".node_id";

pub struct SetMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode + 'static,
{
	api: SA,
	main_key: BoxedBytes,
	linked_list_mapper: LinkedListMapper<SA, T>,
}

impl<SA, T> StorageMapper<SA> for SetMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode,
{
	fn new(api: SA, main_key: BoxedBytes) -> Self {
		SetMapper {
			api: api.clone(),
			main_key: main_key.clone(),
			linked_list_mapper: LinkedListMapper::<SA, T>::new(api, main_key),
		}
	}
}

impl<SA, T> StorageClearable for SetMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode,
{
	fn clear(&mut self) {
		for value in self.linked_list_mapper.iter() {
			self.clear_node_id(&value);
		}
		self.linked_list_mapper.clear();
	}
}

impl<SA, T> SetMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode,
{
	fn build_named_value_key(&self, name: &[u8], value: &T) -> BoxedBytes {
		let bytes = top_encode_to_vec(&value).unwrap();
		BoxedBytes::from_concat(&[self.main_key.as_slice(), name, &bytes])
	}

	fn get_node_id(&self, value: &T) -> u32 {
		storage_get(
			self.api.clone(),
			self.build_named_value_key(NODE_ID_IDENTIFIER, value)
				.as_slice(),
		)
	}

	fn set_node_id(&self, value: &T, node_id: u32) {
		storage_set(
			self.api.clone(),
			self.build_named_value_key(NODE_ID_IDENTIFIER, value)
				.as_slice(),
			&node_id,
		);
	}

	fn clear_node_id(&self, value: &T) {
		storage_set(
			self.api.clone(),
			self.build_named_value_key(NODE_ID_IDENTIFIER, value)
				.as_slice(),
			&BoxedBytes::empty(),
		);
	}

	/// Returns `true` if the set contains no elements.
	pub fn is_empty(&self) -> bool {
		self.linked_list_mapper.is_empty()
	}

	/// Returns the number of elements in the set.
	pub fn len(&self) -> usize {
		self.linked_list_mapper.len()
	}

	/// Returns `true` if the set contains a value.
	pub fn contains(&self, value: &T) -> bool {
		self.get_node_id(value) != NULL_ENTRY
	}

	/// Adds a value to the set.
	///
	/// If the set did not have this value present, `true` is returned.
	///
	/// If the set did have this value present, `false` is returned.
	pub fn insert(&mut self, value: T) -> bool {
		if self.contains(&value) {
			return false;
		}
		let new_node_id = self.linked_list_mapper.push_back_node_id(&value);
		self.set_node_id(&value, new_node_id);
		true
	}

	/// Removes a value from the set. Returns whether the value was
	/// present in the set.
	pub fn remove(&mut self, value: &T) -> bool {
		let node_id = self.get_node_id(value);
		if node_id == NULL_ENTRY {
			return false;
		}
		self.linked_list_mapper.remove_by_node_id(node_id);
		self.clear_node_id(value);
		true
	}

	/// An iterator visiting all elements in arbitrary order.
	/// The iterator element type is `&'a T`.
	pub fn iter(&self) -> Iter<SA, T> {
		self.linked_list_mapper.iter()
	}

	/// Checks the internal consistency of the collection. Used for unit tests.
	pub fn check_internal_consistency(&self) -> bool {
		self.linked_list_mapper.check_internal_consistency()
	}
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, T> EndpointResult for SetMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode + EndpointResult,
{
	type DecodeAs = MultiResultVec<T::DecodeAs>;

	fn finish<FA>(&self, api: FA)
	where
		FA: EndpointFinishApi + Clone + 'static,
	{
		let v: Vec<T> = self.iter().collect();
		MultiResultVec::<T>::from(v).finish(api);
	}
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, T> TypeAbi for SetMapper<SA, T>
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
