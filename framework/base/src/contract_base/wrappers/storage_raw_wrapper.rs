use core::marker::PhantomData;

use unwrap_infallible::UnwrapInfallible;

use crate::api::HandleConstraints;
use crate::codec::{TopDecode, TopEncode};

use crate::types::{SystemSCAddress, TokenIdentifier};
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
    pub fn read<K, V>(&self, storage_key: K) -> V
    where
        K: Into<StorageKey<A>>,
        V: TopDecode,
    {
        let key: StorageKey<A> = storage_key.into();
        storage_get(key.as_ref())
    }

    /// Reads storage from another address (usually a smart contract),
    /// from the given key, and deserializes the value to the provided type.
    ///
    /// This is a synchronous call, so it only works when
    /// both the current contract and the destination are in the same shard.
    pub fn read_from_address<K, V>(&self, address: &ManagedAddress<A>, storage_key: K) -> V
    where
        K: Into<StorageKey<A>>,
        V: TopDecode,
    {
        let key: StorageKey<A> = storage_key.into();
        let result_buffer =
            unsafe { ManagedBuffer::<A>::from_handle(use_raw_handle(MBUF_TEMPORARY_1)) };
        A::storage_read_api_impl().storage_load_from_address(
            address.get_handle(),
            key.get_handle(),
            result_buffer.get_handle(),
        );

        V::top_decode_or_handle_err(
            result_buffer,
            StorageGetErrorHandler::<A>::new(key.get_handle().get_raw_handle_unchecked()),
        )
        .unwrap_infallible()
    }

    /// Write a serializable value to storage under the given key
    ///
    /// Use only if really necessary, storage mappers should be preferred.
    #[inline]
    pub fn write<K, V>(&self, storage_key: K, value: &V)
    where
        K: Into<StorageKey<A>>,
        V: TopEncode,
    {
        let key: StorageKey<A> = storage_key.into();
        storage_set(key.as_ref(), value);
    }

    pub fn token_has_transfer_role(&self, token_identifier: TokenIdentifier<A>) -> bool {
        let key = ManagedBuffer::<A>::new_from_bytes(b"ELRONDtransferesdt");
        let base_key = key.concat(token_identifier.into_managed_buffer());

        // Decoding the response needs more research
        // Empty response means no address has transferRole for the token
        let result = self.read_from_address::<StorageKey<A>, ManagedBuffer<A>>(
            &SystemSCAddress.to_managed_address(),
            StorageKey::<A>::from(base_key),
        );

        !result.is_empty()
    }
}
