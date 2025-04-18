use core::marker::PhantomData;

use super::{
    source::{CurrentStorage, StorageAddress},
    StorageClearable, StorageMapper, StorageMapperFromAddress,
};
use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName},
    api::StorageMapperApi,
    codec::{
        self,
        derive::{TopDecode, TopDecodeOrDefault, TopEncode, TopEncodeOrDefault},
        multi_encode_iter_or_handle_err, DecodeDefault, EncodeDefault, EncodeErrorHandler,
        TopDecode, TopEncode, TopEncodeMulti, TopEncodeMultiOutput,
    },
    storage::{storage_set, StorageKey},
    types::{ManagedAddress, ManagedType, MultiValueEncoded},
};
use alloc::vec::Vec;

const NULL_ENTRY: u32 = 0;
const INFO_IDENTIFIER: &[u8] = b".info";
const NODE_IDENTIFIER: &[u8] = b".node_links";
const VALUE_IDENTIFIER: &[u8] = b".value";

#[derive(TopEncode, TopDecode, PartialEq, Eq, Clone, Copy)]
pub struct Node {
    pub previous: u32,
    pub next: u32,
}

#[derive(TopEncodeOrDefault, TopDecodeOrDefault, PartialEq, Eq, Clone, Copy)]
pub struct QueueMapperInfo {
    pub len: u32,
    pub front: u32,
    pub back: u32,
    pub new: u32,
}

impl EncodeDefault for QueueMapperInfo {
    fn is_default(&self) -> bool {
        self.len == 0
    }
}

impl DecodeDefault for QueueMapperInfo {
    fn default() -> Self {
        Self {
            len: 0,
            front: 0,
            back: 0,
            new: 0,
        }
    }
}

impl QueueMapperInfo {
    pub fn generate_new_node_id(&mut self) -> u32 {
        self.new += 1;
        self.new
    }
}

/// A queue with owned nodes.
///
/// The `QueueMapper` allows pushing and popping elements at either end
/// in constant time.
pub struct QueueMapper<SA, T, A = CurrentStorage>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode + 'static,
{
    _phantom_api: PhantomData<SA>,
    address: A,
    base_key: StorageKey<SA>,
    _phantom_item: PhantomData<T>,
}

impl<SA, T> StorageMapper<SA> for QueueMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        QueueMapper {
            _phantom_api: PhantomData,
            address: CurrentStorage,
            base_key,
            _phantom_item: PhantomData,
        }
    }
}

impl<SA, T> StorageClearable for QueueMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
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
        self.set_info(QueueMapperInfo::default());
    }
}

impl<SA, T> QueueMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
    fn set_info(&mut self, value: QueueMapperInfo) {
        storage_set(self.build_name_key(INFO_IDENTIFIER).as_ref(), &value);
    }

    fn set_node(&mut self, node_id: u32, item: Node) {
        storage_set(
            self.build_node_id_named_key(NODE_IDENTIFIER, node_id)
                .as_ref(),
            &item,
        );
    }

    fn clear_node(&mut self, node_id: u32) {
        storage_set(
            self.build_node_id_named_key(NODE_IDENTIFIER, node_id)
                .as_ref(),
            &codec::Empty,
        );
    }

    fn set_value(&mut self, node_id: u32, value: &T) {
        storage_set(
            self.build_node_id_named_key(VALUE_IDENTIFIER, node_id)
                .as_ref(),
            value,
        )
    }

    fn clear_value(&mut self, node_id: u32) {
        storage_set(
            self.build_node_id_named_key(VALUE_IDENTIFIER, node_id)
                .as_ref(),
            &codec::Empty,
        )
    }

    /// Appends an element to the back of a queue
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
        self.set_value(new_node_id, elt);
        info.len += 1;
        self.set_info(info);
        new_node_id
    }

    /// Appends an element to the back of a queue.
    ///
    /// This operation should compute in *O*(1) time.
    pub fn push_back(&mut self, elt: T) {
        let _ = self.push_back_node_id(&elt);
    }

    /// Adds an element first in the queue.
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

    /// Removes the last element from a queue and returns it, or `None` if
    /// it is empty.
    ///
    /// This operation should compute in *O*(1) time.
    pub fn pop_back(&mut self) -> Option<T> {
        self.remove_by_node_id(self.get_info().back)
    }

    /// Removes the first element and returns it, or `None` if the queue is
    /// empty.
    ///
    /// This operation should compute in *O*(1) time.
    pub fn pop_front(&mut self) -> Option<T> {
        self.remove_by_node_id(self.get_info().front)
    }

    /// Removes element with the given node id and returns it, or `None` if the queue is
    /// empty.
    /// Note: has undefined behavior if there's no node with the given node id in the queue
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
}

impl<SA, T> StorageMapperFromAddress<SA> for QueueMapper<SA, T, ManagedAddress<SA>>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
    fn new_from_address(address: ManagedAddress<SA>, base_key: StorageKey<SA>) -> Self {
        QueueMapper {
            _phantom_api: PhantomData::<SA>,
            address,
            base_key,
            _phantom_item: PhantomData::<T>,
        }
    }
}

impl<SA, A, T> QueueMapper<SA, T, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode + 'static,
{
    fn build_node_id_named_key(&self, name: &[u8], node_id: u32) -> StorageKey<SA> {
        let mut named_key = self.base_key.clone();
        named_key.append_bytes(name);
        named_key.append_item(&node_id);
        named_key
    }

    fn build_name_key(&self, name: &[u8]) -> StorageKey<SA> {
        let mut name_key = self.base_key.clone();
        name_key.append_bytes(name);
        name_key
    }

    pub fn iter(&self) -> Iter<SA, A, T> {
        Iter::new(self)
    }

    pub fn iter_from_node_id(&self, node_id: u32) -> Iter<SA, A, T> {
        Iter::new_from_node_id(self, node_id)
    }

    fn get_info(&self) -> QueueMapperInfo {
        self.address
            .address_storage_get(self.build_name_key(INFO_IDENTIFIER).as_ref())
    }

    pub fn get_node(&self, node_id: u32) -> Node {
        self.address.address_storage_get(
            self.build_node_id_named_key(NODE_IDENTIFIER, node_id)
                .as_ref(),
        )
    }

    fn get_value(&self, node_id: u32) -> T {
        self.address.address_storage_get(
            self.build_node_id_named_key(VALUE_IDENTIFIER, node_id)
                .as_ref(),
        )
    }

    pub fn get_value_option(&self, node_id: u32) -> Option<T> {
        if node_id == NULL_ENTRY {
            return None;
        }
        Some(self.get_value(node_id))
    }

    /// Returns `true` if the `Queue` is empty.
    ///
    /// This operation should compute in *O*(1) time.
    pub fn is_empty(&self) -> bool {
        self.get_info().len == 0
    }

    /// Returns the length of the `Queue`.
    ///
    /// This operation should compute in *O*(1) time.
    pub fn len(&self) -> usize {
        self.get_info().len as usize
    }

    /// Provides a copy to the front element, or `None` if the queue is
    /// empty.
    pub fn front(&self) -> Option<T> {
        self.get_value_option(self.get_info().front)
    }

    /// Provides a copy to the back element, or `None` if the queue is
    /// empty.
    pub fn back(&self) -> Option<T> {
        self.get_value_option(self.get_info().back)
    }

    /// Runs several checks in order to verify that both forwards and backwards iteration
    /// yields the same node entries and that the number of items in the queue is correct.
    /// Used for unit testing.
    ///
    /// This operation should compute in *O*(n) time.
    pub fn check_internal_consistency(&self) -> bool {
        let info = self.get_info();
        let mut front = info.front;
        let mut back = info.back;
        if info.len == 0 {
            // if the queue is empty, both ends should point to null entries
            if front != NULL_ENTRY {
                return false;
            }
            if back != NULL_ENTRY {
                return false;
            }
            true
        } else {
            // if the queue is non-empty, both ends should point to non-null entries
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

impl<'a, SA, A, T> IntoIterator for &'a QueueMapper<SA, T, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode + 'static,
{
    type Item = T;

    type IntoIter = Iter<'a, SA, A, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over the elements of a `QueueMapper`.
///
/// This `struct` is created by [`QueueMapper::iter()`]. See its
/// documentation for more.
pub struct Iter<'a, SA, A, T>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode + 'static,
{
    node_id: u32,
    queue: &'a QueueMapper<SA, T, A>,
}

impl<'a, SA, A, T> Iter<'a, SA, A, T>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode + 'static,
{
    fn new(queue: &'a QueueMapper<SA, T, A>) -> Iter<'a, SA, A, T> {
        Iter {
            node_id: queue.get_info().front,
            queue,
        }
    }

    pub fn new_from_node_id(queue: &'a QueueMapper<SA, T, A>, node_id: u32) -> Iter<'a, SA, A, T> {
        Iter { node_id, queue }
    }
}

impl<SA, A, T> Iterator for Iter<'_, SA, A, T>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode + 'static,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        let current_node_id = self.node_id;
        if current_node_id == NULL_ENTRY {
            return None;
        }
        self.node_id = self.queue.get_node(current_node_id).next;
        Some(self.queue.get_value(current_node_id))
    }
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, T> TopEncodeMulti for QueueMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        multi_encode_iter_or_handle_err(self.iter(), output, h)
    }
}

impl<SA, T> TypeAbiFrom<QueueMapper<SA, T, CurrentStorage>> for MultiValueEncoded<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
}

impl<SA, T> TypeAbiFrom<Self> for QueueMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, T> TypeAbi for QueueMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + TypeAbi,
{
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        crate::abi::type_name_variadic::<T>()
    }

    fn type_name_rust() -> TypeName {
        crate::abi::type_name_multi_value_encoded::<T>()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        true
    }
}
