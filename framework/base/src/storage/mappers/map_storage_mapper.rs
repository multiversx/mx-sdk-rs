use core::marker::PhantomData;

use super::{
    SetMapper, StorageClearable, StorageMapper, StorageMapperFromAddress,
    set_mapper::{self},
    source::{CurrentStorage, StorageAddress},
};
use crate::{
    api::StorageMapperApi,
    codec::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    contract_base::ErrorHelper,
    storage::{self, StorageKey},
    types::ManagedAddress,
};

const MAPPED_STORAGE_VALUE_IDENTIFIER: &[u8] = b".storage";
type Keys<'a, SA, A, T> = set_mapper::Iter<'a, SA, A, T>;

/// A storage mapper implementing a map where values are themselves storage mappers (nested storage).
///
/// # Storage Layout
///
/// The `MapStorageMapper` uses a `SetMapper` to track keys and creates nested storage mappers for values:
///
/// 1. **Key tracking** (via `SetMapper`):
///    - `base_key + ".info"` → `QueueMapperInfo` metadata for the key set
///    - `base_key + ".node_links" + node_id` → node structure (prev/next pointers)
///    - `base_key + ".value" + node_id` → key value
///    - `base_key + ".node_id" + encoded_key` → node ID lookup
///
/// 2. **Nested storage mappers**:
///    - `base_key + ".storage" + encoded_key` → acts as base key for nested mapper `V`
///    - Each nested mapper has its own sub-structure (e.g., if `V` is `VecMapper<SA, u32>`,
///      then `base_key + ".storage" + key + ".len"`, `base_key + ".storage" + key + ".item1"`, etc.)
///
/// # Main Operations
///
/// - **Insert**: `insert_default(key)` - Adds a key with default-initialized nested mapper. O(1).
/// - **Remove**: `remove(key)` - Removes key and clears all nested storage. O(n) where n = nested mapper size.
/// - **Lookup**: `get(key)` - Returns the nested mapper for a key. O(1), lazy-creates mapper instance.
/// - **Contains**: `contains_key(key)` - Checks if key exists. O(1) with one storage read.
/// - **Entry API**: `entry(key)` - Provides entry-based manipulation for conditional initialization.
/// - **Iteration**: `iter()` - Iterates over (key, mapper) pairs; `keys()` - keys only; `values()` - mappers only.
///
/// # Key Characteristics
///
/// - **Nested Storage**: Values are not simple data but entire storage mappers (e.g., `VecMapper`, `SetMapper`)
/// - **Lazy Initialization**: Nested mappers are created on-demand when accessed
/// - **Composition**: Enables complex hierarchical storage structures (e.g., map of lists, map of maps)
///
/// # Comparison with MapMapper
///
/// - **MapMapper**: Stores simple values directly (`MapMapper<SA, K, V>` where V is a plain type)
/// - **MapStorageMapper**: Stores nested mappers (`MapStorageMapper<SA, K, V>` where V is a StorageMapper)
///
/// # Trade-offs
///
/// - **Pros**: Enables powerful nested data structures; each nested mapper is independent; type-safe composition.
/// - **Cons**: Higher storage overhead; removal is more expensive; complexity increases with nesting depth;
///   each access creates mapper instance (lightweight but not zero-cost).
///
/// # Use Cases
///
/// - User-specific collections (e.g., map user address → their token balances as VecMapper)
/// - Category-based grouping (e.g., map category → items as SetMapper)
/// - Multi-level hierarchies (e.g., map project → map milestone → tasks as nested Mappers)
/// - Per-entity state machines (e.g., map entity_id → its own state storage)
///
/// # Example
///
/// ```rust
/// # use multiversx_sc::storage::mappers::{StorageMapper, MapStorageMapper, VecMapper};
/// # use multiversx_sc::types::ManagedAddress;
/// # fn example<SA: multiversx_sc::api::StorageMapperApi>(user1: ManagedAddress<SA>, user2: ManagedAddress<SA>) {
/// # let mut mapper = MapStorageMapper::<SA, ManagedAddress<SA>, VecMapper<SA, u64>>::new(
/// #     multiversx_sc::storage::StorageKey::new(&b"user_tokens"[..])
/// # );
/// // Create nested VecMapper for each user
/// mapper.insert_default(user1.clone());
/// mapper.insert_default(user2.clone());
///
/// // Get user's token list and add tokens
/// if let Some(mut user1_tokens) = mapper.get(&user1) {
///     user1_tokens.push(&100);
///     user1_tokens.push(&200);
/// }
///
/// if let Some(mut user2_tokens) = mapper.get(&user2) {
///     user2_tokens.push(&300);
/// }
///
/// // Check and access nested mapper
/// assert!(mapper.contains_key(&user1));
/// if let Some(user1_tokens) = mapper.get(&user1) {
///     assert_eq!(user1_tokens.len(), 2);
///     assert_eq!(user1_tokens.get(1), 100);
/// }
///
/// // Use entry API
/// mapper.entry(user1.clone())
///     .and_modify(|tokens| { tokens.push(&250); });
///
/// // Iterate over all users and their token lists
/// for (user, tokens) in mapper.iter() {
///     for token_id in tokens.iter() {
///         // Process each user's tokens
///     }
/// }
///
/// // Remove user and all their tokens
/// mapper.remove(&user2);
/// assert!(!mapper.contains_key(&user2));
/// # }
/// ```
pub struct MapStorageMapper<SA, K, V, A = CurrentStorage>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
    A: StorageAddress<SA>,
    V: StorageMapper<SA> + StorageClearable,
{
    _phantom_api: PhantomData<SA>,
    base_key: StorageKey<SA>,
    keys_set: SetMapper<SA, K, A>,
    _phantom_value: PhantomData<V>,
}

impl<SA, K, V> StorageMapper<SA> for MapStorageMapper<SA, K, V, CurrentStorage>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode,
    V: StorageMapper<SA> + StorageClearable,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        Self {
            _phantom_api: PhantomData,
            base_key: base_key.clone(),
            keys_set: SetMapper::new(base_key),
            _phantom_value: PhantomData,
        }
    }
}

impl<SA, K, V> StorageClearable for MapStorageMapper<SA, K, V, CurrentStorage>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode,
    V: StorageMapper<SA> + StorageClearable,
{
    fn clear(&mut self) {
        for mut value in self.values() {
            value.clear();
        }
        self.keys_set.clear();
    }
}

impl<SA, K, V> MapStorageMapper<SA, K, V, CurrentStorage>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode,
    V: StorageMapper<SA> + StorageClearable,
{
    /// Adds a default value for the key, if it is not already present.
    ///
    /// If the map did not have this key present, `true` is returned.
    ///
    /// If the map did have this value present, `false` is returned.
    pub fn insert_default(&mut self, k: K) -> bool {
        self.keys_set.insert(k)
    }

    /// Removes the entry from the map.
    ///
    /// If the entry was removed, `true` is returned.
    ///
    /// If the map didn't contain an entry with this key, `false` is returned.
    pub fn remove(&mut self, k: &K) -> bool {
        if self.keys_set.remove(k) {
            self.get_mapped_storage_value(k).clear();
            return true;
        }
        false
    }
}

impl<SA, K, V> StorageMapperFromAddress<SA> for MapStorageMapper<SA, K, V, ManagedAddress<SA>>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode,
    V: StorageMapper<SA> + StorageClearable,
{
    fn new_from_address(address: ManagedAddress<SA>, base_key: StorageKey<SA>) -> Self {
        MapStorageMapper {
            _phantom_api: PhantomData,
            base_key: base_key.clone(),
            keys_set: SetMapper::new_from_address(address, base_key),
            _phantom_value: PhantomData,
        }
    }
}

impl<SA, A, K, V> MapStorageMapper<SA, K, V, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode,
    V: StorageMapper<SA> + StorageClearable,
{
    fn build_named_key(&self, name: &[u8], key: &K) -> StorageKey<SA> {
        let mut named_key = self.base_key.clone();
        named_key.append_bytes(name);
        named_key.append_item(key);
        named_key
    }

    fn get_mapped_storage_value(&self, key: &K) -> V {
        let key = self.build_named_key(MAPPED_STORAGE_VALUE_IDENTIFIER, key);
        <V as storage::mappers::StorageMapper<SA>>::new(key)
    }

    /// Gets a reference to the value in the entry.
    pub fn get(&self, k: &K) -> Option<V> {
        if self.keys_set.contains(k) {
            return Some(self.get_mapped_storage_value(k));
        }
        None
    }

    pub fn keys(&self) -> Keys<'_, SA, A, K> {
        self.keys_set.iter()
    }

    /// Returns `true` if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.keys_set.is_empty()
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.keys_set.len()
    }

    /// Returns `true` if the map contains a value for the specified key.
    pub fn contains_key(&self, k: &K) -> bool {
        self.keys_set.contains(k)
    }

    /// Gets the given key's corresponding entry in the map for in-place manipulation.
    pub fn entry(&mut self, key: K) -> Entry<'_, SA, A, K, V> {
        if self.contains_key(&key) {
            Entry::Occupied(OccupiedEntry {
                key,
                map: self,
                _marker: PhantomData,
            })
        } else {
            Entry::Vacant(VacantEntry {
                key,
                map: self,
                _marker: PhantomData,
            })
        }
    }

    /// An iterator visiting all values in arbitrary order.
    /// The iterator element type is `&'a V`.
    pub fn values(&self) -> Values<'_, SA, A, K, V> {
        Values::new(self)
    }

    /// An iterator visiting all key-value pairs in arbitrary order.
    /// The iterator element type is `(&'a K, &'a V)`.
    pub fn iter(&self) -> Iter<'_, SA, A, K, V> {
        Iter::new(self)
    }
}

impl<'a, SA, A, K, V> IntoIterator for &'a MapStorageMapper<SA, K, V, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
    V: StorageMapper<SA> + StorageClearable,
{
    type Item = (K, V);

    type IntoIter = Iter<'a, SA, A, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, SA, A, K, V>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
    A: StorageAddress<SA>,
    V: StorageMapper<SA> + StorageClearable,
{
    key_iter: Keys<'a, SA, A, K>,
    hash_map: &'a MapStorageMapper<SA, K, V, A>,
}

impl<'a, SA, A, K, V> Iter<'a, SA, A, K, V>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
    V: StorageMapper<SA> + StorageClearable,
{
    fn new(hash_map: &'a MapStorageMapper<SA, K, V, A>) -> Iter<'a, SA, A, K, V> {
        Iter {
            key_iter: hash_map.keys(),
            hash_map,
        }
    }
}

impl<SA, A, K, V> Iterator for Iter<'_, SA, A, K, V>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
    V: StorageMapper<SA> + StorageClearable,
{
    type Item = (K, V);

    #[inline]
    fn next(&mut self) -> Option<(K, V)> {
        if let Some(key) = self.key_iter.next() {
            let Some(value) = self.hash_map.get(&key) else {
                ErrorHelper::<SA>::signal_error_with_message("missing key")
            };
            return Some((key, value));
        }
        None
    }
}

pub struct Values<'a, SA, A, K, V>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
    A: StorageAddress<SA>,
    V: StorageMapper<SA> + StorageClearable,
{
    key_iter: Keys<'a, SA, A, K>,
    hash_map: &'a MapStorageMapper<SA, K, V, A>,
}

impl<'a, SA, A, K, V> Values<'a, SA, A, K, V>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
    V: StorageMapper<SA> + StorageClearable,
{
    fn new(hash_map: &'a MapStorageMapper<SA, K, V, A>) -> Values<'a, SA, A, K, V> {
        Values {
            key_iter: hash_map.keys(),
            hash_map,
        }
    }
}

impl<SA, A, K, V> Iterator for Values<'_, SA, A, K, V>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
    V: StorageMapper<SA> + StorageClearable,
{
    type Item = V;

    #[inline]
    fn next(&mut self) -> Option<V> {
        if let Some(key) = self.key_iter.next() {
            let value = self.hash_map.get(&key).unwrap();
            return Some(value);
        }
        None
    }
}

pub enum Entry<'a, SA, A, K: 'a, V: 'a>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
    A: StorageAddress<SA>,
    V: StorageMapper<SA> + StorageClearable,
{
    /// A vacant entry.
    Vacant(VacantEntry<'a, SA, A, K, V>),

    /// An occupied entry.
    Occupied(OccupiedEntry<'a, SA, A, K, V>),
}

/// A view into a vacant entry in a `MapStorageMapper`.
/// It is part of the [`Entry`] enum.
pub struct VacantEntry<'a, SA, A, K: 'a, V: 'a>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
    A: StorageAddress<SA>,
    V: StorageMapper<SA> + StorageClearable,
{
    pub(super) key: K,
    pub(super) map: &'a mut MapStorageMapper<SA, K, V, A>,

    // Be invariant in `K` and `V`
    pub(super) _marker: PhantomData<&'a mut (K, V)>,
}

/// A view into an occupied entry in a `MapStorageMapper`.
/// It is part of the [`Entry`] enum.
pub struct OccupiedEntry<'a, SA, A, K: 'a, V: 'a>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
    A: StorageAddress<SA>,
    V: StorageMapper<SA> + StorageClearable,
{
    pub(super) key: K,
    pub(super) map: &'a mut MapStorageMapper<SA, K, V, A>,

    // Be invariant in `K` and `V`
    pub(super) _marker: PhantomData<&'a mut (K, V)>,
}

impl<'a, SA, K, V> Entry<'a, SA, CurrentStorage, K, V>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + Clone + 'static,
    V: StorageMapper<SA> + StorageClearable,
{
    /// Ensures a value is in the entry by inserting the default if empty, and returns
    /// an `OccupiedEntry`.
    pub fn or_insert_default(self) -> OccupiedEntry<'a, SA, CurrentStorage, K, V> {
        match self {
            Entry::Occupied(entry) => entry,
            Entry::Vacant(entry) => entry.insert_default(),
        }
    }

    /// Returns a reference to this entry's key.
    pub fn key(&self) -> &K {
        match *self {
            Entry::Occupied(ref entry) => entry.key(),
            Entry::Vacant(ref entry) => entry.key(),
        }
    }

    /// Provides in-place mutable access to an occupied entry before any
    /// potential inserts into the map.
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        match self {
            Entry::Occupied(mut entry) => {
                entry.update(f);
                Entry::Occupied(entry)
            }
            Entry::Vacant(entry) => Entry::Vacant(entry),
        }
    }
}

impl<'a, SA, K, V> Entry<'a, SA, CurrentStorage, K, V>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + Clone + 'static,
    V: StorageMapper<SA> + StorageClearable,
{
    /// Ensures a value is in the entry by inserting the default value if empty,
    /// and returns an `OccupiedEntry`.
    pub fn or_default(self) -> OccupiedEntry<'a, SA, CurrentStorage, K, V> {
        match self {
            Entry::Occupied(entry) => entry,
            Entry::Vacant(entry) => entry.insert_default(),
        }
    }
}

impl<SA, A, K, V> VacantEntry<'_, SA, A, K, V>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + Clone + 'static,
    V: StorageMapper<SA> + StorageClearable,
{
    /// Gets a reference to the key that would be used when inserting a value
    /// through the VacantEntry.
    pub fn key(&self) -> &K {
        &self.key
    }
}

impl<'a, SA, K, V> VacantEntry<'a, SA, CurrentStorage, K, V>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + Clone + 'static,
    V: StorageMapper<SA> + StorageClearable,
{
    /// Sets the value of the entry with the `VacantEntry`'s key,
    /// and returns an `OccupiedEntry`.
    pub fn insert_default(self) -> OccupiedEntry<'a, SA, CurrentStorage, K, V> {
        self.map.insert_default(self.key.clone());
        OccupiedEntry {
            key: self.key,
            map: self.map,
            _marker: PhantomData,
        }
    }
}

impl<SA, A, K, V> OccupiedEntry<'_, SA, A, K, V>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + Clone + 'static,
    V: StorageMapper<SA> + StorageClearable,
{
    /// Gets a reference to the key in the entry.
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Gets the value in the entry.
    pub fn get(&self) -> V {
        self.map.get(&self.key).unwrap()
    }
}

impl<SA, K, V> OccupiedEntry<'_, SA, CurrentStorage, K, V>
where
    SA: StorageMapperApi,
    K: TopEncode + TopDecode + NestedEncode + NestedDecode + Clone + 'static,
    V: StorageMapper<SA> + StorageClearable,
{
    /// Syntactic sugar, to more compactly express a get, update and set in one line.
    /// Takes whatever lies in storage, apples the given closure and saves the final value back to storage.
    /// Propagates the return value of the given function.
    pub fn update<R, F: FnOnce(&mut V) -> R>(&mut self, f: F) -> R {
        let mut value = self.get();
        f(&mut value)
    }

    /// Removes the entry from the map.
    pub fn remove(self) {
        self.map.remove(&self.key);
    }
}
