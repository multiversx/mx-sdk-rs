use crate::num_bigint::BigInt;
use elrond_wasm::api::{
    const_handles, use_raw_handle, HandleConstraints, HandleTypeInfo, RawHandle,
};
use std::{collections::HashMap, marker::PhantomData};

type ManagedBufferImpl = Vec<u8>;

#[derive(Debug)]
pub struct HandleMap<H: HandleConstraints, V> {
    next_handle: RawHandle,
    pub map: HashMap<RawHandle, V>,
    _phantom: PhantomData<H>,
}

impl<H: HandleConstraints, V> HandleMap<H, V> {
    pub fn new() -> Self {
        HandleMap {
            next_handle: 0,
            map: HashMap::new(),
            _phantom: PhantomData,
        }
    }
}

impl<H: HandleConstraints, V> Default for HandleMap<H, V> {
    fn default() -> Self {
        HandleMap::new()
    }
}

impl<H: HandleConstraints, V> HandleMap<H, V> {
    pub fn insert_new_handle(&mut self, value: V) -> H {
        let new_handle = self.next_handle;
        self.map.insert(new_handle, value);
        self.next_handle += 1;
        use_raw_handle(new_handle)
    }

    pub fn get(&self, handle: H) -> &V {
        // TODO: consider simulating the actual error from the VM
        self.map
            .get(&handle.get_raw_handle())
            .unwrap_or_else(|| panic!("handle not found"))
    }

    pub fn get_mut(&mut self, handle: H) -> &mut V {
        // TODO: consider simulating the actual error from the VM
        self.map
            .get_mut(&handle.get_raw_handle())
            .unwrap_or_else(|| panic!("handle not found"))
    }

    pub fn insert(&mut self, handle: H, value: V) {
        let _ = self.map.insert(handle.get_raw_handle(), value);
    }
}

#[derive(Debug)]
pub struct TxManagedTypes {
    pub(crate) big_int_map: HandleMap<<Self as HandleTypeInfo>::BigIntHandle, BigInt>,
    pub(crate) big_float_map: HandleMap<<Self as HandleTypeInfo>::BigFloatHandle, f64>,
    pub(crate) managed_buffer_map:
        HandleMap<<Self as HandleTypeInfo>::ManagedBufferHandle, ManagedBufferImpl>,
}

impl HandleTypeInfo for TxManagedTypes {
    type ManagedBufferHandle = i32;

    type BigIntHandle = i32;

    type BigFloatHandle = i32;

    type EllipticCurveHandle = i32;
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
