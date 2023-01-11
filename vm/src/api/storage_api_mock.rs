use crate::{
    num_bigint::{BigInt, Sign},
    tx_mock::TxPanic,
    DebugApi,
};
use alloc::vec::Vec;
use multiversx_sc::api::{
    BigIntApi, ManagedBufferApi, StorageReadApi, StorageReadApiImpl, StorageWriteApi,
    StorageWriteApiImpl,
};

impl StorageReadApi for DebugApi {
    type StorageReadApiImpl = DebugApi;

    fn storage_read_api_impl() -> Self::StorageReadApiImpl {
        DebugApi::new_from_static()
    }
}

impl DebugApi {
    fn storage_load_vec_u8(&self, key: &[u8]) -> Vec<u8> {
        self.with_contract_account(|account| match account.storage.get(&key.to_vec()) {
            None => Vec::with_capacity(0),
            Some(value) => value.clone(),
        })
    }
}

impl StorageReadApiImpl for DebugApi {
    fn storage_load_len(&self, key: &[u8]) -> usize {
        self.storage_load_vec_u8(key).len()
    }

    fn storage_load_to_heap(&self, key: &[u8]) -> Box<[u8]> {
        self.storage_load_vec_u8(key).into_boxed_slice()
    }

    fn storage_load_big_uint_raw(&self, key: &[u8], dest: Self::ManagedBufferHandle) {
        let bytes = self.storage_load_vec_u8(key);
        let bi = BigInt::from_bytes_be(Sign::Plus, bytes.as_slice());
        self.bi_overwrite(dest, bi);
    }

    fn storage_load_managed_buffer_raw(
        &self,
        key_handle: Self::ManagedBufferHandle,
        dest: Self::ManagedBufferHandle,
    ) {
        let key_bytes = self.mb_to_boxed_bytes(key_handle);
        let bytes = self.storage_load_vec_u8(key_bytes.as_slice());
        self.mb_overwrite(dest, bytes.as_slice());
    }

    fn storage_load_from_address(
        &self,
        address_handle: Self::ManagedBufferHandle,
        key_handle: Self::ManagedBufferHandle,
        dest: Self::ManagedBufferHandle,
    ) {
        let address = multiversx_sc::types::heap::Address::from_slice(
            self.mb_to_boxed_bytes(address_handle).as_slice(),
        );
        let key_bytes = self.mb_to_boxed_bytes(key_handle);
        self.with_account(&address, |account| {
            match account.storage.get(key_bytes.as_slice()) {
                None => self.mb_overwrite(dest, &[]),
                Some(value) => self.mb_overwrite(dest, value.as_slice()),
            }
        })
    }
}

impl StorageWriteApi for DebugApi {
    type StorageWriteApiImpl = DebugApi;

    fn storage_write_api_impl() -> Self::StorageWriteApiImpl {
        DebugApi::new_from_static()
    }
}

impl StorageWriteApiImpl for DebugApi {
    fn storage_store_slice_u8(&self, key: &[u8], value: &[u8]) {
        // TODO: extract magic strings somewhere
        if key.starts_with(&b"ELROND"[..]) {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: "cannot write to storage under Elrond reserved key".to_string(),
            });
        }

        self.with_contract_account_mut(|account| {
            account.storage.insert(key.to_vec(), value.to_vec());
        });
    }

    fn storage_store_big_uint_raw(&self, key: &[u8], handle: Self::BigIntHandle) {
        self.storage_store_slice_u8(key, self.bi_get_signed_bytes(handle).as_slice());
    }

    fn storage_store_managed_buffer_raw(
        &self,
        key_handle: Self::ManagedBufferHandle,
        value_handle: Self::ManagedBufferHandle,
    ) {
        let key_bytes = self.mb_to_boxed_bytes(key_handle);
        let value_bytes = self.mb_to_boxed_bytes(value_handle);
        self.storage_store_slice_u8(key_bytes.as_slice(), value_bytes.as_slice());
    }

    fn storage_store_managed_buffer_clear(&self, key_handle: Self::ManagedBufferHandle) {
        let key_bytes = self.mb_to_boxed_bytes(key_handle);
        self.storage_store_slice_u8(key_bytes.as_slice(), &[]);
    }
}
