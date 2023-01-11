use crate::num_bigint::BigInt;
use multiversx_sc::api::{const_handles, use_raw_handle, HandleConstraints, RawHandle};
use std::collections::HashMap;

type ManagedBufferImpl = Vec<u8>;

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
    pub fn insert_new_handle<H: HandleConstraints>(&mut self, value: V) -> H {
        let new_handle = self.next_handle;
        self.map.insert(new_handle, value);
        self.next_handle += 1;
        use_raw_handle(new_handle)
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

#[derive(Debug)]
pub struct TxManagedTypes {
    pub(crate) big_int_map: HandleMap<BigInt>,
    pub(crate) big_float_map: HandleMap<f64>,
    pub(crate) managed_buffer_map: HandleMap<ManagedBufferImpl>,
}

impl TxManagedTypes {
    pub fn new() -> Self {
        TxManagedTypes {
            big_int_map: HandleMap::new(),
            big_float_map: HandleMap::new(),
            managed_buffer_map: HandleMap::new(),
        }
    }
}

impl Default for TxManagedTypes {
    fn default() -> Self {
        TxManagedTypes::new()
    }
}

#[derive(Debug)]
pub struct TxStaticVars {
    pub(crate) external_view_target_address_handle: RawHandle,
    pub(crate) next_handle: RawHandle,
    pub(crate) num_arguments: i32,
    pub(crate) call_value_egld_handle: RawHandle,
    pub(crate) call_value_multi_esdt_handle: RawHandle,
}

impl Default for TxStaticVars {
    fn default() -> Self {
        TxStaticVars {
            external_view_target_address_handle: 0,
            next_handle: const_handles::NEW_HANDLE_START_FROM,
            num_arguments: -1,
            call_value_egld_handle: const_handles::UNINITIALIZED_HANDLE,
            call_value_multi_esdt_handle: const_handles::UNINITIALIZED_HANDLE,
        }
    }
}
