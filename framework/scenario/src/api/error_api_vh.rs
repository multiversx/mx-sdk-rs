use multiversx_sc::api::{ErrorApi, ErrorApiImpl};

use super::{VMHooksApi, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> ErrorApi for VMHooksApi<BACKEND_TYPE> {
    type ErrorApiImpl = Self;

    fn error_api_impl() -> Self::ErrorApiImpl {
        Self::api_impl()
    }
}

impl<const BACKEND_TYPE: VMHooksBackendType> ErrorApiImpl for VMHooksApi<BACKEND_TYPE> {
    fn signal_error(&self, _message: &[u8]) -> ! {
        todo!()
    }

    fn signal_error_from_buffer(&self, _message_handle: Self::ManagedBufferHandle) -> ! {
        todo!()
    }
}
