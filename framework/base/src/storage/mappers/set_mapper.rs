use core::marker::PhantomData;

use storage_get_from_address::storage_get_len_from_address;

pub use super::queue_mapper::Iter;
use super::{QueueMapper, StorageClearable, StorageMapper};
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
    api::StorageMapperApi,
    codec::{
        self, multi_encode_iter_or_handle_err, CodecFrom, EncodeErrorHandler, NestedDecode,
        NestedEncode, TopDecode, TopEncode, TopEncodeMulti, TopEncodeMultiOutput,
    },
    storage::{storage_get_from_address, storage_set, StorageKey},
    storage_get, storage_get_len,
    types::{ManagedAddress, ManagedRef, ManagedType, MultiValueEncoded},
};

const NULL_ENTRY: u32 = 0;
const NODE_ID_IDENTIFIER: &[u8] = b".node_id";

pub trait StorageAddress<SA>
where
    SA: StorageMapperApi,
{
    fn address_storage_get<T: TopDecode>(&self, key: ManagedRef<'_, SA, StorageKey<SA>>) -> T;
    fn address_storage_get_len(&self, key: ManagedRef<'_, SA, StorageKey<SA>>) -> usize;
}

pub struct CurrentStorage;

impl<SA> StorageAddress<SA> for CurrentStorage
where
    SA: StorageMapperApi,
{
    fn address_storage_get<T: TopDecode>(&self, key: ManagedRef<'_, SA, StorageKey<SA>>) -> T {
        storage_get(key)
    }

    fn address_storage_get_len(&self, key: ManagedRef<'_, SA, StorageKey<SA>>) -> usize {
        storage_get_len(key)
    }
}

impl<SA> StorageAddress<SA> for ManagedAddress<SA>
where
    SA: StorageMapperApi,
{
    fn address_storage_get<T: TopDecode>(&self, key: ManagedRef<'_, SA, StorageKey<SA>>) -> T {
        storage_get_from_address(self.as_ref(), key)
    }

    fn address_storage_get_len(&self, key: ManagedRef<'_, SA, StorageKey<SA>>) -> usize {
        storage_get_len_from_address(self.as_ref(), key)
    }
}

pub struct SetMapper<SA, T, A = CurrentStorage>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
{
    _phantom_api: PhantomData<SA>,
    address: A,
    base_key: StorageKey<SA>,
    queue_mapper: QueueMapper<SA, T, A>,
}

impl<SA, T> StorageMapper<SA> for SetMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        SetMapper {
            _phantom_api: PhantomData,
            address: CurrentStorage,
            base_key: base_key.clone(),
            queue_mapper: QueueMapper::new(base_key),
        }
    }
}

impl<SA, T> StorageClearable for SetMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    fn clear(&mut self) {
        for value in self.queue_mapper.iter() {
            self.clear_node_id(&value);
        }
        self.queue_mapper.clear();
    }
}

impl<SA, T> SetMapper<SA, T, ManagedAddress<SA>>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    pub fn new_from_address(address: ManagedAddress<SA>, base_key: StorageKey<SA>) -> Self {
        SetMapper {
            _phantom_api: PhantomData,
            address: address.clone(),
            base_key: base_key.clone(),
            queue_mapper: QueueMapper::new_from_address(address, base_key),
        }
    }
}

impl<SA, T> SetMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    fn set_node_id(&self, value: &T, node_id: u32) {
        storage_set(
            self.build_named_value_key(NODE_ID_IDENTIFIER, value)
                .as_ref(),
            &node_id,
        );
    }

    fn clear_node_id(&self, value: &T) {
        storage_set(
            self.build_named_value_key(NODE_ID_IDENTIFIER, value)
                .as_ref(),
            &codec::Empty,
        );
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
        let new_node_id = self.queue_mapper.push_back_node_id(&value);
        self.set_node_id(&value, new_node_id);
        true
    }

    /// Removes a value from the set. Returns whether the value was
    /// present in the set.
    pub fn remove(&mut self, value: &T) -> bool {
        let node_id = self.get_node_id(value);
        if node_id == NULL_ENTRY {
            return false;
        }
        self.queue_mapper.remove_by_node_id(node_id);
        self.clear_node_id(value);
        true
    }

    pub fn remove_all<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in iter {
            self.remove(&item);
        }
    }
}

impl<SA, A, T> SetMapper<SA, T, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    pub fn build_named_value_key(&self, name: &[u8], value: &T) -> StorageKey<SA> {
        let mut named_key = self.base_key.clone();
        named_key.append_bytes(name);
        named_key.append_item(value);
        named_key
    }

    /// An iterator visiting all elements in arbitrary order.
    /// The iterator element type is `&'a T`.
    pub fn iter(&self) -> Iter<SA, A, T> {
        self.queue_mapper.iter()
    }

    pub fn iter_from(&self, value: &T) -> Iter<SA, A, T> {
        let node_id = self.get_node_id(value);
        self.queue_mapper.iter_from_node_id(node_id)
    }

    fn get_node_id(&self, value: &T) -> u32 {
        self.address.address_storage_get(
            self.build_named_value_key(NODE_ID_IDENTIFIER, value)
                .as_ref(),
        )
    }

    /// Returns `true` if the set contains a value.
    pub fn contains(&self, value: &T) -> bool {
        self.get_node_id(value) != NULL_ENTRY
    }

    /// Returns `true` if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.queue_mapper.is_empty()
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        self.queue_mapper.len()
    }

    /// Checks the internal consistency of the collection. Used for unit tests.
    pub fn check_internal_consistency(&self) -> bool {
        self.queue_mapper.check_internal_consistency()
    }

    pub fn next(&self, value: &T) -> Option<T> {
        let node_id = self.get_node_id(value);
        if node_id == NULL_ENTRY {
            return None;
        }

        let next_node_id = self.queue_mapper.get_node(node_id).next;

        self.queue_mapper.get_value_option(next_node_id)
    }

    pub fn previous(&self, value: &T) -> Option<T> {
        let node_id = self.get_node_id(value);
        if node_id == NULL_ENTRY {
            return None;
        }

        let next_node_id = self.queue_mapper.get_node(node_id).previous;

        self.queue_mapper.get_value_option(next_node_id)
    }

    pub fn front(&self) -> Option<T> {
        self.queue_mapper.front()
    }

    pub fn back(&self) -> Option<T> {
        self.queue_mapper.back()
    }
}

impl<'a, SA, A, T> IntoIterator for &'a SetMapper<SA, T, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
{
    type Item = T;

    type IntoIter = Iter<'a, SA, A, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<SA, T> Extend<T> for SetMapper<SA, T, CurrentStorage>
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
impl<SA, T> TopEncodeMulti for SetMapper<SA, T, CurrentStorage>
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

impl<SA, T> CodecFrom<SetMapper<SA, T, CurrentStorage>> for MultiValueEncoded<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
{
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, T> TypeAbi for SetMapper<SA, T, CurrentStorage>
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
