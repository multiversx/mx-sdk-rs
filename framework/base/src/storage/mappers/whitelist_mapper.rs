use core::marker::PhantomData;

use multiversx_sc_codec::{TopDecode, TopEncode};

use super::{
    source::{CurrentStorage, StorageAddress},
    SingleValueMapper, StorageMapper, StorageMapperFromAddress,
};
use crate::{
    api::{ErrorApiImpl, StorageMapperApi},
    codec::NestedEncode,
    storage::StorageKey,
    types::ManagedAddress,
};

type FlagMapper<SA, A> = SingleValueMapper<SA, bool, A>;

static ITEM_NOT_WHITELISTED_ERR_MSG: &[u8] = b"Item not whitelisted";

/// A non-iterable whitelist mapper.
/// Very efficient for storing a whitelist, as each item requires only one storage key.
/// If you need to iterate over the keys, use UnorderedSetMapper or SetMapper instead.
pub struct WhitelistMapper<SA, T, A = CurrentStorage>
where
    SA: StorageMapperApi,
    A: StorageAddress<SA>,
    T: NestedEncode + 'static,
{
    address: A,
    base_key: StorageKey<SA>,
    _phantom: PhantomData<T>,
}

impl<SA, T> StorageMapper<SA> for WhitelistMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: NestedEncode + 'static,
{
    fn new(base_key: StorageKey<SA>) -> Self {
        Self {
            address: CurrentStorage,
            base_key,
            _phantom: PhantomData,
        }
    }
}

impl<SA, T> StorageMapperFromAddress<SA> for WhitelistMapper<SA, T, ManagedAddress<SA>>
where
    SA: StorageMapperApi,
    T: NestedEncode + 'static,
{
    fn new_from_address(address: ManagedAddress<SA>, base_key: StorageKey<SA>) -> Self {
        Self {
            address,
            base_key,
            _phantom: PhantomData,
        }
    }
}

impl<SA, T> WhitelistMapper<SA, T, ManagedAddress<SA>>
where
    SA: StorageMapperApi,
    T: TopDecode + TopEncode + NestedEncode + 'static,
{
    pub fn contains(&self, item: &T) -> bool {
        let mapper = self.build_mapper_for_item(item);
        !mapper.is_empty()
    }

    pub fn require_whitelisted(&self, item: &T) {
        if !self.contains(item) {
            SA::error_api_impl().signal_error(ITEM_NOT_WHITELISTED_ERR_MSG);
        }
    }

    fn build_mapper_for_item(&self, item: &T) -> FlagMapper<SA, ManagedAddress<SA>> {
        let mut key = self.base_key.clone();
        key.append_item(item);

        FlagMapper::<SA, ManagedAddress<SA>>::new_from_address(self.address.clone(), key)
    }
}

impl<SA, T> WhitelistMapper<SA, T, CurrentStorage>
where
    SA: StorageMapperApi,
    T: TopDecode + TopEncode + NestedEncode + 'static,
{
    pub fn contains(&self, item: &T) -> bool {
        let mapper = self.build_mapper_for_item(item);
        !mapper.is_empty()
    }

    pub fn require_whitelisted(&self, item: &T) {
        if !self.contains(item) {
            SA::error_api_impl().signal_error(ITEM_NOT_WHITELISTED_ERR_MSG);
        }
    }

    pub fn add(&self, item: &T) {
        let mapper = self.build_mapper_for_item(item);
        mapper.set(true);
    }

    pub fn remove(&self, item: &T) {
        let mapper = self.build_mapper_for_item(item);
        mapper.clear();
    }

    fn build_mapper_for_item(&self, item: &T) -> FlagMapper<SA, CurrentStorage> {
        let mut key = self.base_key.clone();
        key.append_item(item);

        FlagMapper::<SA, CurrentStorage>::new(key)
    }
}
