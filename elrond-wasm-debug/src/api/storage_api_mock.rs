use crate::{TxContext, TxPanic};
use alloc::vec::Vec;
use elrond_wasm::api::{StorageReadApi, StorageWriteApi};
use num_bigint::{BigInt, BigUint};
use num_traits::ToPrimitive;

impl StorageReadApi for TxContext {
    fn storage_load_len(&self, key: &[u8]) -> usize {
        self.storage_load_vec_u8(key).len()
    }

    fn storage_load_vec_u8(&self, key: &[u8]) -> Vec<u8> {
        let tx_output = self.tx_output_cell.borrow();
        match tx_output.contract_storage.get(&key.to_vec()) {
            None => Vec::with_capacity(0),
            Some(value) => value.clone(),
        }
    }

    fn storage_load_big_uint_raw(&self, _key: &[u8]) -> i32 {
        panic!("cannot call storage_load_big_uint_raw in debug mode");
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

impl StorageWriteApi for TxContext {
    fn storage_store_slice_u8(&self, key: &[u8], value: &[u8]) {
        // TODO: extract magic strings somewhere
        if key.starts_with(&b"ELROND"[..]) {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: b"cannot write to storage under Elrond reserved key".to_vec(),
            });
        }

        let mut tx_output = self.tx_output_cell.borrow_mut();
        tx_output
            .contract_storage
            .insert(key.to_vec(), value.to_vec());
    }

    fn storage_store_big_uint_raw(&self, _key: &[u8], _handle: i32) {
        panic!("cannot call storage_store_big_uint_raw in debug mode");
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
