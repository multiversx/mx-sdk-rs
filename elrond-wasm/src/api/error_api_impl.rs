use super::Handle;

pub trait ErrorApiImpl {
    fn signal_error(&self, message: &[u8]) -> !;

    fn signal_error_from_buffer(&self, message_handle: Handle) -> !;
}

/// An error handler that simply panics whenever `signal_error` is called.
/// Especially useful for unit tests.
/// Implements `ErrorApi`.
pub struct PanickingErrorApiImpl;

impl ErrorApiImpl for PanickingErrorApiImpl {
    fn signal_error(&self, message: &[u8]) -> ! {
        panic!(
            "PanickingErrorApi panicked: {}",
            core::str::from_utf8(message).unwrap()
        )
    }

    fn signal_error_from_buffer(&self, _message_handle: Handle) -> ! {
        panic!("PanickingErrorApi panicked via signal_error_from_buffer")
    }
}
