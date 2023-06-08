use multiversx_sc::api::{uncallable::UncallableApi, LogApi};

use super::{VMHooksApi, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> LogApi for VMHooksApi<BACKEND_TYPE> {
    type LogApiImpl = UncallableApi;

    fn log_api_impl() -> Self::LogApiImpl {
        unreachable!()
    }
}
