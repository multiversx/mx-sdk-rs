use multiversx_sc::api::{HandleConstraints, LogApi, LogApiImpl};

use crate::api::{VMHooksApi, VMHooksApiBackend};

impl<VHB: VMHooksApiBackend> LogApi for VMHooksApi<VHB> {
    type LogApiImpl = Self;

    fn log_api_impl() -> Self::LogApiImpl {
        Self::api_impl()
    }
}

impl<VHB: VMHooksApiBackend> LogApiImpl for VMHooksApi<VHB> {
    fn managed_write_log(
        &self,
        topics_handle: Self::ManagedBufferHandle,
        data_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks(|vh| {
            vh.managed_write_log(
                topics_handle.get_raw_handle_unchecked(),
                data_handle.get_raw_handle_unchecked(),
            )
        });
    }
}
