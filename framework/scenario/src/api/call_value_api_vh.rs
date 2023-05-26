use multiversx_sc::api::{uncallable::UncallableApi, CallValueApi};

use super::StaticApi;

impl CallValueApi for StaticApi {
    type CallValueApiImpl = UncallableApi;

    fn call_value_api_impl() -> Self::CallValueApiImpl {
        unreachable!()
    }
}
