use core::marker::PhantomData;

pub use super::vec_mapper::Iter;
use super::{
    set_mapper::{CurrentStorage, StorageAddress},
    StorageClearable, StorageMapper, VecMapper,
};
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
    api::StorageMapperApi,
    codec::{
        multi_encode_iter_or_handle_err, CodecFrom, EncodeErrorHandler, NestedDecode, NestedEncode,
        TopDecode, TopEncode, TopEncodeMulti, TopEncodeMultiOutput,
    },
    storage::StorageKey,
    storage_clear, storage_set,
    types::{ManagedAddress, ManagedType, MultiValueEncoded},
};

const ITEM_INDEX: &[u8] = b".index";
const NULL_ENTRY: usize = 0;

pub struct UnorderedSetMapper<SA, T, A = CurrentStorage>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
{
    _phantom_api: PhantomData<SA>,
    address: A,
    base_key: StorageKey<SA>,
    vec_mapper: VecMapper<SA, T, A>,
}

impl<SA, T> StorageMapper<SA> for UnorderedSetMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        UnorderedSetMapper {
            _phantom_api: PhantomData,
            address: CurrentStorage,
            base_key: base_key.clone(),
            vec_mapper: VecMapper::<SA, T>::new(base_key),
        }
    }
}

impl<SA, T> UnorderedSetMapper<SA, T, ManagedAddress<SA>>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    pub fn new_from_address(address: ManagedAddress<SA>, base_key: StorageKey<SA>) -> Self {
        UnorderedSetMapper {
            _phantom_api: PhantomData,
            address: address.clone(),
            base_key: base_key.clone(),
            vec_mapper: VecMapper::new_from_address(address, base_key),
        }
    }
}

impl<SA, T> StorageClearable for UnorderedSetMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    fn clear(&mut self) {
        for value in self.vec_mapper.iter() {
            self.clear_index(&value);
        }
        self.vec_mapper.clear();
    }
}

impl<SA, T, A> UnorderedSetMapper<SA, T, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    fn item_index_key(&self, value: &T) -> StorageKey<SA> {
        let mut item_key = self.base_key.clone();
        item_key.append_bytes(ITEM_INDEX);
        item_key.append_item(value);
        item_key
    }

    /// Gets the item's index at the given address' mapper.
    /// Returns `0` if the item is not in the list.
    pub fn get_index(&self, value: &T) -> usize {
        self.address
            .address_storage_get(self.item_index_key(value).as_ref())
    }

    /// Get item at index from the given address.
    /// Index must be valid (1 <= index <= count).
    pub fn get_by_index(&self, index: usize) -> T {
        self.vec_mapper.get(index)
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

    /// An iterator visiting all elements in arbitrary order.
    /// The iterator element type is `&'a T`.
    pub fn iter(&self) -> Iter<SA, T, A> {
        self.vec_mapper.iter()
    }
}

impl<SA, T> UnorderedSetMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    fn set_index(&self, value: &T, index: usize) {
        storage_set(self.item_index_key(value).as_ref(), &index);
    }

    fn clear_index(&self, value: &T) {
        storage_clear(self.item_index_key(value).as_ref());
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

    /// Exchanges the indexes of two values. Returns whether the operation was
    /// successful.
    pub fn swap_indexes(&mut self, index1: usize, index2: usize) -> bool {
        if index1 == NULL_ENTRY || index2 == NULL_ENTRY {
            return false;
        }
        let value1 = self.get_by_index(index1);
        let value2 = self.get_by_index(index2);
        self.vec_mapper.set(index2, &value1);
        self.vec_mapper.set(index1, &value2);
        self.set_index(&value1, index2);
        self.set_index(&value2, index1);
        true
    }
}

impl<'a, SA, T, A> IntoIterator for &'a UnorderedSetMapper<SA, T, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
{
    type Item = T;

    type IntoIter = Iter<'a, SA, T, A>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<SA, T> Extend<T> for UnorderedSetMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in iter {
            self.insert(item);
        }
    }
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, T> TopEncodeMulti for UnorderedSetMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        multi_encode_iter_or_handle_err(self.iter(), output, h)
    }
}

impl<SA, T> CodecFrom<UnorderedSetMapper<SA, T>> for MultiValueEncoded<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
{
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, T> TypeAbi for UnorderedSetMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + TypeAbi,
{
    fn type_name() -> TypeName {
        crate::abi::type_name_variadic::<T>()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        true
    }
}
