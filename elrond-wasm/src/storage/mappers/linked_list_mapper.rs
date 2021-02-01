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

/// A doubly-linked list with owned nodes.
///
/// The `LinkedListMapper` allows pushing and popping elements at either end
/// in constant time.
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
	fn build_node_id_named_key(&self, name: &[u8], node_id: u32) -> BoxedBytes {
		BoxedBytes::from_concat(&[self.main_key.as_slice(), name, &node_id.to_be_bytes()])
	}

	fn build_name_key(&self, name: &[u8]) -> BoxedBytes {
		BoxedBytes::from_concat(&[self.main_key.as_slice(), name])
	}

	fn get_u32(&self, name: &[u8]) -> u32 {
		storage_get(self.api.clone(), self.build_name_key(name).as_slice())
	}

	fn set_u32(&mut self, name: &[u8], value: u32) {
		storage_set(
			self.api.clone(),
			self.build_name_key(name).as_slice(),
			&value,
		);
	}

	fn get_len(&self) -> u32 {
		self.get_u32(LEN_IDENTIFIER)
	}

	fn set_len(&mut self, len: u32) {
		self.set_u32(LEN_IDENTIFIER, len);
	}

	fn get_new_key(&self) -> u32 {
		self.get_u32(NEW_KEY_IDENTIFIER)
	}

	fn generate_new_node_id(&mut self) -> u32 {
		let new_key = self.get_new_key() + 1;
		self.set_u32(NEW_KEY_IDENTIFIER, new_key);
		new_key
	}

	fn get_front(&self) -> u32 {
		self.get_u32(FRONT_IDENTIFIER)
	}

	fn set_front(&mut self, node_id: u32) {
		self.set_u32(FRONT_IDENTIFIER, node_id);
	}

	fn get_back(&self) -> u32 {
		self.get_u32(BACK_IDENTIFIER)
	}

	fn set_back(&mut self, node_id: u32) {
		self.set_u32(BACK_IDENTIFIER, node_id);
	}

	fn get_node(&self, node_id: u32) -> Node {
		storage_get(
			self.api.clone(),
			self.build_node_id_named_key(NODE_IDENTIFIER, node_id)
				.as_slice(),
		)
	}

	fn set_node(&mut self, node_id: u32, item: Node) {
		storage_set(
			self.api.clone(),
			self.build_node_id_named_key(NODE_IDENTIFIER, node_id)
				.as_slice(),
			&item,
		);
	}

	fn clear_node(&mut self, node_id: u32) {
		storage_set(
			self.api.clone(),
			self.build_node_id_named_key(NODE_IDENTIFIER, node_id)
				.as_slice(),
			&BoxedBytes::empty(),
		);
	}

	fn get_value(&self, node_id: u32) -> T {
		storage_get(
			self.api.clone(),
			self.build_node_id_named_key(VALUE_IDENTIFIER, node_id)
				.as_slice(),
		)
	}

	fn get_value_option(&self, node_id: u32) -> Option<T> {
		if node_id == NULL_ENTRY {
			return None;
		}
		Some(self.get_value(node_id))
	}

	fn set_value(&mut self, node_id: u32, value: &T) {
		storage_set(
			self.api.clone(),
			self.build_node_id_named_key(VALUE_IDENTIFIER, node_id)
				.as_slice(),
			value,
		)
	}

	fn clear_value(&mut self, node_id: u32) {
		storage_set(
			self.api.clone(),
			self.build_node_id_named_key(VALUE_IDENTIFIER, node_id)
				.as_slice(),
			&BoxedBytes::empty(),
		)
	}

	/// Returns `true` if the `LinkedList` is empty.
	///
	/// This operation should compute in *O*(1) time.
	pub fn is_empty(&self) -> bool {
		self.get_len() == 0
	}

	/// Returns the length of the `LinkedList`.
	///
	/// This operation should compute in *O*(1) time.
	pub fn len(&self) -> usize {
		self.get_len() as usize
	}

	/// Appends an element to the back of a list
	/// and returns the node id of the newly added node.
	///
	/// This operation should compute in *O*(1) time.
	pub(crate) fn push_back_node_id(&mut self, elt: &T) -> u32 {
		let new_node_id = self.generate_new_node_id();
		let len = self.get_len();
		let mut previous = NULL_ENTRY;
		if len == 0 {
			self.set_front(new_node_id);
		} else {
			let back = self.get_back();
			let mut back_node = self.get_node(back);
			back_node.next = new_node_id;
			previous = back;
			self.set_node(back, back_node);
		}
		self.set_node(
			new_node_id,
			Node {
				previous,
				next: NULL_ENTRY,
			},
		);
		self.set_back(new_node_id);
		self.set_value(new_node_id, &elt);
		self.set_len(len + 1);
		new_node_id
	}

	/// Appends an element to the back of a list.
	///
	/// This operation should compute in *O*(1) time.
	pub fn push_back(&mut self, elt: T) {
		let _ = self.push_back_node_id(&elt);
	}

	/// Adds an element first in the list.
	///
	/// This operation should compute in *O*(1) time.
	pub fn push_front(&mut self, elt: T) {
		let new_node_id = self.generate_new_node_id();
		let len = self.get_len();
		let mut next = NULL_ENTRY;
		if len == 0 {
			self.set_back(new_node_id);
		} else {
			let front = self.get_front();
			let mut front_node = self.get_node(front);
			front_node.previous = new_node_id;
			next = front;
			self.set_node(front, front_node);
		}
		self.set_node(
			new_node_id,
			Node {
				previous: NULL_ENTRY,
				next,
			},
		);
		self.set_front(new_node_id);
		self.set_value(new_node_id, &elt);
		self.set_len(len + 1);
	}

	/// Provides a copy to the front element, or `None` if the list is
	/// empty.
	pub fn front(&self) -> Option<T> {
		self.get_value_option(self.get_front())
	}

	/// Provides a copy to the back element, or `None` if the list is
	/// empty.
	pub fn back(&self) -> Option<T> {
		self.get_value_option(self.get_back())
	}

	/// Removes the last element from a list and returns it, or `None` if
	/// it is empty.
	///
	/// This operation should compute in *O*(1) time.
	pub fn pop_back(&mut self) -> Option<T> {
		self.remove_by_node_id(self.get_back())
	}

	/// Removes the first element and returns it, or `None` if the list is
	/// empty.
	///
	/// This operation should compute in *O*(1) time.
	pub fn pop_front(&mut self) -> Option<T> {
		self.remove_by_node_id(self.get_front())
	}

	/// Removes element with the given node id and returns it, or `None` if the list is
	/// empty.
	/// Note: has undefined behavior if there's no node with the given node id in the list
	///
	/// This operation should compute in *O*(1) time.
	pub(crate) fn remove_by_node_id(&mut self, node_id: u32) -> Option<T> {
		if node_id == NULL_ENTRY {
			return None;
		}
		let node = self.get_node(node_id);

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

		self.clear_node(node_id);
		let removed_value = self.get_value(node_id);
		self.clear_value(node_id);
		self.set_len(self.get_len() - 1);
		Some(removed_value)
	}

	/// Provides a forward iterator.
	pub fn iter(&self) -> Iter<SA, T> {
		Iter::new(self)
	}
}

/// An iterator over the elements of a `LinkedListMapper`.
///
/// This `struct` is created by [`LinkedListMapper::iter()`]. See its
/// documentation for more.
pub struct Iter<'a, SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode + 'static,
{
	node_id: u32,
	linked_list: &'a LinkedListMapper<SA, T>,
}

impl<'a, SA, T> Iter<'a, SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode + 'static,
{
	fn new(linked_list: &'a LinkedListMapper<SA, T>) -> Iter<'a, SA, T> {
		Iter {
			node_id: linked_list.get_front(),
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
		let current_node_id = self.node_id;
		if current_node_id == NULL_ENTRY {
			return None;
		}
		self.node_id = self.linked_list.get_node(current_node_id).next;
		Some(self.linked_list.get_value(current_node_id))
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
		let v: Vec<T> = self.iter().collect();
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

#[cfg(test)]
mod test {
	use super::{BoxedBytes, LinkedListMapper, Vec};
	use elrond_wasm_debug::TxContext;

	fn create_list() -> LinkedListMapper<TxContext, u64> {
		LinkedListMapper::new(TxContext::dummy(), BoxedBytes::from_concat(&[b"my_list"]))
	}

	struct Entry {
		node_id: u32,
		previous: u32,
		next: u32,
		value: u32,
	}

	struct ListState {
		entries: Vec<Entry>,
		front: u32,
		back: u32,
		new: u32,
		len: u32,
	}

	fn extract_list_state(list: &LinkedListMapper<TxContext, u64>) -> ListState {
		let mut state = ListState {
			entries: Vec::new(),
			front: list.get_front(),
			back: list.get_back(),
			new: list.get_new_key(),
			len: list.get_len(),
		};
		let mut ids = Vec::new();
		let mut node_id = list.get_front();
		/*
		while node_id != NULL_ENTRY {
			ids.push_back(node_id);
			node = list;
			//...
		}*/
		state
	}

	fn check_list_nodes(list: &LinkedListMapper<TxContext, u64>, nodes: Vec<Entry>) {}

	#[test]
	fn test_list_internals() {
		let mut list = create_list();
		let range = 40..45;
		range.for_each(|value| list.push_back(value));
		let processed: Vec<u64> = list.iter().map(|val| val + 10).collect();
		let expected: Vec<u64> = (50..55).collect();
		assert_eq!(processed, expected);
	}
}
