use crate::ext_error;

pub struct NodeErrorApi;

impl ErrorApi for NodeErrorApi {
	#[inline]
	fn signal_error(&self, message: &[u8]) -> ! {
		ext_error::signal_error(message)
	}
}
