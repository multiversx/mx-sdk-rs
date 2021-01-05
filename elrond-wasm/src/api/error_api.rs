pub trait ErrorApi {
	fn signal_error(&self, message: &[u8]) -> !;
}
