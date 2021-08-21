use crate::{TxContext, TxPanic};
use elrond_wasm::api::ErrorApi;

impl ErrorApi for TxContext {
    fn signal_error(&self, message: &[u8]) -> ! {
        // can sometimes help in tests
        // run `clear & cargo test -- --nocapture` to see the output
        println!("{}", std::str::from_utf8(message).unwrap());

        std::panic::panic_any(TxPanic {
            status: 4,
            message: message.to_vec(),
        })
    }
}
