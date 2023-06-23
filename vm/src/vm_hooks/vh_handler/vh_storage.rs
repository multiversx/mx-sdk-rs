#![allow(unused)]

use crate::{
    num_bigint::{BigInt, Sign},
    tx_mock::TxPanic,
    vm_hooks::VMHooksHandlerSource,
    DebugApi,
};
use alloc::vec::Vec;
use multiversx_sc::{
    api::{
        BigIntApiImpl, ManagedBufferApiImpl, RawHandle, StorageReadApi, StorageReadApiImpl,
        StorageWriteApi, StorageWriteApiImpl,
    },
    types::heap::Address,
};

use super::VMHooksManagedTypes;

pub trait VMHooksStorageRead: VMHooksHandlerSource {
    fn storage_load_len(&self, key: &[u8]) -> usize {
        self.storage_read(key).len()
    }

    fn storage_load_managed_buffer_raw(&self, key_handle: RawHandle, dest: RawHandle) {
        let value = self.storage_read(self.m_types_borrow().mb_get(key_handle));
        self.m_types_borrow_mut().mb_set(dest, value);
    }

    fn storage_load_from_address(
        &self,
        address_handle: RawHandle,
        key_handle: RawHandle,
        dest: RawHandle,
    ) {
        let address = Address::from_slice(self.m_types_borrow().mb_get(address_handle));
        let value =
            self.storage_read_any_address(&address, self.m_types_borrow().mb_get(key_handle));
        self.m_types_borrow_mut().mb_set(dest, value);
    }
}

pub trait VMHooksStorageWrite: VMHooksHandlerSource + VMHooksManagedTypes {
    fn storage_store_slice_u8(&self, key: &[u8], value: &[u8]) {
        self.storage_write(key, value);
    }

    fn storage_store_big_uint_raw(&self, key: &[u8], handle: RawHandle) {
        self.storage_write(key, self.bi_get_signed_bytes(handle).as_slice());
    }

    fn storage_store_managed_buffer_raw(&self, key_handle: RawHandle, value_handle: RawHandle) {
        self.storage_write(
            self.m_types_borrow().mb_get(key_handle),
            self.m_types_borrow().mb_get(value_handle),
        );
    }
}
