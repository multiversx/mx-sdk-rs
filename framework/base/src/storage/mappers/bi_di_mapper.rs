use core::marker::PhantomData;

use crate::{
    abi::TypeAbiFrom,
    codec::{
        EncodeErrorHandler, NestedDecode, NestedEncode, TopDecode, TopEncode, TopEncodeMulti,
        TopEncodeMultiOutput, multi_encode_iter_or_handle_err, multi_types::MultiValue2,
    },
    types::ManagedAddress,
};

use super::{
    StorageMapper, StorageMapperFromAddress, UnorderedSetMapper,
    source::{CurrentStorage, StorageAddress},
    unordered_set_mapper,
};
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
    api::StorageMapperApi,
    storage::{StorageKey, storage_set},
    storage_clear,
    types::{ManagedType, MultiValueEncoded},
};

const VALUE_SUFFIX: &[u8] = b"_value";
const ID_SUFFIX: &[u8] = b"_id";
const VALUE_TO_ID_SUFFIX: &[u8] = b"_value_to_id";
const ID_TO_VALUE_SUFFIX: &[u8] = b"_id_to_value";

type Keys<'a, SA, T, A> = unordered_set_mapper::Iter<'a, SA, T, A>;

/// A storage mapper implementing a bidirectional map with one-to-one correspondence between IDs and values.
///
/// # Storage Layout
///
/// The `BiDiMapper` uses two `UnorderedSetMapper` instances and maintains bidirectional lookups:
///
/// 1. **ID set** (via `UnorderedSetMapper`):
///    - `base_key + "_id.len"` → count of IDs
///    - `base_key + "_id.item" + index` → ID at index (1-based)
///    - `base_key + "_id.index" + encoded_id` → index of ID
///
/// 2. **Value set** (via `UnorderedSetMapper`):
///    - `base_key + "_value.len"` → count of values
///    - `base_key + "_value.item" + index` → value at index (1-based)
///    - `base_key + "_value.index" + encoded_value` → index of value
///
/// 3. **Bidirectional mappings**:
///    - `base_key + "_value_to_id" + encoded_value` → corresponding ID
///    - `base_key + "_id_to_value" + encoded_id` → corresponding value
///
/// # Main Operations
///
/// - **Insert**: `insert(id, value)` - Adds bidirectional mapping. O(1). Fails if ID or value already exists.
/// - **Lookup ID**: `get_id(value)` - Retrieves ID for a value. O(1) with one storage read.
/// - **Lookup Value**: `get_value(id)` - Retrieves value for an ID. O(1) with one storage read.
/// - **Contains**: `contains_id(id)` / `contains_value(value)` - Checks existence. O(1).
/// - **Remove by ID**: `remove_by_id(id)` - Removes by ID, clears both directions. O(1).
/// - **Remove by Value**: `remove_by_value(value)` - Removes by value, clears both directions. O(1).
/// - **Batch Remove**: `remove_all_by_ids(iter)` / `remove_all_by_values(iter)` - Removes multiple entries.
/// - **Iteration**: `iter()` - Iterates over (ID, value) pairs; `get_all_ids()` / `get_all_values()` - specific direction.
///
/// # Uniqueness Guarantee
///
/// Both IDs and values must be unique:
/// - Each ID maps to exactly one value
/// - Each value maps to exactly one ID
/// - Inserting a duplicate ID or value fails and returns `false`
///
/// # Trade-offs
///
/// - **Pros**: O(1) bidirectional lookup; enforces one-to-one correspondence; efficient for reverse lookups.
/// - **Cons**: Double storage overhead (two sets + two mappings); removal changes element positions (swap_remove);
///   cannot have duplicate IDs or values; more complex than unidirectional maps.
///
/// # Use Cases
///
/// - Token ID ↔ Token name mappings
/// - User address ↔ Username associations
/// - NFT ID ↔ Metadata hash relationships
/// - Any scenario requiring efficient lookup in both directions
/// - Implementing invertible mappings
///
/// # Example
///
/// ```rust
/// # use multiversx_sc::storage::mappers::{StorageMapper, BiDiMapper};
/// # fn example<SA: multiversx_sc::api::StorageMapperApi>() {
/// # let mut mapper = BiDiMapper::<SA, u32, u64>::new(
/// #     multiversx_sc::storage::StorageKey::new(&b"token_mapping"[..])
/// # );
/// // Insert bidirectional mappings
/// assert!(mapper.insert(1, 100));
/// assert!(mapper.insert(2, 200));
/// assert!(mapper.insert(3, 300));
///
/// // Cannot insert duplicate ID or value
/// assert!(!mapper.insert(1, 400));  // ID 1 already exists
/// assert!(!mapper.insert(4, 100));  // Value 100 already exists
///
/// assert_eq!(mapper.len(), 3);
///
/// // Bidirectional lookup
/// assert_eq!(mapper.get_value(&2), 200);
/// assert_eq!(mapper.get_id(&200), 2);
///
/// // Check existence in both directions
/// assert!(mapper.contains_id(&1));
/// assert!(mapper.contains_value(&300));
///
/// // Remove by ID
/// assert!(mapper.remove_by_id(&2));
/// assert!(!mapper.contains_id(&2));
/// assert!(!mapper.contains_value(&200));  // Both directions removed
/// assert_eq!(mapper.len(), 2);
///
/// // Remove by value
/// assert!(mapper.remove_by_value(&100));
/// assert!(!mapper.contains_id(&1));
/// assert!(!mapper.contains_value(&100));
///
/// // Iterate over all mappings
/// for (id, value) in mapper.iter() {
///     // Process each bidirectional pair
/// }
///
/// // Iterate only IDs or values
/// for id in mapper.get_all_ids() {
///     // Process IDs
/// }
/// for value in mapper.get_all_values() {
///     // Process values
/// }
///
/// // Batch removal
/// mapper.remove_all_by_ids(vec![3]);
/// assert!(mapper.is_empty());
/// # }
/// ```
pub struct BiDiMapper<SA, K, V, A = CurrentStorage>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
    V: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
{
    _phantom_api: PhantomData<SA>,
    address: A,
    id_set_mapper: UnorderedSetMapper<SA, K, A>,
    value_set_mapper: UnorderedSetMapper<SA, V, A>,
    base_key: StorageKey<SA>,
}

impl<SA, K, V> StorageMapper<SA> for BiDiMapper<SA, K, V, CurrentStorage>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
    V: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        let mut id_key = base_key.clone();
        id_key.append_bytes(ID_SUFFIX);

        let mut value_key = base_key.clone();
        value_key.append_bytes(VALUE_SUFFIX);
        BiDiMapper {
            _phantom_api: PhantomData,
            address: CurrentStorage,
            id_set_mapper: UnorderedSetMapper::<SA, K>::new(id_key),
            value_set_mapper: UnorderedSetMapper::<SA, V>::new(value_key),
            base_key,
        }
    }
}

impl<SA, K, V> StorageMapperFromAddress<SA> for BiDiMapper<SA, K, V, ManagedAddress<SA>>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
    V: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
{
    fn new_from_address(address: ManagedAddress<SA>, base_key: StorageKey<SA>) -> Self {
        let mut id_key = base_key.clone();
        id_key.append_bytes(ID_SUFFIX);

        let mut value_key = base_key.clone();
        value_key.append_bytes(VALUE_SUFFIX);
        BiDiMapper {
            _phantom_api: PhantomData,
            address: address.clone(),
            id_set_mapper: UnorderedSetMapper::new_from_address(address.clone(), id_key),
            value_set_mapper: UnorderedSetMapper::new_from_address(address, value_key),
            base_key,
        }
    }
}

impl<SA, K, V, A> BiDiMapper<SA, K, V, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
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
        self.address
            .address_storage_get(self.get_id_key(value).as_ref())
    }

    pub fn get_value(&self, id: &K) -> V {
        self.address
            .address_storage_get(self.get_value_key(id).as_ref())
    }

    pub fn contains_id(&self, id: &K) -> bool {
        self.id_set_mapper.contains(id)
    }

    pub fn contains_value(&self, value: &V) -> bool {
        self.value_set_mapper.contains(value)
    }

    pub fn get_all_values(&self) -> unordered_set_mapper::Iter<'_, SA, V, A> {
        self.value_set_mapper.iter()
    }

    pub fn get_all_ids(&self) -> unordered_set_mapper::Iter<'_, SA, K, A> {
        self.id_set_mapper.iter()
    }

    pub fn iter(&self) -> Iter<'_, SA, K, V, A> {
        Iter::new(self)
    }

    pub fn is_empty(&self) -> bool {
        self.value_set_mapper.is_empty()
    }

    pub fn len(&self) -> usize {
        self.value_set_mapper.len()
    }
}

impl<SA, K, V> BiDiMapper<SA, K, V, CurrentStorage>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
    V: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
{
    fn set_id(&mut self, value: &V, id: &K) {
        storage_set(self.get_id_key(value).as_ref(), id);
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
}

impl<'a, SA, K, V, A> IntoIterator for &'a BiDiMapper<SA, K, V, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
    V: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
{
    type Item = (K, V);

    type IntoIter = Iter<'a, SA, K, V, A>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, SA, K, V, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
    V: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
{
    key_iter: Keys<'a, SA, K, A>,
    hash_map: &'a BiDiMapper<SA, K, V, A>,
}

impl<'a, SA, K, V, A> Iter<'a, SA, K, V, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
    V: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
{
    fn new(hash_map: &'a BiDiMapper<SA, K, V, A>) -> Iter<'a, SA, K, V, A> {
        Iter {
            key_iter: hash_map.get_all_ids(),
            hash_map,
        }
    }
}

impl<SA, K, V, A> Iterator for Iter<'_, SA, K, V, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
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

impl<SA, K, V> TopEncodeMulti for BiDiMapper<SA, K, V, CurrentStorage>
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

impl<SA, K, V> TypeAbiFrom<BiDiMapper<SA, K, V, CurrentStorage>>
    for MultiValueEncoded<SA, MultiValue2<K, V>>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
    V: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
{
}

impl<SA, K, V> TypeAbiFrom<Self> for BiDiMapper<SA, K, V, CurrentStorage>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
    V: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static + Default + PartialEq,
{
}

impl<SA, K, V> TypeAbi for BiDiMapper<SA, K, V, CurrentStorage>
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
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        MultiValueEncoded::<SA, MultiValue2<K, V>>::type_name()
    }

    fn type_name_rust() -> TypeName {
        MultiValueEncoded::<SA, MultiValue2<K, V>>::type_name_rust()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        K::provide_type_descriptions(accumulator);
        V::provide_type_descriptions(accumulator);
    }
    fn is_variadic() -> bool {
        true
    }
}
