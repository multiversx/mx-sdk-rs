use super::{
    set_mapper::{CurrentStorage, StorageAddress},
    StorageClearable, StorageMapper,
};
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
    api::{ErrorApiImpl, StorageMapperApi},
    codec::{
        multi_encode_iter_or_handle_err, CodecFrom, EncodeErrorHandler, TopDecode, TopEncode,
        TopEncodeMulti, TopEncodeMultiOutput,
    },
    storage::{storage_clear, storage_set, StorageKey},
    types::{ManagedAddress, ManagedType, MultiValueEncoded},
};
use core::{marker::PhantomData, usize};

const ITEM_SUFFIX: &[u8] = b".item";
const LEN_SUFFIX: &[u8] = b".len";

static INDEX_OUT_OF_RANGE_ERR_MSG: &[u8] = b"index out of range";

/// Manages a list of items of the same type.
/// Saves each of the items under a separate key in storage.
/// To produce each individual key, it concatenates the main key with a serialized 4-byte index.
/// Indexes start from 1, instead of 0. (We avoid 0-value indexes to prevent confusion between an uninitialized variable and zero.)
/// It also stores the count separately, at what would be index 0.
/// The count is always kept in sync automatically.
pub struct VecMapper<SA, T, A = CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + 'static,
{
    _phantom_api: PhantomData<SA>,
    address: A,
    base_key: StorageKey<SA>,
    len_key: StorageKey<SA>,
    _phantom_item: PhantomData<T>,
}

impl<SA, T> StorageMapper<SA> for VecMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        let mut len_key = base_key.clone();
        len_key.append_bytes(LEN_SUFFIX);

        VecMapper {
            _phantom_api: PhantomData,
            address: CurrentStorage,
            base_key,
            len_key,
            _phantom_item: PhantomData,
        }
    }
}

impl<SA, T> VecMapper<SA, T, ManagedAddress<SA>>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
    pub fn new_from_address(address: ManagedAddress<SA>, base_key: StorageKey<SA>) -> Self {
        let mut len_key = base_key.clone();
        len_key.append_bytes(LEN_SUFFIX);

        VecMapper {
            _phantom_api: PhantomData,
            address,
            base_key,
            len_key,
            _phantom_item: PhantomData,
        }
    }
}

impl<SA, T> StorageClearable for VecMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
    fn clear(&mut self) {
        self.clear();
    }
}

impl<SA, T, A> VecMapper<SA, T, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode,
{
    fn item_key(&self, index: usize) -> StorageKey<SA> {
        let mut item_key = self.base_key.clone();
        item_key.append_bytes(ITEM_SUFFIX);
        item_key.append_item(&index);
        item_key
    }

    /// Number of items managed by the mapper.
    pub fn len(&self) -> usize {
        self.address.address_storage_get(self.len_key.as_ref())
    }

    /// True if no items present in the mapper.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get item at index from storage.
    /// Index must be valid (1 <= index <= count).
    pub fn get(&self, index: usize) -> T {
        if index == 0 || index > self.len() {
            SA::error_api_impl().signal_error(INDEX_OUT_OF_RANGE_ERR_MSG);
        }
        self.get_unchecked(index)
    }

    /// Get item at index from storage.
    /// There are no restrictions on the index,
    /// calling for an invalid index will simply return the zero-value.
    pub fn get_unchecked(&self, index: usize) -> T {
        self.address
            .address_storage_get(self.item_key(index).as_ref())
    }

    /// Get item at index from storage.
    /// If index is valid (1 <= index <= count), returns value at index,
    /// else calls lambda given as argument.
    /// The lambda only gets called lazily if the index is not valid.
    pub fn get_or_else<F: FnOnce() -> T>(self, index: usize, or_else: F) -> T {
        if index == 0 || index > self.len() {
            or_else()
        } else {
            self.get_unchecked(index)
        }
    }

    /// Checks whether or not there is anything ins storage at index.
    /// Index must be valid (1 <= index <= count).
    pub fn item_is_empty(&self, index: usize) -> bool {
        if index == 0 || index > self.len() {
            SA::error_api_impl().signal_error(INDEX_OUT_OF_RANGE_ERR_MSG);
        }
        self.item_is_empty_unchecked(index)
    }

    /// Checks whether or not there is anything in storage at index.
    /// There are no restrictions on the index,
    /// calling for an invalid index will simply return `true`.
    pub fn item_is_empty_unchecked(&self, index: usize) -> bool {
        self.address
            .address_storage_get_len(self.item_key(index).as_ref())
            == 0
    }

    /// Loads all items from storage and places them in a Vec.
    /// Can easily consume a lot of gas.
    #[cfg(feature = "alloc")]
    pub fn load_as_vec(&self) -> alloc::vec::Vec<T> {
        self.iter().collect()
    }

    /// Provides a forward iterator.
    pub fn iter(&self) -> Iter<SA, T, A> {
        Iter::new(self)
    }
}

impl<SA, T> VecMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
    fn save_count(&self, new_len: usize) {
        storage_set(self.len_key.as_ref(), &new_len);
    }

    /// Add one item at the end of the list.
    /// Returns the index of the newly inserted item, which is also equal to the new number of elements.
    pub fn push(&mut self, item: &T) -> usize {
        let mut len = self.len();
        len += 1;
        storage_set(self.item_key(len).as_ref(), item);
        self.save_count(len);
        len
    }

    /// Adds multiple items at the end of the list.
    /// Cheaper than multiple `push`-es because the count only gets updated once at the end.
    /// Returns the index of the last inserted item, which is also equal to the new number of elements.
    pub fn extend_from_slice(&mut self, items: &[T]) -> usize {
        let mut len = self.len();
        for item in items {
            len += 1;
            storage_set(self.item_key(len).as_ref(), item);
        }
        self.save_count(len);
        len
    }

    /// Set item at index in storage.
    /// Index must be valid (1 <= index <= count).
    pub fn set(&self, index: usize, item: &T) {
        if index == 0 || index > self.len() {
            SA::error_api_impl().signal_error(&b"index out of range"[..]);
        }
        self.set_unchecked(index, item);
    }

    /// Keeping `set_unchecked` private on purpose, so developers don't write out of index limits by accident.
    fn set_unchecked(&self, index: usize, item: &T) {
        storage_set(self.item_key(index).as_ref(), item);
    }

    /// Clears item at index from storage.
    /// Index must be valid (1 <= index <= count).
    pub fn clear_entry(&self, index: usize) {
        if index == 0 || index > self.len() {
            SA::error_api_impl().signal_error(&b"index out of range"[..]);
        }
        self.clear_entry_unchecked(index)
    }

    /// Clears item at index from storage.
    /// There are no restrictions on the index,
    /// calling for an invalid index will simply do nothing.
    pub fn clear_entry_unchecked(&self, index: usize) {
        storage_clear(self.item_key(index).as_ref());
    }

    /// Clears item at index from storage by swap remove
    /// last item takes the index of the item to remove
    /// and we remove the last index.
    pub fn swap_remove(&mut self, index: usize) {
        let _ = self.swap_remove_and_get_old_last(index);
    }

    pub(crate) fn swap_remove_and_get_old_last(&mut self, index: usize) -> Option<T> {
        let last_item_index = self.len();
        if index == 0 || index > last_item_index {
            SA::error_api_impl().signal_error(&b"index out of range"[..]);
        }

        let mut last_item_as_option = Option::None;
        if index != last_item_index {
            let last_item = self.get(last_item_index);
            self.set(index, &last_item);
            last_item_as_option = Some(last_item);
        }
        self.clear_entry(last_item_index);
        self.save_count(last_item_index - 1);
        last_item_as_option
    }

    /// Deletes all contents form storage and sets count to 0.
    /// Can easily consume a lot of gas.
    pub fn clear(&mut self) {
        let len = self.len();
        for i in 1..=len {
            storage_clear(self.item_key(i).as_ref());
        }
        self.save_count(0);
    }
}

impl<'a, SA, T, A> IntoIterator for &'a VecMapper<SA, T, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode + 'static,
{
    type Item = T;

    type IntoIter = Iter<'a, SA, T, A>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over the elements of a `VecMapper`.
///
/// This `struct` is created by [`VecMapper::iter()`]. See its
/// documentation for more.
pub struct Iter<'a, SA, T, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode + 'static,
{
    index: usize,
    len: usize,
    vec: &'a VecMapper<SA, T, A>,
}

impl<'a, SA, T, A> Iter<'a, SA, T, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode + 'static,
{
    fn new(vec: &'a VecMapper<SA, T, A>) -> Iter<'a, SA, T, A> {
        Iter {
            index: 1,
            len: vec.len(),
            vec,
        }
    }
}

impl<'a, SA, T, A> Iterator for Iter<'a, SA, T, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: TopEncode + TopDecode + 'static,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        let current_index = self.index;
        if current_index > self.len {
            return None;
        }
        self.index += 1;
        Some(self.vec.get_unchecked(current_index))
    }
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, T> TopEncodeMulti for VecMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        multi_encode_iter_or_handle_err(self.iter(), output, h)
    }
}

impl<SA, T> CodecFrom<VecMapper<SA, T, CurrentStorage>> for MultiValueEncoded<SA, T>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode,
{
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA, T> TypeAbi for VecMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopEncode + TopDecode + TypeAbi,
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
