use multiversx_sc::api::{uncallable::UncallableApi, EndpointArgumentApi};

use super::{VMHooksApi, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> EndpointArgumentApi for VMHooksApi<BACKEND_TYPE> {
    type EndpointArgumentApiImpl = UncallableApi;

    fn argument_api_impl() -> Self::EndpointArgumentApiImpl {
        unreachable!()
    }
}
