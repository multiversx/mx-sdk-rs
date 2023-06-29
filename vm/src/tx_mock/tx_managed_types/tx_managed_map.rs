use crate::types::RawHandle;

use super::{ManagedMapImpl, TxManagedTypes};

impl TxManagedTypes {
    pub fn mm_new(&mut self) -> RawHandle {
        self.managed_map_map
            .insert_new_handle_raw(ManagedMapImpl::new())
    }

    pub fn mm_values_insert(&mut self, map_handle: RawHandle, key: Vec<u8>, value: Vec<u8>) {
        let mmap = self.managed_map_map.get_mut(map_handle);
        mmap.insert(key, value);
    }

    pub fn mm_values_get(&self, map_handle: RawHandle, key: &[u8]) -> Vec<u8> {
        let mmap = self.managed_map_map.get(map_handle);
        mmap.get(key).cloned().unwrap_or_default()
    }

    pub fn mm_contains(&self, map_handle: RawHandle, key: &[u8]) -> bool {
        let mmap = self.managed_map_map.get(map_handle);
        if let Some(value) = mmap.get(key) {
            !value.is_empty()
        } else {
            false
        }
    }

    pub fn mm_values_remove(&mut self, map_handle: RawHandle, key: &[u8]) -> Vec<u8> {
        let mmap = self.managed_map_map.get_mut(map_handle);
        mmap.remove(key).unwrap_or_default()
    }
}
