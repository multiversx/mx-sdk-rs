use core::marker::PhantomData;

use elrond_codec::{
    multi_encode_iter_or_handle_err, EncodeErrorHandler, NestedDecode, NestedEncode, TopDecode,
    TopEncode, TopEncodeMulti, TopEncodeMultiOutput,
};

use super::{SetMapper, StorageMapper};
use crate::{
    abi::{TypeAbi, TypeName},
    api::StorageMapperApi,
    storage::{storage_get, storage_get_len, storage_set, StorageKey},
    types::{ManagedAddress, ManagedType, ManagedVec, ManagedVecItem, MultiValueEncoded},
};

const VALUE_SUFIX: &[u8] = b"_value";
const ID_SUFIX: &[u8] = b"_id";
const VALUE_TO_ID_SUFFIX: &[u8] = b"_value_to_id";
const ID_TO_VALUE_SUFFIX: &[u8] = b"_id_to_value";

/// A bi-directional map, from value to ids and viceversa.
/// This is so we can easily iterate over all users, using their ids.
/// Also holds the user count in sync. This is also necessary for iteration.
///
/// It also doesn't allow removing values. Once in, their ids are reserved forever.
pub struct BiDiMapper<SA, K, V>
where
    SA: StorageMapperApi,
    K: TopEncode
        + TopDecode
        + NestedEncode
        + NestedDecode
        + 'static
        + Default
        + PartialEq
        + ManagedVecItem,
    V: TopEncode
        + TopDecode
        + NestedEncode
        + NestedDecode
        + 'static
        + Default
        + PartialEq
        + ManagedVecItem,
{
    _phantom_api: PhantomData<SA>,
    id_set_mapper: SetMapper<SA, K>,
    value_set_mapper: SetMapper<SA, V>,
    base_key: StorageKey<SA>,
}

impl<SA, K, V> StorageMapper<SA> for BiDiMapper<SA, K, V>
where
    SA: StorageMapperApi,
    K: TopEncode
        + TopDecode
        + NestedEncode
        + NestedDecode
        + 'static
        + Default
        + PartialEq
        + ManagedVecItem,
    V: TopEncode
        + TopDecode
        + NestedEncode
        + NestedDecode
        + 'static
        + Default
        + PartialEq
        + ManagedVecItem,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        let mut id_key = base_key.clone();
        id_key.append_bytes(ID_SUFIX);

        let mut value_key = base_key.clone();
        value_key.append_bytes(VALUE_SUFIX);
        BiDiMapper {
            _phantom_api: PhantomData,
            id_set_mapper: SetMapper::<SA, K>::new(id_key),
            value_set_mapper: SetMapper::<SA, V>::new(value_key),
            base_key,
        }
    }
}

impl<SA, K, V> BiDiMapper<SA, K, V>
where
    SA: StorageMapperApi,
    K: TopEncode
        + TopDecode
        + NestedEncode
        + NestedDecode
        + 'static
        + Default
        + PartialEq
        + ManagedVecItem,
    V: TopEncode
        + TopDecode
        + NestedEncode
        + NestedDecode
        + 'static
        + Default
        + PartialEq
        + ManagedVecItem,
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

    pub fn get_value_unchecked(&self, id: &K) -> V {
        storage_get(self.get_value_key(id).as_ref())
    }

    pub fn get_value_or_zero(&self, id: &K) -> V {
        let key = self.get_value_key(id);
        if storage_get_len(key.as_ref()) > 0 {
            storage_get(key.as_ref())
        } else {
            V::default()
        }
    }

    fn set_value(&mut self, id: &K, value: &V) {
        storage_set(self.get_value_key(id).as_ref(), value);
    }

    pub fn insert(&mut self, id: K, value: V) -> bool {
        if self.id_set_mapper.contains(&id) || self.value_set_mapper.contains(&value) {
            return false;
        }
        self.set_id(&value, &id);
        self.set_value(&id, &value);

        self.id_set_mapper.insert(id);
        self.value_set_mapper.insert(value);
        true
    }

    pub fn get_all_values(&self) -> ManagedVec<SA, V> {
        let mut result = ManagedVec::new();
        for value in self.value_set_mapper.iter() {
            result.push(value);
        }
        result
    }

    pub fn get_all_ids(&self) -> ManagedVec<SA, K> {
        let mut result = ManagedVec::new();
        for id in self.id_set_mapper.iter() {
            result.push(id);
        }
        result
    }

    pub fn len(&self) -> usize {
        self.value_set_mapper.len()
    }
}

impl<SA, K, V> TopEncodeMulti for BiDiMapper<SA, K, V>
where
    SA: StorageMapperApi,
    K: TopEncode
        + TopDecode
        + NestedEncode
        + NestedDecode
        + 'static
        + Default
        + PartialEq
        + ManagedVecItem,
    V: TopEncode
        + TopDecode
        + NestedEncode
        + NestedDecode
        + 'static
        + Default
        + PartialEq
        + ManagedVecItem,
{
    type DecodeAs = MultiValueEncoded<SA, V>;

    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        let all_values = self.get_all_values();
        multi_encode_iter_or_handle_err(all_values.into_iter(), output, h)
    }
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
        + ManagedVecItem,
    V: TopEncode
        + TopDecode
        + NestedEncode
        + NestedDecode
        + 'static
        + Default
        + PartialEq
        + ManagedVecItem,
{
    fn type_name() -> TypeName {
        crate::types::MultiResultVec::<ManagedAddress<SA>>::type_name()
    }

    fn is_variadic() -> bool {
        true
    }
}
