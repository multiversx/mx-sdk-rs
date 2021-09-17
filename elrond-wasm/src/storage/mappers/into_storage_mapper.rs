use elrond_codec::{TopDecode, TopEncode};

use crate::{
    api::{ErrorApi, ManagedTypeApi, StorageReadApi, StorageWriteApi},
    storage::StorageKey,
};

use super::{SingleValueMapper, StorageMapper};

pub trait IntoStorageMapper<SA>
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    Self::StorageMapperType: StorageMapper<SA>,
{
    type StorageMapperType;

    fn item(api: SA, base_key: StorageKey<SA>) -> Self::StorageMapperType {
        Self::StorageMapperType::new(api, base_key)
    }
}

impl<SA, T> IntoStorageMapper<SA> for T
where
    SA: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi + Clone + 'static,
    T: TopEncode + TopDecode + 'static,
{
    type StorageMapperType = SingleValueMapper<SA, T>;
}
