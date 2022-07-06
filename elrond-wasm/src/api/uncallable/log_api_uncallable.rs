use crate::{
    api::{LogApi, LogApiImpl},
    types::heap::ArgBuffer,
};

use super::UncallableApi;

impl LogApi for UncallableApi {
    type LogApiImpl = UncallableApi;

    fn log_api_impl() -> Self::LogApiImpl {
        unreachable!()
    }
}

impl LogApiImpl for UncallableApi {
    fn write_event_log(&self, _topics_buffer: &ArgBuffer, _data: &[u8]) {
        unreachable!()
    }

    fn write_legacy_log(&self, _topics: &[[u8; 32]], _data: &[u8]) {
        unreachable!()
    }

    fn managed_write_log(
        &self,
        _topics_handle: Self::ManagedBufferHandle,
        _data_handle: Self::ManagedBufferHandle,
    ) {
        unreachable!()
    }
}
