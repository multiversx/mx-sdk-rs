use crate::{
    codec::{
        multi_encode_iter_or_handle_err, CodecFrom, EncodeErrorHandler, TopEncodeMulti,
        TopEncodeMultiOutput,
    },
    types::ManagedAddress,
};

use super::{
    set_mapper::{CurrentStorage, StorageAddress},
    StorageMapper, VecMapper,
};
use crate::{
    abi::{TypeAbi, TypeDescriptionContainer, TypeName},
    api::{ErrorApiImpl, StorageMapperApi},
    storage::StorageKey,
    storage_set,
    types::{ManagedType, MultiValueEncoded},
};

pub type UniqueId = usize;
const EMPTY_ENTRY: UniqueId = 0;

/// Holds the values from 1 to N with as little storage interaction as possible
/// If Mapper[i] = i, then it stores nothing, i.e. "0"
/// If Mapper[i] is equal to another value, then it stores the value
pub struct UniqueIdMapper<SA, A = CurrentStorage>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
{
    _address: A,
    base_key: StorageKey<SA>,
    vec_mapper: VecMapper<SA, UniqueId, A>,
}

impl<SA> StorageMapper<SA> for UniqueIdMapper<SA, CurrentStorage>
where
    SA: StorageMapperApi,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        Self {
            _address: CurrentStorage,
            base_key: base_key.clone(),
            vec_mapper: VecMapper::new(base_key),
        }
    }
}

impl<SA> UniqueIdMapper<SA, ManagedAddress<SA>>
where
    SA: StorageMapperApi,
{
    pub fn new_from_address(address: ManagedAddress<SA>, base_key: StorageKey<SA>) -> Self {
        Self {
            _address: address.clone(),
            base_key: base_key.clone(),
            vec_mapper: VecMapper::new_from_address(address, base_key),
        }
    }
}

impl<SA, A> UniqueIdMapper<SA, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
{
    #[inline]
    pub fn len(&self) -> usize {
        self.vec_mapper.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.vec_mapper.is_empty()
    }

    /// Gets the value for the given `index`. If the entry is empty, `index` is returned.
    pub fn get(&self, index: usize) -> UniqueId {
        let id: UniqueId = self.vec_mapper.get(index);
        if id == EMPTY_ENTRY {
            index
        } else {
            id
        }
    }

    /// Provides a forward iterator.
    pub fn iter(&self) -> Iter<SA, A> {
        Iter::new(self)
    }
}

impl<SA> UniqueIdMapper<SA, CurrentStorage>
where
    SA: StorageMapperApi,
{
    /// Initializes the mapper's length. This may not be set again afterwards.
    pub fn set_initial_len(&mut self, len: usize) {
        if !self.vec_mapper.is_empty() {
            SA::error_api_impl().signal_error(b"len already set");
        }

        self.set_internal_mapper_len(len);
    }

    /// Gets the value from the index and removes it.
    /// The value is replaced by the last item, and length is decremented.
    pub fn swap_remove(&mut self, index: usize) -> UniqueId {
        let last_item_index = self.len();
        let last_item = self.get(last_item_index);

        let current_item = if index != last_item_index {
            let item_at_index = self.get(index);
            self.set(index, last_item);

            item_at_index
        } else {
            last_item
        };

        self.vec_mapper.set(last_item_index, &EMPTY_ENTRY);
        self.set_internal_mapper_len(last_item_index - 1);

        current_item
    }

    /// Sets the value at the given index. If index == id, then the entry is cleared.
    pub fn set(&mut self, index: usize, id: UniqueId) {
        if index == id {
            self.vec_mapper.set(index, &EMPTY_ENTRY);
        } else {
            self.vec_mapper.set(index, &id);
        }
    }

    // Manually sets the internal VecMapper's len value
    fn set_internal_mapper_len(&mut self, new_len: usize) {
        let mut len_key = self.base_key.clone();
        len_key.append_bytes(&b".len"[..]);
        storage_set(len_key.as_ref(), &new_len);
    }
}

impl<'a, SA, A> IntoIterator for &'a UniqueIdMapper<SA, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
{
    type Item = usize;

    type IntoIter = Iter<'a, SA, A>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, SA, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
{
    index: usize,
    len: usize,
    id_mapper: &'a UniqueIdMapper<SA, A>,
}

impl<'a, SA, A> Iter<'a, SA, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
{
    fn new(id_mapper: &'a UniqueIdMapper<SA, A>) -> Iter<'a, SA, A> {
        Iter {
            index: 1,
            len: id_mapper.len(),
            id_mapper,
        }
    }
}

impl<'a, SA, A> Iterator for Iter<'a, SA, A>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
{
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let current_index = self.index;
        if current_index > self.len {
            return None;
        }

        self.index += 1;
        Some(self.id_mapper.get(current_index))
    }
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA> TopEncodeMulti for UniqueIdMapper<SA, CurrentStorage>
where
    SA: StorageMapperApi,
{
    fn multi_encode_or_handle_err<O, H>(&self, output: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeMultiOutput,
        H: EncodeErrorHandler,
    {
        multi_encode_iter_or_handle_err(self.iter(), output, h)
    }
}

impl<SA> CodecFrom<UniqueIdMapper<SA, CurrentStorage>> for MultiValueEncoded<SA, usize> where
    SA: StorageMapperApi
{
}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA> TypeAbi for UniqueIdMapper<SA, CurrentStorage>
where
    SA: StorageMapperApi,
{
    fn type_name() -> TypeName {
        crate::abi::type_name_variadic::<usize>()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        usize::provide_type_descriptions(accumulator);
    }

    fn is_variadic() -> bool {
        true
    }
}
