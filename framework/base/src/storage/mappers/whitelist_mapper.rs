use core::marker::PhantomData;

use super::{SingleValueMapper, StorageMapper};
use crate::{
    api::{ErrorApiImpl, StorageMapperApi},
    codec::NestedEncode,
    storage::StorageKey,
    types::ManagedAddress,
};

type FlagMapper<SA> = SingleValueMapper<SA, bool>;

static ITEM_NOT_WHITELISTED_ERR_MSG: &[u8] = b"Item not whitelisted";

/// A non-iterable whitelist mapper.
/// Very efficient for storing a whitelist, as each item requires only one storage key.
/// If you need to iterate over the keys, use UnorderedSetMapper or SetMapper instead.
pub struct WhitelistMapper<SA, T>
where
    SA: StorageMapperApi,
    T: NestedEncode + 'static,
{
    base_key: StorageKey<SA>,
    _phantom: PhantomData<T>,
}

impl<SA, T> StorageMapper<SA> for WhitelistMapper<SA, T>
where
    SA: StorageMapperApi,
    T: NestedEncode + 'static,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        Self {
            base_key,
            _phantom: PhantomData,
        }
    }
}

impl<SA, T> WhitelistMapper<SA, T>
where
    SA: StorageMapperApi,
    T: NestedEncode + 'static,
{
    pub fn add(&self, item: &T) {
        let mapper = self.build_mapper_for_item(item);
        mapper.set(true);
    }

    pub fn remove(&self, item: &T) {
        let mapper = self.build_mapper_for_item(item);
        mapper.clear();
    }

    pub fn contains(&self, item: &T) -> bool {
        let mapper = self.build_mapper_for_item(item);
        !mapper.is_empty()
    }

    pub fn contains_at_address(&self, address: &ManagedAddress<SA>, item: &T) -> bool {
        let mapper = self.build_mapper_for_item(item);
        !mapper.is_empty_at_address(address)
    }

    pub fn require_whitelisted(&self, item: &T) {
        if !self.contains(item) {
            SA::error_api_impl().signal_error(ITEM_NOT_WHITELISTED_ERR_MSG);
        }
    }

    pub fn require_whitelisted_at_address(&self, address: &ManagedAddress<SA>, item: &T) {
        if !self.contains_at_address(address, item) {
            SA::error_api_impl().signal_error(ITEM_NOT_WHITELISTED_ERR_MSG);
        }
    }

    fn build_mapper_for_item(&self, item: &T) -> FlagMapper<SA> {
        let mut key = self.base_key.clone();
        key.append_item(item);

        FlagMapper::<SA>::new(key)
    }
}
