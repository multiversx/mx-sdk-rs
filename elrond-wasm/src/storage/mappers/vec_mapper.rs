use super::{StorageClearable, StorageMapper};
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
    api::{EndpointFinishApi, ErrorApi, ManagedTypeApi, StorageReadApi, StorageWriteApi},
    finish_all,
    io::EndpointResult,
    storage::{storage_clear, storage_get, storage_get_len, storage_set, StorageKey},
    types::MultiResultVec,
};
use alloc::vec::Vec;
use core::{marker::PhantomData, usize};
use elrond_codec::{TopDecode, TopEncode};

const ITEM_SUFFIX: &[u8] = b".item";
const LEN_SUFFIX: &[u8] = b".len";

/// Manages a list of items of the same type.
/// Saves each of the items under a separate key in storage.
/// To produce each individual key, it concatenates the main key with a serialized 4-byte index.
/// Indexes start from 1, instead of 0. (We avoid 0-value indexes to prevent confusion between an uninitialized variable and zero.)
/// It also stores the count separately, at what would be index 0.
/// The count is always kept in sync automatically.
pub struct VecMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + 'static,
{
    api: SA,
    base_key: StorageKey<SA>,
    len_key: StorageKey<SA>,
    _phantom: core::marker::PhantomData<T>,
}

impl<SA, T> StorageMapper<SA> for VecMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode,
{
    fn new(api: SA, base_key: StorageKey<SA>) -> Self {
        let mut len_key = base_key.clone();
        len_key.append_bytes(LEN_SUFFIX);

        VecMapper {
            api,
            base_key,
            len_key,
            _phantom: PhantomData,
        }
    }
}

impl<SA, T> StorageClearable for VecMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode,
{
    fn clear(&mut self) {
        self.clear();
    }
}

impl<SA, T> VecMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode,
{
    fn item_key(&self, index: usize) -> StorageKey<SA> {
        let mut item_key = self.base_key.clone();
        item_key.append_bytes(ITEM_SUFFIX);
        item_key.append_item(&index);
        item_key
    }

    fn save_count(&self, new_len: usize) {
        storage_set(self.api.clone(), &self.len_key, &new_len);
    }

    /// Number of items managed by the mapper.
    pub fn len(&self) -> usize {
        storage_get(self.api.clone(), &self.len_key)
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
        storage_set(self.api.clone(), &self.item_key(len), item);
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
            storage_set(self.api.clone(), &self.item_key(len), item);
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
        storage_get(self.api.clone(), &self.item_key(index))
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
        storage_get_len(self.api.clone(), &self.item_key(index)) == 0
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
        storage_set(self.api.clone(), &self.item_key(index), item);
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
        storage_clear(self.api.clone(), &self.item_key(index));
    }

    /// Clears item at index from storage by swap remove
    /// last item takes the index of the item to remove
    /// and we remove the last index.
    pub fn swap_remove(&mut self, index: usize) {
        let _ = self.swap_remove_and_get_old_last(index);
    }

    pub(crate) fn swap_remove_and_get_old_last(&mut self, index: usize) -> Option<T> {
        let last_item_index = self.len();
        if index == 0 || index > last_item_index {
            self.api.signal_error(&b"index out of range"[..]);
        }

        let mut last_item_as_option = Option::None;
        if index != last_item_index {
            let last_item = self.get(last_item_index);
            self.set(index, &last_item);
            last_item_as_option = Some(last_item);
        }
        self.clear_entry(last_item_index);
        self.save_count(last_item_index - 1);
        last_item_as_option
    }

    /// Loads all items from storage and places them in a Vec.
    /// Can easily consume a lot of gas.
    pub fn load_as_vec(&self) -> Vec<T> {
        self.iter().collect()
    }

    /// Deletes all contents form storage and sets count to 0.
    /// Can easily consume a lot of gas.
    pub fn clear(&mut self) {
        let len = self.len();
        for i in 1..=len {
            storage_clear(self.api.clone(), &self.item_key(i));
        }
        self.save_count(0);
    }

    /// Provides a forward iterator.
    pub fn iter(&self) -> Iter<SA, T> {
        Iter::new(self)
    }
}

/// An iterator over the elements of a `VecMapper`.
///
/// This `struct` is created by [`VecMapper::iter()`]. See its
/// documentation for more.
pub struct Iter<'a, SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + 'static,
{
    index: usize,
    len: usize,
    vec: &'a VecMapper<SA, T>,
}

impl<'a, SA, T> Iter<'a, SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + 'static,
{
    fn new(vec: &'a VecMapper<SA, T>) -> Iter<'a, SA, T> {
        Iter {
            index: 1,
            len: vec.len(),
            vec,
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
        let current_index = self.index;
        if current_index > self.len {
            return None;
        }
        self.index += 1;
        Some(self.vec.get_unchecked(current_index))
    }
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, T> EndpointResult for VecMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + EndpointResult,
{
    type DecodeAs = MultiResultVec<T::DecodeAs>;

    fn finish<FA>(&self, api: FA)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
    {
        finish_all(api, self.iter());
    }
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, T> TypeAbi for VecMapper<SA, T>
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
