use super::{StorageClearable, StorageMapper};
use crate::abi::{TypeAbi, TypeDescriptionContainer, TypeName};
use crate::api::{EndpointFinishApi, ErrorApi, StorageReadApi, StorageWriteApi};
use crate::io::EndpointResult;
use crate::storage::{storage_get, storage_set};
use crate::types::{BoxedBytes, MultiResultVec};
use alloc::vec::Vec;
use core::marker::PhantomData;
use elrond_codec::elrond_codec_derive::{
	TopDecode, TopDecodeOrDefault, TopEncode, TopEncodeOrDefault,
};
use elrond_codec::{DecodeDefault, EncodeDefault, TopDecode, TopEncode};

const NULL_ENTRY: u32 = 0;
const INFO_IDENTIFIER: &[u8] = b".info";
const NODE_IDENTIFIER: &[u8] = b".node_links";
const VALUE_IDENTIFIER: &[u8] = b".value";

#[derive(TopEncode, TopDecode, PartialEq, Clone, Copy)]
pub struct Node {
	pub previous: u32,
	pub next: u32,
}

#[derive(TopEncodeOrDefault, TopDecodeOrDefault, PartialEq, Clone, Copy)]
pub struct LinkedListMapperInfo {
	pub len: u32,
	pub front: u32,
	pub back: u32,
	pub new: u32,
}

impl EncodeDefault for LinkedListMapperInfo {
	fn is_default(&self) -> bool {
		self.len == 0
	}
}

impl DecodeDefault for LinkedListMapperInfo {
	fn default() -> Self {
		Self {
			len: 0,
			front: 0,
			back: 0,
			new: 0,
		}
	}
}

impl LinkedListMapperInfo {
	pub fn generate_new_node_id(&mut self) -> u32 {
		self.new += 1;
		self.new
	}
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

impl<SA, T> StorageClearable for LinkedListMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode,
{
	fn clear(&mut self) {
		let info = self.get_info();
		let mut node_id = info.front;
		while node_id != NULL_ENTRY {
			let node = self.get_node(node_id);
			self.clear_node(node_id);
			self.clear_value(node_id);
			node_id = node.next;
		}
		self.set_info(LinkedListMapperInfo::default());
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

	fn get_info(&self) -> LinkedListMapperInfo {
		storage_get(
			self.api.clone(),
			self.build_name_key(INFO_IDENTIFIER).as_slice(),
		)
	}

	fn set_info(&mut self, value: LinkedListMapperInfo) {
		storage_set(
			self.api.clone(),
			self.build_name_key(INFO_IDENTIFIER).as_slice(),
			&value,
		);
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
		self.get_info().len == 0
	}

	/// Returns the length of the `LinkedList`.
	///
	/// This operation should compute in *O*(1) time.
	pub fn len(&self) -> usize {
		self.get_info().len as usize
	}

	/// Appends an element to the back of a list
	/// and returns the node id of the newly added node.
	///
	/// This operation should compute in *O*(1) time.
	pub(crate) fn push_back_node_id(&mut self, elt: &T) -> u32 {
		let mut info = self.get_info();
		let new_node_id = info.generate_new_node_id();
		let mut previous = NULL_ENTRY;
		if info.len == 0 {
			info.front = new_node_id;
		} else {
			let back = info.back;
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
		info.back = new_node_id;
		self.set_value(new_node_id, &elt);
		info.len += 1;
		self.set_info(info);
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
		let mut info = self.get_info();
		let new_node_id = info.generate_new_node_id();
		let mut next = NULL_ENTRY;
		if info.len == 0 {
			info.back = new_node_id;
		} else {
			let front = info.front;
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
		info.front = new_node_id;
		self.set_value(new_node_id, &elt);
		info.len += 1;
		self.set_info(info);
	}

	/// Provides a copy to the front element, or `None` if the list is
	/// empty.
	pub fn front(&self) -> Option<T> {
		self.get_value_option(self.get_info().front)
	}

	/// Provides a copy to the back element, or `None` if the list is
	/// empty.
	pub fn back(&self) -> Option<T> {
		self.get_value_option(self.get_info().back)
	}

	/// Removes the last element from a list and returns it, or `None` if
	/// it is empty.
	///
	/// This operation should compute in *O*(1) time.
	pub fn pop_back(&mut self) -> Option<T> {
		self.remove_by_node_id(self.get_info().back)
	}

	/// Removes the first element and returns it, or `None` if the list is
	/// empty.
	///
	/// This operation should compute in *O*(1) time.
	pub fn pop_front(&mut self) -> Option<T> {
		self.remove_by_node_id(self.get_info().front)
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

		let mut info = self.get_info();
		if node.previous == NULL_ENTRY {
			info.front = node.next;
		} else {
			let mut previous = self.get_node(node.previous);
			previous.next = node.next;
			self.set_node(node.previous, previous);
		}

		if node.next == NULL_ENTRY {
			info.back = node.previous;
		} else {
			let mut next = self.get_node(node.next);
			next.previous = node.previous;
			self.set_node(node.next, next);
		}

		self.clear_node(node_id);
		let removed_value = self.get_value(node_id);
		self.clear_value(node_id);
		info.len -= 1;
		self.set_info(info);
		Some(removed_value)
	}

	/// Provides a forward iterator.
	pub fn iter(&self) -> Iter<SA, T> {
		Iter::new(self)
	}

	/// Runs several checks in order to verify that both forwards and backwards iteration
	/// yields the same node entries and that the number of items in the list is correct.
	/// Used for unit testing.
	///
	/// This operation should compute in *O*(n) time.
	pub fn check_internal_consistency(&self) -> bool {
		let info = self.get_info();
		let mut front = info.front;
		let mut back = info.back;
		if info.len == 0 {
			// if the list is empty, both ends should point to null entries
			if front != NULL_ENTRY {
				return false;
			}
			if back != NULL_ENTRY {
				return false;
			}
			true
		} else {
			// if the list is non-empty, both ends should point to non-null entries
			if front == NULL_ENTRY {
				return false;
			}
			if back == NULL_ENTRY {
				return false;
			}

			// the node before the first and the one after the last should both be null
			if self.get_node(front).previous != NULL_ENTRY {
				return false;
			}
			if self.get_node(back).next != NULL_ENTRY {
				return false;
			}

			// iterate forwards
			let mut forwards = Vec::new();
			while front != NULL_ENTRY {
				forwards.push(front);
				front = self.get_node(front).next;
			}
			if forwards.len() != info.len as usize {
				return false;
			}

			// iterate backwards
			let mut backwards = Vec::new();
			while back != NULL_ENTRY {
				backwards.push(back);
				back = self.get_node(back).previous;
			}
			if backwards.len() != info.len as usize {
				return false;
			}

			// check that both iterations match element-wise
			let backwards_reversed: Vec<u32> = backwards.iter().rev().cloned().collect();
			if forwards != backwards_reversed {
				return false;
			}

			// check that the node IDs are unique
			forwards.sort_unstable();
			forwards.dedup();
			if forwards.len() != info.len as usize {
				return false;
			}
			true
		}
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
			node_id: linked_list.get_info().front,
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
impl<SA, T> EndpointResult for LinkedListMapper<SA, T>
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
