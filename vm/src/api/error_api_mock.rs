use crate::{tx_mock::TxPanic, DebugApi};
use multiversx_sc::api::{ErrorApi, ErrorApiImpl, ManagedBufferApi};

impl ErrorApi for DebugApi {
    type ErrorApiImpl = DebugApi;

    fn error_api_impl() -> Self {
        DebugApi::new_from_static()
    }
}

impl ErrorApiImpl for DebugApi {
    fn signal_error(&self, message: &[u8]) -> ! {
        // can sometimes help in tests
        // run `clear & cargo test -- --nocapture` to see the output
        println!("{}", std::str::from_utf8(message).unwrap());

        std::panic::panic_any(TxPanic {
            status: 4,
            message: String::from_utf8(message.to_vec()).unwrap(),
        })
    }

    fn signal_error_from_buffer(&self, message_handle: Self::ManagedBufferHandle) -> ! {
        let message = self.mb_to_boxed_bytes(message_handle);
        self.signal_error(message.as_slice())
    }
}
