pub use super::linked_list_mapper::Iter;
use super::{LinkedListMapper, StorageMapper};
use crate::abi::{TypeAbi, TypeDescriptionContainer, TypeName};
use crate::api::{EndpointFinishApi, ErrorApi, StorageReadApi, StorageWriteApi};
use crate::io::EndpointResult;
use crate::storage::{storage_get, storage_set};
use crate::types::{BoxedBytes, MultiResultVec};
use alloc::vec::Vec;
use elrond_codec::{TopDecode, TopEncode};

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

pub fn top_encode_to_vec<T: TopEncode>(obj: &T) -> Vec<u8> {
	let mut bytes = Vec::<u8>::new();
	obj.top_encode(&mut bytes).unwrap();
	bytes
}

impl<SA, T> SetMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode,
{
	fn build_named_value_key(&self, name: &[u8], value: &T) -> BoxedBytes {
		let bytes = top_encode_to_vec(&value);
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

	pub fn is_empty(&self) -> bool {
		self.linked_list_mapper.is_empty()
	}

	pub fn len(&self) -> usize {
		self.linked_list_mapper.len()
	}

	pub fn contains(&self, value: &T) -> bool {
		self.get_node_id(value) != NULL_ENTRY
	}

	pub fn insert(&mut self, value: T) -> bool {
		if self.contains(&value) {
			return false;
		}
		let new_node_id = self.linked_list_mapper.push_back_node_id(&value);
		self.set_node_id(&value, new_node_id);
		true
	}

	pub fn remove(&mut self, value: &T) -> bool {
		let node_id = self.get_node_id(value);
		if node_id == NULL_ENTRY {
			return false;
		}
		self.linked_list_mapper.remove_by_node_id(node_id);
		self.clear_node_id(value);
		true
	}

	pub fn iter(&self) -> Iter<SA, T> {
		self.linked_list_mapper.iter()
	}
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, FA, T> EndpointResult<FA> for SetMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	FA: EndpointFinishApi + Clone + 'static,
	T: TopEncode + TopDecode + EndpointResult<FA>,
{
	fn finish(&self, api: FA) {
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
