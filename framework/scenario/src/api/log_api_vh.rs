use multiversx_sc::api::{uncallable::UncallableApi, LogApi};

use super::StaticApi;

impl LogApi for StaticApi {
    type LogApiImpl = UncallableApi;

    fn log_api_impl() -> Self::LogApiImpl {
        unreachable!()
    }
}
