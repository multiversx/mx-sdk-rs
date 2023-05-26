use multiversx_sc::api::{uncallable::UncallableApi, EndpointFinishApi};

use super::StaticApi;

impl EndpointFinishApi for StaticApi {
    type EndpointFinishApiImpl = UncallableApi;

    fn finish_api_impl() -> Self::EndpointFinishApiImpl {
        unreachable!()
    }
}
