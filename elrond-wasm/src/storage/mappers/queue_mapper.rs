use super::{IntoStorageMapper, SingleValueMapper, StorageClearable, StorageMapper};
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
    api::{EndpointFinishApi, ErrorApi, ManagedTypeApi, StorageReadApi, StorageWriteApi},
    io::EndpointResult,
    storage::StorageKey,
    types::MultiResultVec,
};
use alloc::vec::Vec;
use core::marker::PhantomData;
use elrond_codec::{
    elrond_codec_derive::{TopDecode, TopDecodeOrDefault, TopEncode, TopEncodeOrDefault},
    DecodeDefault, EncodeDefault, TopDecode, TopEncode,
};

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
pub struct QueueMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: 'static,
{
    api: SA,
    base_key: StorageKey<SA>,
    _phantom: core::marker::PhantomData<T>,
}

impl<SA, T> StorageMapper<SA> for QueueMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: 'static,
{
    fn new(api: SA, base_key: StorageKey<SA>) -> Self {
        QueueMapper {
            api,
            base_key,
            _phantom: PhantomData,
        }
    }
}

impl<SA, T> IntoStorageMapper<SA> for QueueMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
{
    type StorageMapperType = Self;
}

impl<SA, T> QueueMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: IntoStorageMapper<SA>,
{
    fn value(&self, node_id: u32) -> T::StorageMapperType {
        T::item(
            self.api.clone(),
            self.build_node_id_named_key(VALUE_IDENTIFIER, node_id),
        )
    }
}

impl<SA, T> StorageClearable for QueueMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: IntoStorageMapper<SA>,
    T::StorageMapperType: StorageClearable,
{
    fn clear(&mut self) {
        let info = self.info().get();
        let mut node_id = info.front;
        while node_id != NULL_ENTRY {
            let node = self.node(node_id).get();
            self.node(node_id).clear();
            self.value(node_id).clear();
            node_id = node.next;
        }
        self.info().set(&QueueMapperInfo::default());
    }
}

impl<SA, T> QueueMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: 'static,
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

    fn info(&self) -> SingleValueMapper<SA, QueueMapperInfo> {
        SingleValueMapper::new(self.api.clone(), self.build_name_key(INFO_IDENTIFIER))
    }

    fn node(&self, node_id: u32) -> SingleValueMapper<SA, Node> {
        SingleValueMapper::new(
            self.api.clone(),
            self.build_node_id_named_key(NODE_IDENTIFIER, node_id),
        )
    }

    /// Returns `true` if the `Queue` is empty.
    ///
    /// This operation should compute in *O*(1) time.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the length of the `Queue`.
    ///
    /// This operation should compute in *O*(1) time.
    pub fn len(&self) -> usize {
        self.info().get().len as usize
    }

    /// Runs several checks in order to verify that both forwards and backwards iteration
    /// yields the same node entries and that the number of items in the queue is correct.
    /// Used for unit testing.
    ///
    /// This operation should compute in *O*(n) time.
    pub fn check_internal_consistency(&self) -> bool {
        let info = self.info().get();
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
            if self.node(front).get().previous != NULL_ENTRY {
                return false;
            }
            if self.node(back).get().next != NULL_ENTRY {
                return false;
            }

            // iterate forwards
            let mut forwards = Vec::new();
            while front != NULL_ENTRY {
                forwards.push(front);
                front = self.node(front).get().next;
            }
            if forwards.len() != info.len as usize {
                return false;
            }

            // iterate backwards
            let mut backwards = Vec::new();
            while back != NULL_ENTRY {
                backwards.push(back);
                back = self.node(back).get().previous;
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

impl<SA, T> QueueMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode,
{
    fn get_value_option(&self, node_id: u32) -> Option<T> {
        self.get_value_option_nested(node_id)
            .map(|mapper| mapper.get())
    }

    /// Appends an element to the back of a queue
    /// and returns the node id of the newly added node.
    ///
    /// This operation should compute in *O*(1) time.
    pub(crate) fn push_back_node_id(&mut self, elt: &T) -> u32 {
        let (new_node_id, mut mapper) = self.push_back_node_id_nested();
        mapper.set(elt);
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
        let (_, mut mapper) = self.push_front_node_id_nested();
        mapper.set(&elt);
    }

    /// Provides a copy to the front element, or `None` if the queue is
    /// empty.
    pub fn front(&self) -> Option<T> {
        self.get_value_option(self.info().get().front)
    }

    /// Provides a copy to the back element, or `None` if the queue is
    /// empty.
    pub fn back(&self) -> Option<T> {
        self.get_value_option(self.info().get().back)
    }

    /// Removes the last element from a queue and returns it, or `None` if
    /// it is empty.
    ///
    /// This operation should compute in *O*(1) time.
    pub fn pop_back(&mut self) -> Option<T> {
        self.remove_by_node_id(self.info().get().back)
    }

    /// Removes the first element and returns it, or `None` if the queue is
    /// empty.
    ///
    /// This operation should compute in *O*(1) time.
    pub fn pop_front(&mut self) -> Option<T> {
        self.remove_by_node_id(self.info().get().front)
    }

    /// Removes element with the given node id and returns it, or `None` if the queue is
    /// empty.
    /// Note: has undefined behavior if there's no node with the given node id in the queue
    ///
    /// This operation should compute in *O*(1) time.
    pub(crate) fn remove_by_node_id(&mut self, node_id: u32) -> Option<T> {
        self.remove_by_node_id_nested(node_id).map(|mut mapper| {
            let removed_value = mapper.get();
            mapper.clear();
            removed_value
        })
    }

    /// Provides a forward iterator.
    pub fn iter(&self) -> Iter<SA, T> {
        Iter::new(self)
    }
}

impl<SA, T> QueueMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: IntoStorageMapper<SA>,
    T::StorageMapperType: StorageClearable,
{
    fn get_value_option_nested(&self, node_id: u32) -> Option<T::StorageMapperType> {
        if node_id == NULL_ENTRY {
            return None;
        }
        Some(self.value(node_id))
    }

    /// Creates an entry to the back of a queue
    /// Returns the node id and the entry
    ///
    /// This operation should compute in *O*(1) time.
    pub(crate) fn push_back_node_id_nested(&mut self) -> (u32, T::StorageMapperType) {
        let mut info = self.info().get();
        let new_node_id = info.generate_new_node_id();
        let mut previous = NULL_ENTRY;
        if info.len == 0 {
            info.front = new_node_id;
        } else {
            let back = info.back;
            let mut back_node = self.node(back).get();
            back_node.next = new_node_id;
            previous = back;
            self.node(back).set(&back_node);
        }
        self.node(new_node_id).set(&Node {
            previous,
            next: NULL_ENTRY,
        });
        info.back = new_node_id;
        info.len += 1;
        self.info().set(&info);
        (new_node_id, self.value(new_node_id))
    }

    /// Appends an element to the back of a queue.
    ///
    /// This operation should compute in *O*(1) time.
    pub fn push_back_nested(&mut self) -> T::StorageMapperType {
        let (_, mapper) = self.push_back_node_id_nested();
        mapper
    }

    pub(crate) fn push_front_node_id_nested(&mut self) -> (u32, T::StorageMapperType) {
        let mut info = self.info().get();
        let new_node_id = info.generate_new_node_id();
        let mut next = NULL_ENTRY;
        if info.len == 0 {
            info.back = new_node_id;
        } else {
            let front = info.front;
            let mut front_node = self.node(front).get();
            front_node.previous = new_node_id;
            next = front;
            self.node(front).set(&front_node);
        }
        self.node(new_node_id).set(&Node {
            previous: NULL_ENTRY,
            next,
        });
        info.front = new_node_id;
        info.len += 1;
        self.info().set(&info);
        (new_node_id, self.value(new_node_id))
    }

    /// Adds an element first in the queue.
    ///
    /// This operation should compute in *O*(1) time.
    pub fn push_front_nested(&mut self) -> T::StorageMapperType {
        let (_, mapper) = self.push_front_node_id_nested();
        mapper
    }

    /// Provides a copy to the front element, or `None` if the queue is
    /// empty.
    pub fn front_nested(&self) -> Option<T::StorageMapperType> {
        self.get_value_option_nested(self.info().get().front)
    }

    /// Provides a copy to the back element, or `None` if the queue is
    /// empty.
    pub fn back_nested(&self) -> Option<T::StorageMapperType> {
        self.get_value_option_nested(self.info().get().back)
    }

    /// Removes the last element from a queue and returns it, or `None` if
    /// it is empty.
    ///
    /// This operation should compute in *O*(1) time.
    pub fn pop_back_nested(&mut self) {
        if let Some(mut mapper) = self.remove_by_node_id_nested(self.info().get().back) {
            mapper.clear();
        }
    }

    /// Removes the first element and returns it, or `None` if the queue is
    /// empty.
    ///
    /// This operation should compute in *O*(1) time.
    pub fn pop_front_nested(&mut self) {
        if let Some(mut mapper) = self.remove_by_node_id_nested(self.info().get().front) {
            mapper.clear();
        }
    }

    /// Removes the node information. The caller is responsible to clear the returned mapper.
    pub(crate) fn remove_by_node_id_nested(
        &mut self,
        node_id: u32,
    ) -> Option<T::StorageMapperType> {
        if node_id == NULL_ENTRY {
            return None;
        }
        let node = self.node(node_id).get();

        let mut info = self.info().get();
        if node.previous == NULL_ENTRY {
            info.front = node.next;
        } else {
            let mut previous = self.node(node.previous).get();
            previous.next = node.next;
            self.node(node.previous).set(&previous);
        }

        if node.next == NULL_ENTRY {
            info.back = node.previous;
        } else {
            let mut next = self.node(node.next).get();
            next.previous = node.previous;
            self.node(node.next).set(&next);
        }

        self.node(node_id).clear();
        info.len -= 1;
        self.info().set(&info);
        Some(self.value(node_id))
    }

    /// Provides a forward iterator which returns the nested storage mappers
    pub fn iter_nested(&self) -> IterNested<SA, T> {
        IterNested::new(self)
    }
}

/// An iterator over the elements of a `QueueMapper`.
///
/// This `struct` is created by [`QueueMapper::iter()`]. See its
/// documentation for more.
pub struct Iter<'a, SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + 'static,
{
    node_id: u32,
    queue: &'a QueueMapper<SA, T>,
}

impl<'a, SA, T> Iter<'a, SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + 'static,
{
    fn new(queue: &'a QueueMapper<SA, T>) -> Self {
        Self {
            node_id: queue.info().get().front,
            queue,
        }
    }
}

impl<'a, SA, T> Iterator for Iter<'a, SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + 'static,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        let current_node_id = self.node_id;
        if current_node_id == NULL_ENTRY {
            return None;
        }
        self.node_id = self.queue.node(current_node_id).get().next;
        Some(self.queue.value(current_node_id).get())
    }
}

/// An iterator over the elements of a `QueueMapper`.
///
/// This `struct` is created by [`QueueMapper::iter_nested()`]. See its
/// documentation for more.
pub struct IterNested<'a, SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: IntoStorageMapper<SA> + 'static,
{
    node_id: u32,
    queue: &'a QueueMapper<SA, T>,
}

impl<'a, SA, T> IterNested<'a, SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: IntoStorageMapper<SA> + 'static,
{
    fn new(queue: &'a QueueMapper<SA, T>) -> Self {
        Self {
            node_id: queue.info().get().front,
            queue,
        }
    }
}

impl<'a, SA, T> Iterator for IterNested<'a, SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: IntoStorageMapper<SA> + 'static,
{
    type Item = T::StorageMapperType;

    #[inline]
    fn next(&mut self) -> Option<T::StorageMapperType> {
        let current_node_id = self.node_id;
        if current_node_id == NULL_ENTRY {
            return None;
        }
        self.node_id = self.queue.node(current_node_id).get().next;
        Some(self.queue.value(current_node_id))
    }
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, T> EndpointResult for QueueMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + EndpointResult,
{
    type DecodeAs = MultiResultVec<T::DecodeAs>;

    fn finish<FA>(&self, api: FA)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
    {
        let v: Vec<T> = self.iter().collect();
        MultiResultVec::<T>::from(v).finish(api);
    }
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, T> TypeAbi for QueueMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
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
