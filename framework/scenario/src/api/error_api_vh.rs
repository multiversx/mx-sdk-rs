use multiversx_sc::api::{ErrorApi, ErrorApiImpl};

use super::{VMHooksApi, VMHooksBackend, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> ErrorApi for VMHooksApi<BACKEND_TYPE> {
    type ErrorApiImpl = VMHooksBackend;

    fn error_api_impl() -> Self::ErrorApiImpl {
        todo!()
    }
}

impl ErrorApi for VMHooksBackend {
    type ErrorApiImpl = Self;

    fn error_api_impl() -> Self::ErrorApiImpl {
        panic!("TODO: bad dependency, sort it out!")
    }
}

impl ErrorApiImpl for VMHooksBackend {
    fn signal_error(&self, _message: &[u8]) -> ! {
        todo!()
    }

    fn signal_error_from_buffer(&self, _message_handle: Self::ManagedBufferHandle) -> ! {
        todo!()
    }
}
