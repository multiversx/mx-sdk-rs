use crate::{error_hook, VmApiImpl};
use elrond_wasm::api::{ErrorApi, Handle};

extern "C" {
    #[cfg(not(feature = "unmanaged-ei"))]
    fn managedSignalError(messageHandle: i32) -> !;
}

impl ErrorApi for VmApiImpl {
    #[inline(always)]
    fn signal_error(&self, message: &[u8]) -> ! {
        error_hook::signal_error(message)
    }

    #[cfg(feature = "unmanaged-ei")]
    fn signal_error_from_buffer(&self, message_handle: Handle) -> ! {
        use elrond_wasm::api::ManagedBufferApi;
        let message = self.mb_to_boxed_bytes(message_handle);
        self.signal_error(message.as_slice())
    }

    #[inline(always)]
    #[cfg(not(feature = "unmanaged-ei"))]
    fn signal_error_from_buffer(&self, message_handle: Handle) -> ! {
        unsafe { managedSignalError(message_handle) }
    }
}
