use multiversx_sc::api::{uncallable::UncallableApi, SendApi};

use super::StaticApi;

impl SendApi for StaticApi {
    type SendApiImpl = UncallableApi;

    fn send_api_impl() -> Self::SendApiImpl {
        unreachable!()
    }
}
