use crate::codec::{
    multi_encode_iter_or_handle_err, CodecFrom, EncodeErrorHandler, TopEncodeMulti,
    TopEncodeMultiOutput,
};

use super::{StorageMapper, VecMapper};
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
pub struct UniqueIdMapper<SA>
where
    SA: StorageMapperApi,
{
    base_key: StorageKey<SA>,
    vec_mapper: VecMapper<SA, UniqueId>,
}

impl<SA> StorageMapper<SA> for UniqueIdMapper<SA>
where
    SA: StorageMapperApi,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        Self {
            base_key: base_key.clone(),
            vec_mapper: VecMapper::new(base_key),
        }
    }
}

impl<SA> UniqueIdMapper<SA>
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

    /// Provides a forward iterator.
    pub fn iter(&self) -> Iter<SA> {
        Iter::new(self)
    }
}

impl<'a, SA> IntoIterator for &'a UniqueIdMapper<SA>
where
    SA: StorageMapperApi,
{
    type Item = usize;

    type IntoIter = Iter<'a, SA>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, SA>
where
    SA: StorageMapperApi,
{
    index: usize,
    len: usize,
    id_mapper: &'a UniqueIdMapper<SA>,
}

impl<'a, SA> Iter<'a, SA>
where
    SA: StorageMapperApi,
{
    fn new(id_mapper: &'a UniqueIdMapper<SA>) -> Iter<'a, SA> {
        Iter {
            index: 1,
            len: id_mapper.len(),
            id_mapper,
        }
    }
}

impl<'a, SA> Iterator for Iter<'a, SA>
where
    SA: StorageMapperApi,
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
impl<SA> TopEncodeMulti for UniqueIdMapper<SA>
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

impl<SA> CodecFrom<UniqueIdMapper<SA>> for MultiValueEncoded<SA, usize> where SA: StorageMapperApi {}

/// Behaves like a MultiResultVec when an endpoint result.
impl<SA> TypeAbi for UniqueIdMapper<SA>
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
