use crate::{error_hook, VmApiImpl};
use elrond_wasm::api::{ErrorApi, ErrorApiImpl, Handle};

extern "C" {
    fn managedSignalError(messageHandle: i32) -> !;
}

impl ErrorApi for VmApiImpl {
    type ErrorApiImpl = VmApiImpl;

    #[inline]
    fn error_api_impl() -> Self {
        VmApiImpl {}
    }
}

impl ErrorApiImpl for VmApiImpl {
    #[inline(always)]
    fn signal_error(&self, message: &[u8]) -> ! {
        error_hook::signal_error(message)
    }

    #[inline(always)]
    fn signal_error_from_buffer(&self, message_handle: Handle) -> ! {
        unsafe { managedSignalError(message_handle) }
    }
}
