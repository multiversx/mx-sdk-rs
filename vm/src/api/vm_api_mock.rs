use multiversx_sc::api::{CallTypeApi, HandleTypeInfo, StorageMapperApi, VMApi};

use crate::DebugApi;

use super::debug_handle_mock::DebugHandle;

impl CallTypeApi for DebugApi {}

impl StorageMapperApi for DebugApi {}

impl PartialEq for DebugApi {
    fn eq(&self, _: &Self) -> bool {
        panic!("Comparing DebugApi/TxContextRef not allowed.")
    }
}

impl Eq for DebugApi {}

impl VMApi for DebugApi {}

impl HandleTypeInfo for DebugApi {
    type ManagedBufferHandle = DebugHandle;

    type BigIntHandle = DebugHandle;

    type BigFloatHandle = DebugHandle;

    type EllipticCurveHandle = DebugHandle;
}
