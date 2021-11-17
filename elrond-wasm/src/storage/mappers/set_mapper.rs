pub use super::queue_mapper::Iter;
use super::{QueueMapper, StorageClearable, StorageMapper};
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
    api::{EndpointFinishApi, ErrorApi, ManagedTypeApi, StorageReadApi, StorageWriteApi},
    finish_all,
    io::EndpointResult,
    storage::{storage_get, storage_set, StorageKey},
    types::MultiResultVec,
};
use elrond_codec::{NestedDecode, NestedEncode, TopDecode, TopEncode};

const NULL_ENTRY: u32 = 0;
const NODE_ID_IDENTIFIER: &[u8] = b".node_id";

pub struct SetMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
{
    api: SA,
    base_key: StorageKey<SA>,
    queue_mapper: QueueMapper<SA, T>,
}

impl<SA, T> StorageMapper<SA> for SetMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    fn new(api: SA, base_key: StorageKey<SA>) -> Self {
        SetMapper {
            api: api.clone(),
            base_key: base_key.clone(),
            queue_mapper: QueueMapper::<SA, T>::new(api, base_key),
        }
    }
}

impl<SA, T> StorageClearable for SetMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
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
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
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
            self.api.clone(),
            &self.build_named_value_key(NODE_ID_IDENTIFIER, value),
        )
    }

    fn set_node_id(&self, value: &T, node_id: u32) {
        storage_set(
            self.api.clone(),
            &self.build_named_value_key(NODE_ID_IDENTIFIER, value),
            &node_id,
        );
    }

    fn clear_node_id(&self, value: &T) {
        storage_set(
            self.api.clone(),
            &self.build_named_value_key(NODE_ID_IDENTIFIER, value),
            &(),
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

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, T> EndpointResult for SetMapper<SA, T>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + EndpointResult,
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
impl<SA, T> TypeAbi for SetMapper<SA, T>
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
