use multiversx_sc::api::{LogApi, LogApiImpl};

use super::{VMHooksApi, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> LogApi for VMHooksApi<BACKEND_TYPE> {
    type LogApiImpl = Self;

    fn log_api_impl() -> Self::LogApiImpl {
        Self::api_impl()
    }
}

impl<const BACKEND_TYPE: VMHooksBackendType> LogApiImpl for VMHooksApi<BACKEND_TYPE> {
    fn managed_write_log(
        &self,
        topics_handle: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks(|vh| vh.managed_write_log(topics_handle, data_handle));
    }
}
