use elrond_wasm::api::{CallTypeApi, HandleTypeInfo, StorageMapperApi, VMApi};

use crate::DebugApi;

impl CallTypeApi for DebugApi {}

impl StorageMapperApi for DebugApi {}

impl PartialEq for DebugApi {
    fn eq(&self, _: &Self) -> bool {
        panic!("Comparing DebugApi/TxContextRef not allowed.")
    }
}

impl Eq for DebugApi {}

impl VMApi for DebugApi {}

type DebugHandle = i32;

impl HandleTypeInfo for DebugApi {
    type ManagedBufferHandle = DebugHandle;

    type BigIntHandle = DebugHandle;

    type BigFloatHandle = DebugHandle;

    type EllipticCurveHandle = DebugHandle;
}
