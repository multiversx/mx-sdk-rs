use crate::{TxContext, TxPanic};
use elrond_wasm::api::ErrorApi;

impl ErrorApi for TxContext {
	fn signal_error(&self, message: &[u8]) -> ! {
		std::panic::panic_any(TxPanic {
			status: 4,
			message: message.to_vec(),
		})
	}
}
