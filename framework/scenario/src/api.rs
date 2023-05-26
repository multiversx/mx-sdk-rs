mod blockchain_api_vh;
mod call_value_api_vh;
mod crypto_api_vh;
mod endpoint_arg_api_vh;
mod endpoint_finish_api_vh;
mod error_api_vh;
mod log_api_vh;
mod managed_type_api_vh;
mod print_api_vh;
mod send_api_vh;
mod storage_api_vh;

use std::{ops::Deref, thread::LocalKey};

use multiversx_chain_vm::{executor::VMHooks, vm_hooks::VMHooksDispatcher};
use multiversx_sc::api::{HandleTypeInfo, RawHandle};

#[derive(Clone)]
pub struct StaticApi;

impl HandleTypeInfo for StaticApi {
    type ManagedBufferHandle = RawHandle;
    type BigIntHandle = RawHandle;
    type BigFloatHandle = RawHandle;
    type EllipticCurveHandle = RawHandle;
    type ManagedMapHandle = RawHandle;
}

thread_local!(
    static MANAGED_TYPES_CELL: Box<dyn VMHooks> = Box::new(VMHooksDispatcher::new_managed_type_cell())
);

pub struct VMHooksBackend {
    pub vh_local: &'static LocalKey<Box<dyn VMHooks>>,
}

impl VMHooksBackend {
    pub fn static_managed_type_backend() -> Self {
        VMHooksBackend {
            vh_local: &MANAGED_TYPES_CELL,
        }
    }

    pub fn with_vm_hooks<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&dyn VMHooks) -> R,
    {
        self.vh_local.with(|vh| f(vh.deref()))
    }
}

impl HandleTypeInfo for VMHooksBackend {
    type ManagedBufferHandle = RawHandle;
    type BigIntHandle = RawHandle;
    type BigFloatHandle = RawHandle;
    type EllipticCurveHandle = RawHandle;
    type ManagedMapHandle = RawHandle;
}
