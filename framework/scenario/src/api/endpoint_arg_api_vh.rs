use multiversx_sc::api::{uncallable::UncallableApi, EndpointArgumentApi};

use super::StaticApi;

impl EndpointArgumentApi for StaticApi {
    type EndpointArgumentApiImpl = UncallableApi;

    fn argument_api_impl() -> Self::EndpointArgumentApiImpl {
        unreachable!()
    }
}
