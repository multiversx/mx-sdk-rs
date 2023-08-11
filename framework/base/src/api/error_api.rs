use super::HandleTypeInfo;

pub trait ErrorApi: HandleTypeInfo {
    type ErrorApiImpl: ErrorApiImpl
        + HandleTypeInfo<
            ManagedBufferHandle = Self::ManagedBufferHandle,
            BigIntHandle = Self::BigIntHandle,
            BigFloatHandle = Self::BigFloatHandle,
            EllipticCurveHandle = Self::EllipticCurveHandle,
        >;

    fn error_api_impl() -> Self::ErrorApiImpl;
}

pub trait ErrorApiImpl: HandleTypeInfo {
    fn signal_error(&self, message: &[u8]) -> !;

    fn signal_error_from_buffer(&self, message_handle: Self::ManagedBufferHandle) -> !;
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

    fn signal_error_from_buffer(&self, _message_handle: Self::ManagedBufferHandle) -> ! {
        panic!("PanickingErrorApi panicked via signal_error_from_buffer")
    }
}

impl HandleTypeInfo for PanickingErrorApiImpl {
    type ManagedBufferHandle = i32;

    type BigIntHandle = i32;

    type BigFloatHandle = i32;

    type EllipticCurveHandle = i32;

    type ManagedMapHandle = i32;
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

impl HandleTypeInfo for PanickingErrorApi {
    type ManagedBufferHandle = i32;

    type BigIntHandle = i32;

    type BigFloatHandle = i32;

    type EllipticCurveHandle = i32;

    type ManagedMapHandle = i32;
}
