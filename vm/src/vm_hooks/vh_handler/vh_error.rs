use multiversx_sc::api::RawHandle;

use crate::tx_mock::TxPanic;

use super::VMHooksManagedTypes;

pub trait VMHooksError {
    fn signal_vm_error(&self, message: &str) -> ! {
        // can sometimes help in tests
        // run `clear & cargo test -- --nocapture` to see the output
        println!("{message}");

        std::panic::panic_any(TxPanic {
            status: 10,
            message: message.to_string(),
        })
    }

    fn signal_error(&self, message: &[u8]) -> ! {
        // can sometimes help in tests
        // run `clear & cargo test -- --nocapture` to see the output
        println!("{}", std::str::from_utf8(message).unwrap());

        std::panic::panic_any(TxPanic {
            status: 4,
            message: String::from_utf8(message.to_vec()).unwrap(),
        })
    }
}

pub trait VMHooksErrorManaged: VMHooksManagedTypes + VMHooksError {
    fn signal_error_from_buffer(&self, message_handle: RawHandle) -> ! {
        self.signal_error(self.m_types_borrow().mb_get(message_handle))
    }
}
