use std::collections::HashMap;

use elrond_wasm::{api::Handle, types::LockableStaticBuffer};
use num_bigint::BigInt;

type ManagedBufferImpl = Vec<u8>;

#[derive(Debug)]
pub struct HandleMap<V> {
    next_handle: Handle,
    pub map: HashMap<Handle, V>,
}

impl<V> HandleMap<V> {
    pub fn new() -> Self {
        HandleMap {
            next_handle: 0,
            map: HashMap::new(),
        }
    }
}

impl<V> Default for HandleMap<V> {
    fn default() -> Self {
        HandleMap::new()
    }
}

impl<V> HandleMap<V> {
    pub fn insert_new_handle(&mut self, value: V) -> Handle {
        let new_handle = self.next_handle;
        self.map.insert(new_handle, value);
        self.next_handle += 1;
        new_handle
    }

    pub fn get(&self, handle: Handle) -> &V {
        self.map.get(&handle).unwrap()
    }

    pub fn get_mut(&mut self, handle: Handle) -> &mut V {
        self.map.get_mut(&handle).unwrap()
    }

    pub fn insert(&mut self, handle: Handle, value: V) {
        let _ = self.map.insert(handle, value);
    }
}

#[derive(Debug)]
pub struct TxManagedTypes {
    pub(crate) big_int_map: HandleMap<BigInt>,
    pub(crate) managed_buffer_map: HandleMap<ManagedBufferImpl>,
    pub(crate) lockable_static_buffer: LockableStaticBuffer,
}

impl TxManagedTypes {
    pub fn new() -> Self {
        TxManagedTypes {
            big_int_map: HandleMap::new(),
            managed_buffer_map: HandleMap::new(),
            lockable_static_buffer: LockableStaticBuffer::new(),
        }
    }
}

impl Default for TxManagedTypes {
    fn default() -> Self {
        TxManagedTypes::new()
    }
}
