use crate::api::ErrorApi;

impl ErrorApi for super::UncallableApi {
	fn signal_error(&self, _message: &[u8]) -> ! {
		unreachable!()
	}
}
