use crate::{
    abi::EndpointLocationAbi,
    api::{CallTypeApi, StorageMapperApi, VMApi},
};

use super::UncallableApi;

impl CallTypeApi for UncallableApi {}

impl StorageMapperApi for UncallableApi {}

impl VMApi for UncallableApi {
    fn has_location(_location: EndpointLocationAbi) -> bool {
        unreachable!()
    }
}
