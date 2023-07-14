use crate::types::RawHandle;
use std::collections::HashMap;

#[derive(Debug)]
pub struct HandleMap<V> {
    next_handle: RawHandle,
    pub map: HashMap<RawHandle, V>,
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
    pub fn insert_new_handle_raw(&mut self, value: V) -> RawHandle {
        let new_handle = self.next_handle;
        self.map.insert(new_handle, value);
        self.next_handle += 1;
        new_handle
    }

    pub fn get(&self, handle: RawHandle) -> &V {
        // TODO: consider simulating the actual error from the VM
        self.map
            .get(&handle)
            .unwrap_or_else(|| panic!("handle not found"))
    }

    pub fn get_mut(&mut self, handle: RawHandle) -> &mut V {
        // TODO: consider simulating the actual error from the VM
        self.map
            .get_mut(&handle)
            .unwrap_or_else(|| panic!("handle not found"))
    }

    pub fn insert(&mut self, handle: RawHandle, value: V) {
        let _ = self.map.insert(handle, value);
    }
}
