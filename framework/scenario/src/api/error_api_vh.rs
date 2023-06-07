use multiversx_sc::api::{ErrorApi, ErrorApiImpl};

use super::{VMHooksApi, VMHooksApiImpl, VMHooksBackendType};

impl<const BACKEND_TYPE: VMHooksBackendType> ErrorApi for VMHooksApi<BACKEND_TYPE> {
    type ErrorApiImpl = VMHooksApiImpl;

    fn error_api_impl() -> Self::ErrorApiImpl {
        todo!()
    }
}

impl ErrorApi for VMHooksApiImpl {
    type ErrorApiImpl = Self;

    fn error_api_impl() -> Self::ErrorApiImpl {
        panic!("TODO: bad dependency, sort it out!")
    }
}

impl ErrorApiImpl for VMHooksApiImpl {
    fn signal_error(&self, _message: &[u8]) -> ! {
        todo!()
    }

    fn signal_error_from_buffer(&self, _message_handle: Self::ManagedBufferHandle) -> ! {
        todo!()
    }
}
