use super::{ErrorApiImpl, Handle, PanickingErrorApiImpl};

pub trait ErrorApi {
    type ErrorApiImpl: ErrorApiImpl;

    fn error_api_impl() -> Self::ErrorApiImpl;
}

/// An error handler that simply panics whenever `signal_error` is called.
/// Especially useful for unit tests.
/// Implements `ErrorApi`.
pub struct PanickingErrorApi;

impl ErrorApi for PanickingErrorApi {
    type ErrorApiImpl = PanickingErrorApiImpl;

    fn error_api_impl() -> Self::ErrorApiImpl {
        PanickingErrorApiImpl
    }
}
