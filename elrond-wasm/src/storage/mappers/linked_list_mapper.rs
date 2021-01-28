use super::StorageMapper;
use crate::abi::{TypeAbi, TypeDescriptionContainer, TypeName};
use crate::api::{EndpointFinishApi, ErrorApi, StorageReadApi, StorageWriteApi};
use crate::io::EndpointResult;
use crate::storage::{storage_get, storage_set};
use crate::types::{BoxedBytes, MultiResultVec};
use alloc::vec::Vec;
use core::marker::PhantomData;
use elrond_codec::elrond_codec_derive::{TopDecode, TopEncode};
use elrond_codec::{TopDecode, TopEncode};

const NULL_ENTRY: u32 = 0;
const LEN_IDENTIFIER: &[u8] = b".len";
const NEW_KEY_IDENTIFIER: &[u8] = b".new";
const FRONT_IDENTIFIER: &[u8] = b".front";
const BACK_IDENTIFIER: &[u8] = b".back";
const NODE_IDENTIFIER: &[u8] = b".node";
const VALUE_IDENTIFIER: &[u8] = b".value";

#[derive(TopEncode, TopDecode, PartialEq, Clone, Copy)]
pub struct Node {
	pub previous: u32,
	pub next: u32,
}

/// Manages a doubly-linked list of items
pub struct LinkedListMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode + 'static,
{
	api: SA,
	main_key: BoxedBytes,
	_phantom: core::marker::PhantomData<T>,
}

impl<SA, T> StorageMapper<SA> for LinkedListMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode,
{
	fn new(api: SA, main_key: BoxedBytes) -> Self {
		LinkedListMapper {
			api,
			main_key,
			_phantom: PhantomData,
		}
	}
}

impl<SA, T> LinkedListMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode,
{
	fn build_index_named_key(&self, name: &[u8], index: u32) -> BoxedBytes {
		BoxedBytes::from_concat(&[self.main_key.as_slice(), name, &index.to_be_bytes()])
	}

	fn build_name_key(&self, name: &[u8]) -> BoxedBytes {
		BoxedBytes::from_concat(&[self.main_key.as_slice(), name])
	}

	fn get_u32(&self, name: &[u8]) -> u32 {
		storage_get(self.api.clone(), self.build_name_key(name).as_slice())
	}

	fn set_u32(&self, name: &[u8], value: u32) {
		storage_set(
			self.api.clone(),
			self.build_name_key(name).as_slice(),
			&value,
		);
	}

	fn get_len(&self) -> u32 {
		self.get_u32(LEN_IDENTIFIER)
	}

	fn set_len(&self, len: u32) {
		self.set_u32(LEN_IDENTIFIER, len);
	}

	fn generate_new_index(&self) -> u32 {
		let new_key = self.get_u32(NEW_KEY_IDENTIFIER) + 1;
		self.set_u32(NEW_KEY_IDENTIFIER, new_key);
		new_key
	}

	fn get_front(&self) -> u32 {
		let front = self.get_u32(FRONT_IDENTIFIER);
		front
	}

	fn set_front(&self, index: u32) {
		self.set_u32(FRONT_IDENTIFIER, index);
	}

	fn get_back(&self) -> u32 {
		self.get_u32(BACK_IDENTIFIER)
	}

	fn set_back(&self, index: u32) {
		self.set_u32(BACK_IDENTIFIER, index);
	}

	fn get_node(&self, index: u32) -> Node {
		storage_get(
			self.api.clone(),
			self.build_index_named_key(NODE_IDENTIFIER, index)
				.as_slice(),
		)
	}

	fn set_node(&self, index: u32, item: Node) {
		storage_set(
			self.api.clone(),
			self.build_index_named_key(NODE_IDENTIFIER, index)
				.as_slice(),
			&item,
		);
	}

	fn clear_node(&self, index: u32) {
		storage_set(
			self.api.clone(),
			self.build_index_named_key(NODE_IDENTIFIER, index)
				.as_slice(),
			&BoxedBytes::empty(),
		);
	}

	fn get_value(&self, index: u32) -> T {
		storage_get(
			self.api.clone(),
			self.build_index_named_key(VALUE_IDENTIFIER, index)
				.as_slice(),
		)
	}

	fn get_value_option(&self, index: u32) -> Option<T> {
		if index == NULL_ENTRY {
			return None;
		}
		Some(self.get_value(index))
	}

	fn set_value(&self, index: u32, value: &T) {
		storage_set(
			self.api.clone(),
			self.build_index_named_key(VALUE_IDENTIFIER, index)
				.as_slice(),
			value,
		)
	}

	fn clear_value(&self, index: u32) {
		storage_set(
			self.api.clone(),
			self.build_index_named_key(VALUE_IDENTIFIER, index)
				.as_slice(),
			&BoxedBytes::empty(),
		)
	}

	pub fn is_empty(&self) -> bool {
		self.get_len() == 0
	}

	pub fn len(&self) -> usize {
		self.get_len() as usize
	}

	pub(crate) fn push_back_indexed(&mut self, elt: &T) -> u32 {
		let new_index = self.generate_new_index();
		let len = self.get_len();
		let mut previous = NULL_ENTRY;
		if len == 0 {
			self.set_front(new_index);
		} else {
			let back = self.get_back();
			let mut back_node = self.get_node(back);
			back_node.next = new_index;
			previous = back;
			self.set_node(back, back_node);
		}
		self.set_node(
			new_index,
			Node {
				previous,
				next: NULL_ENTRY,
			},
		);
		self.set_back(new_index);
		self.set_value(new_index, &elt);
		self.set_len(len + 1);
		new_index
	}

	pub fn push_back(&mut self, elt: T) {
		let _ = self.push_back_indexed(&elt);
	}

	pub fn push_front(&mut self, elt: T) {
		let new_index = self.generate_new_index();
		let len = self.get_len();
		let mut next = NULL_ENTRY;
		if len == 0 {
			self.set_back(new_index);
		} else {
			let front = self.get_front();
			let mut front_node = self.get_node(front);
			front_node.previous = new_index;
			next = front;
			self.set_node(front, front_node);
		}
		self.set_node(
			new_index,
			Node {
				previous: NULL_ENTRY,
				next,
			},
		);
		self.set_front(new_index);
		self.set_value(new_index, &elt);
		self.set_len(len + 1);
	}

	pub fn front(&self) -> Option<T> {
		self.get_value_option(self.get_front())
	}

	pub fn back(&self) -> Option<T> {
		self.get_value_option(self.get_back())
	}

	pub fn pop_back(&mut self) -> Option<T> {
		self.remove_at_index(self.get_back())
	}

	pub fn pop_front(&mut self) -> Option<T> {
		self.remove_at_index(self.get_front())
	}

	pub(crate) fn remove_at_index(&self, index: u32) -> Option<T> {
		if index == NULL_ENTRY {
			return None;
		}
		let node = self.get_node(index);

		if node.previous == NULL_ENTRY {
			self.set_front(node.next);
		} else {
			let mut previous = self.get_node(node.previous);
			previous.next = node.next;
			self.set_node(node.previous, previous);
		}

		if node.next == NULL_ENTRY {
			self.set_back(node.previous);
		} else {
			let mut next = self.get_node(node.next);
			next.previous = node.previous;
			self.set_node(node.next, next);
		}

		self.clear_node(index);
		let removed_value = self.get_value(index);
		self.clear_value(index);
		self.set_len(self.get_len() - 1);
		Some(removed_value)
	}

	pub fn iter(&self) -> Iter<SA, T> {
		Iter::new(self)
	}

	pub fn load_as_vec(&self) -> Vec<T> {
		self.iter().collect()
	}
}

pub struct Iter<'a, SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode + 'static,
{
	index: u32,
	linked_list: &'a LinkedListMapper<SA, T>,
}

impl<'a, SA, T> Iter<'a, SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode + 'static,
{
	fn new(linked_list: &'a LinkedListMapper<SA, T>) -> Iter<'a, SA, T> {
		Iter {
			index: linked_list.get_front(),
			linked_list,
		}
	}
}

impl<'a, SA, T> Iterator for Iter<'a, SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode + 'static,
{
	type Item = T;

	#[inline]
	fn next(&mut self) -> Option<T> {
		let current_index = self.index;
		if current_index == NULL_ENTRY {
			return None;
		}
		self.index = self.linked_list.get_node(current_index).next;
		Some(self.linked_list.get_value(current_index))
	}
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, FA, T> EndpointResult<FA> for LinkedListMapper<SA, T>
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
impl<SA, T> TypeAbi for LinkedListMapper<SA, T>
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
