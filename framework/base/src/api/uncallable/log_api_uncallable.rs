use crate::api::{LogApi, LogApiImpl};

use super::UncallableApi;

impl LogApi for UncallableApi {
    type LogApiImpl = UncallableApi;

    fn log_api_impl() -> Self::LogApiImpl {
        unreachable!()
    }
}

impl LogApiImpl for UncallableApi {
    fn managed_write_log(
        &self,
        _topics_handle: Self::ManagedBufferHandle,
        _data_handle: Self::ManagedBufferHandle,
    ) {
        unreachable!()
    }
}
