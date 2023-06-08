use multiversx_sc::api::{uncallable::UncallableApi, EndpointFinishApi};

use super::{VMHooksApi, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> EndpointFinishApi for VMHooksApi<BACKEND_TYPE> {
    type EndpointFinishApiImpl = UncallableApi;

    fn finish_api_impl() -> Self::EndpointFinishApiImpl {
        unreachable!()
    }
}
