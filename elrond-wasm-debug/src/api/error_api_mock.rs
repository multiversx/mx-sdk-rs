use crate::{TxContext, TxPanic};
use alloc::vec::Vec;
use elrond_wasm::api::ErrorApi;

impl ErrorApi for TxContext {
	fn signal_error(&self, message: &[u8]) -> ! {
		panic!(TxPanic {
			status: 4,
			message: message.to_vec()
		})
	}
}
