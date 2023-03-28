use core::marker::PhantomData;

use crate::codec::{
    multi_encode_iter_or_handle_err, multi_types::MultiValue2, CodecFrom, EncodeErrorHandler,
    NestedDecode, NestedEncode, TopDecode, TopEncode, TopEncodeMulti, TopEncodeMultiOutput,
};

use super::{unordered_set_mapper, StorageMapper, UnorderedSetMapper};
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
    api::StorageMapperApi,
    storage::{storage_get, storage_set, StorageKey},
    storage_clear,
    types::{ManagedType, MultiValueEncoded},
};

const VALUE_SUFIX: &[u8] = b"_value";
const ID_SUFIX: &[u8] = b"_id";
const VALUE_TO_ID_SUFFIX: &[u8] = b"_value_to_id";
const ID_TO_VALUE_SUFFIX: &[u8] = b"_id_to_value";

type Keys<'a, SA, T> = unordered_set_mapper::Iter<'a, SA, T>;

/// A bi-directional map, from values to ids and viceversa.
/// The mapper is based on UnorderedSetMapper, reason why the remove is done by swap_remove
pub struct BiDiMapper<SA, K, V>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
    V: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
{
    _phantom_api: PhantomData<SA>,
    id_set_mapper: UnorderedSetMapper<SA, K>,
    value_set_mapper: UnorderedSetMapper<SA, V>,
    base_key: StorageKey<SA>,
}

impl<SA, K, V> StorageMapper<SA> for BiDiMapper<SA, K, V>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
    V: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        let mut id_key = base_key.clone();
        id_key.append_bytes(ID_SUFIX);

        let mut value_key = base_key.clone();
        value_key.append_bytes(VALUE_SUFIX);
        BiDiMapper {
            _phantom_api: PhantomData,
            id_set_mapper: UnorderedSetMapper::<SA, K>::new(id_key),
            value_set_mapper: UnorderedSetMapper::<SA, V>::new(value_key),
            base_key,
        }
    }
}

impl<SA, K, V> BiDiMapper<SA, K, V>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
    V: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
{
    fn get_id_key(&self, value: &V) -> StorageKey<SA> {
        let mut key = self.base_key.clone();
        key.append_bytes(VALUE_TO_ID_SUFFIX);
        key.append_item(value);
        key
    }

    fn get_value_key(&self, key: &K) -> StorageKey<SA> {
        let mut value = self.base_key.clone();
        value.append_bytes(ID_TO_VALUE_SUFFIX);
        value.append_item(&key);
        value
    }

    pub fn get_id(&self, value: &V) -> K {
        storage_get(self.get_id_key(value).as_ref())
    }

    fn set_id(&mut self, value: &V, id: &K) {
        storage_set(self.get_id_key(value).as_ref(), id);
    }

    pub fn get_value(&self, id: &K) -> V {
        storage_get(self.get_value_key(id).as_ref())
    }

    fn set_value(&mut self, id: &K, value: &V) {
        storage_set(self.get_value_key(id).as_ref(), value);
    }

    fn clear_id_by_value(&self, value: &V) {
        storage_clear(self.get_id_key(value).as_ref());
    }
    fn clear_value_by_id(&self, id: &K) {
        storage_clear(self.get_value_key(id).as_ref());
    }

    pub fn contains_id(&self, id: &K) -> bool {
        self.id_set_mapper.contains(id)
    }

    pub fn contains_value(&self, value: &V) -> bool {
        self.value_set_mapper.contains(value)
    }

    pub fn insert(&mut self, id: K, value: V) -> bool {
        if self.contains_id(&id) || self.contains_value(&value) {
            return false;
        }
        self.set_id(&value, &id);
        self.set_value(&id, &value);

        self.id_set_mapper.insert(id);
        self.value_set_mapper.insert(value);
        true
    }

    pub fn remove_by_id(&mut self, id: &K) -> bool {
        if self.id_set_mapper.swap_remove(id) {
            let value = self.get_value(id);
            self.clear_id_by_value(&value);
            self.clear_value_by_id(id);
            storage_clear(self.get_value_key(id).as_ref());
            self.value_set_mapper.swap_remove(&value);
            return true;
        }
        false
    }
    pub fn remove_by_value(&mut self, value: &V) -> bool {
        if self.value_set_mapper.swap_remove(value) {
            let id = self.get_id(value);
            self.clear_id_by_value(value);
            self.clear_value_by_id(&id);
            self.id_set_mapper.swap_remove(&id);
            return true;
        }
        false
    }

    pub fn remove_all_by_ids<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = K>,
    {
        for item in iter {
            self.remove_by_id(&item);
        }
    }

    pub fn remove_all_by_values<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = V>,
    {
        for item in iter {
            self.remove_by_value(&item);
        }
    }

    pub fn get_all_values(&self) -> unordered_set_mapper::Iter<SA, V> {
        self.value_set_mapper.iter()
    }

    pub fn get_all_ids(&self) -> unordered_set_mapper::Iter<SA, K> {
        self.id_set_mapper.iter()
    }

    pub fn iter(&self) -> Iter<SA, K, V> {
        Iter::new(self)
    }

    pub fn is_empty(&self) -> bool {
        self.value_set_mapper.is_empty()
    }

    pub fn len(&self) -> usize {
        self.value_set_mapper.len()
    }
}

impl<'a, SA, K, V> IntoIterator for &'a BiDiMapper<SA, K, V>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
    V: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
{
    type Item = (K, V);

    type IntoIter = Iter<'a, SA, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, SA, K, V>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
    V: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
{
    key_iter: Keys<'a, SA, K>,
    hash_map: &'a BiDiMapper<SA, K, V>,
}

impl<'a, SA, K, V> Iter<'a, SA, K, V>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
    V: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
{
    fn new(hash_map: &'a BiDiMapper<SA, K, V>) -> Iter<'a, SA, K, V> {
        Iter {
            key_iter: hash_map.get_all_ids(),
            hash_map,
        }
    }
}

impl<'a, SA, K, V> Iterator for Iter<'a, SA, K, V>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
    V: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
{
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<(K, V)> {
        if let Some(key) = self.key_iter.next() {
            let value = self.hash_map.get_value(&key);
            return Some((key, value));
        }
        None
    }
}

impl<SA, K, V> TopEncodeMulti for BiDiMapper<SA, K, V>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
    V: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        let iter = self.iter().map(MultiValue2::<K, V>::from);
        multi_encode_iter_or_handle_err(iter, output, h)
    }
}

impl<SA, K, V> CodecFrom<BiDiMapper<SA, K, V>> for MultiValueEncoded<SA, MultiValue2<K, V>>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
    V: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
{
}

impl<SA, K, V> TypeAbi for BiDiMapper<SA, K, V>
where
    SA: StorageMapperApi,
    K: TopEncode
        + TopDecode
        + NestedEncode
        + NestedDecode
        + 'static
        + Default
        + PartialEq
        + TypeAbi,
    V: TopEncode
        + TopDecode
        + NestedEncode
        + NestedDecode
        + 'static
        + Default
        + PartialEq
        + TypeAbi,
{
    fn type_name() -> TypeName {
        MultiValueEncoded::<SA, MultiValue2<K, V>>::type_name()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        K::provide_type_descriptions(accumulator);
        V::provide_type_descriptions(accumulator);
    }
    fn is_variadic() -> bool {
        true
    }
}
