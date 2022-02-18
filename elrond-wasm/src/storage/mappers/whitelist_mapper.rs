use core::marker::PhantomData;

use super::{SingleValueMapper, StorageMapper};
use crate::{
    api::{ErrorApiImpl, StorageMapperApi},
    storage::StorageKey,
};
use elrond_codec::NestedEncode;

type FlagMapper<SA> = SingleValueMapper<SA, bool>;

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
    pub fn contains(&self, item: &T) -> bool {
        let mapper = self.build_mapper_for_item(item);
        !mapper.is_empty()
    }

    pub fn add(&self, item: &T) {
        let mapper = self.build_mapper_for_item(item);
        mapper.set(&true);
    }

    pub fn remove(&self, item: &T) {
        let mapper = self.build_mapper_for_item(item);
        mapper.clear();
    }

    pub fn require_whitelisted(&self, item: &T) {
        if !self.contains(item) {
            SA::error_api_impl().signal_error(b"Item not whitelisted");
        }
    }

    fn build_mapper_for_item(&self, item: &T) -> FlagMapper<SA> {
        let mut key = self.base_key.clone();
        key.append_item(item);

        FlagMapper::<SA>::new(key)
    }
}
