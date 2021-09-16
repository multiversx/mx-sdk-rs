pub use super::queue_mapper::Iter;
use super::{QueueMapper, SingleValueMapper, StorageClearable, StorageMapper};
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
    api::{EndpointFinishApi, ErrorApi, ManagedTypeApi, StorageReadApi, StorageWriteApi},
    io::EndpointResult,
    storage::StorageKey,
    types::MultiResultVec,
};
use alloc::vec::Vec;
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
            self.node_id(&value).clear();
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

    fn node_id(&self, value: &T) -> SingleValueMapper<SA, u32> {
        SingleValueMapper::new(
            self.api.clone(),
            self.build_named_value_key(NODE_ID_IDENTIFIER, value),
        )
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
        self.node_id(value).get() != NULL_ENTRY
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
        self.node_id(&value).set(&new_node_id);
        true
    }

    /// Removes a value from the set. Returns whether the value was
    /// present in the set.
    pub fn remove(&mut self, value: &T) -> bool {
        let node_id = self.node_id(value).get();
        if node_id == NULL_ENTRY {
            return false;
        }
        self.queue_mapper.remove_by_node_id(node_id);
        self.node_id(value).clear();
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
        let v: Vec<T> = self.iter().collect();
        MultiResultVec::<T>::from(v).finish(api);
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
