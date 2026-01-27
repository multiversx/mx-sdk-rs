use core::marker::PhantomData;

pub use super::queue_mapper::Iter;
use super::{QueueMapper, StorageClearable, StorageMapper, StorageMapperFromAddress};
use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeDescriptionContainer, TypeName},
    api::StorageMapperApi,
    codec::{
        self, EncodeErrorHandler, NestedDecode, NestedEncode, TopDecode, TopEncode, TopEncodeMulti,
        TopEncodeMultiOutput, multi_encode_iter_or_handle_err,
    },
    storage::{
        StorageKey,
        mappers::source::{CurrentStorage, StorageAddress},
        storage_set,
    },
    types::{ManagedAddress, ManagedType, MultiValueEncoded},
};

const NULL_ENTRY: u32 = 0;
const NODE_ID_IDENTIFIER: &[u8] = b".node_id";

/// A storage mapper implementing an ordered set with efficient membership testing and iteration.
///
/// # Storage Layout
///
/// The `SetMapper` uses a `QueueMapper` for ordering and separate storage for value-to-node mapping:
///
/// 1. **Ordered elements** (via `QueueMapper`):
///    - `base_key + ".info"` → metadata (length, front, back, new node counter)
///    - `base_key + ".node_links" + node_id` → node structure (previous, next)
///    - `base_key + ".value" + node_id` → the stored value
///
/// 2. **Value lookup** (for fast membership testing):
///    - `base_key + ".node_id" + encoded_value` → node ID (0 means not present)
///
/// This dual structure enables both O(1) membership testing and ordered iteration.
///
/// # Main Operations
///
/// - **Insert**: `insert(value)` - Adds a value if not already present. O(1) with storage writes.
/// - **Remove**: `remove(value)` - Removes a value from the set. O(1) with storage writes.
/// - **Contains**: `contains(value)` - Checks membership. O(1) with one storage read.
/// - **Iteration**: `iter()` - Iterates in insertion order; `iter_from(value)` - starts from specific value.
/// - **Navigation**: `next(value)` / `previous(value)` - Gets adjacent elements in insertion order.
/// - **Batch**: `remove_all(iter)` - Removes multiple values efficiently.
///
/// # Insertion Order
///
/// Unlike typical sets, `SetMapper` maintains **insertion order** - elements are stored in the order
/// they were added. This makes it a hybrid between a set and a sequence.
///
/// # Trade-offs
///
/// - **Pros**: O(1) insert, remove, and contains; maintains insertion order; efficient iteration.
/// - **Cons**: Higher storage overhead than `UnorderedSetMapper` (uses QueueMapper internally);
///   no random access; removed elements leave gaps in node ID space.
///
/// # Comparison with UnorderedSetMapper
///
/// - **SetMapper**: Maintains insertion order, uses queue-based structure, slightly higher storage cost
/// - **UnorderedSetMapper**: No ordering guarantees, uses vec-based structure, more compact storage
///
/// # Use Cases
///
/// - Whitelists/blacklists where insertion order matters
/// - Unique collections requiring ordered iteration
/// - Sets where you need to navigate between elements
/// - Scenarios requiring both fast lookup and sequential processing
///
/// # Example
///
/// ```rust
/// # use multiversx_sc::storage::mappers::{StorageMapper, SetMapper};
/// # fn example<SA: multiversx_sc::api::StorageMapperApi>() {
/// # let mut mapper = SetMapper::<SA, u32>::new(
/// #     multiversx_sc::storage::StorageKey::new(&b"whitelist"[..])
/// # );
/// // Insert values
/// assert!(mapper.insert(100));
/// assert!(mapper.insert(200));
/// assert!(mapper.insert(300));
/// assert!(!mapper.insert(200));  // Already exists, returns false
///
/// assert_eq!(mapper.len(), 3);
/// assert!(mapper.contains(&200));
///
/// // Navigate between elements
/// let next = mapper.next(&200);
/// assert_eq!(next, Some(300));
///
/// let prev = mapper.previous(&200);
/// assert_eq!(prev, Some(100));
///
/// // Remove element
/// assert!(mapper.remove(&200));
/// assert!(!mapper.contains(&200));
/// assert_eq!(mapper.len(), 2);
///
/// // Iterate in insertion order
/// for value in mapper.iter() {
///     // Process in order: 100, 300
/// }
///
/// // Batch removal
/// mapper.remove_all(vec![100, 300]);
/// assert!(mapper.is_empty());
/// # }
/// ```
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

impl<SA, T> StorageMapperFromAddress<SA> for SetMapper<SA, T, ManagedAddress<SA>>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode,
{
    fn new_from_address(address: ManagedAddress<SA>, base_key: StorageKey<SA>) -> Self {
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
    pub fn iter(&self) -> Iter<'_, SA, A, T> {
        self.queue_mapper.iter()
    }

    pub fn iter_from(&self, value: &T) -> Iter<'_, SA, A, T> {
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

impl<SA, T> TypeAbiFrom<SetMapper<SA, T, CurrentStorage>> for MultiValueEncoded<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + NestedEncode + NestedDecode + 'static,
{
}

impl<SA, T> TypeAbiFrom<Self> for SetMapper<SA, T, CurrentStorage>
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
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        crate::abi::type_name_variadic::<T>()
    }

    fn type_name_rust() -> TypeName {
        crate::abi::type_name_multi_value_encoded::<T>()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        true
    }
}
