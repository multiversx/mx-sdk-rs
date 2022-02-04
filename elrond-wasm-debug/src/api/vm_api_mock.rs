use elrond_wasm::api::{CallTypeApi, StorageMapperApi, VMApi};

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
