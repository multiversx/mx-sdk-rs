use crate::{tx_mock::TxPanic, DebugApi};
use alloc::vec::Vec;
use elrond_wasm::api::{BigIntApi, Handle, ManagedBufferApi, StorageReadApi, StorageWriteApi};
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::ToPrimitive;

impl StorageReadApi for DebugApi {
    fn storage_load_len(&self, key: &[u8]) -> usize {
        self.storage_load_vec_u8(key).len()
    }

    fn storage_load_vec_u8(&self, key: &[u8]) -> Vec<u8> {
        self.with_contract_account(|account| match account.storage.get(&key.to_vec()) {
            None => Vec::with_capacity(0),
            Some(value) => value.clone(),
        })
    }

    fn storage_load_big_uint_raw(&self, key: &[u8]) -> Handle {
        let bytes = self.storage_load_vec_u8(key);
        let bi = BigInt::from_bytes_be(Sign::Plus, bytes.as_slice());
        let mut managed_types = self.m_types_borrow_mut();
        managed_types.big_int_map.insert_new_handle(bi)
    }

    fn storage_load_managed_buffer_raw(&self, key_handle: Handle) -> Handle {
        let key_bytes = self.mb_to_boxed_bytes(key_handle);
        let bytes = self.storage_load_vec_u8(key_bytes.as_slice());
        self.mb_new_from_bytes(bytes.as_slice())
    }

    fn storage_load_managed_buffer_len(&self, key_handle: Handle) -> usize {
        let key_bytes = self.mb_to_boxed_bytes(key_handle);
        let bytes = self.storage_load_vec_u8(key_bytes.as_slice());
        bytes.len()
    }

    fn storage_load_u64(&self, key: &[u8]) -> u64 {
        let value = self.storage_load_vec_u8(key);
        let bu = BigUint::from_bytes_be(value.as_slice());
        if let Some(v) = bu.to_u64() {
            v
        } else {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: b"storage value out of range".to_vec(),
            })
        }
    }

    fn storage_load_i64(&self, key: &[u8]) -> i64 {
        let value = self.storage_load_vec_u8(key);
        let bi = BigInt::from_signed_bytes_be(value.as_slice());
        if let Some(v) = bi.to_i64() {
            v
        } else {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: b"storage value out of range".to_vec(),
            })
        }
    }
}

impl StorageWriteApi for DebugApi {
    fn storage_store_slice_u8(&self, key: &[u8], value: &[u8]) {
        // TODO: extract magic strings somewhere
        if key.starts_with(&b"ELROND"[..]) {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: b"cannot write to storage under Elrond reserved key".to_vec(),
            });
        }

        self.with_contract_account_mut(|account| {
            account.storage.insert(key.to_vec(), value.to_vec());
        });
    }

    fn storage_store_big_uint_raw(&self, key: &[u8], handle: i32) {
        self.storage_store_slice_u8(key, self.bi_get_signed_bytes(handle).as_slice());
    }

    fn storage_store_managed_buffer_raw(&self, key_handle: Handle, value_handle: Handle) {
        let key_bytes = self.mb_to_boxed_bytes(key_handle);
        let value_bytes = self.mb_to_boxed_bytes(value_handle);
        self.storage_store_slice_u8(key_bytes.as_slice(), value_bytes.as_slice());
    }

    fn storage_store_managed_buffer_clear(&self, key_handle: Handle) {
        let key_bytes = self.mb_to_boxed_bytes(key_handle);
        self.storage_store_slice_u8(key_bytes.as_slice(), &[]);
    }

    fn storage_store_u64(&self, key: &[u8], value: u64) {
        if value == 0 {
            self.storage_store_slice_u8(key, &[]);
        } else {
            self.storage_store_slice_u8(key, &BigUint::from(value).to_bytes_be());
        }
    }

    fn storage_store_i64(&self, key: &[u8], value: i64) {
        if value == 0 {
            self.storage_store_slice_u8(key, &[]);
        } else {
            self.storage_store_slice_u8(key, &BigInt::from(value).to_signed_bytes_be());
        }
    }
}
