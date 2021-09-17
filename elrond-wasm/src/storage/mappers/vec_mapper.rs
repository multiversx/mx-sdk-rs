use super::{IntoStorageMapper, SingleValueMapper, StorageClearable, StorageMapper};
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
    api::{EndpointFinishApi, ErrorApi, ManagedTypeApi, StorageReadApi, StorageWriteApi},
    io::EndpointResult,
    storage::StorageKey,
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
    T: 'static,
{
    api: SA,
    base_key: StorageKey<SA>,
    len_mapper: SingleValueMapper<SA, usize>,
    _phantom: core::marker::PhantomData<T>,
}

impl<SA, T> StorageMapper<SA> for VecMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
{
    fn new(api: SA, base_key: StorageKey<SA>) -> Self {
        let mut len_key = base_key.clone();
        len_key.append_bytes(LEN_SUFFIX);

        VecMapper {
            api: api.clone(),
            base_key,
            len_mapper: SingleValueMapper::new(api, len_key),
            _phantom: PhantomData,
        }
    }
}

impl<SA, T> IntoStorageMapper<SA> for VecMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
{
    type StorageMapperType = Self;
}

impl<SA, T> VecMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: 'static,
{
    fn item_key(&self, index: usize) -> StorageKey<SA> {
        let mut item_key = self.base_key.clone();
        item_key.append_bytes(ITEM_SUFFIX);
        item_key.append_item(&index);
        item_key
    }

    fn save_count(&mut self, new_len: usize) {
        self.len_mapper.set(&new_len);
    }

    fn index_out_of_bounds(&self, index: usize) -> bool {
        index == 0 || index > self.len()
    }

    fn check_index(&self, index: usize) {
        if self.index_out_of_bounds(index) {
            self.api.signal_error(&b"index out of range"[..]);
        }
    }

    /// Number of items managed by the mapper.
    pub fn len(&self) -> usize {
        self.len_mapper.get()
    }

    /// True if no items present in the mapper.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<SA, T> VecMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode,
{
    /// Add one item at the end of the list.
    /// Returns the index of the newly inserted item, which is also equal to the new number of elements.
    pub fn push(&mut self, item: &T) -> usize {
        let mut len = self.len();
        len += 1;
        self.item(len).set(item);
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
            self.item(len).set(item);
        }
        self.save_count(len);
        len
    }

    /// Get item at index from storage.
    /// Index must be valid (1 <= index <= count).
    pub fn get(&self, index: usize) -> T {
        self.check_index(index);
        self.get_unchecked(index)
    }

    /// Get item at index from storage.
    /// There are no restrictions on the index,
    /// calling for an invalid index will simply return the zero-value.
    pub fn get_unchecked(&self, index: usize) -> T {
        self.item(index).get()
    }

    /// Get item at index from storage.
    /// If index is valid (1 <= index <= count), returns value at index,
    /// else calls lambda given as argument.
    /// The lambda only gets called lazily if the index is not valid.
    pub fn get_or_else<F: FnOnce() -> T>(self, index: usize, or_else: F) -> T {
        if self.index_out_of_bounds(index) {
            or_else()
        } else {
            self.get_unchecked(index)
        }
    }

    /// Checks whether or not there is anything in storage at index.
    /// There are no restrictions on the index,
    /// calling for an invalid index will simply return `true`.
    pub fn item_is_empty_unchecked(&self, index: usize) -> bool {
        self.item(index).is_empty()
    }

    /// Checks whether or not there is anything ins storage at index.
    /// Index must be valid (1 <= index <= count).
    pub fn item_is_empty(&self, index: usize) -> bool {
        self.check_index(index);
        self.item_is_empty_unchecked(index)
    }

    /// Get item at index from storage.
    /// Index must be valid (1 <= index <= count).
    pub fn set(&mut self, index: usize, item: &T) {
        self.check_index(index);
        self.set_unchecked(index, item);
    }

    /// Keeping `set_unchecked` private on purpose, so developers don't write out of index limits by accident.
    fn set_unchecked(&mut self, index: usize, item: &T) {
        self.item(index).set(item);
    }

    /// Clears item at index from storage.
    /// Index must be valid (1 <= index <= count).
    pub fn clear_entry(&self, index: usize) {
        self.check_index(index);
        self.clear_entry_unchecked(index)
    }

    /// Clears item at index from storage.
    /// There are no restrictions on the index,
    /// calling for an invalid index will simply do nothing.
    pub fn clear_entry_unchecked(&self, index: usize) {
        self.item(index).clear();
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
}

impl<SA, T> StorageClearable for VecMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: IntoStorageMapper<SA>,
    T::StorageMapperType: StorageClearable,
{
    /// Deletes all contents form storage and sets count to 0.
    /// Can easily consume a lot of gas.
    fn clear(&mut self) {
        let len = self.len();
        for i in 1..=len {
            self.get_nested(i).clear()
        }
        self.save_count(0);
    }
}

impl<SA, T> VecMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: IntoStorageMapper<SA>,
    T::StorageMapperType: StorageMapper<SA>,
{
    fn item(&self, index: usize) -> T::StorageMapperType {
        T::item(self.api.clone(), self.item_key(index))
    }

    /// Add one nested storage mapper at the end of the list. Returns the inserted mapper.
    pub fn push_nested(&mut self) -> T::StorageMapperType {
        let (_, mapper) = self.push_with_index_nested()
        mapper
    }

    /// Add one nested storage mapper at the end of the list.
    /// Returns the index of the newly inserted item and the mapper itself
    pub fn push_with_index_nested(&mut self) -> (usize, T::StorageMapperType) {
        let mut len = self.len();
        len += 1;
        self.save_count(len);
        (len, self.item(len))
    }

    /// Get item at index from storage.
    /// Index must be valid (1 <= index <= count).
    fn get_nested(&self, index: usize) -> T::StorageMapperType {
        self.check_index(index);
        self.item(index)
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
        let v = self.load_as_vec();
        MultiResultVec::<T>::from(v).finish(api);
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
