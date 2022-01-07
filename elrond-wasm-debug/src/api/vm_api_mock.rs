use elrond_wasm::{
    abi::EndpointLocationAbi,
    api::{CallTypeApi, StorageMapperApi, VMApi},
};

use crate::DebugApi;

impl CallTypeApi for DebugApi {}

impl StorageMapperApi for DebugApi {}

impl VMApi for DebugApi {
    fn has_location(location: EndpointLocationAbi) -> bool {
        location == EndpointLocationAbi::MainContract
    }
}
