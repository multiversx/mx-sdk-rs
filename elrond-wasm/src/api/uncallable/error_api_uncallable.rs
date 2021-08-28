use crate::api::{ErrorApi, Handle};

impl ErrorApi for super::UncallableApi {
    fn signal_error(&self, _message: &[u8]) -> ! {
        unreachable!()
    }

    fn signal_error_from_buffer(&self, _message_handle: Handle) -> ! {
        unreachable!()
    }
}
