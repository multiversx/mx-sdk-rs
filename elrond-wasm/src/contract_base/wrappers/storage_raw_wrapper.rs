use core::marker::PhantomData;

use elrond_codec::{TopDecode, TopEncode};

use crate::{
    api::{
        const_handles::MBUF_TEMPORARY_1, use_raw_handle, ErrorApi, ManagedTypeApi, StorageReadApi,
        StorageReadApiImpl, StorageWriteApi,
    },
    storage::StorageKey,
    storage_get,
    storage_get::StorageGetErrorHandler,
    storage_set,
    types::{ManagedAddress, ManagedBuffer, ManagedType},
};

#[derive(Default)]
pub struct StorageRawWrapper<A>
where
    A: StorageReadApi + ManagedTypeApi + ErrorApi,
{
    _phantom: PhantomData<A>,
}

impl<A> StorageRawWrapper<A>
where
    A: StorageReadApi + StorageWriteApi + ManagedTypeApi + ErrorApi,
{
    pub fn new() -> Self {
        StorageRawWrapper {
            _phantom: PhantomData,
        }
    }

    /// Reads storage from the given key, and deserializes the value to the provided type.
    ///
    /// Use only if really necessary, storage mappers should be preferred.
    #[inline]
    pub fn read<T: TopDecode>(&self, storage_key: &StorageKey<A>) -> T {
        storage_get(storage_key.as_ref())
    }

    /// Reads storage from another address (usually a smart contract),
    /// from the given key, and deserializes the value to the provided type.
    ///
    /// This is a synchronous call, so it only works when
    /// both the current contract and the destination are in the same shard.
    #[inline]
    pub fn read_from_address<T: TopDecode>(
        &self,
        address: &ManagedAddress<A>,
        storage_key: &StorageKey<A>,
    ) -> T {
        A::storage_read_api_impl().storage_load_from_address(
            address.get_handle(),
            storage_key.get_handle(),
            use_raw_handle(MBUF_TEMPORARY_1),
        );

        let Ok(value) = T::top_decode_or_handle_err(
            ManagedBuffer::<A>::from_raw_handle(MBUF_TEMPORARY_1),
            StorageGetErrorHandler::<A>::default(),
        );
        value
    }

    /// Write a serializable value to the given storage key
    ///
    /// Use only if really necessary, storage mappers should be preferred.
    #[inline]
    pub fn write<T: TopEncode>(&self, storage_key: &StorageKey<A>, value: &T) {
        storage_set(storage_key.as_ref(), value);
    }
}
