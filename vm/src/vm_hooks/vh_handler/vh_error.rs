use crate::{types::RawHandle, vm_hooks::VMHooksHandlerSource};

use super::VMHooksManagedTypes;

pub trait VMHooksError: VMHooksHandlerSource {
    fn signal_error(&self, message: &[u8]) -> ! {
        // can sometimes help in tests
        // run `clear & cargo test -- --nocapture` to see the output
        println!("{}", std::str::from_utf8(message).unwrap());

        self.halt_with_error(4, std::str::from_utf8(message).unwrap())
    }
}

pub trait VMHooksErrorManaged: VMHooksManagedTypes + VMHooksError {
    fn signal_error_from_buffer(&self, message_handle: RawHandle) -> ! {
        self.signal_error(self.m_types_lock().mb_get(message_handle))
    }
}
