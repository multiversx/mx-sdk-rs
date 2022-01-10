use elrond_wasm::api::{CallTypeApi, StorageMapperApi, VMApi};

use crate::DebugApi;

impl CallTypeApi for DebugApi {}

impl StorageMapperApi for DebugApi {}

impl VMApi for DebugApi {}
