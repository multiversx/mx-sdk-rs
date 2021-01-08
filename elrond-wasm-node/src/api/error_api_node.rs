use crate::ext_error;
use crate::ArwenApiImpl;
use elrond_wasm::api::ErrorApi;

impl ErrorApi for ArwenApiImpl {
	#[inline]
	fn signal_error(&self, message: &[u8]) -> ! {
		ext_error::signal_error(message)
	}
}
