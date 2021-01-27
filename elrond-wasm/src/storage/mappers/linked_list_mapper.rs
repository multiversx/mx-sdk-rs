use super::StorageMapper;
use crate::api::{ErrorApi, StorageReadApi, StorageWriteApi};
use crate::storage::{storage_get, storage_set};
use crate::types::{BoxedBytes};
use core::marker::PhantomData;
use elrond_codec::{TopDecode, TopEncode};
use elrond_codec::elrond_codec_derive::{TopDecode, TopEncode};

const NULL_ENTRY : u32 = 0;
const LEN_IDENTIFIER : &[u8] = b".len";
const NEW_KEY_IDENTIFIER : &[u8] = b".new";
const FRONT_IDENTIFIER : &[u8] = b".front";
const BACK_IDENTIFIER : &[u8] = b".back";
const NODE_IDENTIFIER : &[u8] = b"*";
const VALUE_IDENTIFIER : &[u8] = b"&";

/// A unit of balance, usually stake.
/// Contains a description of the source/intent of the funds, together with a balance.
#[derive(TopEncode, TopDecode, PartialEq, Clone, Copy)]
pub struct Node {
    pub previous: u32,
    pub next: u32
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
		let result = LinkedListMapper {
			api,
			main_key,
			_phantom: PhantomData,
		};
		result.set_len(0);
		result.set_front(NULL_ENTRY);
		result.set_back(NULL_ENTRY);
		result
	}
}

impl<SA, T> LinkedListMapper<SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode,
{
	fn build_index_named_key(&self, name : &[u8], index : u32) -> BoxedBytes {
		BoxedBytes::from_concat(&[self.main_key.as_slice(), name, &index.to_be_bytes()])
	}

	fn build_name_key(&self, name : &[u8]) -> BoxedBytes {
		BoxedBytes::from_concat(&[self.main_key.as_slice(), name])
	}

	fn get_u32(&self, name : &[u8]) -> u32 {
		storage_get(self.api.clone(), self.build_name_key(name).as_slice())
	}

	fn set_u32(&self, name : &[u8], value : u32) {
		storage_set(self.api.clone(), self.build_name_key(name).as_slice(), &value);
	}

	fn get_len(&self) -> u32 {
		self.get_u32(LEN_IDENTIFIER)
	}

	fn set_len(&self, len : u32) {
		self.set_u32(LEN_IDENTIFIER, len);
	}

	fn generate_new_key(&self) -> u32 {
		let new_key = self.get_u32(NEW_KEY_IDENTIFIER) + 1;
		self.set_u32(NEW_KEY_IDENTIFIER, new_key);
		new_key
	}

	fn get_front(&self) -> u32 {
		self.get_u32(FRONT_IDENTIFIER)
	}

	fn set_front(&self, index : u32) {
		self.set_u32(FRONT_IDENTIFIER, index);
	}

	fn get_back(&self) -> u32 {
		self.get_u32(BACK_IDENTIFIER)
	}

	fn set_back(&self, index : u32) {
		self.set_u32(BACK_IDENTIFIER, index);
	}

	fn get_node(&self, index : u32) -> Node {
		storage_get(self.api.clone(), self.build_index_named_key(NODE_IDENTIFIER, index).as_slice())
	}

	fn set_node(&self, index : u32, item : Node) {
		storage_set(self.api.clone(), self.build_index_named_key(NODE_IDENTIFIER, index).as_slice(), &item);
	}

	fn clear_node(&self, index : u32) {
		storage_set(self.api.clone(), self.build_index_named_key(NODE_IDENTIFIER, index).as_slice(), &BoxedBytes::empty());
	}

	fn get_value(&self, index : u32) -> T {
		storage_get(self.api.clone(), self.build_index_named_key(VALUE_IDENTIFIER, index).as_slice())
	}

	fn set_value(&self, index : u32, value : T) {
		storage_set(self.api.clone(), self.build_index_named_key(VALUE_IDENTIFIER, index).as_slice(), &value)
	}

	fn clear_value(&self, index : u32) {
		storage_set(self.api.clone(), self.build_index_named_key(VALUE_IDENTIFIER, index).as_slice(), &BoxedBytes::empty())
	}

	pub fn is_empty(&self) -> bool {
		self.get_len() == 0
	}

	pub fn len(&self) -> usize {
		self.get_len() as usize
	}

	pub fn push_back(&mut self, elt: T) {
		let new_key = self.generate_new_key();
		let len = self.get_len();
		let mut previous = NULL_ENTRY;
		if len == 0 {
			self.set_front(new_key);
		} else {
			let back = self.get_back();
			let mut back_node = self.get_node(back);
			back_node.next = new_key;
			previous = back;
			self.set_node(back, back_node);
		}
		self.set_node(new_key, Node {
			previous,
			next : NULL_ENTRY
		});
		self.set_back(new_key);
		self.set_value(new_key, elt);
		self.set_len(len + 1);
	}

	pub fn front(&self) -> Option<T> {
		let front = self.get_front();
		if front == NULL_ENTRY {
			return None;
		}
		return Some(self.get_value(front));
	}

	pub fn remove_at_index(&self, index : u32) {
		let len = self.get_len();
		if len == 0 {
			return;
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
		self.clear_value(index);
		self.set_len(len - 1);
	}

	#[inline]
    pub fn iter(&self) -> Iter<SA, T> {
        Iter::new(self)
    }
}

pub struct Iter<'a, SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode + 'static,
{
	index : u32,
	linked_list : &'a LinkedListMapper<SA, T>
}

impl<'a, SA, T> Iter<'a, SA, T>
where
	SA: StorageReadApi + StorageWriteApi + ErrorApi + Clone + 'static,
	T: TopEncode + TopDecode + 'static,
{
	fn new(linked_list : &'a LinkedListMapper<SA, T>) -> Iter<'a, SA, T>
	{
        Iter { 
			index : linked_list.get_front(),
			linked_list
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
