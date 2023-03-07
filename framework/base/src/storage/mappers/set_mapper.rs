use core::marker::PhantomData;

pub use super::queue_mapper::Iter;
use super::{QueueMapper, StorageClearable, StorageMapper};
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
    api::StorageMapperApi,
    codec::{
        self, multi_encode_iter_or_handle_err, CodecFrom, EncodeErrorHandler, NestedDecode,
        NestedEncode, TopDecode, TopEncode, TopEncodeMulti, TopEncodeMultiOutput,
    },
    storage::{storage_get, storage_set, StorageKey},
    types::{ManagedType, MultiValueEncoded},
};

const NULL_ENTRY: u32 = 0;
const NODE_ID_IDENTIFIER: &[u8] = b".node_id";

pub struct SetMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
{
    _phantom_api: PhantomData<SA>,
    base_key: StorageKey<SA>,
    queue_mapper: QueueMapper<SA, T>,
}

impl<SA, T> StorageMapper<SA> for SetMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        SetMapper {
            _phantom_api: PhantomData,
            base_key: base_key.clone(),
            queue_mapper: QueueMapper::<SA, T>::new(base_key),
        }
    }
}

impl<SA, T> StorageClearable for SetMapper<SA, T>
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

impl<SA, T> SetMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    fn build_named_value_key(&self, name: &[u8], value: &T) -> StorageKey<SA> {
        let mut named_key = self.base_key.clone();
        named_key.append_bytes(name);
        named_key.append_item(value);
        named_key
    }

    fn get_node_id(&self, value: &T) -> u32 {
        storage_get(
            self.build_named_value_key(NODE_ID_IDENTIFIER, value)
                .as_ref(),
        )
    }

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

    /// Returns `true` if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.queue_mapper.is_empty()
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        self.queue_mapper.len()
    }

    /// Returns `true` if the set contains a value.
    pub fn contains(&self, value: &T) -> bool {
        self.get_node_id(value) != NULL_ENTRY
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

    /// An iterator visiting all elements in arbitrary order.
    /// The iterator element type is `&'a T`.
    pub fn iter(&self) -> Iter<SA, T> {
        self.queue_mapper.iter()
    }

    /// Checks the internal consistency of the collection. Used for unit tests.
    pub fn check_internal_consistency(&self) -> bool {
        self.queue_mapper.check_internal_consistency()
    }
}

impl<'a, SA, T> IntoIterator for &'a SetMapper<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
{
    type Item = T;

    type IntoIter = Iter<'a, SA, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<SA, T> Extend<T> for SetMapper<SA, T>
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
impl<SA, T> TopEncodeMulti for SetMapper<SA, T>
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

impl<SA, T> CodecFrom<SetMapper<SA, T>> for MultiValueEncoded<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
{
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, T> TypeAbi for SetMapper<SA, T>
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
