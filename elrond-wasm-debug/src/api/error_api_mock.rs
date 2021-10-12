use crate::{tx_mock::TxPanic, DebugApi};
use elrond_wasm::api::{ErrorApi, Handle, ManagedBufferApi};

impl ErrorApi for DebugApi {
    fn signal_error(&self, message: &[u8]) -> ! {
        // can sometimes help in tests
        // run `clear & cargo test -- --nocapture` to see the output
        println!("{}", std::str::from_utf8(message).unwrap());

        std::panic::panic_any(TxPanic {
            status: 4,
            message: message.to_vec(),
        })
    }

    fn signal_error_from_buffer(&self, message_handle: Handle) -> ! {
        let message = self.mb_to_boxed_bytes(message_handle);
        self.signal_error(message.as_slice())
    }
}
