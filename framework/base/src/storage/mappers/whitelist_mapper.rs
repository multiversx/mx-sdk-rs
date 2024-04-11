use core::marker::PhantomData;

use multiversx_sc_codec::{TopDecode, TopEncode};

use super::{
    set_mapper::{CurrentStorage, StorageAddress},
    SingleValueMapper, StorageMapper,
};
use crate::{
    api::{ErrorApiImpl, StorageMapperApi},
    codec::NestedEncode,
    storage::StorageKey,
    types::ManagedAddress,
};

type FlagMapper<'a, SA, A> = SingleValueMapper<'a, SA, bool, A>;

static ITEM_NOT_WHITELISTED_ERR_MSG: &[u8] = b"Item not whitelisted";

/// A non-iterable whitelist mapper.
/// Very efficient for storing a whitelist, as each item requires only one storage key.
/// If you need to iterate over the keys, use UnorderedSetMapper or SetMapper instead.
pub struct WhitelistMapper<'a, SA, T, A = CurrentStorage>
where
    SA: StorageMapperApi<'a>,
    A: StorageAddress<'a, SA>,
    T: NestedEncode + 'static,
{
    address: A,
    base_key: StorageKey<'a, SA>,
    _phantom: PhantomData<T>,
}

impl<'a, SA, T> StorageMapper<'a, SA> for WhitelistMapper<'a, SA, T, CurrentStorage>
where
    SA: StorageMapperApi<'a>,
    T: NestedEncode + 'static,
{
    fn new(base_key: StorageKey<'a, SA>) -> Self {
        Self {
            address: CurrentStorage,
            base_key,
            _phantom: PhantomData,
        }
    }
}

impl<'a, SA, T> WhitelistMapper<'a, SA, T, ManagedAddress<'a, SA>>
where
    SA: StorageMapperApi<'a>,
    T: NestedEncode + 'static,
{
    pub fn new_from_address(address: ManagedAddress<'a, SA>, base_key: StorageKey<'a, SA>) -> Self {
        Self {
            address,
            base_key,
            _phantom: PhantomData,
        }
    }
}

impl<'a, SA, T> WhitelistMapper<'a, SA, T, ManagedAddress<'a, SA>>
where
    SA: StorageMapperApi<'a>,
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

    fn build_mapper_for_item(&self, item: &T) -> FlagMapper<'a, SA, ManagedAddress<'a, SA>> {
        let mut key = self.base_key.clone();
        key.append_item(item);

        FlagMapper::<'a, SA, ManagedAddress<'a, SA>>::new_from_address(self.address.clone(), key)
    }
}

impl<'a, SA, T> WhitelistMapper<'a, SA, T, CurrentStorage>
where
    SA: StorageMapperApi<'a>,
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

    fn build_mapper_for_item(&self, item: &T) -> FlagMapper<'a, SA, CurrentStorage> {
        let mut key = self.base_key.clone();
        key.append_item(item);

        FlagMapper::<'a, SA, CurrentStorage>::new(key)
    }
}
