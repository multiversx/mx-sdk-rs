use multiversx_chain_vm::mem_conv;
use multiversx_sc::api::{ErrorApi, ErrorApiImpl};

use super::{VMHooksApi, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> ErrorApi for VMHooksApi<BACKEND_TYPE> {
    type ErrorApiImpl = Self;

    fn error_api_impl() -> Self::ErrorApiImpl {
        Self::api_impl()
    }
}

impl<const BACKEND_TYPE: VMHooksBackendType> ErrorApiImpl for VMHooksApi<BACKEND_TYPE> {
    fn signal_error(&self, message: &[u8]) -> ! {
        self.with_vm_hooks(|vh| {
            mem_conv::with_mem_ptr(message, |offset, length| {
                vh.signal_error(offset, length);
            })
        });

        // even though not explicitly stated in the VM hooks definition,
        // `signal_error` is expected to terminate execution
        unreachable!()
    }

    fn signal_error_from_buffer(&self, message_handle: Self::ManagedBufferHandle) -> ! {
        self.with_vm_hooks(|vh| vh.managed_signal_error(message_handle));

        // even though not explicitly stated in the VM hooks definition,
        // `managed_signal_error` is expected to terminate execution
        unreachable!()
    }
}
