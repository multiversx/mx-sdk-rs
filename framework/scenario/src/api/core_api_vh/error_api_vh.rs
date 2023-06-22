use multiversx_chain_vm::mem_conv;
use multiversx_sc::api::{ErrorApi, ErrorApiImpl, HandleConstraints};

use crate::api::{VMHooksApi, VMHooksApiBackend};

impl<VHB: VMHooksApiBackend> ErrorApi for VMHooksApi<VHB> {
    type ErrorApiImpl = Self;

    fn error_api_impl() -> Self::ErrorApiImpl {
        Self::api_impl()
    }
}

impl<VHB: VMHooksApiBackend> ErrorApiImpl for VMHooksApi<VHB> {
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
        self.assert_live_handle(&message_handle);
        self.with_vm_hooks(|vh| vh.managed_signal_error(message_handle.get_raw_handle()));

        // even though not explicitly stated in the VM hooks definition,
        // `managed_signal_error` is expected to terminate execution
        unreachable!()
    }
}
