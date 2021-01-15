pub trait ErrorApi {
	fn signal_error(&self, message: &[u8]) -> !;
}

/// An error handler that simply panics whenever `signal_error` is called.
/// Especially useful for unit tests.
/// Implements `ErrorApi`.
pub struct PanickingErrorApi;

impl ErrorApi for PanickingErrorApi {
	fn signal_error(&self, message: &[u8]) -> ! {
		panic!(
			"PanickingErrorApi panicked: {}",
			core::str::from_utf8(message).unwrap()
		)
	}
}
