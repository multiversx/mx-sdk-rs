use super::VmApiImpl;
use multiversx_sc::api::{LogApi, LogApiImpl};

extern "C" {
    fn managedWriteLog(topicsHandle: i32, dataHandle: i32);
}

impl LogApi for VmApiImpl {
    type LogApiImpl = VmApiImpl;

    #[inline]
    fn log_api_impl() -> Self::LogApiImpl {
        VmApiImpl {}
    }
}

impl LogApiImpl for VmApiImpl {
    fn managed_write_log(
        &self,
        topics_handle: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        unsafe {
            managedWriteLog(topics_handle, data_handle);
        }
    }
}
