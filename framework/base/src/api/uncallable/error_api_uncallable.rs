use crate::api::{ErrorApi, ErrorApiImpl};

impl ErrorApi for super::UncallableApi {
    type ErrorApiImpl = super::UncallableApi;

    fn error_api_impl() -> Self::ErrorApiImpl {
        super::UncallableApi
    }
}

impl ErrorApiImpl for super::UncallableApi {
    fn signal_error(&self, _message: &[u8]) -> ! {
        unreachable!()
    }

    fn signal_error_from_buffer(&self, _message_handle: Self::ManagedBufferHandle) -> ! {
        unreachable!()
    }
}
