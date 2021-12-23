use core::marker::PhantomData;

pub use super::vec_mapper::Iter;
use super::{StorageClearable, StorageMapper, VecMapper};
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
    api::{
        EndpointFinishApi, ErrorApi, ManagedTypeApi, StorageReadApi, StorageReadApiImpl,
        StorageWriteApi, StorageWriteApiImpl,
    },
    finish_all,
    storage::StorageKey,
    storage_clear, storage_get, storage_set,
    types::{ManagedType, MultiResultVec},
    EndpointResult,
};
use elrond_codec::{NestedDecode, NestedEncode, TopDecode, TopEncode};

const ITEM_INDEX: &[u8] = b".index";
const NULL_ENTRY: usize = 0;

pub struct UnorderedSetMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
{
    _phantom_api: PhantomData<SA>,
    base_key: StorageKey<SA>,
    vec_mapper: VecMapper<SA, T>,
}

impl<SA, T> StorageMapper<SA> for UnorderedSetMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        UnorderedSetMapper {
            _phantom_api: PhantomData,
            base_key: base_key.clone(),
            vec_mapper: VecMapper::<SA, T>::new(base_key),
        }
    }
}

impl<SA, T> StorageClearable for UnorderedSetMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    fn clear(&mut self) {
        for value in self.vec_mapper.iter() {
            self.clear_index(&value);
        }
        self.vec_mapper.clear();
    }
}

impl<SA, T> UnorderedSetMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    fn item_index_key(&self, value: &T) -> StorageKey<SA> {
        let mut item_key = self.base_key.clone();
        item_key.append_bytes(ITEM_INDEX);
        item_key.append_item(value);
        item_key
    }

    pub fn get_index(&self, value: &T) -> usize {
        storage_get(self.item_index_key(value).as_ref())
    }

    fn set_index(&self, value: &T, index: usize) {
        storage_set(&self.item_index_key(value), &index);
    }

    fn clear_index(&self, value: &T) {
        storage_clear(&self.item_index_key(value));
    }

    /// Returns `true` if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.vec_mapper.is_empty()
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        self.vec_mapper.len()
    }

    /// Returns `true` if the set contains a value.
    pub fn contains(&self, value: &T) -> bool {
        self.get_index(value) != NULL_ENTRY
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
        self.vec_mapper.push(&value);
        self.set_index(&value, self.len());
        true
    }

    /// Removes a value from the set. Returns whether the value was
    /// present in the set.
    pub fn swap_remove(&mut self, value: &T) -> bool {
        let index = self.get_index(value);
        if index == NULL_ENTRY {
            return false;
        }
        if let Some(last_item) = self.vec_mapper.swap_remove_and_get_old_last(index) {
            self.set_index(&last_item, index);
        }
        self.clear_index(value);
        true
    }

    /// An iterator visiting all elements in arbitrary order.
    /// The iterator element type is `&'a T`.
    pub fn iter(&self) -> Iter<SA, T> {
        self.vec_mapper.iter()
    }
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, T> EndpointResult for UnorderedSetMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + EndpointResult,
{
    type DecodeAs = MultiResultVec<T::DecodeAs>;

    fn finish<FA>(&self)
    where
        FA: ManagedTypeApi + EndpointFinishApi + Clone + 'static,
    {
        finish_all::<FA, _, _>(self.iter());
    }
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, T> TypeAbi for UnorderedSetMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + TypeAbi,
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
